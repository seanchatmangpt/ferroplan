//! The `validate_files` example's gate (parse -> static validation ->
//! `ground_methods`), exercised on the crate's real fixtures and on a real
//! falsifier: a FOND `oneof` wrapped in `and` must be refused. The example
//! source is pulled in by `#[path]` so these tests run exactly the code the
//! example binary runs. No doubles.

#[allow(dead_code)]
#[path = "../examples/validate_files.rs"]
mod validate_files;

use validate_files::{validate_pair, validate_paths};

const C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
const C_PROBLEM: &str = include_str!("../fixtures/c/problem.hddl");

#[test]
fn admits_every_bundled_fixture_pair() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    for fixture in ["a", "b", "c", "d", "e", "f", "g"] {
        let summary = validate_paths(
            &format!("{root}/{fixture}/domain.hddl"),
            &format!("{root}/{fixture}/problem.hddl"),
        )
        .unwrap_or_else(|e| panic!("fixture {fixture} refused: {e}"));
        assert!(summary.methods > 0, "fixture {fixture}: {summary:?}");
        assert!(summary.ground_methods > 0, "fixture {fixture}: {summary:?}");
    }
}

#[test]
fn fixture_c_grounds_its_two_reach_methods() {
    let summary = validate_pair(C_DOMAIN, C_PROBLEM).expect("fixture c admits");
    assert_eq!(summary.domain, "bridge-c");
    assert_eq!(summary.methods, 2);
    assert_eq!(summary.actions, 2);
}

#[test]
fn refuses_a_oneof_wrapped_in_and() {
    let wrapped = C_DOMAIN
        .replace(":effect (oneof", ":effect (and (oneof")
        .replace(
            "(at ?s))))\n  (:action walk",
            "(at ?s)))))\n  (:action walk",
        );
    assert_ne!(wrapped, C_DOMAIN, "mutation must have applied");
    let err = validate_pair(&wrapped, C_PROBLEM).expect_err("and-wrapped oneof must be refused");
    assert!(err.starts_with("domain parse:"), "{err}");
    assert!(err.contains("oneof"), "{err}");
}

#[test]
fn names_an_unreadable_file() {
    let err = validate_paths("/nonexistent/domain.hddl", "/nonexistent/problem.hddl")
        .expect_err("missing file refused");
    assert!(err.starts_with("read /nonexistent/domain.hddl"), "{err}");
}

#[test]
fn refuses_a_problem_for_an_undeclared_task() {
    let broken = C_PROBLEM.replace("(reach ", "(no-such-task ");
    assert_ne!(broken, C_PROBLEM, "mutation must have applied");
    let err = validate_pair(C_DOMAIN, &broken).expect_err("unknown root task refused");
    assert!(err.starts_with("problem validation:"), "{err}");
}
