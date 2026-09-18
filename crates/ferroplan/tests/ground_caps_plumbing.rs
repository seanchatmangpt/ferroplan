//! Grounding-capacity plumbing through the public `solve_hddl` API (tickets
//! `fond-htn-43-ground-caps-plumbing` and `fond-htn-60-grounding-prune`).
//!
//! Before this plumbing, `solve_hddl_inner` hard-coded
//! `GroundingLimits::default()` (10 000 ground actions / 10 000 ground
//! methods / 10 s internal wall), so a caller could never raise the
//! grounding caps — 17 IPC-2023 domains refused on exactly that cap in the
//! wave-4 sweep (ticket fond-htn-27) while refusing to make any progress.
//! The plumbing derives `GroundingLimits` from the caller's
//! [`PlannerLimits`] (see `ferroplan::hddl::grounding_limits_from`): both
//! instance caps scale as `max_states / 10` — calibrated so
//! `PlannerLimits::default()` reproduces the historical envelope exactly —
//! and the internal ground wall follows `max_wall_ms` (0 = unbounded).
//!
//! Ticket fond-htn-60 changed WHAT the caps bound on this pipeline:
//! `solve_hddl` now runs hierarchical task-relevance pruning
//! (`prune_irrelevant`), so the caps bind the *relevant* ground-instance
//! count — instances on some decomposition path from the root task
//! network(s) — not the raw combinatorial count. A domain whose RAW count
//! crosses 10 000 but whose RELEVANT count is tiny now grounds and solves
//! at default limits (previously a verbatim cap refusal); this is sound
//! because relevance pruning provably cannot change the translated problem
//! (see the grounder's module docs), and it is exactly the lever that
//! brings the 16 stuck IPC-2023 domains (true raw counts > 1 000 000) back
//! under the default envelope. The caps still bind — on the relevant
//! count — when a domain genuinely demands more instances than the caller
//! declared appetite allows. Both directions are pinned below.
//!
//! The probe fixture below is hand-authored (no external provenance) and
//! deliberately shaped so grounding is the binding stage while everything
//! downstream stays trivial: one 2-parameter action over N objects yields
//! N² ground actions, but the reachable state space is 2–3 states (the
//! effect is a single fact), so translate + solve are instant once
//! grounding is allowed to finish.

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::PlannerLimits;

/// Hand-authored cap-probe domain: `probe` grounds once per ordered object
/// pair (N² instances), `probe-task` decomposes it exactly once (the second
/// method covers the already-done state, so no recursion), and the only
/// effect is the single fact `(done)` — N² grounding work, O(1) search.
fn probe_domain() -> String {
    // hand-authored (ticket fond-htn-43); no external provenance
    r#";; hand-authored cap-probe domain (ticket fond-htn-43)
(define (domain ground-caps-probe)
  (:types loc)
  (:predicates
    (ready)
    (done))
  (:task probe-task :parameters (?x ?y - loc))
  (:action probe
    :parameters (?x ?y - loc)
    :precondition (ready)
    :effect (done))
  (:method probe-once
    :parameters (?x ?y - loc)
    :task (probe-task ?x ?y)
    :precondition (and (ready))
    :ordered-subtasks (and (t1 (probe ?x ?y))))
  (:method probe-done
    :parameters (?x ?y - loc)
    :task (probe-task ?x ?y)
    :precondition (and (done))
    :ordered-subtasks (and)))
"#
    .to_owned()
}

/// Hand-authored probe problem over `n` objects: N² ground actions, N²·2
/// ground methods (two methods × all pairs), one-step plan to `(done)`.
fn probe_problem(n: usize) -> String {
    let objects = (0..n)
        .map(|i| format!("l{i}"))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        ";; hand-authored cap-probe problem (ticket fond-htn-43)\n\
         (define (problem ground-caps-probe-p1)\n\
         \x20 (:domain ground-caps-probe)\n\
         \x20 (:objects {objects} - loc)\n\
         \x20 (:htn :ordered-subtasks (and (g1 (probe-task l0 l1))))\n\
         \x20 (:init (ready))\n\
         \x20 (:goal (and (done))))\n"
    )
}

/// The raw combinatorial count alone no longer refuses (ticket fond-htn-60,
/// re-pinned from fond-htn-43's "raw count > 10k refuses"): this probe's raw
/// grounding is 105² = 11 025 actions + 22 050 methods — both above the
/// default 10 000 caps — but its root network demands exactly one
/// `probe-task(l0,l1)`, so the RELEVANT count is 1 action + 2 methods and
/// the domain now grounds and solves at plain `PlannerLimits::default()`.
/// The pruned instances are provably irrelevant: no decomposition of the
/// root network can ever call any `probe(li,lj)` with (li,lj) ≠ (l0,l1),
/// and `translate` never leaves the decomposition closure, so the answer
/// (modulo the cap refusal it replaces) is unchanged.
#[test]
fn raw_count_above_the_cap_no_longer_refuses_when_the_relevant_count_fits() {
    let plan = solve_hddl(
        &probe_domain(),
        &probe_problem(105),
        &PlannerLimits::default(),
    )
    .expect("raw 11k/22k counts but 1 relevant action: must solve at defaults");
    assert!(plan.solved, "probe domain (relevant count 1) must solve");
    assert!(
        !plan.policy.is_empty(),
        "solved probe domain must carry a non-empty policy"
    );
}

/// The caps still bind — on the RELEVANT count: with `:htn :parameters`,
/// every admissible binding is its own root network, so demanding
/// `probe-task(li,lj)` for all 105² pairs makes the relevant action count
/// 11 025 > 10 000 and the default envelope must refuse, verbatim on the
/// *action* cap (ground_actions enumerates before ground_methods, and the
/// relevant method count is even higher, but the action cap fires first).
#[test]
fn caps_still_bind_when_the_relevant_count_exceeds_the_envelope() {
    let objects = (0..105)
        .map(|i| format!("l{i}"))
        .collect::<Vec<_>>()
        .join(" ");
    let problem = format!(
        ";; hand-authored cap-probe problem (ticket fond-htn-60, relevant-count direction)\n\
         (define (problem ground-caps-probe-demand-all)\n\
         \x20 (:domain ground-caps-probe)\n\
         \x20 (:objects {objects} - loc)\n\
         \x20 (:htn :parameters (?x ?y - loc)\n\
         \x20       :ordered-subtasks (and (g (probe-task ?x ?y))))\n\
         \x20 (:init (ready))\n\
         \x20 (:goal (and (done))))\n"
    );
    let result = solve_hddl(&probe_domain(), &problem, &PlannerLimits::default());
    match result {
        Err(HddlError::Ground(msg)) => assert!(
            msg.contains("max_ground_actions"),
            "expected the ground-ACTION cap to bind on the relevant count, got: {msg}"
        ),
        other => panic!(
            "expected a ground-cap refusal when the RELEVANT count exceeds the envelope, got {other:?}"
        ),
    }
}

/// The point of the plumbing: the same domain *solves end-to-end* once the
/// caller raises `max_states` (ground caps scale as `max_states / 10`, so
/// 1 000 000 → 100 000 ground instances — comfortably above the domain's
/// 11 025 ground actions and 22 050 ground methods). Solved through the
/// public API with no fixture-level grounder access.
#[test]
fn raised_max_states_lifts_the_ground_cap_and_solves_via_the_public_api() {
    let raised = PlannerLimits {
        max_states: 1_000_000,
        ..PlannerLimits::default()
    };
    let plan = solve_hddl(&probe_domain(), &probe_problem(105), &raised)
        .expect("raised ground caps must let the probe domain solve");
    assert!(plan.solved, "probe domain must solve under raised caps");
    assert!(
        !plan.policy.is_empty(),
        "solved probe domain must carry a non-empty policy"
    );
}

/// Positive control for "default behavior unchanged": a domain comfortably
/// under the historical caps still solves under plain
/// `PlannerLimits::default()` (50² = 2 500 actions, 5 000 methods).
#[test]
fn domains_under_the_historical_caps_still_solve_at_default_limits() {
    let plan = solve_hddl(
        &probe_domain(),
        &probe_problem(50),
        &PlannerLimits::default(),
    )
    .expect("sub-cap domain must solve unchanged at default limits");
    assert!(plan.solved, "probe domain (n=50) must solve at defaults");
    assert!(
        !plan.policy.is_empty(),
        "solved probe domain must carry a non-empty policy"
    );
}
