//! Text preprocessing for the `:probabilistic` HDDL effect extension.
//!
//! Mirrors koala-planner's own probabilistic-effect syntax extension
//! (`serializer/prob_preprocessor.py` in that project): a
//! `(:probabilistic w1 e1 w2 e2 ...)` effect block is pure syntactic sugar
//! over a standard `(oneof e1 e2 ...)` block, plus a separately-tracked list
//! of weights per outcome. That project implements this as a pure TEXT
//! preprocessing pass run before real HDDL parsing (find matching-paren
//! blocks, read alternating weight/effect tokens, rewrite, hand the weight
//! map back to the caller); this module reimplements that same shape in
//! Rust so `parser::parse_domain` can call it before tokenizing and
//! transparently accept `:probabilistic` blocks with zero grammar changes to
//! the recursive-descent parser itself.
//!
//! Weights are kept as raw source text here, not parsed to `f64` -- the same
//! "preserve source text, parse only where it's used" discipline
//! `ast::NumericValue::Number(String)` already follows elsewhere in this
//! crate, so no float `Eq`/`PartialEq` concerns leak into `ast::ActionDef`
//! (see `probability_weights` there). Weights are tracked per *enclosing
//! `:action` name*, not by structural position, matching koala-planner's own
//! `prob_map` shape exactly -- this is also exactly the scope the grounder
//! can actually use a `oneof` in: at the top of a single action's `:effect`
//! (see `ast`'s module docs and `grounder::collect_effect`'s refusal of a
//! nested `oneof`), so there is at most one functional weight list per
//! action to track.

use std::collections::BTreeMap;

/// Find the index (into `chars`) of the closing paren matching the opening
/// paren at `chars[start]`. `None` on unbalanced parentheses.
fn find_matching_paren(chars: &[char], start: usize) -> Option<usize> {
    if chars.get(start) != Some(&'(') {
        return None;
    }
    let mut depth = 0i32;
    for (i, &c) in chars.iter().enumerate().skip(start) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Parse a `(:probabilistic w1 e1 w2 e2 ...)` block (the whole block, outer
/// parens included) into its alternating `(weights, effect-sexp-text)`
/// pairs, both in declaration order. `None` on any malformed shape (missing
/// a weight token, missing/unbalanced an effect s-expression, or an empty
/// body) -- the caller leaves such a block untouched so the real parser
/// reports it as a normal syntax error rather than this pass silently
/// dropping or mis-rewriting it.
fn parse_probabilistic_block(chars: &[char]) -> Option<(Vec<String>, Vec<String>)> {
    let inner_end = chars.len().checked_sub(1)?; // index of the closing ')'
    if chars.first() != Some(&'(') || chars.get(inner_end) != Some(&')') {
        return None;
    }
    let mut i = 1;
    while i < inner_end && chars[i].is_whitespace() {
        i += 1;
    }
    // Consume the ":probabilistic" keyword token itself.
    while i < inner_end && !chars[i].is_whitespace() && chars[i] != '(' {
        i += 1;
    }

    let mut weights = Vec::new();
    let mut effects = Vec::new();
    loop {
        while i < inner_end && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= inner_end {
            break;
        }
        let w_start = i;
        while i < inner_end && !chars[i].is_whitespace() && chars[i] != '(' {
            i += 1;
        }
        if i == w_start {
            return None; // expected a weight token here
        }
        weights.push(chars[w_start..i].iter().collect::<String>());
        while i < inner_end && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= inner_end || chars[i] != '(' {
            return None; // expected an effect s-expression after the weight
        }
        let end = find_matching_paren(chars, i)?;
        if end > inner_end {
            return None; // effect's own parens run past the block's own close
        }
        effects.push(chars[i..=end].iter().collect::<String>());
        i = end + 1;
    }
    if weights.is_empty() || weights.len() != effects.len() {
        return None;
    }
    Some((weights, effects))
}

/// Search backward from `pos` (a `:probabilistic` block's own opening-paren
/// index) for the nearest enclosing `(:action <name> ...)` and return
/// `<name>`, matching koala-planner's `_find_action_name`. `None` when no
/// `:action` keyword precedes `pos` at all -- the block is still rewritten
/// by the caller, it just contributes no entry to the returned weight map.
fn find_action_name(chars: &[char], pos: usize) -> Option<String> {
    let needle: Vec<char> = ":action".chars().collect();
    let mut last_match = None;
    let mut i = 0usize;
    while i + needle.len() <= pos {
        if chars[i..i + needle.len()] == needle[..] {
            last_match = Some(i);
        }
        i += 1;
    }
    let idx = last_match?;
    let mut j = idx + needle.len();
    while j < pos && chars[j].is_whitespace() {
        j += 1;
    }
    let name_start = j;
    while j < pos && !chars[j].is_whitespace() && chars[j] != '(' && chars[j] != ')' {
        j += 1;
    }
    if j == name_start {
        return None;
    }
    Some(chars[name_start..j].iter().collect())
}

/// Rewrite every `(:probabilistic w1 e1 w2 e2 ...)` block in `src` into a
/// standard `(oneof e1 e2 ...)` block, returning the cleaned text alongside
/// a map from enclosing `:action` name to that action's declared weight
/// list (raw source text, in declaration order). A `:probabilistic` block
/// found outside any `:action` section is still rewritten to `oneof` (so the
/// text stays well-formed for the real parser) but contributes no entry to
/// the returned map. A malformed block (see `parse_probabilistic_block`) is
/// left completely untouched, so the real tokenizer/parser reports it as an
/// ordinary syntax error at the correct location instead of this pass
/// silently mis-rewriting it.
pub fn preprocess(src: &str) -> (String, BTreeMap<String, Vec<String>>) {
    let chars: Vec<char> = src.chars().collect();
    let needle: Vec<char> = ":probabilistic".chars().collect();

    // Find every `(:probabilistic` block start, left to right.
    let mut block_starts = Vec::new();
    let mut i = 0usize;
    while i + needle.len() <= chars.len() {
        if chars[i..i + needle.len()] == needle[..] {
            // Must immediately follow an opening paren (only whitespace, if
            // any, in between) to count as a real effect-keyword head rather
            // than e.g. a substring appearing inside an unrelated token.
            let mut open = i;
            while open > 0 && chars[open - 1].is_whitespace() {
                open -= 1;
            }
            if open > 0 && chars[open - 1] == '(' {
                block_starts.push(open - 1);
            }
            i += needle.len();
        } else {
            i += 1;
        }
    }

    let mut weight_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // (block start, block end inclusive, replacement text)
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();

    for &open in &block_starts {
        let Some(close) = find_matching_paren(&chars, open) else {
            continue;
        };
        let Some((weights, effects)) = parse_probabilistic_block(&chars[open..=close]) else {
            continue;
        };
        if let Some(action_name) = find_action_name(&chars, open) {
            weight_map.insert(action_name, weights);
        }
        replacements.push((open, close, format!("(oneof {})", effects.join(" "))));
    }

    // Apply right-to-left so earlier character offsets stay valid as later
    // (leftward) replacements are spliced in.
    replacements.sort_by_key(|r| std::cmp::Reverse(r.0));
    let mut out: Vec<char> = chars;
    for (open, close, replacement) in replacements {
        out.splice(open..=close, replacement.chars());
    }
    (out.into_iter().collect(), weight_map)
}

/// Whether `src` contains a `:probabilistic` effect block at all -- a cheap
/// pre-check callers can use to skip the full `preprocess` pass on domains
/// that don't use the extension.
pub fn has_probabilistic(src: &str) -> bool {
    src.contains(":probabilistic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_probabilistic_block_to_oneof_and_extracts_weights() {
        let src = "(define (domain d)\n  (:action toss\n    :parameters ()\n    :precondition ()\n    :effect (:probabilistic 0.8 (on) 0.2 (off))))";
        let (cleaned, weights) = preprocess(src);
        assert!(!cleaned.contains(":probabilistic"));
        assert!(cleaned.contains("(oneof (on) (off))"));
        assert_eq!(
            weights.get("toss"),
            Some(&vec!["0.8".to_owned(), "0.2".to_owned()])
        );
    }

    #[test]
    fn handles_three_way_probabilistic_split_with_compound_effects() {
        let src = "(:action a :effect (:probabilistic 1 (and (p) (q)) 2 (not (p)) 1 (r)))";
        let (cleaned, weights) = preprocess(src);
        assert_eq!(
            cleaned,
            "(:action a :effect (oneof (and (p) (q)) (not (p)) (r)))"
        );
        assert_eq!(
            weights.get("a"),
            Some(&vec!["1".to_owned(), "2".to_owned(), "1".to_owned()])
        );
    }

    #[test]
    fn leaves_plain_oneof_untouched() {
        let src = "(:effect (oneof (p) (q)))";
        let (cleaned, weights) = preprocess(src);
        assert_eq!(cleaned, src);
        assert!(weights.is_empty());
    }

    #[test]
    fn probabilistic_block_outside_any_action_is_rewritten_with_no_weight_entry() {
        let src = "(:probabilistic 1 (p) 1 (q))";
        let (cleaned, weights) = preprocess(src);
        assert_eq!(cleaned, "(oneof (p) (q))");
        assert!(weights.is_empty());
    }

    #[test]
    fn malformed_block_missing_a_weight_is_left_untouched() {
        let src = "(:action a :effect (:probabilistic (p) 1 (q)))";
        let (cleaned, weights) = preprocess(src);
        assert_eq!(cleaned, src);
        assert!(weights.is_empty());
    }

    #[test]
    fn has_probabilistic_detects_the_keyword() {
        assert!(has_probabilistic("(:effect (:probabilistic 1 (p) 1 (q)))"));
        assert!(!has_probabilistic("(:effect (oneof (p) (q)))"));
    }
}
