//! Translate-capacity plumbing through the public `solve_hddl` API (ticket
//! `fond-htn-65-translate-plumbing`, superseding the plumbing half of the
//! never-landed ticket `fond-htn-23-translate-capacity`).
//!
//! Before this plumbing, `solve_hddl_inner` hard-coded
//! `TranslateLimits::default()` (10 s wall / 200 000 composite states /
//! depth 64), so a caller could never lift the translate wall — 9 IPC-2023
//! instances refused on exactly that internal 10 s wall in the wave-4 sweep
//! (ticket fond-htn-27) while the koala oracle solved four of them in under
//! 1.3 s (ticket fond-htn-23's finding). The plumbing derives
//! `TranslateLimits` from the caller's [`PlannerLimits`] (see
//! `ferroplan::hddl::translate_limits_from`): the composite-state ceiling
//! scales as `max_states / 2` — calibrated so `PlannerLimits::default()`
//! reproduces the historical envelope exactly — and the internal translate
//! wall follows `max_wall_ms` (0 = unbounded), mirroring
//! `ferroplan::hddl::grounding_limits_from` (ticket fond-htn-43).
//!
//! Default behavior is pinned byte-identical by the existing suites —
//! `tests/htn_ipc2023.rs` asserts the four `TIMEOUT @ 10 s` /
//! `LIMIT:translate-wall` outcomes verbatim and `tests/ipc_sweep.rs`
//! re-derives six `solve_hddl` defaults per gate run — both green in this
//! ticket's gates.
//!
//! The probe fixture below is hand-authored (no external provenance) and
//! deliberately shaped so *translate* is the only expensive stage. A chain
//! of decomposition tasks `t{K-1} .. t0` where every `t{i}` offers TWO
//! methods, each refining to `t{i-1}` and stamping a distinct
//! decomposition-time effect BLOCK of PROBE_BLOCK literals (`ma{i}_*` vs
//! `mb{i}_*`, the ticket fond-htn-24 construct): every method-choice path
//! leaves a distinguishable composite state (2^K paths), and translate's
//! per-transition intern key must clone every block chosen so far — the
//! `intern_state` clone-and-compare work ticket fond-htn-23 profiled as
//! translate's hot spot, here amplified per state instead of per state
//! count, so the BFS stays SMALL (the solver's graph) while its per-step
//! cost is what crosses the 10 s wall. Only the all-a corridor reaches
//! `(goal_f)` (`t0_a` stamps it, `t0_b` does not), so the solver's strong
//! fixpoint converges over a graph with diameter ~K. The knobs were
//! calibrated in a debug build (the gate profile) so translate alone needs
//! beyond 10 s — strictly past the historical default wall — while the
//! whole test stays under the 30 s un-ignored bound.

use ferroplan::hddl::solve_hddl;
use ferroplan::planning_runtime::PlannerLimits;

/// Decomposition levels (method-choice depth): 2^K composite-state paths.
/// Calibrated in a debug build on the gate machine (Apple M3 Max,
/// 2026-09-18): K=8/B=3000 measures parse ~0.2 s + ground ~0.2 s +
/// translate ~14-16 s ALONE (the historical default wall fires at ~10.1 s
/// with 465 states interned / 464 transitions / 233 queued) + solve
/// ~0.06 s — past the 10 s wall with margin, under the 30 s bound with
/// margin.
const PROBE_LEVELS: usize = 8;
/// Effect literals per method per level: the per-intern clone cost lever.
const PROBE_BLOCK: usize = 3000;

/// Hand-authored translate-wall probe domain: `t{i}` (i > 0) has two
/// methods, each refining to `t{i-1}` and stamping a distinct effect block;
/// `t0_a` terminates stamping its block PLUS `(goal_f)`, `t0_b` terminates
/// without the goal fact.
fn probe_domain() -> String {
    // hand-authored (ticket fond-htn-65); no external provenance
    let k = PROBE_LEVELS;
    let b = PROBE_BLOCK;
    let block = |pfx: &str, i: usize| -> String {
        (0..b)
            .map(|j| format!("({pfx}{i}_{j})"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let mut preds: Vec<String> = vec!["(goal_f)".to_owned()];
    let mut tasks: Vec<String> = Vec::new();
    let mut methods: Vec<String> = Vec::new();
    for i in 0..k {
        for j in 0..b {
            preds.push(format!("(ma{i}_{j})"));
            preds.push(format!("(mb{i}_{j})"));
        }
        tasks.push(format!("  (:task t{i} :parameters ())\n"));
        let (sub, eff_a, eff_b) = if i > 0 {
            (
                format!("(and (t{}))", i - 1),
                format!("(and {})", block("ma", i)),
                format!("(and {})", block("mb", i)),
            )
        } else {
            (
                "(and)".to_owned(),
                format!("(and {} (goal_f))", block("ma", i)),
                format!("(and {})", block("mb", i)),
            )
        };
        methods.push(format!(
            "  (:method t{i}_a :task (t{i}) :effect {eff_a}\n    :ordered-subtasks {sub})\n"
        ));
        methods.push(format!(
            "  (:method t{i}_b :task (t{i}) :effect {eff_b}\n    :ordered-subtasks {sub})\n"
        ));
    }
    format!(
        ";; hand-authored translate-wall probe domain (ticket fond-htn-65)\n\
         (define (domain translate-wall-probe)\n\
         \x20 (:predicates {})\n\
         {}{}\
         )\n",
        preds.join(" "),
        tasks.join(""),
        methods.join(""),
    )
}

/// Hand-authored probe problem: the root network is the top of the task
/// chain; goal `(goal_f)` is added by the `t0_a` terminal method, so the
/// all-a decomposition reaches it (each `t0_b` path ends goal-less).
fn probe_problem() -> String {
    let k = PROBE_LEVELS;
    format!(
        ";; hand-authored translate-wall probe problem (ticket fond-htn-65)\n\
         (define (problem translate-wall-probe-p1)\n\
         \x20 (:domain translate-wall-probe)\n\
         \x20 (:htn :ordered-subtasks (and (g1 (t{}))))\n\
         \x20 (:init)\n\
         \x20 (:goal (and (goal_f))))\n",
        k - 1
    )
}

/// The point of the plumbing: the same probe that used to die on
/// translate's internal 10 s wall now *solves end-to-end* once the caller
/// raises `max_wall_ms` — translate demonstrably runs past 10 s (the
/// debug-build wall assert below; a 10 s-capped translate cannot produce a
/// translate phase beyond 10 s), hands the graph to the solver, and the
/// plan comes back solved through the public API alone.
#[test]
fn raised_max_wall_ms_lifts_the_translate_wall_end_to_end() {
    let raised = PlannerLimits {
        max_wall_ms: 60_000,
        ..PlannerLimits::default()
    };
    let start = std::time::Instant::now();
    let plan = solve_hddl(&probe_domain(), &probe_problem(), &raised)
        .expect("raised caller wall must lift translate's internal 10 s default wall");
    let wall_ms = start.elapsed().as_millis();
    println!(
        "translate-plumbing|probe|solved|wall_ms={wall_ms}|levels={PROBE_LEVELS}|block={PROBE_BLOCK}"
    );
    assert!(
        plan.solved,
        "probe domain must solve end-to-end under the raised caller wall"
    );
    assert!(
        !plan.policy.is_empty(),
        "solved probe domain must carry a non-empty policy"
    );
    // The lift proof, in the profile the gates run (debug): the probe needs
    // > 10 s of translate alone (calibrated; see the module docs), so a
    // solve_hddl still capped at the historical `TranslateLimits::default()`
    // 10 s wall could not have produced this `Ok`. In a release build the
    // same probe translates in well under 10 s and only the end-to-end
    // assertions above are meaningful (the `ground_wall.rs` convention of
    // gating timing bounds on `cfg!(debug_assertions)`).
    if cfg!(debug_assertions) {
        assert!(
            wall_ms > 10_000,
            "probe must need > 10 s of translate in a debug build to prove the \
             wall was lifted past the historical default; took {wall_ms} ms — \
             re-calibrate PROBE_LEVELS/PROBE_BLOCK on this machine"
        );
        assert!(
            wall_ms < 30_000,
            "probe took {wall_ms} ms — over the 30 s un-ignored test bound; \
             re-calibrate PROBE_LEVELS/PROBE_BLOCK or #[ignore] this test per \
             the ticket"
        );
    }
}
