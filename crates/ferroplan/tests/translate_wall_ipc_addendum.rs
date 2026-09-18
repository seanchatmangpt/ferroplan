//! Raised-caller-walls re-run of the 9 translate-wall-refused IPC-2023
//! instances — append-only addendum measurements for ticket
//! `fond-htn-65-translate-plumbing`.
//!
//! `#[ignore]`d long-run (run with
//! `cargo test -p ferroplan --test translate_wall_ipc_addendum -- --ignored
//! --nocapture`); NOT part of CI. It reads its corpus from the COMMITTED
//! sweep fixtures (`tests/fixtures/ipc-sweep/`, ticket fond-htn-27) — the
//! same directory ticket fond-htn-43's addendum had to substitute the
//! external `/tmp/fond-review/HDDL-Parser/tests/ipc/` checkout for, because
//! the fixtures had not landed on that branch's base. On this ticket's base
//! they have; every one of the 18 files below was verified byte-identical
//! (`diff`) with that external read-only corpus (IPC-2023 hierarchical-track
//! competition data, never koala code), so these rows are
//! instance-for-instance comparable with BOTH the wave-4 rows in
//! `fixtures/ipc-sweep/RESULTS.md` and the ticket-43 addendum rows in
//! `fixtures/ipc-sweep/RESULTS-wavec.md`.
//!
//! The 9 (domain, first-listed problem) pairs are exactly the instances the
//! wave-4 sweep classified `LIMIT:translate-wall` (ticket fond-htn-23's
//! finding: `solve_hddl_inner` hard-coded `TranslateLimits::default()`, so
//! no caller could lift the internal 10 s translate wall).
//!
//! Configuration under test — the raised caps ticket fond-htn-43's addendum
//! declared, which THIS ticket's plumbing finally lets reach translate
//! through the public `solve_hddl` API:
//!
//! - `PlannerLimits { max_wall_ms: 60_000, max_states: 10_000_000 }`:
//!   `translate_limits_from` derives translate's internal wall = 60 s and
//!   composite-state ceiling = `10_000_000 / 2 = 5_000_000`;
//!   `grounding_limits_from` derives ground caps 1 000 000 / 1 000 000 and
//!   ground wall 60 s; the solver's own wall budget is 60 s; the cumulative
//!   `solve_hddl` watchdog is also 60 s (so a translate phase that eats
//!   most of the minute reports honestly as `LIMIT:pipeline-watchdog`).
//!
//! Contract (wave-4 rules: no cherry-picking, honest typed outcomes): every
//! instance yields SOLVED / NOPLAN / `LIMIT:*` / `GAP:*`, printed as
//! machine-readable `TADDENDUM|...` lines for the
//! `fixtures/ipc-sweep/RESULTS-wavec.md` addendum. The test panics — and
//! fails — only on contract violations (worker panic, garbage `Ok` with
//! `solved=false`); it exits 0 iff all 9 outcomes are honest typed answers.

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits};
use std::time::Instant;

/// Raised caller limits: 60 s walls everywhere, translate state ceiling 5 M,
/// ground caps 1 000 000 / 1 000 000 — one caller-side declaration.
fn raised_limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 60_000,
        max_states: 10_000_000,
        ..PlannerLimits::default()
    }
}

/// (key, domain source, first-listed problem source) — identical pairings to
/// the committed fond-htn-27 sweep runner and ticket fond-htn-43's addendum,
/// restricted to the 9 `LIMIT:translate-wall` instances.
const CASES: &[(&str, &str, &str)] = &[
    (
        "pcp_1",
        include_str!("fixtures/ipc-sweep/PCP_1/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_1/p-pcp01.hddl"),
    ),
    (
        "pcp_16",
        include_str!("fixtures/ipc-sweep/PCP_16/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_16/p-pcp16.hddl"),
    ),
    (
        "pcp_17",
        include_str!("fixtures/ipc-sweep/PCP_17/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_17/p-pcp17.hddl"),
    ),
    (
        "pcp_2",
        include_str!("fixtures/ipc-sweep/PCP_2/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_2/p-pcp02.hddl"),
    ),
    (
        "po_colouring",
        include_str!("fixtures/ipc-sweep/PO_Colouring/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Colouring/pfile01.hddl"),
    ),
    (
        "po_rover",
        include_str!("fixtures/ipc-sweep/PO_Rover/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Rover/pfile01.hddl"),
    ),
    (
        "po_transport",
        include_str!("fixtures/ipc-sweep/PO_Transport/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Transport/pfile01.hddl"),
    ),
    (
        "satellite_gtohp",
        include_str!("fixtures/ipc-sweep/Satellite-GTOHP/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Satellite-GTOHP/p01.hddl"),
    ),
    (
        "transport",
        include_str!("fixtures/ipc-sweep/Transport/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Transport/pfile01.hddl"),
    ),
];

/// Classify one `solve_hddl` outcome into the sweep's honest vocabulary
/// (same shape as `ground_caps_ipc_addendum.rs`, so the two addenda read as
/// one table). Returns `(outcome, detail)`.
fn classify(
    result: Result<ferroplan::planning_runtime::UniversalPlan, HddlError>,
) -> (String, String) {
    match result {
        Ok(plan) => {
            if !plan.solved {
                panic!("contract violation: Ok with solved=false (garbage answer)");
            }
            (
                "SOLVED".to_owned(),
                format!(
                    "plan_len={} policy_entries={}",
                    plan.steps.len(),
                    plan.policy.len()
                ),
            )
        }
        Err(HddlError::Planner(PlannerError::NoPlan)) => ("NOPLAN".to_owned(), String::new()),
        Err(HddlError::Parse(msg)) => ("GAP:parse".to_owned(), msg),
        Err(HddlError::Ground(msg)) => {
            if msg.contains("wall-clock limit exceeded") {
                ("LIMIT:ground-wall".to_owned(), msg)
            } else if msg.contains("max_ground_actions") {
                ("LIMIT:ground-actions".to_owned(), msg)
            } else if msg.contains("max_ground_methods") {
                ("LIMIT:ground-methods".to_owned(), msg)
            } else {
                ("GAP:ground".to_owned(), msg)
            }
        }
        Err(HddlError::Translate(msg)) => {
            if msg.contains("wall-clock limit exceeded") {
                ("LIMIT:translate-wall".to_owned(), msg)
            } else if msg.contains("max_states") || msg.contains("max_task_network_depth") {
                ("LIMIT:translate-cap".to_owned(), msg)
            } else {
                ("GAP:translate".to_owned(), msg)
            }
        }
        Err(HddlError::Planner(PlannerError::Timeout {
            elapsed_ms,
            limit_ms,
        })) => (
            "LIMIT:solve-wall".to_owned(),
            format!("Timeout {{ elapsed_ms: {elapsed_ms}, limit_ms: {limit_ms} }}"),
        ),
        Err(HddlError::Planner(other)) => ("GAP:solve".to_owned(), other.to_string()),
        Err(HddlError::Timeout {
            elapsed_ms,
            limit_ms,
        }) => (
            "LIMIT:pipeline-watchdog".to_owned(),
            format!("Timeout {{ elapsed_ms: {elapsed_ms}, limit_ms: {limit_ms} }}"),
        ),
        Err(HddlError::RootTaskMismatch { .. }) => {
            ("GAP:root-task".to_owned(), "root task mismatch".to_owned())
        }
        Err(HddlError::WorkerPanicked(msg)) => {
            panic!("contract violation: worker panicked: {msg}")
        }
    }
}

#[test]
#[ignore = "long-run raised-walls re-run (ticket fond-htn-65): 9 instances x up to 60 s cumulative"]
fn raised_walls_rerun_of_the_9_translate_wall_instances() {
    let mut rows = Vec::new();
    for (key, domain_src, problem_src) in CASES {
        let start = Instant::now();
        let result = solve_hddl(domain_src, problem_src, &raised_limits());
        let wall_ms = start.elapsed().as_millis();
        let (outcome, detail) = classify(result);
        println!("TADDENDUM|{key}|{outcome}|{wall_ms}|{detail}");
        rows.push(format!("{key}|{outcome}|{wall_ms}|{detail}"));
    }
    assert_eq!(
        rows.len(),
        CASES.len(),
        "every case must produce exactly one honest row"
    );
}
