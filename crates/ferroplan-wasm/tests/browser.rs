//! Real `wasm_bindgen_test` coverage for the browser-facing WASM surface
//! (`crates/ferroplan-wasm/src/lib.rs`), run against an actual wasm binary
//! in a real browser or Node engine — not a mock, not a description. Run
//! with `wasm-pack test --headless --chrome` (or `--firefox`/`--node`)
//! from this crate's directory.
//!
//! Closes the gap the WASM capabilities audit named: previously the only
//! coverage of this surface was `smoke.js`, a manual Playwright script with
//! no `cargo`-runnable path. This file doesn't replace `smoke.js` (which
//! exercises the full `web/index.html`/`web/village-live.html` demo pages
//! end to end); it gives the crate's own exported functions a fast,
//! CI-runnable regression net that needs no HTML fixture at all.

#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use ferroplan_wasm::{explain, plan, plan_production, readiness, version, WasmSession};
use wasm_bindgen_test::*;

// No `run_in_browser` config: every function under test here is pure
// Rust/JSON logic (no DOM, no browser-only Web API) — `console.error` in
// `WasmSession::new`'s panic hook is the one JS-global touch, and Node
// provides `console` too, so this suite runs identically under
// `wasm-pack test --node` or `--headless --chrome/--firefox`.

const DOMAIN: &str = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))";

const PROBLEM: &str = "(define (problem two-room)
  (:domain rooms)
  (:objects a b - room)
  (:init (at a) (link a b))
  (:goal (at b)))";

#[wasm_bindgen_test]
fn plan_solves_the_two_room_domain_in_one_step() {
    let json = plan(DOMAIN, PROBLEM, None, None, None);
    let sol: serde_json::Value = serde_json::from_str(&json).expect("valid Solution JSON");
    assert_eq!(sol["solved"], true, "{json}");
    assert_eq!(sol["plan"]["length"], 1, "{json}");
}

#[wasm_bindgen_test]
fn plan_reports_a_structured_error_on_a_malformed_domain() {
    let json = plan("not a pddl domain", PROBLEM, None, None, None);
    let v: serde_json::Value = serde_json::from_str(&json).expect("valid error JSON");
    assert!(v.get("error").is_some(), "{json}");
}

#[wasm_bindgen_test]
fn plan_production_returns_a_versioned_solved_envelope() {
    let json = plan_production(
        DOMAIN,
        PROBLEM,
        None,
        None,
        None,
        None,
        None,
        Some("browser-test-1".to_string()),
    );
    let envelope: serde_json::Value = serde_json::from_str(&json).expect("valid envelope JSON");
    assert_eq!(
        envelope["schema_version"], "ferroplan.operation.v1",
        "{json}"
    );
    assert_eq!(envelope["authority"], "candidate_only", "{json}");
    assert_eq!(envelope["outcome"], "solved", "{json}");
    assert_eq!(envelope["request_id"], "browser-test-1", "{json}");
}

#[wasm_bindgen_test]
fn plan_production_refuses_an_unknown_mode_with_a_structured_error() {
    let json = plan_production(
        DOMAIN,
        PROBLEM,
        Some("not-a-real-mode".to_string()),
        None,
        None,
        None,
        None,
        None,
    );
    let envelope: serde_json::Value = serde_json::from_str(&json).expect("valid envelope JSON");
    assert_eq!(envelope["outcome"], "refused", "{json}");
    assert_eq!(envelope["error"]["code"], "FP_INVALID_REQUEST", "{json}");
}

#[wasm_bindgen_test]
fn readiness_reports_a_valid_fingerprinted_manifest() {
    let json = readiness();
    let manifest: serde_json::Value = serde_json::from_str(&json).expect("valid manifest JSON");
    assert_eq!(
        manifest["schema_version"], "ferroplan.readiness-contract.v1",
        "{json}"
    );
    assert_eq!(manifest["contract_valid"], true, "{json}");
    assert!(manifest["manifest_fingerprint"].is_string(), "{json}");
}

#[wasm_bindgen_test]
fn version_reports_the_crate_semver() {
    let v = version();
    assert!(!v.is_empty());
    assert_eq!(v.split('.').count(), 3, "expected semver x.y.z, got {v}");
}

#[wasm_bindgen_test]
fn explain_explains_a_real_solved_plan() {
    let sol_json = plan(DOMAIN, PROBLEM, None, None, None);
    let sol: serde_json::Value = serde_json::from_str(&sol_json).unwrap();
    let plan_json = serde_json::to_string(&sol["plan"]).unwrap();

    let explanation = explain(DOMAIN, PROBLEM, &plan_json);
    let v: serde_json::Value = serde_json::from_str(&explanation).expect("valid explanation JSON");
    assert!(v.get("error").is_none(), "{explanation}");
}

#[wasm_bindgen_test]
fn wasm_session_think_step_advance_walks_a_real_plan() {
    let mut session = WasmSession::new(DOMAIN, PROBLEM).expect("session grounds");
    let sol_json = session.think(10_000, 64);
    let sol: serde_json::Value = serde_json::from_str(&sol_json).expect("valid Solution JSON");
    assert_eq!(sol["solved"], true, "{sol_json}");

    assert!(session.has_plan());
    assert!(session.valid());

    let step_json = session.step_json();
    assert_ne!(step_json, "null", "expected a real first step, got null");

    session.advance();
    let suffix_json = session.suffix_json();
    let suffix: serde_json::Value = serde_json::from_str(&suffix_json).expect("valid suffix JSON");
    assert!(suffix.is_array());
}

#[wasm_bindgen_test]
fn wasm_session_fact_round_trips_a_real_world_mutation() {
    let mut session = WasmSession::new(DOMAIN, PROBLEM).expect("session grounds");
    session
        .set_fact("(at a)", false)
        .expect("set_fact succeeds");
    let value = session.fact("(at a)");
    assert_eq!(value.as_bool(), Some(false));
}

#[wasm_bindgen_test]
fn wasm_session_new_refuses_an_oversized_domain() {
    // 65 MiB of 'a' — over the WASM_HARD_INPUT_BYTES-equivalent
    // ProductionLimits::default().max_domain_bytes bound `new` enforces.
    let huge = "a".repeat(65 * 1024 * 1024);
    let result = WasmSession::new(&huge, PROBLEM);
    assert!(
        result.is_err(),
        "an oversized domain must be refused, not accepted"
    );
}
