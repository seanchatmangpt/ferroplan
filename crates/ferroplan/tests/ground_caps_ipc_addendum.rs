//! Raised-caps re-run of the 17 ground-cap-refused IPC-2023 domains —
//! append-only addendum measurements for ticket
//! `fond-htn-43-ground-caps-plumbing`.
//!
//! `#[ignore]`d long-run (run with
//! `cargo test -p ferroplan --test ground_caps_ipc_addendum -- --ignored
//! --nocapture`); NOT part of CI. It reads its corpus from an **external,
//! read-only** checkout — `/tmp/fond-review/HDDL-Parser/tests/ipc/` —
//! because the committed sweep fixtures (`tests/fixtures/ipc-sweep/`, ticket
//! fond-htn-27) had not landed on this branch's base. Per the KOALA POLICY
//! the corpus is run from `/tmp`, never vendored: these are IPC-2023
//! hierarchical-track competition files (canonical inputs, not koala code),
//! at HDDL-Parser checkout `1f2977eb512f46c69a82fac7a8e9bdcc112c41be`. The
//! 17 (domain, first-listed problem) pairs are exactly the instances ticket
//! fond-htn-27's committed sweep classified
//! `LIMIT:ground-actions` (14) / `LIMIT:ground-methods` (3) — see
//! `fixtures/ipc-sweep/RESULTS.md` once ticket 41 lands it on this line of
//! history; the pairings are copied from that sweep's committed runner so
//! the rows are instance-for-instance comparable.
//!
//! Configuration under test — the raised caps this ticket's plumbing makes
//! expressible through the public `solve_hddl` API:
//!
//! - `PlannerLimits { max_wall_ms: 60_000, max_states: 10_000_000 }`:
//!   ground caps scale to `10_000_000 / 10 = 1_000_000` ground actions and
//!   methods (see `ferroplan::hddl::grounding_limits_from`), the internal
//!   grounding wall follows `max_wall_ms` (60 s), and the solver's own wall
//!   budget is 60 s — all one caller-side declaration, no hard-coded
//!   internals.
//!
//! The translate stage's internal `TranslateLimits::default()` (10 s wall /
//! 200 000 states / depth 64) is deliberately NOT lifted here — that is
//! ticket fond-htn-23's seam, not this one; where it binds, the verbatim
//! refusal is recorded and the row says so.
//!
//! Contract (wave-4 rules: no cherry-picking, honest typed outcomes): every
//! domain yields SOLVED / NOPLAN / `LIMIT:*` / `GAP:*`, printed as
//! machine-readable `ADDENDUM|...` lines for
//! `fixtures/ipc-sweep/RESULTS-wavec.md`. The test panics — and fails —
//  only on contract violations (worker panic, garbage `Ok` with
//! `solved=false`); it exits 0 iff all 17 outcomes are honest typed answers.

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits};
use std::time::Instant;

/// External, read-only corpus root (never copied into the repo).
const CORPUS: &str = "/tmp/fond-review/HDDL-Parser/tests/ipc";

/// Raised caller limits: 60 s walls, ground caps 1 000 000 / 1 000 000.
fn raised_limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 60_000,
        max_states: 10_000_000,
        ..PlannerLimits::default()
    }
}

/// (key, domain dir, first-listed problem file) — identical pairings to the
/// committed fond-htn-27 sweep runner, restricted to the 17 ground-cap
/// refusals.
const CASES: &[(&str, &str, &str)] = &[
    (
        "freecell_learned_ecai_16",
        "Freecell-Learned-ECAI-16",
        "probfreecell-02-1.hddl",
    ),
    ("hiking", "Hiking", "p01.hddl"),
    (
        "minecraft_player",
        "Minecraft-Player",
        "p-003-003-003-003.hddl",
    ),
    (
        "minecraft_regular",
        "Minecraft-Regular",
        "p-003-003-003-003.hddl",
    ),
    (
        "monroe_fo_1",
        "Monroe_FO_1",
        "pfile01-p-0092-set-up-shelter-no-pref-tlt.hddl",
    ),
    (
        "monroe_fo_19",
        "Monroe_FO_19",
        "pfile19-p-0037-clear-road-hazard-3-tlt.hddl",
    ),
    (
        "monroe_fo_2",
        "Monroe_FO_2",
        "pfile02-p-0063-clear-road-wreck-5-tlt.hddl",
    ),
    (
        "monroe_fo_20",
        "Monroe_FO_20",
        "pfile20-p-0037-clear-road-hazard-4-tlt.hddl",
    ),
    (
        "monroe_po_1",
        "Monroe_PO_1",
        "pfile01-p-0014-fix-power-line-4.hddl",
    ),
    (
        "monroe_po_19",
        "Monroe_PO_19",
        "pfile19-p-0084-provide-temp-heat-2.hddl",
    ),
    (
        "monroe_po_2",
        "Monroe_PO_2",
        "pfile02-p-0051-plow-road-3.hddl",
    ),
    (
        "monroe_po_20",
        "Monroe_PO_20",
        "pfile20-p-0080-provide-temp-heat-5.hddl",
    ),
    (
        "po_monroe_po_1",
        "PO_Monroe_PO_1",
        "pfile01-p-0088-quell-riot-1.hddl",
    ),
    (
        "po_monroe_po_2",
        "PO_Monroe_PO_2",
        "pfile02-p-0068-provide-medical-attention-4.hddl",
    ),
    (
        "po_monroe_po_24",
        "PO_Monroe_PO_24",
        "pfile24-p-0059-clear-road-hazard-3.hddl",
    ),
    (
        "po_monroe_po_25",
        "PO_Monroe_PO_25",
        "pfile25-p-0050-clear-road-hazard-2.hddl",
    ),
    ("snake", "Snake", "pb-10slots-seed1.snake.hddl"),
];

/// Classify one `solve_hddl` outcome into the sweep's honest vocabulary.
/// Returns `(outcome, detail)`; `detail` carries the verbatim refusal or
/// plan/policy shape for the addendum row.
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
#[ignore = "long-run external-corpus re-run (ticket fond-htn-43); needs /tmp/fond-review/HDDL-Parser/tests/ipc"]
fn raised_caps_rerun_of_the_17_ground_cap_refused_domains() {
    let mut rows = Vec::new();
    for (key, dir, problem) in CASES {
        let domain_path = format!("{CORPUS}/{dir}/domain.hddl");
        let problem_path = format!("{CORPUS}/{dir}/{problem}");
        let domain_src = std::fs::read_to_string(&domain_path).unwrap_or_else(|e| {
            panic!("external corpus missing ({e}): {domain_path} — see this file's header")
        });
        let problem_src = std::fs::read_to_string(&problem_path).unwrap_or_else(|e| {
            panic!("external corpus missing ({e}): {problem_path} — see this file's header")
        });
        let start = Instant::now();
        let result = solve_hddl(&domain_src, &problem_src, &raised_limits());
        let wall_ms = start.elapsed().as_millis();
        let (outcome, detail) = classify(result);
        println!("ADDENDUM|{key}|{outcome}|{wall_ms}|{detail}");
        rows.push(format!("{key}|{outcome}|{wall_ms}|{detail}"));
    }
    assert_eq!(
        rows.len(),
        CASES.len(),
        "every case must produce exactly one honest row"
    );
}
