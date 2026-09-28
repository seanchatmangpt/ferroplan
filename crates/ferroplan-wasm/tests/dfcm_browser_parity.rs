//! Native-host parity tests for the browser `WasmSession` DfCM surface
//! (`repair`, `replan_following`, `probe_json`) and `fond_validate`.
//!
//! `tests/browser.rs` needs `wasm-pack` plus a matching `wasm-bindgen` CLI
//! and a JS engine; this file runs the same exported Rust functions on the
//! native host with plain `cargo test`, so the browser surface's admission
//! guards are exercised on every ordinary test run. No doubles: every call
//! goes through a real grounded `ferroplan::Session`.
//!
//! `WasmSession::new` installs its `console.error` panic hook only on
//! `wasm32`, so a failing assertion here reports its own test natively.
//!
//! The corridor tests below compare the browser surface against the shared
//! `ferroplan_wasm::dfcm_route` kernel (the code the WASI surface runs) on a
//! separately grounded `ferroplan::Session` with the same history, instead
//! of against hand-written literals, and hold the browser surface to the
//! same strict follow-before-rethink bound as the WASI court.

#![cfg(not(target_family = "wasm"))]

use ferroplan_wasm::dfcm_route::{self, fixture, ProbeCandidate};
use ferroplan_wasm::{fond_validate, WasmSession};
use serde_json::{json, Value};

const DOMAIN: &str = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))";

const PROBLEM: &str = "(define (problem repair)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c b))
  (:goal (at b)))";

fn session() -> WasmSession {
    WasmSession::new(DOMAIN, PROBLEM).unwrap_or_else(|_| panic!("session must ground"))
}

fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap_or_else(|e| panic!("{e}: {text}"))
}

#[test]
fn browser_repair_routes_reuse_follow_and_goal_met() {
    let mut s = session();
    let full = parse(&s.repair(10_000, 64));
    assert_eq!(full["decision"], json!("replanned_full"), "{full}");

    let reuse = parse(&s.repair(10_000, 64));
    assert_eq!(reuse["decision"], json!("reuse_suffix"), "{reuse}");
    assert_eq!(reuse["solution"], Value::Null, "{reuse}");

    s.observe(r#"[["(at a)", false], ["(at c)", true]]"#)
        .unwrap_or_else(|_| panic!("drift must be observable"));
    let follow = parse(&s.repair(10_000, 64));
    assert_eq!(follow["decision"], json!("replanned_following"), "{follow}");
    assert!(s.valid());

    let again = parse(&s.repair(10_000, 64));
    assert_eq!(again["decision"], json!("reuse_suffix"), "{again}");
    assert_eq!(again["suffix"], follow["suffix"], "{follow} vs {again}");

    s.observe(r#"[["(at c)", false], ["(at b)", true]]"#)
        .unwrap_or_else(|_| panic!("goal must be observable"));
    let met = parse(&s.repair(10_000, 64));
    assert_eq!(met["decision"], json!("goal_met"), "{met}");
}

#[test]
fn browser_repair_and_follow_refuse_out_of_budget_and_missing_plans() {
    let mut s = session();
    let no_plan = parse(&s.replan_following(10_000, 64));
    assert_eq!(no_plan["error"]["code"], json!("FP_NO_PLAN"), "{no_plan}");
    let evals = parse(&s.repair(0, 64));
    assert_eq!(evals["error"]["code"], json!("FP_LIMIT_SEARCH"), "{evals}");
    let mem = parse(&s.repair(10_000, 0));
    assert_eq!(mem["error"]["code"], json!("FP_LIMIT_MEMORY"), "{mem}");
    assert!(!s.has_plan(), "a refused repair must not stash a plan");
}

#[test]
fn browser_probe_refuses_duplicate_and_empty_ids_before_forking() {
    let s = session();
    let dup = parse(&s.probe_json(r#"[{"id":"x"},{"id":"x"}]"#, 10_000, 64));
    assert_eq!(
        dup["error"]["code"],
        json!("FP_DUPLICATE_CANDIDATE"),
        "{dup}"
    );
    let empty = parse(&s.probe_json(r#"[{"id":""}]"#, 10_000, 64));
    assert_eq!(
        empty["error"]["code"],
        json!("FP_LIMIT_CANDIDATE"),
        "{empty}"
    );
    let none = parse(&s.probe_json("[]", 10_000, 64));
    assert_eq!(
        none["error"]["code"],
        json!("FP_LIMIT_CANDIDATES"),
        "{none}"
    );
    let malformed = parse(&s.probe_json("{\"id\":\"x\"}", 10_000, 64));
    assert_eq!(
        malformed["error"]["code"],
        json!("FP_ADAPTER"),
        "{malformed}"
    );
}

#[test]
fn browser_probe_matches_the_wasi_counterfactual_contract() {
    let s = session();
    let probed = parse(&s.probe_json(
        r#"[
            {"id":"baseline","goal":"(at b)"},
            {"id":"counterfactual-c","sight":[["(at a)",false],["(at c)",true]]},
            {"id":"stranded","sight":[["(at a)",false]]},
            {"id":"ungrounded-d","goal":"(at d)"},
            {"id":"contradiction","sight":[["(at c)",true],["(AT C)",false]]}
        ]"#,
        10_000,
        64,
    ));
    let outcomes: Vec<&str> = probed["results"]
        .as_array()
        .expect("results")
        .iter()
        .map(|r| r["outcome"].as_str().unwrap_or("?"))
        .collect();
    assert_eq!(
        outcomes,
        ["solved", "solved", "unsolved", "refused", "refused"],
        "{probed}"
    );
    assert_eq!(probed["results"][4]["stage"], json!("observe"), "{probed}");
    assert!(!s.has_plan(), "probing must not stash into the parent");
    assert!(!s.goal_met());
}

fn retry_loop_problem() -> Value {
    json!({
        "states": [{ "id": "s0" }, { "id": "g", "facts": ["done"] }],
        "initial_states": ["s0"],
        "goal": { "facts": ["done"] },
        "transitions": [
            { "action": "flip", "from": "s0", "to": "g", "probability_ppm": 500_000 },
            { "action": "flip", "from": "s0", "to": "s0", "probability_ppm": 500_000 },
        ],
    })
}

/// The real strong-cyclic policy, synthesized by the same library entry
/// point the WASI `fond_policy` op uses (no hand-written policy JSON).
fn retry_loop_policy() -> Value {
    use ferroplan::planning_runtime::{solve_planning_type, UniversalPlanningRequest};
    use ferroplan::planning_types::PlanningType;
    let request = UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: serde_json::from_value(retry_loop_problem()).expect("problem"),
        limits: ferroplan::PlannerLimits {
            max_wall_ms: 0,
            ..ferroplan::PlannerLimits::default()
        },
    };
    let plan = solve_planning_type(&request).expect("retry loop is strong-cyclic");
    serde_json::to_value(plan).expect("plan serializes")
}

#[test]
fn browser_fond_validate_typed_refusals_and_subject_binding() {
    let bad_problem = parse(&fond_validate("{not json", "{}"));
    assert_eq!(
        bad_problem["error"]["code"],
        json!("FP_INVALID_PROBLEM"),
        "{bad_problem}"
    );
    let bad_plan = parse(&fond_validate(&retry_loop_problem().to_string(), "[1,2]"));
    assert_eq!(
        bad_plan["error"]["code"],
        json!("FP_INVALID_POLICY"),
        "{bad_plan}"
    );

    let mut wrong_subject = retry_loop_problem();
    wrong_subject["transitions"][1]["to"] = json!("dead");
    wrong_subject["states"]
        .as_array_mut()
        .unwrap()
        .push(json!({ "id": "dead" }));
    let report = parse(&fond_validate(
        &wrong_subject.to_string(),
        &retry_loop_policy().to_string(),
    ));
    assert_eq!(report["valid"], json!(false), "{report}");

    let own = parse(&fond_validate(
        &retry_loop_problem().to_string(),
        &retry_loop_policy().to_string(),
    ));
    assert_eq!(own["valid"], json!(true), "{own}");
    assert_eq!(own["guarantee"], json!("STRONG_CYCLIC"), "{own}");
}

fn kernel_corridor(n: usize) -> ferroplan::Session {
    let opts = ferroplan::Options {
        threads: 1,
        max_evaluated: Some(ferroplan::ProductionLimits::default().max_evaluated),
        ..Default::default()
    };
    ferroplan::Session::new(
        fixture::CORRIDOR_DOMAIN,
        &fixture::corridor_problem(n),
        &opts,
    )
    .expect("corridor grounds")
}

fn evaluated(v: &Value) -> u64 {
    v["statistics"]["evaluated_states"]
        .as_u64()
        .unwrap_or_else(|| panic!("{v}"))
}

/// Browser twin of the WASI regression court, and a differential against
/// the kernel: every route decision on the browser surface must be
/// byte-identical to the kernel's decision on the same world history.
#[test]
fn browser_repair_matches_the_kernel_and_follow_is_strictly_cheaper() {
    let n = 24;
    let k = n / 2;
    let mut browser = WasmSession::new(fixture::CORRIDOR_DOMAIN, &fixture::corridor_problem(n))
        .unwrap_or_else(|_| panic!("corridor must ground"));
    let mut kernel = kernel_corridor(n);
    let (mut plan, mut cursor) = (None, 0_usize);

    let b_first = parse(&browser.repair(10_000, 64));
    let k_first = dfcm_route::repair(&kernel, &mut plan, &mut cursor, 10_000, 64);
    assert_eq!(b_first, k_first);
    assert_eq!(b_first["decision"], json!("replanned_full"), "{b_first}");

    let b_reuse = parse(&browser.repair(10_000, 64));
    let k_reuse = dfcm_route::repair(&kernel, &mut plan, &mut cursor, 10_000, 64);
    assert_eq!(b_reuse, k_reuse);
    assert_eq!(b_reuse["decision"], json!("reuse_suffix"), "{b_reuse}");

    let blocked = format!("(clear r{k})");
    browser
        .observe(&json!([[blocked, false]]).to_string())
        .unwrap_or_else(|_| panic!("drift must be observable"));
    kernel
        .observe(&[(blocked.as_str(), false)])
        .expect("drift observable");

    let mut full = browser.fork();
    let full = parse(&full.think(10_000, 64));
    assert_eq!(full["solved"], json!(true), "{full}");

    let b_follow = parse(&browser.repair(10_000, 64));
    let k_follow = dfcm_route::repair(&kernel, &mut plan, &mut cursor, 10_000, 64);
    assert_eq!(b_follow, k_follow);
    assert_eq!(
        b_follow["decision"],
        json!("replanned_following"),
        "{b_follow}"
    );
    assert!(
        evaluated(&b_follow["solution"]) < evaluated(&full),
        "browser follow {} must be strictly below full {}",
        evaluated(&b_follow["solution"]),
        evaluated(&full)
    );
    let witness = format!("followed {} still-applicable step(s)", k - 1);
    assert!(
        b_follow["solution"]["notes"]
            .as_array()
            .is_some_and(|notes| notes
                .iter()
                .any(|n| n.as_str().is_some_and(|n| n.contains(&witness)))),
        "missing kept-prefix witness `{witness}`: {b_follow}"
    );
    assert!(browser.valid());
}

/// Probe differential: the browser probe's per-candidate results equal the
/// kernel's, including the budget pass-through (evals=1 must not solve).
#[test]
fn browser_probe_matches_the_kernel_including_the_search_budget() {
    let n = 24;
    let browser = WasmSession::new(fixture::CORRIDOR_DOMAIN, &fixture::corridor_problem(n))
        .unwrap_or_else(|_| panic!("corridor must ground"));
    let kernel = kernel_corridor(n);
    let raw = json!([
        {"id": "blocked", "sight": [[format!("(clear r{})", n / 2), false]]},
        {"id": "at-goal", "sight": [["(at r0)", false], [format!("(at r{n})"), true]]},
        {"id": "seal-only", "restrict_contains": "SEAL"},
        {"id": "contradiction", "sight": [["(at r1)", true], ["(AT  R1)", false]]}
    ]);
    let candidates: Vec<ProbeCandidate> = serde_json::from_value(raw.clone()).unwrap();
    for evals in [1_usize, 10_000] {
        let b = parse(&browser.probe_json(&raw.to_string(), evals, 64));
        let k = dfcm_route::probe_all(&kernel, &candidates, evals, 64);
        assert_eq!(b["results"], json!(k), "evals={evals}");
        let blocked = &b["results"][0];
        if evals == 1 {
            assert_eq!(blocked["outcome"], json!("unsolved"), "{blocked}");
        } else {
            assert_eq!(blocked["outcome"], json!("solved"), "{blocked}");
        }
        assert_eq!(
            b["results"][1]["goal_met_before_search"],
            json!(true),
            "{b}"
        );
        assert_eq!(
            b["results"][0]["goal_met_before_search"],
            json!(false),
            "{b}"
        );
        assert_eq!(b["results"][3]["outcome"], json!("refused"), "{b}");
    }
    assert!(!browser.has_plan() && !browser.goal_met());
}
