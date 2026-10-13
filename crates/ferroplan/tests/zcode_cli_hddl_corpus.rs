//! zcode-cli HDDL work-order corpus — first-ever parse court for the
//! semantic-jira-pack projection against ferroplan's HDDL front-end.
//!
//! Fixtures (vendored byte-for-byte, with provenance):
//! * `fixtures/zcode-cli-v26.9.22/ZOCEL-014.hddl`
//! * `fixtures/zcode-cli-v26.9.22/ZOCEL-016.hddl`
//!   copied from `/Users/sac/zcode-cli/docs/sjira/v26.9.22/plan/ZOCEL-{014,016}.hddl`
//!   (repo `zcode-cli`, `docs/sjira/v26.9.22/plan/`).
//!
//! ## REAL FINDINGS (this court's reason to exist)
//!
//! **Finding F1 — grammar mismatch (real, confirmed by execution).**
//! The generated corpus files open with the PDDL4J-style shorthand
//! `(defproblem <name> <domain> ...)`. ferroplan's HDDL problem parser
//! (`ferroplan-hddl/src/parser.rs::parse_problem`) admits exactly one
//! top-level form: `(define (problem <name>) ...)` with sections from the
//! closed set `:domain/:objects/:init/:goal/:htn/:constraints`. The corpus's
//! `defproblem` form — and its problem-level `(:task ...)` / `(:method ...)`
//! sections, which are not in the parser's closed section set — is outside
//! the declared subset (`ferroplan-hddl/src/lib.rs` "Scope"). Observed
//! verdict, byte-precise: `Err(ParseError::Syntax(
//! "syntax error: expected 'define', found 'defproblem'"))` for BOTH files.
//! The corpus therefore cannot be consumed by ferroplan's HDDL front-end as
//! generated; per finding discipline the parser was NOT modified to
//! force-fit, and these tests assert the honest refusal.
//!
//! **Finding F2 — the corpus ships no domain (real).** The `defproblem`
//! header names domain `semantic-jira-workorder`, but no
//! `(define (domain semantic-jira-workorder) ...)` HDDL file exists anywhere
//! in `zcode-cli` (disk search: every `.hddl` under the repo, plus
//! ggen-igniter/ggen-marketplace/xaas searched for the domain body's action
//! names like `close-court` — only problem-side files reference them; the
//! domain exists only inside ggen-igniter's Elixir templates, never emitted
//! as a consumable HDDL file). So even with F1 fixed, no domain/problem pair
//! exists to load: the projection is one half of a two-file contract.
//!
//! Both findings are zcode-cli/semantic-jira-pack-side, not ferroplan-side:
//! ferroplan's parser behaves exactly as its declared scope says it does.
//!
//! Court design: each refusal is asserted with the byte-precise error text
//! (a different refusal would itself be a finding worth naming), a positive
//! control proves the same parser accepts real supported-surface HDDL in the
//! same run (so the refusals are content-caused, not environment-caused),
//! and determinism twins prove the refusals are stable, not flaky.

use ferroplan_hddl::parser::{parse_domain, parse_problem};

const ZOCEL_014: &str = include_str!("fixtures/zcode-cli-v26.9.22/ZOCEL-014.hddl");
const ZOCEL_016: &str = include_str!("fixtures/zcode-cli-v26.9.22/ZOCEL-016.hddl");
// Positive control: vendored supported-surface pair (sa2a-v26.9.17, see
// sa2a_goal_set_ruling.rs for its provenance and ruling).
const CONTROL_DOMAIN: &str = include_str!("fixtures/sa2a-v26.9.17/sa2a-v26.9.17-domain.hddl");
const CONTROL_PROBLEM: &str = include_str!("fixtures/sa2a-v26.9.17/sa2a-v26.9.17-problem.hddl");

const EXPECTED_REFUSAL: &str = "syntax error: expected 'define', found 'defproblem'";

/// F1 witness: the corpus's shape facts, so a silent upstream regeneration
/// into a different shape changes THIS test's expectations deliberately.
#[test]
fn corpus_shape_facts() {
    for (name, src) in [("ZOCEL-014", ZOCEL_014), ("ZOCEL-016", ZOCEL_016)] {
        assert!(
            src.contains("(defproblem promote-"),
            "{name}: expected the defproblem shorthand header"
        );
        assert!(
            src.contains(" semantic-jira-workorder"),
            "{name}: expected the semantic-jira-workorder domain name"
        );
        assert!(
            src.contains("\n  (:task (promote "),
            "{name}: expected a problem-level (:task ...) section"
        );
        assert!(
            src.contains("\n  (:method "),
            "{name}: expected a problem-level (:method ...) section"
        );
    }
}

/// F1 court: both corpus files are refused, byte-precisely, deterministically.
#[test]
fn corpus_files_are_refused_by_the_problem_parser() {
    for (name, src) in [("ZOCEL-014", ZOCEL_014), ("ZOCEL-016", ZOCEL_016)] {
        let e1 = parse_problem(src).expect_err("F1: corpus must be refused, not silently accepted");
        let e2 = parse_problem(src).expect_err("F1: refusal must be deterministic");
        let rendered = e1.to_string();
        assert_eq!(
            rendered, EXPECTED_REFUSAL,
            "{name}: refusal text changed — re-derive F1 against the new grammar"
        );
        assert_eq!(
            e1.to_string(),
            e2.to_string(),
            "{name}: refusal must be stable across parses"
        );
    }
}

/// F2 witness: the corpus declares a domain it does not ship. The declared
/// domain name is checked against every fixture; the absence of the domain
/// file on disk is asserted for the sibling repo location ferroplan would
/// see (when present), skipped when zcode-cli itself is absent.
#[test]
fn corpus_declares_a_domain_it_does_not_ship() {
    let _ = (ZOCEL_014, ZOCEL_016);
    // zcode-cli checkout: problem files exist, domain file does not.
    let zc = PathBuf::from("/Users/sac/zcode-cli");
    if !zc.is_dir() {
        eprintln!("BLOCKED:zcode-cli-absent");
        return;
    }
    let plan_dir = zc.join("docs/sjira/v26.9.22/plan");
    let mut problems = 0usize;
    let mut domains = 0usize;
    for entry in std::fs::read_dir(&plan_dir).expect("read plan dir") {
        let p = entry.unwrap().path();
        if p.extension().and_then(|e| e.to_str()) == Some("hddl") {
            let text = std::fs::read_to_string(&p).unwrap();
            if text.contains("(defproblem") {
                problems += 1;
            }
            if text.contains("(define (domain") {
                domains += 1;
            }
        }
    }
    assert!(
        problems >= 2,
        "expected the generated problem files to still exist"
    );
    assert_eq!(
        domains, 0,
        "F2 regression tripwire: if a domain file appears in the corpus, \
         upgrade this court to a real domain/problem round-trip"
    );
}

/// Positive control: the same parser accepts real supported-surface HDDL in
/// the same run — the corpus refusals are content-caused, not
/// environment-caused.
#[test]
fn positive_control_supported_surface_still_parses() {
    let domain = parse_domain(CONTROL_DOMAIN).expect("control domain parses");
    let problem = parse_problem(CONTROL_PROBLEM).expect("control problem parses");
    assert_eq!(problem.domain_name, domain.name);
}

use std::path::PathBuf;
