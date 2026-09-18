//! Grounding-capacity plumbing through the public `solve_hddl` API (ticket
//! `fond-htn-43-ground-caps-plumbing`).
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

/// The historical default envelope, reproduced: `PlannerLimits::default()`
/// grounds `100_000 / 10 = 10_000`-capped, so a domain whose ground-action
/// count crosses 10 000 must still refuse — on the *action* cap, verbatim.
#[test]
fn default_limits_still_refuse_above_the_historical_10k_ground_action_cap() {
    // 105² = 11 025 ground actions > the 10 000 default cap.
    let result = solve_hddl(
        &probe_domain(),
        &probe_problem(105),
        &PlannerLimits::default(),
    );
    match result {
        Err(HddlError::Ground(msg)) => assert!(
            msg.contains("max_ground_actions"),
            "expected the ground-ACTION cap to bind, got: {msg}"
        ),
        other => panic!("expected a ground-cap refusal at default limits, got {other:?}"),
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
