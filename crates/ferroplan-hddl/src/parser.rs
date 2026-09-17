//! HDDL text -> AST: a tokenizer + generic S-expression reader, then a
//! recursive-descent translation into `crate::ast` types.

use crate::ast::*;
use std::collections::BTreeMap;
use std::fmt;

/// An error produced by `parse_domain`/`parse_problem`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// A malformed s-expression or an unexpected/missing token — the message
    /// carries a human-readable description of what was expected.
    Syntax(String),
    /// A syntactically well-formed construct this crate deliberately does not
    /// parse (see `ast`'s module docs for scope), e.g. `:functions` or a
    /// `:durative-action` domain section (temporal/durative actions are a
    /// permanent non-goal for this grounder/translate pipeline — see the
    /// "Deliberately NOT represented at all" note in `ast`'s module docs:
    /// the whole state representation here is an instantaneous ground-fact
    /// set with no time dimension, so refusing loudly at parse time is the
    /// correct place to stop rather than accepting `:duration` and
    /// `:condition`/`#t`-style timed conditions and silently ignoring them).
    UnsupportedConstruct(String),
    /// A `(oneof ...)` construct violating the koala-planner reference
    /// position/branch rules: `oneof` is legal ONLY as the entire top-level
    /// `:effect` of an action (`:effect (oneof e1 ... ek)`) — never nested
    /// under `and`/`when`/another `oneof`, and never in a
    /// precondition/method-condition/`:goal` position; a branch may not
    /// itself be a `when` (koala parses that shape but silently drops it
    /// downstream — see the `when` arm of `parse_effect`); and k >= 1 (a
    /// bare `(oneof)` with no branches).
    MalformedOneof(String),
    /// A `(:probabilistic ...)` block was found textually nested inside
    /// another `:probabilistic` block's captured effect text — refused by
    /// `probabilistic::preprocess` before it ever reaches this parser's
    /// tokenizer (see that function's doc comment for why nesting can't be
    /// rewritten safely).
    NestedProbabilisticBlock(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(msg) => write!(f, "syntax error: {msg}"),
            Self::UnsupportedConstruct(what) => write!(f, "unsupported construct: {what}"),
            Self::MalformedOneof(msg) => write!(f, "malformed oneof: {msg}"),
            Self::NestedProbabilisticBlock(msg) => {
                write!(f, "nested ':probabilistic' block: {msg}")
            }
        }
    }
}
impl std::error::Error for ParseError {}

// ---------------------------------------------------------------------------
// Tokenizer + S-expression reader
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

fn tokenize(src: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c == ';' {
            while let Some(&c2) = chars.peek() {
                if c2 == '\n' {
                    break;
                }
                chars.next();
            }
            continue;
        }
        if c == '(' || c == ')' {
            tokens.push(c.to_string());
            chars.next();
            continue;
        }
        let mut s = String::new();
        while let Some(&c2) = chars.peek() {
            if c2.is_whitespace() || c2 == '(' || c2 == ')' || c2 == ';' {
                break;
            }
            s.push(c2);
            chars.next();
        }
        tokens.push(s);
    }
    tokens
}

fn read_one(tokens: &[String], pos: usize) -> Result<(Sexp, usize), ParseError> {
    match tokens.get(pos) {
        None => Err(ParseError::Syntax("unexpected end of input".to_owned())),
        Some(t) if t == "(" => {
            let mut items = Vec::new();
            let mut p = pos + 1;
            loop {
                match tokens.get(p) {
                    None => return Err(ParseError::Syntax("unbalanced parentheses".to_owned())),
                    Some(t2) if t2 == ")" => {
                        p += 1;
                        break;
                    }
                    _ => {
                        let (item, next) = read_one(tokens, p)?;
                        items.push(item);
                        p = next;
                    }
                }
            }
            Ok((Sexp::List(items), p))
        }
        Some(t) if t == ")" => Err(ParseError::Syntax("unexpected ')'".to_owned())),
        Some(t) => Ok((Sexp::Atom(t.clone()), pos + 1)),
    }
}

fn read_top(src: &str) -> Result<Sexp, ParseError> {
    let tokens = tokenize(src);
    if tokens.is_empty() {
        return Err(ParseError::Syntax("empty input".to_owned()));
    }
    let (sexp, _) = read_one(&tokens, 0)?;
    Ok(sexp)
}

/// Canonical re-serialization of an `Sexp`, used to preserve `:constraints`
/// entries verbatim (as `ConstraintDef::raw`) without needing a full nested
/// constraint-GD grammar in the AST.
fn sexp_to_string(s: &Sexp) -> String {
    match s {
        Sexp::Atom(a) => a.clone(),
        Sexp::List(items) => {
            let inner = items
                .iter()
                .map(sexp_to_string)
                .collect::<Vec<_>>()
                .join(" ");
            format!("({inner})")
        }
    }
}

/// Whether an atom lexes as a (possibly signed, possibly fractional) numeral,
/// per PDDL/HDDL numeric-literal syntax — used to tell a plain numeral value
/// apart from a fluent-reference term.
fn is_numeral(atom: &str) -> bool {
    !atom.is_empty() && atom.parse::<f64>().is_ok()
}

fn as_atom(s: &Sexp) -> Result<&str, ParseError> {
    match s {
        Sexp::Atom(a) => Ok(a.as_str()),
        Sexp::List(_) => Err(ParseError::Syntax(
            "expected an atom, found a list".to_owned(),
        )),
    }
}

fn as_list(s: &Sexp) -> Result<&[Sexp], ParseError> {
    match s {
        Sexp::List(items) => Ok(items.as_slice()),
        Sexp::Atom(a) => Err(ParseError::Syntax(format!(
            "expected a list, found atom '{a}'"
        ))),
    }
}

fn expect_atom(s: &Sexp, expected: &str) -> Result<(), ParseError> {
    let a = as_atom(s)?;
    if a != expected {
        return Err(ParseError::Syntax(format!(
            "expected '{expected}', found '{a}'"
        )));
    }
    Ok(())
}

/// Parse a `(<kind> <name>)` header list (the `(domain ...)`/`(problem ...)`
/// second top-level form of a `define`), returning `<name>`.
///
/// Checks `header.len() >= 2` up front rather than indexing `header[0]`/
/// `header[1]` directly: a truncated header (`(domain)` with no name, or an
/// entirely empty `()`) is a plausible malformed/truncated-input shape, not
/// an exotic one, and used to panic here with an out-of-bounds index before
/// this check existed.
fn parse_header(header: &[Sexp], kind: &str) -> Result<String, ParseError> {
    if header.len() < 2 {
        return Err(ParseError::Syntax(format!(
            "'({kind} <name>)' header is missing its name (found {} element(s))",
            header.len()
        )));
    }
    expect_atom(&header[0], kind)?;
    Ok(as_atom(&header[1])?.to_owned())
}

/// The value token/list following a section keyword, e.g. the `(open d1)` in
/// `(:goal (open d1))`. Sections whose value is a single required form
/// (`:domain`, `:goal`, `:constraints`) index `sec[1]` directly rather than
/// slicing `sec[1..]` (unlike `:types`/`:predicates`/etc., which take zero or
/// more values and degrade gracefully to an empty slice) — going through
/// this helper instead of a bare `sec[1]` turns a bare `(:goal)` with no
/// value into a normal `ParseError` instead of an out-of-bounds panic.
fn section_value<'a>(sec: &'a [Sexp], keyword: &str) -> Result<&'a Sexp, ParseError> {
    sec.get(1)
        .ok_or_else(|| ParseError::Syntax(format!("section '{keyword}' is missing its value")))
}

/// A flat run of `name name ... - type` groups, as used by `:types`,
/// `:constants`/`:objects` and typed parameter lists. Names with no trailing
/// `- type` default to `object`.
fn parse_typed_group(items: &[Sexp]) -> Result<Vec<(String, String)>, ParseError> {
    let mut result = Vec::new();
    let mut pending = Vec::new();
    let mut i = 0;
    while i < items.len() {
        let atom = as_atom(&items[i])?;
        if atom == "-" {
            i += 1;
            let ty =
                as_atom(items.get(i).ok_or_else(|| {
                    ParseError::Syntax("expected a type name after '-'".to_owned())
                })?)?;
            for name in pending.drain(..) {
                result.push((name, ty.to_owned()));
            }
            i += 1;
        } else {
            pending.push(atom.to_owned());
            i += 1;
        }
    }
    for name in pending.drain(..) {
        result.push((name, "object".to_owned()));
    }
    Ok(result)
}

fn parse_typed_objects(items: &[Sexp]) -> Result<Vec<TypedObject>, ParseError> {
    Ok(parse_typed_group(items)?
        .into_iter()
        .map(|(name, type_name)| TypedObject { name, type_name })
        .collect())
}

fn parse_typed_params(items: &[Sexp]) -> Result<Vec<TypedParam>, ParseError> {
    parse_typed_group(items)?
        .into_iter()
        .map(|(var, type_name)| {
            let var = var.strip_prefix('?').ok_or_else(|| {
                ParseError::Syntax(format!("expected a variable ('?name'), found '{var}'"))
            })?;
            Ok(TypedParam {
                var: var.to_owned(),
                type_name,
            })
        })
        .collect()
}

fn parse_types(items: &[Sexp]) -> Result<TypeDef, ParseError> {
    let mut parent = BTreeMap::new();
    let mut declared = Vec::new();
    for (child, ty) in parse_typed_group(items)? {
        declared.push(child.clone());
        if ty != "object" {
            parent.insert(child, ty);
        }
    }
    Ok(TypeDef { parent, declared })
}

fn parse_term(s: &Sexp) -> Result<Term, ParseError> {
    let atom = as_atom(s)?;
    match atom.strip_prefix('?') {
        Some(v) => Ok(Term::Var(v.to_owned())),
        None => Ok(Term::Const(atom.to_owned())),
    }
}

fn parse_atomic(s: &Sexp) -> Result<AtomicFormula, ParseError> {
    let items = as_list(s)?;
    let predicate = as_atom(items.first().ok_or_else(|| {
        ParseError::Syntax("expected a predicate name in atomic formula".to_owned())
    })?)?
    .to_owned();
    let args = items[1..]
        .iter()
        .map(parse_term)
        .collect::<Result<_, _>>()?;
    Ok(AtomicFormula { predicate, args })
}

/// A numeric-fluent value: either a bare numeral (kept as source text) or a
/// reference to another fluent, `(fluent-name ?args...)`.
fn parse_numeric_value(s: &Sexp) -> Result<NumericValue, ParseError> {
    match s {
        Sexp::Atom(a) if is_numeral(a) => Ok(NumericValue::Number(a.clone())),
        Sexp::Atom(a) => Err(ParseError::Syntax(format!(
            "expected a numeral or a fluent reference, found '{a}'"
        ))),
        Sexp::List(_) => Ok(NumericValue::Fluent(parse_atomic(s)?)),
    }
}

/// `(= (fluent-name ?params...) value)`, as found nested inside `:predicates`.
fn parse_numeric_fluent_decl(items: &[Sexp]) -> Result<NumericFluentDecl, ParseError> {
    if items.len() != 3 {
        return Err(ParseError::Syntax(
            "expected '(= (fluent-name ?params...) value)'".to_owned(),
        ));
    }
    let sig = as_list(&items[1])?;
    let name = as_atom(sig.first().ok_or_else(|| {
        ParseError::Syntax("expected a fluent name in numeric fluent declaration".to_owned())
    })?)?
    .to_owned();
    let params = parse_typed_params(&sig[1..])?;
    let value = parse_numeric_value(&items[2])?;
    Ok(NumericFluentDecl {
        name,
        params,
        value,
    })
}

/// `:predicates` entries are either an ordinary predicate declaration
/// (`(name ?params...)`) or a numeric-fluent function declaration
/// (`(= (name ?params...) value)`) — the two are split apart here rather
/// than requiring a separate `:functions` section, matching real-world HDDL
/// files that declare numeric fluents this way.
fn parse_predicates(
    items: &[Sexp],
) -> Result<(Vec<PredicateDef>, Vec<NumericFluentDecl>), ParseError> {
    let mut predicates = Vec::new();
    let mut numeric_fluents = Vec::new();
    for s in items {
        let list = as_list(s)?;
        let head =
            as_atom(list.first().ok_or_else(|| {
                ParseError::Syntax("expected a predicate declaration".to_owned())
            })?)?;
        if head == "=" {
            numeric_fluents.push(parse_numeric_fluent_decl(list)?);
            continue;
        }
        let params = parse_typed_params(&list[1..])?;
        predicates.push(PredicateDef {
            name: head.to_owned(),
            params,
        });
    }
    Ok((predicates, numeric_fluents))
}

/// A single `:constraints` entry, e.g. `(always (connected ?a ?b))`. `kind`
/// is the modal-operator keyword; the whole sexp is also preserved verbatim
/// in `raw` — see `ConstraintDef`'s docs for why nothing deeper is parsed.
fn parse_constraint_entry(s: &Sexp) -> Result<ConstraintDef, ParseError> {
    let items = as_list(s)?;
    let kind = as_atom(
        items
            .first()
            .ok_or_else(|| ParseError::Syntax("expected a constraint entry".to_owned()))?,
    )?
    .to_owned();
    Ok(ConstraintDef {
        kind,
        raw: sexp_to_string(s),
    })
}

/// A `:constraints` section body: `(and c1 c2 ...)` flattens into one
/// `ConstraintDef` per conjunct; anything else is a single constraint entry.
fn parse_constraints(s: &Sexp) -> Result<Vec<ConstraintDef>, ParseError> {
    let items = as_list(s)?;
    if !items.is_empty() {
        if let Ok("and") = as_atom(&items[0]) {
            return items[1..].iter().map(parse_constraint_entry).collect();
        }
    }
    Ok(vec![parse_constraint_entry(s)?])
}

fn parse_goal(s: &Sexp) -> Result<GoalDesc, ParseError> {
    let items = as_list(s)?;
    if items.is_empty() {
        return Ok(GoalDesc::Empty);
    }
    let head = as_atom(&items[0])?;
    match head {
        "and" => Ok(GoalDesc::And(
            items[1..]
                .iter()
                .map(parse_goal)
                .collect::<Result<_, _>>()?,
        )),
        "not" => {
            if items.len() != 2 {
                return Err(ParseError::Syntax(
                    "'not' takes exactly one argument".to_owned(),
                ));
            }
            Ok(GoalDesc::Not(Box::new(parse_goal(&items[1])?)))
        }
        "or" => Ok(GoalDesc::Or(
            items[1..]
                .iter()
                .map(parse_goal)
                .collect::<Result<_, _>>()?,
        )),
        "imply" => {
            if items.len() != 3 {
                return Err(ParseError::Syntax(
                    "'imply' takes exactly two arguments".to_owned(),
                ));
            }
            Ok(GoalDesc::Imply(
                Box::new(parse_goal(&items[1])?),
                Box::new(parse_goal(&items[2])?),
            ))
        }
        "forall" | "exists" => {
            if items.len() != 3 {
                return Err(ParseError::Syntax(format!(
                    "'{head}' takes a variable list and a body"
                )));
            }
            let vars = parse_typed_params(as_list(&items[1])?)?;
            if vars.is_empty() {
                return Err(ParseError::Syntax(format!(
                    "'{head}' requires at least one bound variable"
                )));
            }
            let body = Box::new(parse_goal(&items[2])?);
            if head == "forall" {
                Ok(GoalDesc::Forall(vars, body))
            } else {
                Ok(GoalDesc::Exists(vars, body))
            }
        }
        // `oneof` is an effect construct only — the koala reference grammar
        // admits it in exactly one position (a whole action's `:effect`), so
        // an occurrence in any goal-description position (action/method
        // `:precondition`, a `when`-condition, `:goal`) is a hard, typed
        // error rather than the old fall-through behavior, which silently
        // mis-parsed `(oneof p q)` as an atom with predicate "oneof" (and
        // surfaced `(oneof (p) (q))` only as a generic `Syntax` error).
        "oneof" => Err(ParseError::MalformedOneof(
            "'oneof' is an effect construct: it is only permitted as a whole action's \
             ':effect', never in a precondition, method condition, or goal position"
                .to_owned(),
        )),
        _ => {
            let args = items[1..]
                .iter()
                .map(parse_term)
                .collect::<Result<_, _>>()?;
            Ok(GoalDesc::Atom(AtomicFormula {
                predicate: head.to_owned(),
                args,
            }))
        }
    }
}

/// Where an effect s-expression appears — the context that decides which
/// effect constructs are legal there. This encodes the koala-planner
/// reference position rules for `oneof` (that grammar, hddl.y, admits
/// `(oneof ...)` in exactly one position: the entire top-level `:effect` of
/// an action):
///
/// - [`EffectCtx::Top`] — the whole `:effect` of a `:action`. The ONLY
///   context in which `oneof` is legal.
/// - [`EffectCtx::OneofBranch`] — inside a `oneof` branch (directly or under
///   an `and`). Branches may be plain literal effects, `and`-conjunctions, or
///   the empty effect `()`; a nested `oneof` and a `when` are both refused
///   (for `when`: koala parses it but silently drops conditional effects
///   downstream — a known koala TODO — so ferroplan refuses loudly instead;
///   see the `when` arm of `parse_effect`).
/// - [`EffectCtx::Nested`] — every other effect position: under a top-level
///   `when`, or under a top-level `and` (`and` never creates a oneof
///   position — `(:effect (and (p) (oneof ...)))` is outside the language).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EffectCtx {
    Top,
    Nested,
    OneofBranch,
}

/// Parse one effect s-expression in context `ctx` (see [`EffectCtx`] for the
/// exact per-context rules; see [`ParseError::MalformedOneof`] for what gets
/// refused where).
fn parse_effect(s: &Sexp, ctx: EffectCtx) -> Result<Effect, ParseError> {
    let items = as_list(s)?;
    if items.is_empty() {
        // The empty effect `()` — legal in every effect position, including
        // as a `oneof` branch (where it survives as an `Effect::Empty`
        // branch and grounds/translates to a genuine no-change outcome).
        return Ok(Effect::Empty);
    }
    let head = as_atom(&items[0])?;
    match head {
        "and" => {
            // `and` is legal in every context, but it NEVER creates a oneof
            // position: a top-level `(:effect (and ... (oneof ...)))` puts
            // the `oneof` under an `and`, which is outside the language
            // (koala admits `oneof` only as the ENTIRE top-level effect).
            // An `and` inside a `oneof` branch stays inside the branch —
            // its children keep the branch's own restrictions.
            let child = match ctx {
                EffectCtx::Top => EffectCtx::Nested,
                other => other,
            };
            Ok(Effect::And(
                items[1..]
                    .iter()
                    .map(|e| parse_effect(e, child))
                    .collect::<Result<_, _>>()?,
            ))
        }
        "not" => {
            if items.len() != 2 {
                return Err(ParseError::Syntax(
                    "'not' takes exactly one argument".to_owned(),
                ));
            }
            Ok(Effect::Literal(Literal::Neg(parse_atomic(&items[1])?)))
        }
        "when" => {
            if items.len() != 3 {
                return Err(ParseError::Syntax(
                    "'when' takes a condition and an effect".to_owned(),
                ));
            }
            if ctx == EffectCtx::OneofBranch {
                // Deliberate, documented deviation from koala: koala's
                // grammar parses `when` inside a `oneof` branch, but its
                // pipeline then silently DROPS the conditional effect
                // downstream (a known koala TODO), so accepting the shape
                // here would manufacture a silently-weaker domain. Refuse
                // loudly instead.
                return Err(ParseError::MalformedOneof(
                    "'when' inside a 'oneof' branch is not supported: koala-planner parses \
                     this shape but silently drops the conditional effect downstream, so \
                     ferroplan refuses it loudly rather than accept a silently-weaker domain"
                        .to_owned(),
                ));
            }
            let cond = parse_goal(&items[1])?;
            // The guarded sub-effect is never a oneof position (a oneof
            // under `when` is outside the language), hence `Nested` here
            // regardless of this `when`'s own context.
            let eff = parse_effect(&items[2], EffectCtx::Nested)?;
            Ok(Effect::When(cond, Box::new(eff)))
        }
        "oneof" => match ctx {
            EffectCtx::Nested => Err(ParseError::MalformedOneof(
                "'oneof' is only permitted as a whole action's ':effect' — it may not be \
                 nested under a top-level 'and' or 'when'"
                    .to_owned(),
            )),
            EffectCtx::OneofBranch => Err(ParseError::MalformedOneof(
                "nested 'oneof' is not permitted: each branch must be a plain literal \
                 effect, an 'and' conjunction, or the empty effect ()"
                    .to_owned(),
            )),
            EffectCtx::Top => {
                let branches = items[1..]
                    .iter()
                    .map(|e| parse_effect(e, EffectCtx::OneofBranch))
                    .collect::<Result<Vec<_>, _>>()?;
                match branches.len() {
                    // k >= 1: a bare `(oneof)` with no branches is outside
                    // the language (it would silently ground to an action
                    // with zero outcomes, i.e. an unexecutable dead-end).
                    0 => Err(ParseError::MalformedOneof(
                        "'oneof' requires at least one branch (k >= 1)".to_owned(),
                    )),
                    // k == 1 degenerates to a deterministic effect (the
                    // reference grammar treats a single-branch `oneof`
                    // exactly this way): normalize the wrapper away so an
                    // `Effect::Oneof` in a parsed AST always carries k >= 2
                    // genuinely non-deterministic branches.
                    1 => Ok(branches.into_iter().next().expect("branches.len() == 1")),
                    _ => Ok(Effect::Oneof(branches)),
                }
            }
        },
        "increase" | "decrease" => {
            if items.len() != 3 {
                return Err(ParseError::Syntax(format!(
                    "'{head}' takes a fluent term and a value"
                )));
            }
            let fluent = parse_atomic(&items[1])?;
            let value = parse_numeric_value(&items[2])?;
            if head == "increase" {
                Ok(Effect::Increase(fluent, value))
            } else {
                Ok(Effect::Decrease(fluent, value))
            }
        }
        _ => Ok(Effect::Literal(Literal::Pos(parse_atomic(s)?))),
    }
}

/// Keyword/value pairs as used by `:action`, `:task`, `:method` and `:htn`
/// sections, e.g. `:parameters (...) :precondition (...) :effect (...)`.
fn keyed_map(items: &[Sexp]) -> Result<BTreeMap<String, &Sexp>, ParseError> {
    let mut map = BTreeMap::new();
    let mut i = 0;
    while i < items.len() {
        let key = as_atom(&items[i])?;
        if !key.starts_with(':') {
            return Err(ParseError::Syntax(format!(
                "expected a ':keyword', found '{key}'"
            )));
        }
        let value = items
            .get(i + 1)
            .ok_or_else(|| ParseError::Syntax(format!("missing value for '{key}'")))?;
        map.insert(key.to_owned(), value);
        i += 2;
    }
    Ok(map)
}

fn parse_task_call(s: &Sexp) -> Result<TaskCall, ParseError> {
    let items = as_list(s)?;
    let name = as_atom(
        items
            .first()
            .ok_or_else(|| ParseError::Syntax("expected a task name".to_owned()))?,
    )?
    .to_owned();
    let args = items[1..]
        .iter()
        .map(parse_term)
        .collect::<Result<_, _>>()?;
    Ok(TaskCall { name, args })
}

/// A subtask list: `(and s1 s2 ...)`, or — real IPC2020/PANDA-dialect HDDL
/// permits omitting the `and` wrapper when there is exactly one subtask — a
/// single bare entry `s1` on its own (e.g. `:ordered-subtasks (pickup ?b)`).
///
/// Each entry `si` is either the labeled form `(id (task args...))` or a
/// bare unlabeled task call `(task args...)`. The two are disambiguated by
/// shape rather than by a separate marker: a labeled entry is always
/// exactly 2 elements whose *second* element is itself a list (the nested
/// task-call sexp); this is unambiguous because `parse_term` (used for a
/// task call's own arguments) never accepts a list term, so a bare call's
/// arguments are always atoms and a bare call can never accidentally take
/// the `(atom list)` shape. Unlabeled entries get a synthetic id `t{i}`
/// where `i` is the entry's 0-based position in this list — safe because
/// callers only use subtask ids for positional sequential-order edges
/// (`:ordered-subtasks`/`:ordered-tasks`) or for `:order` edges that, in
/// practice, only ever reference explicitly-labeled entries.
fn parse_subtasks(s: &Sexp) -> Result<Vec<Subtask>, ParseError> {
    let items = as_list(s)?;
    if items.is_empty() {
        return Ok(vec![]);
    }
    let entries: Vec<&Sexp> = if matches!(&items[0], Sexp::Atom(a) if a == "and") {
        items[1..].iter().collect()
    } else {
        // No `(and ...)` wrapper: the whole list is the one subtask entry.
        vec![s]
    };
    entries
        .into_iter()
        .enumerate()
        .map(|(i, entry)| {
            let pair = as_list(entry)?;
            if pair.len() == 2 {
                if let Sexp::List(_) = &pair[1] {
                    let id = as_atom(&pair[0])?.to_owned();
                    let task = parse_task_call(&pair[1])?;
                    return Ok(Subtask { id, task });
                }
            }
            let task = parse_task_call(entry)?;
            Ok(Subtask {
                id: format!("t{i}"),
                task,
            })
        })
        .collect()
}

/// `(and (before after) (before after) ...)` ordering-edge pairs.
fn parse_order_edges(s: &Sexp) -> Result<Vec<OrderEdge>, ParseError> {
    let items = as_list(s)?;
    if !items.is_empty() {
        expect_atom(&items[0], "and")?;
    }
    let rest = if items.is_empty() { items } else { &items[1..] };
    rest.iter()
        .map(|entry| {
            let pair = as_list(entry)?;
            match pair.len() {
                2 => Ok(OrderEdge {
                    before: as_atom(&pair[0])?.to_owned(),
                    after: as_atom(&pair[1])?.to_owned(),
                }),
                // Standard HDDL ordering-edge grammar (matches the PANDA-derived
                // koala-planner/Planner hddl.y `ordering_def` rule): an explicit
                // `<` token, either prefix `(< before after)` or infix
                // `(before < after)`.
                3 if as_atom(&pair[0]).ok() == Some("<") => Ok(OrderEdge {
                    before: as_atom(&pair[1])?.to_owned(),
                    after: as_atom(&pair[2])?.to_owned(),
                }),
                3 if as_atom(&pair[1]).ok() == Some("<") => Ok(OrderEdge {
                    before: as_atom(&pair[0])?.to_owned(),
                    after: as_atom(&pair[2])?.to_owned(),
                }),
                _ => Err(ParseError::Syntax(
                    "expected '(before after)', '(< before after)', or '(before < after)' ordering edge"
                        .to_owned(),
                )),
            }
        })
        .collect()
}

fn parse_task_network(map: &BTreeMap<String, &Sexp>) -> Result<TaskNetwork, ParseError> {
    if let Some(s) = map
        .get(":ordered-subtasks")
        .or_else(|| map.get(":ordered-tasks"))
    {
        let subtasks = parse_subtasks(s)?;
        let order = subtasks
            .windows(2)
            .map(|w| OrderEdge {
                before: w[0].id.clone(),
                after: w[1].id.clone(),
            })
            .collect();
        return Ok(TaskNetwork { subtasks, order });
    }
    if let Some(s) = map.get(":subtasks").or_else(|| map.get(":tasks")) {
        let subtasks = parse_subtasks(s)?;
        let order = match map.get(":order").or_else(|| map.get(":ordering")) {
            Some(o) => parse_order_edges(o)?,
            None => vec![],
        };
        return Ok(TaskNetwork { subtasks, order });
    }
    Ok(TaskNetwork::default())
}

fn parse_task_def(rest: &[Sexp]) -> Result<TaskDef, ParseError> {
    let name = as_atom(
        rest.first()
            .ok_or_else(|| ParseError::Syntax("expected a task name".to_owned()))?,
    )?
    .to_owned();
    let map = keyed_map(&rest[1..])?;
    let params = match map.get(":parameters") {
        Some(s) => parse_typed_params(as_list(s)?)?,
        None => vec![],
    };
    Ok(TaskDef { name, params })
}

fn parse_action_def(rest: &[Sexp]) -> Result<ActionDef, ParseError> {
    let name = as_atom(
        rest.first()
            .ok_or_else(|| ParseError::Syntax("expected an action name".to_owned()))?,
    )?
    .to_owned();
    let map = keyed_map(&rest[1..])?;
    let params = match map.get(":parameters") {
        Some(s) => parse_typed_params(as_list(s)?)?,
        None => vec![],
    };
    let precondition = match map.get(":precondition") {
        Some(s) => parse_goal(s)?,
        None => GoalDesc::Empty,
    };
    let effect = match map.get(":effect") {
        Some(s) => parse_effect(s, EffectCtx::Top)?,
        None => Effect::Empty,
    };
    Ok(ActionDef {
        name,
        params,
        precondition,
        effect,
        // No `crate::probabilistic::preprocess` rewrite pass exists yet to
        // populate this from a `(:probabilistic ...)` block — see the field's
        // doc comment in `ast.rs`. `None` is the documented default for a
        // plain/deterministic effect and is what every current parse path
        // (this is the only `ActionDef` construction site) produces.
        probability_weights: None,
    })
}

fn parse_method_def(rest: &[Sexp]) -> Result<MethodDef, ParseError> {
    let name = as_atom(
        rest.first()
            .ok_or_else(|| ParseError::Syntax("expected a method name".to_owned()))?,
    )?
    .to_owned();
    let map = keyed_map(&rest[1..])?;
    let params = match map.get(":parameters") {
        Some(s) => parse_typed_params(as_list(s)?)?,
        None => vec![],
    };
    let task_sexp = map
        .get(":task")
        .ok_or_else(|| ParseError::Syntax("method is missing ':task'".to_owned()))?;
    let task = parse_task_call(task_sexp)?;
    let precondition = match map.get(":precondition") {
        Some(s) => parse_goal(s)?,
        None => GoalDesc::Empty,
    };
    let network = parse_task_network(&map)?;
    Ok(MethodDef {
        name,
        params,
        task,
        precondition,
        network,
    })
}

fn parse_htn(rest: &[Sexp]) -> Result<TaskNetwork, ParseError> {
    let map = keyed_map(rest)?;
    parse_task_network(&map)
}

/// Parse a full `(define (domain ...) ...)` HDDL text into an `ast::Domain`.
///
/// Runs `probabilistic::preprocess` first (rewriting any koala-planner-style
/// `:probabilistic` effect blocks into `oneof`), then tokenizes and
/// recursive-descent parses `:types`, `:constants`, `:predicates`, `:task`,
/// `:action`, `:method`, and `:constraints` sections. `:requirements` is
/// accepted and ignored; `:functions` is a syntactically recognized but
/// unsupported construct (see `ast`'s module docs for the full scope
/// boundary — parsing succeeds for the constructs it lexes, with unsupported
/// *semantics* refused later by `grounder::ground`).
///
/// # Errors
///
/// Returns `ParseError::Syntax` for a malformed s-expression, a missing
/// `(domain <name>)` header, or an unknown domain section keyword;
/// `ParseError::UnsupportedConstruct` for `:functions` or `:durative-action`
/// (temporal/durative actions are a permanent non-goal — see `ast`'s module
/// docs); and `ParseError::MalformedOneof` for a `oneof` construct outside
/// the supported surface (see `EffectCtx`/`parse_effect` and
/// `ParseError::MalformedOneof` for the exact position/branch rules).
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::parser::parse_domain;
///
/// let src = r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#;
///
/// let domain = parse_domain(src).expect("valid HDDL domain");
/// assert_eq!(domain.name, "doors");
/// assert_eq!(domain.actions.len(), 1);
/// assert_eq!(domain.actions[0].name, "open-door");
/// ```
pub fn parse_domain(src: &str) -> Result<Domain, ParseError> {
    // Pure text preprocessing pass, run before real tokenizing: rewrites any
    // koala-planner-style `(:probabilistic w1 e1 w2 e2 ...)` effect block
    // into a standard `(oneof e1 e2 ...)` block, handing back the declared
    // weights keyed by enclosing action name (see `probabilistic::preprocess`
    // for why the map is keyed that way). Everything below this line parses
    // `cleaned` exactly as before -- `:probabilistic` never reaches the
    // tokenizer/recursive-descent grammar at all.
    let (cleaned, weight_map) = crate::probabilistic::preprocess(src)?;
    let top = read_top(&cleaned)?;
    let items = as_list(&top)?;
    if items.is_empty() {
        return Err(ParseError::Syntax("empty 'define' form".to_owned()));
    }
    expect_atom(&items[0], "define")?;
    let header = as_list(
        items
            .get(1)
            .ok_or_else(|| ParseError::Syntax("missing '(domain name)' header".to_owned()))?,
    )?;
    let name = parse_header(header, "domain")?;

    let mut domain = Domain {
        name,
        ..Domain::default()
    };
    for section in &items[2..] {
        let sec = as_list(section)?;
        let keyword = as_atom(
            sec.first()
                .ok_or_else(|| ParseError::Syntax("empty domain section".to_owned()))?,
        )?;
        match keyword {
            ":requirements" => {}
            ":types" => domain.types = parse_types(&sec[1..])?,
            ":constants" => domain.constants = parse_typed_objects(&sec[1..])?,
            ":predicates" => {
                let (predicates, numeric_fluents) = parse_predicates(&sec[1..])?;
                domain.predicates = predicates;
                domain.numeric_fluents = numeric_fluents;
            }
            ":task" => domain.tasks.push(parse_task_def(&sec[1..])?),
            ":action" => domain.actions.push(parse_action_def(&sec[1..])?),
            ":method" => domain.methods.push(parse_method_def(&sec[1..])?),
            ":constraints" => {
                domain.constraints = parse_constraints(section_value(sec, ":constraints")?)?
            }
            ":functions" | ":durative-action" => {
                return Err(ParseError::UnsupportedConstruct(keyword.to_owned()));
            }
            other => {
                return Err(ParseError::Syntax(format!(
                    "unknown domain section '{other}'"
                )))
            }
        }
    }

    // Attach each action's declared `:probabilistic` weights (extracted by
    // the preprocessing pass above, before this action's effect was even a
    // `oneof` yet) now that the action itself exists as a real `ActionDef`.
    for action in &mut domain.actions {
        if let Some(weights) = weight_map.get(&action.name) {
            action.probability_weights = Some(weights.clone());
        }
    }

    Ok(domain)
}

/// Parse a full `(define (problem ...) ...)` HDDL text into an `ast::Problem`.
///
/// Parses `(:domain <name>)`, `:objects`, `:init`, `:goal`, `:htn`, and
/// `:constraints` sections. Does not itself check that `domain_name` matches
/// any particular `Domain`, or that referenced objects/tasks are declared —
/// those cross-checks are `validate::validate_problem`'s job, run by
/// `grounder::ground` before grounding.
///
/// # Errors
///
/// Returns `ParseError::Syntax` for a malformed s-expression, a missing
/// `(problem <name>)` header, or an unknown problem section keyword.
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::parser::{parse_domain, parse_problem};
///
/// let domain_src = r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#;
/// let problem_src = r#"
///     (define (problem doors-p1)
///       (:domain doors)
///       (:objects d1)
///       (:init)
///       (:goal (open d1))
///       (:htn :ordered-subtasks (open-door d1)))
/// "#;
///
/// let _domain = parse_domain(domain_src).expect("valid HDDL domain");
/// let problem = parse_problem(problem_src).expect("valid HDDL problem");
/// assert_eq!(problem.domain_name, "doors");
/// assert_eq!(problem.objects.len(), 1);
/// ```
pub fn parse_problem(src: &str) -> Result<Problem, ParseError> {
    let top = read_top(src)?;
    let items = as_list(&top)?;
    if items.is_empty() {
        return Err(ParseError::Syntax("empty 'define' form".to_owned()));
    }
    expect_atom(&items[0], "define")?;
    let header = as_list(
        items
            .get(1)
            .ok_or_else(|| ParseError::Syntax("missing '(problem name)' header".to_owned()))?,
    )?;
    let name = parse_header(header, "problem")?;

    let mut problem = Problem {
        name,
        ..Problem::default()
    };
    for section in &items[2..] {
        let sec = as_list(section)?;
        let keyword = as_atom(
            sec.first()
                .ok_or_else(|| ParseError::Syntax("empty problem section".to_owned()))?,
        )?;
        match keyword {
            ":domain" => problem.domain_name = as_atom(section_value(sec, ":domain")?)?.to_owned(),
            ":objects" => problem.objects = parse_typed_objects(&sec[1..])?,
            ":init" => {
                problem.init = sec[1..]
                    .iter()
                    .map(parse_atomic)
                    .collect::<Result<_, _>>()?;
            }
            ":goal" => problem.goal = parse_goal(section_value(sec, ":goal")?)?,
            ":htn" => problem.htn = parse_htn(&sec[1..])?,
            ":constraints" => {
                problem.constraints = parse_constraints(section_value(sec, ":constraints")?)?
            }
            other => {
                return Err(ParseError::Syntax(format!(
                    "unknown problem section '{other}'"
                )))
            }
        }
    }
    Ok(problem)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_A_DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
    const FIXTURE_A_PROBLEM: &str = include_str!("../fixtures/a/problem.hddl");
    const FIXTURE_B_DOMAIN: &str = include_str!("../fixtures/b/domain.hddl");
    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_E_DOMAIN: &str = include_str!("../fixtures/e/domain.hddl");

    #[test]
    fn fixture_e_probabilistic_effect_rewrites_to_oneof_with_weights() {
        let domain = parse_domain(FIXTURE_E_DOMAIN).expect("fixture E domain parses");
        let action = domain
            .actions
            .iter()
            .find(|a| a.name == "toss-coin")
            .expect("toss-coin action present");
        match &action.effect {
            Effect::Oneof(branches) => assert_eq!(branches.len(), 2),
            other => panic!("expected a oneof effect, got {other:?}"),
        }
        assert_eq!(
            action.probability_weights,
            Some(vec!["3".to_owned(), "1".to_owned()])
        );
    }

    /// Temporal/durative actions are a permanent non-goal (see `ast`'s
    /// module docs: "Deliberately NOT represented at all"). A
    /// `:durative-action` domain section must be refused loudly with a
    /// dedicated, named `ParseError::UnsupportedConstruct` variant -- the
    /// same treatment `:functions` gets -- rather than falling through to
    /// the generic `ParseError::Syntax("unknown domain section ...")` catch-
    /// all, and never silently dropped or mis-parsed as a regular `:action`.
    #[test]
    fn refuses_durative_action_section_with_typed_error() {
        const DOMAIN: &str = "(define (domain temporal-d)
  (:durative-action fly
    :parameters (?a ?b)
    :duration (= ?duration 10)
    :condition (at start (at ?a))
    :effect (at end (at ?b))))";
        let err = parse_domain(DOMAIN).unwrap_err();
        assert_eq!(err, ParseError::UnsupportedConstruct(":durative-action".to_owned()));
    }

    #[test]
    fn fixture_a_domain_structure() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).expect("fixture A domain parses");
        assert_eq!(domain.name, "transport-a");
        assert_eq!(domain.actions.len(), 3);
        assert_eq!(domain.methods.len(), 1);
        let method = &domain.methods[0];
        assert_eq!(method.network.subtasks.len(), 3);
        assert_eq!(
            method.network.order,
            vec![
                OrderEdge {
                    before: "t1".to_owned(),
                    after: "t2".to_owned()
                },
                OrderEdge {
                    before: "t2".to_owned(),
                    after: "t3".to_owned()
                },
            ]
        );
    }

    #[test]
    fn fixture_a_problem_structure() {
        let problem = parse_problem(FIXTURE_A_PROBLEM).expect("fixture A problem parses");
        assert_eq!(problem.domain_name, "transport-a");
        assert_eq!(problem.objects.len(), 2);
        assert_eq!(problem.init.len(), 2);
        assert_eq!(problem.htn.subtasks.len(), 1);
        assert_eq!(problem.htn.order.len(), 0);
    }

    #[test]
    fn fixture_b_partial_order_structure() {
        let domain = parse_domain(FIXTURE_B_DOMAIN).expect("fixture B domain parses");
        assert_eq!(domain.methods.len(), 1);
        let method = &domain.methods[0];
        assert_eq!(method.network.subtasks.len(), 5);
        assert_eq!(method.network.order.len(), 4);
        let edges = method
            .network
            .order
            .iter()
            .map(|e| (e.before.clone(), e.after.clone()))
            .collect::<std::collections::BTreeSet<_>>();
        let expected = [("t1", "t3"), ("t2", "t4"), ("t3", "t5"), ("t4", "t5")]
            .into_iter()
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(edges, expected);
    }

    /// Standard HDDL ordering-edge syntax uses an explicit `<` operator
    /// (either prefix `(< before after)` or infix `(before < after)`), per
    /// the PANDA-derived reference grammar (koala-planner/Planner
    /// `parser/src/hddl.y`'s `ordering_def` rule) and real-world domains
    /// (koala-planner/domains' Transport and Satellite both write
    /// `(< task0 task1)`). Before this fix, `parse_order_edges` only
    /// accepted a bare 2-element `(before after)` pair and rejected both
    /// `<`-operator forms with a hard parse error.
    #[test]
    fn parses_ordering_edges_with_explicit_less_than_operator_prefix_and_infix() {
        const DOMAIN: &str = "(define (domain ordering-operator)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task deliver :parameters (?l - loc))
          (:method m-deliver
            :parameters (?l - loc)
            :task (deliver ?l)
            :subtasks (and
              (t1 (deliver ?l))
              (t2 (deliver ?l))
              (t3 (deliver ?l)))
            :ordering (and
              (< t1 t2)
              (t2 < t3))))";
        let domain = parse_domain(DOMAIN).expect("prefix/infix `<` ordering parses");
        let method = &domain.methods[0];
        let edges = method
            .network
            .order
            .iter()
            .map(|e| (e.before.clone(), e.after.clone()))
            .collect::<std::collections::BTreeSet<_>>();
        let expected = [("t1", "t2"), ("t2", "t3")]
            .into_iter()
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(edges, expected);
    }

    #[test]
    fn fixture_c_oneof_structure() {
        let domain = parse_domain(FIXTURE_C_DOMAIN).expect("fixture C domain parses");
        assert_eq!(domain.methods.len(), 2);
        let action = domain
            .actions
            .iter()
            .find(|a| a.name == "cross-bridge")
            .expect("cross-bridge action present");
        match &action.effect {
            Effect::Oneof(branches) => assert_eq!(branches.len(), 2),
            other => panic!("expected a oneof effect, got {other:?}"),
        }
    }

    #[test]
    fn parses_forall_in_precondition() {
        let src = r#"(define (domain bad)
          (:predicates (p ?x - object))
          (:action a
            :parameters (?x - object)
            :precondition (forall (?y - object) (p ?y))
            :effect (and (p ?x))))"#;
        let domain = parse_domain(src).expect("'forall' precondition parses");
        match &domain.actions[0].precondition {
            GoalDesc::Forall(vars, body) => {
                assert_eq!(vars.len(), 1);
                assert_eq!(vars[0].var, "y");
                assert_eq!(vars[0].type_name, "object");
                assert!(matches!(body.as_ref(), GoalDesc::Atom(a) if a.predicate == "p"));
            }
            other => panic!("expected a 'forall' goal description, got {other:?}"),
        }
    }

    #[test]
    fn forall_requires_at_least_one_bound_variable() {
        let src = r#"(define (domain bad)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition (forall () (p))
            :effect (and (p))))"#;
        let err = parse_domain(src).unwrap_err();
        assert!(matches!(err, ParseError::Syntax(_)));
    }

    #[test]
    fn rejects_nested_oneof() {
        let src = r#"(define (domain bad)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (and (p)) (oneof (and (q)) (and (p))))))"#;
        let err = parse_domain(src).unwrap_err();
        assert!(matches!(err, ParseError::MalformedOneof(_)));
    }

    // -- oneof surface rules (koala-planner reference alignment) -----------
    // The reference language admits `oneof` in exactly one position — the
    // ENTIRE top-level `:effect` of an action, k >= 1 — with branch shapes
    // limited to plain literals, `and`-conjunctions, and the empty effect
    // `()`. Branches need not be mutually exclusive. Each rule below has an
    // accept-case and a reject-case pinning the exact `ParseError` variant.

    /// Accept-case, empty branch: a Transport-style drop whose second branch
    /// is the empty effect `()` parses, with the empty branch surviving as a
    /// real `Effect::Empty` branch (later grounding to a no-change outcome —
    /// see `translate::tests::empty_branch_translates_to_a_no_change_outcome`).
    #[test]
    fn accepts_oneof_with_an_empty_branch() {
        const DOMAIN: &str = "(define (domain drop-d)
          (:types loc truck)
          (:predicates (at ?t - truck ?l - loc))
          (:action drop
            :parameters (?t - truck ?from - loc ?to - loc)
            :precondition (at ?t ?from)
            :effect (oneof
              (and (not (at ?t ?from)) (at ?t ?to))
              ())))";
        let domain = parse_domain(DOMAIN).expect("oneof with an empty branch parses");
        let drop = domain
            .actions
            .iter()
            .find(|a| a.name == "drop")
            .expect("drop action present");
        match &drop.effect {
            Effect::Oneof(branches) => {
                assert_eq!(branches.len(), 2);
                assert!(
                    matches!(&branches[0], Effect::And(parts) if parts.len() == 2),
                    "first branch must be the move conjunction, got {:?}",
                    branches[0]
                );
                assert_eq!(branches[1], Effect::Empty, "second branch must be ()");
            }
            other => panic!("expected a oneof effect, got {other:?}"),
        }
    }

    /// Accept-case, overlap: Childsnack-style overlapping branches (branch 2
    /// is a strict superset of branch 1, exactly as in the koala corpus's
    /// Childsnack serve/drop tray branches). Branches need NOT be mutually
    /// exclusive — no exclusivity/disjointness validation exists or should
    /// exist — and both branches are kept verbatim, in declaration order.
    #[test]
    fn accepts_overlapping_oneof_branches_verbatim() {
        const DOMAIN: &str = "(define (domain childsnack-style)
          (:types child)
          (:predicates (served ?c - child) (dirty ?c - child))
          (:action putdown
            :parameters (?c - child)
            :precondition ()
            :effect (oneof
              (and (served ?c))
              (and (served ?c) (not (dirty ?c))))))";
        let domain = parse_domain(DOMAIN).expect("overlapping oneof branches parse");
        let action = domain.actions[0].clone();
        match &action.effect {
            Effect::Oneof(branches) => {
                assert_eq!(branches.len(), 2, "both overlapping branches are kept");
                assert!(
                    matches!(&branches[0], Effect::And(parts) if parts.len() == 1),
                    "branch 1 kept verbatim"
                );
                assert!(
                    matches!(&branches[1], Effect::And(parts) if parts.len() == 2),
                    "branch 2 (the superset) kept verbatim"
                );
            }
            other => panic!("expected a oneof effect, got {other:?}"),
        }
    }

    /// Accept-case, k == 3: every declared branch becomes exactly one AST
    /// branch (outcome-count fidelity starts at parse time; see
    /// `translate::tests::oneof_branches_translate_to_exactly_one_outcome_transition_each`
    /// for the translated-graph version of this invariant).
    #[test]
    fn accepts_three_branch_oneof() {
        const DOMAIN: &str = "(define (domain three-way)
          (:predicates (p) (q) (r))
          (:action tri
            :parameters ()
            :precondition ()
            :effect (oneof (p) (q) (r))))";
        let domain = parse_domain(DOMAIN).expect("3-branch oneof parses");
        match &domain.actions[0].effect {
            Effect::Oneof(branches) => {
                assert_eq!(branches.len(), 3);
                assert_eq!(
                    branches[0],
                    Effect::Literal(Literal::Pos(AtomicFormula {
                        predicate: "p".to_owned(),
                        args: vec![],
                    }))
                );
                assert_eq!(
                    branches[2],
                    Effect::Literal(Literal::Pos(AtomicFormula {
                        predicate: "r".to_owned(),
                        args: vec![],
                    }))
                );
            }
            other => panic!("expected a oneof effect, got {other:?}"),
        }
    }

    /// Accept-case, k == 1: a single-branch `oneof` degenerates to a plain
    /// deterministic effect (koala hddl.y:327-330 treats k == 1 exactly this
    /// way), so the parser normalizes `(oneof e)` to just `e` — a parsed
    /// `Effect::Oneof` always carries k >= 2 genuinely non-deterministic
    /// branches.
    #[test]
    fn oneof_with_a_single_branch_degenerates_to_a_deterministic_effect() {
        const DOMAIN: &str = "(define (domain one-way)
          (:predicates (p) (q))
          (:action once
            :parameters ()
            :precondition ()
            :effect (oneof (and (p) (q)))))";
        let domain = parse_domain(DOMAIN).expect("k == 1 oneof parses");
        match &domain.actions[0].effect {
            // NOT an Effect::Oneof — the wrapper is normalized away.
            Effect::And(parts) => assert_eq!(parts.len(), 2),
            other => panic!(
                "expected k == 1 oneof to normalize to a deterministic effect, got {other:?}"
            ),
        }
    }

    /// Reject-case: `oneof` mixed with other top-level effects —
    /// `(:effect (and (p) (oneof ...)))` — is outside the language: koala's
    /// grammar admits `oneof` only as the ENTIRE top-level `:effect`, never
    /// as a conjunct of an `and`.
    #[test]
    fn rejects_oneof_nested_under_top_level_and() {
        const DOMAIN: &str = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (and (p) (oneof (q) (r)))))";
        let err = parse_domain(DOMAIN).unwrap_err();
        assert!(
            matches!(err, ParseError::MalformedOneof(_)),
            "expected MalformedOneof for a oneof under a top-level 'and', got {err:?}"
        );
    }

    /// Reject-case: `oneof` in a precondition position (action
    /// `:precondition` here; the same `parse_goal` guard covers method
    /// `:precondition`, `when`-conditions, and `:goal`) must be a typed
    /// `MalformedOneof`, not the old fall-through that silently mis-parsed
    /// `(oneof p q)` as an atom with predicate "oneof".
    #[test]
    fn rejects_oneof_in_precondition_position() {
        const DOMAIN: &str = "(define (domain bad)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (oneof (p) (q))
            :effect (and (p))))";
        let err = parse_domain(DOMAIN).unwrap_err();
        assert!(
            matches!(err, ParseError::MalformedOneof(_)),
            "expected MalformedOneof for a oneof in :precondition, got {err:?}"
        );
    }

    /// Reject-case: `when` inside a `oneof` branch — direct or nested under
    /// the branch's `and`. Deliberate deviation from koala, documented in
    /// `parse_effect`'s `when` arm: koala parses this shape but silently
    /// drops the conditional effect downstream (a known koala TODO), so
    /// ferroplan refuses it loudly instead of accepting a silently-weaker
    /// domain.
    #[test]
    fn rejects_when_inside_a_oneof_branch() {
        const DIRECT: &str = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (when (r) (p)) (q))))";
        let err = parse_domain(DIRECT).unwrap_err();
        assert!(
            matches!(err, ParseError::MalformedOneof(_)),
            "expected MalformedOneof for a 'when' directly in a branch, got {err:?}"
        );

        const UNDER_AND: &str = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (and (when (r) (p))) (q))))";
        let err = parse_domain(UNDER_AND).unwrap_err();
        assert!(
            matches!(err, ParseError::MalformedOneof(_)),
            "expected MalformedOneof for a 'when' under an 'and' inside a branch, got {err:?}"
        );
    }

    /// Position-rule completeness: `oneof` under a top-level `when` (nesting
    /// inside `when`) is likewise refused with the same typed error.
    #[test]
    fn rejects_oneof_under_when_effect() {
        const DOMAIN: &str = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (when (r) (oneof (p) (q)))))";
        let err = parse_domain(DOMAIN).unwrap_err();
        assert!(
            matches!(err, ParseError::MalformedOneof(_)),
            "expected MalformedOneof for a oneof under 'when', got {err:?}"
        );
    }

    /// k >= 1: a bare `(oneof)` with no branches is refused — it would
    /// otherwise silently ground to an action with zero outcomes, i.e. an
    /// unexecutable dead-end at translate time.
    #[test]
    fn rejects_empty_oneof() {
        const DOMAIN: &str = "(define (domain bad)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof)))";
        let err = parse_domain(DOMAIN).unwrap_err();
        assert!(
            matches!(err, ParseError::MalformedOneof(_)),
            "expected MalformedOneof for a branchless (oneof), got {err:?}"
        );
    }

    const NUMERIC_DOMAIN: &str = r#"(define (domain numeric-d)
      (:types loc vehicle)
      (:predicates
        (at ?l - loc)
        (= (fuel-level ?v - vehicle) 0))
      (:constraints
        (and
          (always (at ?l))
          (sometime (at ?l))))
      (:action refuel
        :parameters (?v - vehicle)
        :precondition ()
        :effect (and
          (increase (fuel-level ?v) 10)
          (decrease (fuel-level ?v) 1))))"#;

    #[test]
    fn lexes_and_parses_numeric_fluent_declaration() {
        let domain = parse_domain(NUMERIC_DOMAIN).expect("numeric-fluent domain parses");
        assert_eq!(domain.predicates.len(), 1);
        assert_eq!(domain.predicates[0].name, "at");
        assert_eq!(domain.numeric_fluents.len(), 1);
        let fluent = &domain.numeric_fluents[0];
        assert_eq!(fluent.name, "fuel-level");
        assert_eq!(fluent.params.len(), 1);
        assert_eq!(fluent.params[0].var, "v");
        assert_eq!(fluent.params[0].type_name, "vehicle");
        assert_eq!(fluent.value, NumericValue::Number("0".to_owned()));
    }

    #[test]
    fn lexes_and_parses_domain_constraints_block() {
        let domain = parse_domain(NUMERIC_DOMAIN).expect("constraints domain parses");
        assert_eq!(domain.constraints.len(), 2);
        assert_eq!(domain.constraints[0].kind, "always");
        assert_eq!(domain.constraints[1].kind, "sometime");
        assert!(domain.constraints[0].raw.contains("at ?l"));
    }

    #[test]
    fn lexes_and_parses_problem_constraints_block() {
        let src = r#"(define (problem p)
          (:domain d)
          (:objects a b - object)
          (:init (p a))
          (:goal (p a))
          (:constraints (and (always (p a)) (sometime-after (p a) (p b)))))"#;
        let problem = parse_problem(src).expect("problem with :constraints parses");
        assert_eq!(problem.constraints.len(), 2);
        assert_eq!(problem.constraints[0].kind, "always");
        assert_eq!(problem.constraints[1].kind, "sometime-after");
    }

    #[test]
    fn lexes_and_parses_increase_and_decrease_effects() {
        let domain = parse_domain(NUMERIC_DOMAIN).expect("increase/decrease domain parses");
        let action = domain
            .actions
            .iter()
            .find(|a| a.name == "refuel")
            .expect("refuel action present");
        match &action.effect {
            Effect::And(effects) => {
                assert_eq!(effects.len(), 2);
                match &effects[0] {
                    Effect::Increase(fluent, value) => {
                        assert_eq!(fluent.predicate, "fuel-level");
                        assert_eq!(*value, NumericValue::Number("10".to_owned()));
                    }
                    other => panic!("expected an 'increase' effect, got {other:?}"),
                }
                match &effects[1] {
                    Effect::Decrease(fluent, value) => {
                        assert_eq!(fluent.predicate, "fuel-level");
                        assert_eq!(*value, NumericValue::Number("1".to_owned()));
                    }
                    other => panic!("expected a 'decrease' effect, got {other:?}"),
                }
            }
            other => panic!("expected an 'and' effect, got {other:?}"),
        }
    }

    #[test]
    fn increase_value_may_reference_another_fluent() {
        let src = r#"(define (domain d)
          (:predicates (= (a) 0) (= (b) 0))
          (:action bump
            :parameters ()
            :precondition ()
            :effect (and (increase (a) (b)))))"#;
        let domain = parse_domain(src).expect("fluent-valued increase parses");
        let action = &domain.actions[0];
        match &action.effect {
            Effect::And(effects) => match &effects[0] {
                Effect::Increase(fluent, NumericValue::Fluent(value_fluent)) => {
                    assert_eq!(fluent.predicate, "a");
                    assert_eq!(value_fluent.predicate, "b");
                }
                other => panic!("expected an increase-by-fluent effect, got {other:?}"),
            },
            other => panic!("expected an 'and' effect, got {other:?}"),
        }
    }

    #[test]
    fn parses_or_precondition() {
        let src = r#"(define (domain d)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (or (p) (q))
            :effect (and (p))))"#;
        let domain = parse_domain(src).expect("'or' precondition parses");
        match &domain.actions[0].precondition {
            GoalDesc::Or(branches) => {
                assert_eq!(branches.len(), 2);
                assert!(matches!(&branches[0], GoalDesc::Atom(a) if a.predicate == "p"));
                assert!(matches!(&branches[1], GoalDesc::Atom(a) if a.predicate == "q"));
            }
            other => panic!("expected an 'or' goal description, got {other:?}"),
        }
    }

    #[test]
    fn parses_imply_precondition() {
        let src = r#"(define (domain d)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (imply (p) (q))
            :effect (and (q))))"#;
        let domain = parse_domain(src).expect("'imply' precondition parses");
        match &domain.actions[0].precondition {
            GoalDesc::Imply(ante, conseq) => {
                assert!(matches!(ante.as_ref(), GoalDesc::Atom(a) if a.predicate == "p"));
                assert!(matches!(conseq.as_ref(), GoalDesc::Atom(a) if a.predicate == "q"));
            }
            other => panic!("expected an 'imply' goal description, got {other:?}"),
        }
    }

    #[test]
    fn imply_requires_exactly_two_arguments() {
        let src = r#"(define (domain d)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition (imply (p))
            :effect (and (p))))"#;
        let err = parse_domain(src).unwrap_err();
        assert!(matches!(err, ParseError::Syntax(_)));
    }

    #[test]
    fn parses_exists_in_precondition() {
        let src = r#"(define (domain bad)
          (:predicates (p ?x - object))
          (:action a
            :parameters (?x - object)
            :precondition (exists (?y - object) (p ?y))
            :effect (and (p ?x))))"#;
        let domain = parse_domain(src).expect("'exists' precondition parses");
        match &domain.actions[0].precondition {
            GoalDesc::Exists(vars, body) => {
                assert_eq!(vars.len(), 1);
                assert_eq!(vars[0].var, "y");
                assert_eq!(vars[0].type_name, "object");
                assert!(matches!(body.as_ref(), GoalDesc::Atom(a) if a.predicate == "p"));
            }
            other => panic!("expected an 'exists' goal description, got {other:?}"),
        }
    }

    // -- fixture F: real external IPC 2020 corpus domain --------------------

    const FIXTURE_F_DOMAIN: &str = include_str!("../fixtures/f/domain.hddl");
    const FIXTURE_F_PROBLEM: &str = include_str!("../fixtures/f/problem.hddl");

    /// Fixture F is fetched **verbatim** (byte-for-byte, via `curl`, no hand
    /// edits) from the real IPC 2020 HTN-planning benchmark corpus:
    /// `panda-planner-dev/ipc2020-domains`, `total-order/Blocksworld-HPDDL/`
    /// (`domain.hddl` + `pfile_005.hddl`) — see `fixtures/f/domain.hddl`'s
    /// and `fixtures/f/problem.hddl`'s content for the exact text; source
    /// URLs:
    /// `https://raw.githubusercontent.com/panda-planner-dev/ipc2020-domains/master/total-order/Blocksworld-HPDDL/domain.hddl`
    /// and `.../pfile_005.hddl`. This is a classical (non-FOND) total-order
    /// HTN domain — no `oneof` — chosen because it was the one domain in
    /// that research pass with full verbatim text and a source URL.
    ///
    /// PANDA's own HDDL dialect allows a bare, unlabeled task call inside
    /// `:ordered-tasks`/`:ordered-subtasks` when no explicit ordering-by-id
    /// is needed — both as an entry inside an `(and ...)` list (e.g. this
    /// domain's very first method body,
    /// `:ordered-tasks (and (mark_done ?b) (achieve-goals))`) and, for a
    /// single subtask, with the `(and ...)` wrapper omitted entirely (e.g.
    /// `newMethod9`'s `:ordered-subtasks (pickup ?b)`). `parse_subtasks`
    /// (formerly `parse_labeled_subtasks`, when every subtask had to be an
    /// explicit `(id (task args...))` pair) now accepts both the labeled
    /// and bare forms, synthesizing a positional id `t{i}` for each bare
    /// entry. The `setdone` method's `(forall (?b - BLOCK) (done ?b))`
    /// method-level `:precondition` — and every other method's own real
    /// `:precondition` clause in this domain (`mark-done-table`,
    /// `pickup-ready-block`, `unstack-block`, `release-stack`, ...) — is now
    /// read by `parse_method_def` into `MethodDef::precondition` and checked
    /// against the world state by `translate::translate` before a method is
    /// ever offered as a decomposition branch; see the assertion on
    /// `setdone.precondition` below and
    /// `translate::tests::method_precondition_gates_which_decomposition_is_offered`
    /// for the behavioral proof.
    #[test]
    fn fixture_f_real_ipc2020_blocksworld_parses_unlabeled_subtask_calls() {
        let domain = parse_domain(FIXTURE_F_DOMAIN).expect("fixture F domain parses");
        assert_eq!(domain.methods.len(), 12);
        assert_eq!(domain.actions.len(), 6);
        assert_eq!(domain.tasks.len(), 5);

        let mark_done_table = domain
            .methods
            .iter()
            .find(|m| m.name == "mark-done-table")
            .expect("mark-done-table method present");
        assert_eq!(
            mark_done_table.network.subtasks,
            vec![
                Subtask {
                    id: "t0".to_owned(),
                    task: TaskCall {
                        name: "mark_done".to_owned(),
                        args: vec![Term::Var("b".to_owned())],
                    },
                },
                Subtask {
                    id: "t1".to_owned(),
                    task: TaskCall {
                        name: "achieve-goals".to_owned(),
                        args: vec![],
                    },
                },
            ]
        );
        assert_eq!(
            mark_done_table.network.order,
            vec![OrderEdge {
                before: "t0".to_owned(),
                after: "t1".to_owned(),
            }]
        );

        // `newMethod9`: a single bare subtask with no `(and ...)` wrapper at
        // all (`:ordered-subtasks (pickup ?b)`).
        let new_method9 = domain
            .methods
            .iter()
            .find(|m| m.name == "newMethod9")
            .expect("newMethod9 method present");
        assert_eq!(
            new_method9.network.subtasks,
            vec![Subtask {
                id: "t0".to_owned(),
                task: TaskCall {
                    name: "pickup".to_owned(),
                    args: vec![Term::Var("b".to_owned())],
                },
            }]
        );
        assert!(new_method9.network.order.is_empty());

        // `setdone`: an empty `(and )` subtask list parses to zero subtasks.
        let setdone = domain
            .methods
            .iter()
            .find(|m| m.name == "setdone")
            .expect("setdone method present");
        assert!(setdone.network.subtasks.is_empty());
        assert!(setdone.network.order.is_empty());

        // `setdone`'s method-level `:precondition (forall (?b - BLOCK)
        // (done ?b))` must now parse into a real `Forall` node, not be
        // silently dropped.
        match &setdone.precondition {
            GoalDesc::Forall(vars, body) => {
                assert_eq!(vars.len(), 1);
                assert_eq!(vars[0].var, "b");
                assert_eq!(vars[0].type_name, "BLOCK");
                assert!(matches!(body.as_ref(), GoalDesc::Atom(a) if a.predicate == "done"));
            }
            other => {
                panic!("expected setdone's precondition to parse as a 'forall', got {other:?}")
            }
        }

        // A method with a plain conjunctive `:precondition` (no quantifier)
        // parses into the expected `And` shape too.
        let mark_done_table_precond = &mark_done_table.precondition;
        assert!(
            matches!(mark_done_table_precond, GoalDesc::And(parts) if parts.len() == 2),
            "expected mark-done-table's precondition to parse as a 2-way 'and', got {mark_done_table_precond:?}"
        );

        // `newMethod9` declares no `:precondition` at all — must default to
        // `GoalDesc::Empty` (vacuously true), matching `ActionDef`'s
        // no-`:precondition` convention.
        assert_eq!(new_method9.precondition, GoalDesc::Empty);
    }

    /// Fixture F's problem file (`pfile_005.hddl`, same corpus as the domain
    /// above) also parses: its `:htn` uses the labeled form
    /// (`(task0 (achieve-goals))`), which `parse_subtasks` still accepts.
    #[test]
    fn fixture_f_real_ipc2020_blocksworld_problem_parses() {
        let problem = parse_problem(FIXTURE_F_PROBLEM).expect("fixture F problem parses");
        assert_eq!(problem.objects.len(), 5);
        assert_eq!(
            problem.htn.subtasks,
            vec![Subtask {
                id: "task0".to_owned(),
                task: TaskCall {
                    name: "achieve-goals".to_owned(),
                    args: vec![],
                },
            }]
        );
    }

    // -----------------------------------------------------------------
    // Adversarial regressions for the header/section-value panic risks
    // found by the panic-safety audit. Each of these constructed a
    // genuinely malformed/truncated HDDL text that used to index past the
    // end of a short `header`/`sec` slice (`header[0]`/`header[1]`/
    // `sec[1]`) instead of returning a typed `ParseError`. `catch_unwind`
    // proves no panic; the `Err` assertion proves it's refused cleanly.
    // -----------------------------------------------------------------

    #[test]
    fn adversarial_domain_header_missing_name_returns_err_not_panic() {
        // `(domain)` has only one element (`domain`), so the old code's
        // `as_atom(&header[1])` indexed past the end of a 1-element slice.
        let src = "(define (domain) (:types))";
        let result = std::panic::catch_unwind(|| parse_domain(src));
        let result = result.expect("parse_domain must not panic on a truncated domain header");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }

    #[test]
    fn adversarial_domain_header_empty_returns_err_not_panic() {
        // `()` has zero elements, so even the old code's
        // `expect_atom(&header[0], "domain")` indexed past the end of an
        // empty slice, one step earlier than the missing-name case above.
        let src = "(define () (:types))";
        let result = std::panic::catch_unwind(|| parse_domain(src));
        let result = result.expect("parse_domain must not panic on an empty domain header");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }

    #[test]
    fn adversarial_problem_header_missing_name_returns_err_not_panic() {
        let src = "(define (problem) (:objects))";
        let result = std::panic::catch_unwind(|| parse_problem(src));
        let result = result.expect("parse_problem must not panic on a truncated problem header");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }

    #[test]
    fn adversarial_problem_header_empty_returns_err_not_panic() {
        let src = "(define () (:objects))";
        let result = std::panic::catch_unwind(|| parse_problem(src));
        let result = result.expect("parse_problem must not panic on an empty problem header");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }

    #[test]
    fn adversarial_domain_constraints_section_missing_value_returns_err_not_panic() {
        // A bare `(:constraints)` with no argument used to hit `sec[1]`
        // directly (unlike `:types`/`:predicates`/etc., which slice
        // `sec[1..]` and degrade gracefully to empty).
        let src = "(define (domain d) (:predicates (p)) (:constraints))";
        let result = std::panic::catch_unwind(|| parse_domain(src));
        let result =
            result.expect("parse_domain must not panic on a valueless :constraints section");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }

    #[test]
    fn adversarial_problem_domain_section_missing_value_returns_err_not_panic() {
        let src = "(define (problem p) (:domain))";
        let result = std::panic::catch_unwind(|| parse_problem(src));
        let result = result.expect("parse_problem must not panic on a valueless :domain section");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }

    #[test]
    fn adversarial_problem_goal_section_missing_value_returns_err_not_panic() {
        let src = "(define (problem p) (:domain d) (:goal))";
        let result = std::panic::catch_unwind(|| parse_problem(src));
        let result = result.expect("parse_problem must not panic on a valueless :goal section");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }

    #[test]
    fn adversarial_problem_constraints_section_missing_value_returns_err_not_panic() {
        let src = "(define (problem p) (:domain d) (:constraints))";
        let result = std::panic::catch_unwind(|| parse_problem(src));
        let result =
            result.expect("parse_problem must not panic on a valueless :constraints section");
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }
}
