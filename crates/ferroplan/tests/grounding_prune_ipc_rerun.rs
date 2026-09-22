//! Default-caps re-run of the 16 stuck IPC-2023 domains under hierarchical
//! task-relevance pruning — ticket `fond-htn-60-grounding-prune`.
//!
//! `#[ignore]`d long-run (run with
//! `cargo test -p ferroplan --test grounding_prune_ipc_rerun -- --ignored
//! --nocapture`); NOT part of CI. Reads the same **external, read-only**
//! corpus as ticket fond-htn-43's addendum runner
//! (`/tmp/fond-review/HDDL-Parser/tests/ipc/`, IPC-2023 hierarchical-track
//! competition data at HDDL-Parser checkout
//! `1f2977eb512f46c69a82fac7a8e9bdcc112c41be`; run from `/tmp` per the KOALA
//! POLICY, never vendored). The 16 cases are ticket fond-htn-43's 17
//! raised-caps rows **minus `hiking`** — the one domain whose refusal class
//! already moved to the translate seam (ticket fond-htn-23's scope, not a
//! ground-count problem).
//!
//! Two measurements per domain, both honest and both printed:
//!
//! 1. **Envelope probe** (the ticket's core falsifier): `ground` with
//!    `GroundingLimits { prune_irrelevant: true, ..Default::default() }` —
//!    caps at the default 10 000 ground actions / 10 000 ground methods, wall
//!    unbounded for the *count measurement only* (so a wall refusal can never
//!    masquerade as a count failure; `solve_hddl`'s real default walls are
//!    measured separately in (2)). `Ok(ir)` proves the post-relevance ground
//!    counts are under the default envelope; `LimitExceeded`/`Timeout` is
//!    recorded verbatim — a domain where the claim fails.
//! 2. **End-to-end default-caps outcome**: `solve_hddl` at
//!    `PlannerLimits::default()` verbatim (ground caps 10 000 via the
//!    fond-htn-43 plumbing, 10 s walls) — the honest default-configuration
//!    answer, whatever class it lands in.
//!
//! Contract (wave-4 rules: no cherry-picking, honest typed outcomes): rows
//! print as machine-readable `PRUNE|key|actions|methods|outcome|wall_ms|detail`
//! lines for `fixtures/ipc-sweep/RESULTS-wavec.md`; the test panics — and
//! fails — only on contract violations (missing corpus, worker panic, garbage
//! `Ok` with `solved=false`), never on an honest limit refusal. It exits 0
//! iff all 16 domains produced their two honest rows.

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, UniversalPlan};
use ferroplan_hddl::grounder::GroundingLimits;
use std::time::Instant;

/// External, read-only corpus root (never copied into the repo).
const CORPUS: &str = "/tmp/fond-review/HDDL-Parser/tests/ipc";

/// The 16 domains whose true ground-instance counts exceed 1 000 000 (ticket
/// fond-htn-43 addendum) — identical (domain dir, first-listed problem)
/// pairings to that addendum's runner, minus `hiking`.
const CASES: &[(&str, &str, &str)] = &[
    (
        "freecell_learned_ecai_16",
        "Freecell-Learned-ECAI-16",
        "probfreecell-02-1.hddl",
    ),
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

fn classify_solve(result: Result<UniversalPlan, HddlError>) -> (String, String) {
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
        Err(HddlError::Planner(PlannerError::Timeout {
            elapsed_ms,
            limit_ms,
        })) => (
            "LIMIT:solve-wall".to_owned(),
            format!("Timeout {{ elapsed_ms: {elapsed_ms}, limit_ms: {limit_ms} }}"),
        ),
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
#[ignore = "long-run external-corpus re-run (ticket fond-htn-60); needs /tmp/fond-review/HDDL-Parser/tests/ipc"]
fn default_caps_rerun_of_the_16_stuck_domains_under_relevance_pruning() {
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

        // (1) Envelope probe: counts under the default 10k envelope?
        let probe_start = Instant::now();
        let domain = ferroplan_hddl::parser::parse_domain(&domain_src)
            .unwrap_or_else(|e| panic!("{key}: corpus domain must parse: {e}"));
        let prob = ferroplan_hddl::parser::parse_problem(&problem_src)
            .unwrap_or_else(|e| panic!("{key}: corpus problem must parse: {e}"));
        let probe_limits = GroundingLimits {
            prune_irrelevant: true,
            // Unbounded wall for the *count* measurement only — a wall
            // refusal must never masquerade as a count failure. The
            // end-to-end default walls are measured honestly by (2).
            max_wall: None,
            ..GroundingLimits::default()
        };
        let (probe_outcome, probe_detail) = match ferroplan_hddl::grounder::ground(
            &domain,
            &prob,
            &probe_limits,
        ) {
            Ok(ir) => (
                format!("UNDER_ENVELOPE a={} m={}", ir.actions.len(), ir.methods.len()),
                String::new(),
            ),
            Err(e) => ("OVER_ENVELOPE".to_owned(), e.to_string()),
        };
        let probe_wall = probe_start.elapsed().as_millis();
        println!("PROBE|{key}|{probe_outcome}|{probe_wall}|{probe_detail}");

        // (2) End-to-end default-caps outcome through the public pipeline.
        let solve_start = Instant::now();
        let result = solve_hddl(&domain_src, &problem_src, &ferroplan::planning_runtime::PlannerLimits::default());
        let wall_ms = solve_start.elapsed().as_millis();
        let (outcome, detail) = classify_solve(result);
        println!("PRUNE|{key}|{outcome}|{wall_ms}|{detail}");
        rows.push(format!("{key}|{outcome}|{wall_ms}|{detail}"));
    }
    assert_eq!(
        rows.len(),
        CASES.len(),
        "every case must produce exactly one honest row"
    );
}
