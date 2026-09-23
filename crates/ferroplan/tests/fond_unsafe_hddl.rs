//! fond-htn-54: unsafe-states semantics through the HDDL path.
//!
//! `unsafe_states` is exercised in library-level FOND tests only before this
//! file existed — nothing proved the HDDL → ground → translate → solve path
//! honors a never-enter contract (there is no HDDL syntax for "unsafe"; the
//! explicit `PlanningProblem` side is where it lives). This file closes that
//! with hand-authored miniatures (provenance comments in each fixture):
//!
//! (a) **abyss-avoid** — at every safe state a `gamble` action offers the
//!     goal on one `oneof` branch and an unavoidable dead-end fact pattern
//!     (`in-abyss`: zero applicable actions, not the goal) on the other. The
//!     domain is solvable, but only by a policy that never selects a gamble.
//! (b) **abyss-both** — BOTH branches of the only applicable action
//!     dead-end: typed `NoPlan`.
//! (c) the explicit `PlanningProblem` twin whose only route to the goal runs
//!     through a true `unsafe_states` entry — the library refuses it
//!     (`NoPlan` via the unsafe exclusion). This pins the seam ticket
//!     fond-htn-54 asked to verify: `solve_hddl` hardcodes `unsafe_states`
//!     to `Default::default()` (crates/ferroplan/src/hddl.rs), so the
//!     never-enter contract at the HDDL level is exactly dead-sink freedom,
//!     which (a) proves by construction and this file's structural check
//!     walks.
//!
//! No finding (an `#[ignore]`d misbehavior reproducer) was needed: the HDDL
//! path already refuses to gamble into the abyss at the respawn SHA.

use ferroplan::hddl::solve_hddl;
use ferroplan::planning_runtime::{
    Goal, PlannerError, PlanningProblem, State, Transition, UniversalPlanningRequest,
};
use ferroplan::planning_types::PlanningType;
use ferroplan::HddlError;
use std::collections::BTreeSet;

const ABYSS_AVOID_DOMAIN: &str = include_str!("fixtures/fond-unsafe/abyss-avoid.hddl");
const ABYSS_AVOID_PROBLEM: &str = include_str!("fixtures/fond-unsafe/abyss-avoid-problem.hddl");
const ABYSS_BOTH_DOMAIN: &str = include_str!("fixtures/fond-unsafe/abyss-both.hddl");
const ABYSS_BOTH_PROBLEM: &str = include_str!("fixtures/fond-unsafe/abyss-both-problem.hddl");

fn facts(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// (a) The policy must solve while never gambling into the abyss: no policy
/// entry and no plan step may name a `gamble*` action.
#[test]
fn abyss_avoid_solves_without_gambling() {
    let plan = solve_hddl(ABYSS_AVOID_DOMAIN, ABYSS_AVOID_PROBLEM, &Default::default())
        .expect("abyss-avoid must solve by avoiding the gambles");
    assert!(plan.solved);
    assert!(!plan.policy.is_empty(), "the FOND answer is a policy");
    for entry in &plan.policy {
        assert!(
            !entry.action.starts_with("gamble"),
            "the policy must never contain a gamble action (state {})",
            entry.state
        );
    }
    for step in &plan.steps {
        assert!(
            !step.action.starts_with("gamble"),
            "no plan step may gamble (FOND plans normally carry none)"
        );
    }
}

/// Structural dead-sink freedom over the solved (a) policy: every outcome
/// state is either covered by another policy entry or is a terminal leaf,
/// and the number of leaves equals the number of initial states (one
/// accepted terminal per start — the goal). A policy that walked into
/// `in-abyss` would have to either cover the uncovered abyss state (it
/// cannot: nothing is applicable there) or grow a second leaf, and either
/// way `solved` would be false. This is the invariant the HDDL level owes
/// in place of an `unsafe_states` declaration it cannot express.
#[test]
fn solved_policy_is_dead_sink_free() {
    let plan = solve_hddl(ABYSS_AVOID_DOMAIN, ABYSS_AVOID_PROBLEM, &Default::default())
        .expect("abyss-avoid must solve");
    assert!(plan.solved);
    let covered: BTreeSet<&str> = plan.policy.iter().map(|e| e.state.as_str()).collect();
    let mut leaves: BTreeSet<&str> = BTreeSet::new();
    for entry in &plan.policy {
        assert!(
            !entry.outcomes.is_empty(),
            "policy entry for {} has no outcomes",
            entry.state
        );
        for outcome in &entry.outcomes {
            if !covered.contains(outcome.state.as_str()) {
                leaves.insert(outcome.state.as_str());
            }
        }
    }
    assert_eq!(
        leaves.len(),
        1,
        "exactly one accepted terminal leaf is allowed; got {leaves:?}"
    );
    assert!(
        !leaves
            .iter()
            .any(|s| plan.policy.iter().any(|e| &e.state == s)),
        "a leaf must not also be a policy decision state"
    );
}

/// (b) Both branches dead-end: the typed answer is `NoPlan`, never a plan
/// that gambles into the abyss and hopes.
#[test]
fn abyss_both_is_typed_noplan() {
    let result = solve_hddl(ABYSS_BOTH_DOMAIN, ABYSS_BOTH_PROBLEM, &Default::default());
    match result {
        Err(HddlError::Planner(PlannerError::NoPlan)) => {}
        other => panic!("abyss-both must be a typed NoPlan, got {other:?}"),
    }
}

/// (c) The explicit twin: the only route to the goal runs through
/// `s-bridge`, and `s-bridge` is in `unsafe_states` — the library refuses
/// with `NoPlan`. The unsafe exclusion is real, not decoration: the same
/// problem without the unsafe entry solves (asserted in the same test).
#[test]
fn explicit_twin_with_unsafe_state_refuses() {
    let build = || PlanningProblem {
        states: vec![
            State {
                id: "s-start".to_owned(),
                facts: facts(&["at-start"]),
                ..State::default()
            },
            State {
                id: "s-bridge".to_owned(),
                facts: facts(&["on-bridge"]),
                ..State::default()
            },
            State {
                id: "s-goal".to_owned(),
                facts: facts(&["at-goal"]),
                ..State::default()
            },
        ],
        initial_states: vec!["s-start".to_owned()],
        goal: Goal {
            facts: facts(&["at-goal"]),
            ..Goal::default()
        },
        unsafe_states: ["s-bridge".to_owned()].into_iter().collect(),
        transitions: vec![
            Transition {
                action: "cross-to-bridge".to_owned(),
                from: "s-start".to_owned(),
                to: "s-bridge".to_owned(),
                cost: 1,
                duration: 1,
                reward: 0,
                probability_ppm: 1_000_000,
                observation: None,
                requires: BTreeSet::new(),
            },
            Transition {
                action: "cross-to-goal".to_owned(),
                from: "s-bridge".to_owned(),
                to: "s-goal".to_owned(),
                cost: 1,
                duration: 1,
                reward: 0,
                probability_ppm: 1_000_000,
                observation: None,
                requires: BTreeSet::new(),
            },
        ],
        ..PlanningProblem::default()
    };

    match ferroplan::solve_planning_type(&UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: build(),
        limits: Default::default(),
    }) {
        Err(PlannerError::NoPlan) => {}
        other => panic!(
            "the explicit twin whose only route runs through an unsafe state \
             must refuse; got {other:?}"
        ),
    }

    let mut without_unsafe = build();
    without_unsafe.unsafe_states = BTreeSet::new();
    let plan = ferroplan::solve_planning_type(&UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: without_unsafe,
        limits: Default::default(),
    })
    .expect("the same route without the unsafe entry must solve");
    assert!(plan.solved);
}
