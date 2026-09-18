//! Hand-authored FOND-HTN micro-domain regression suite for `solve_hddl`
//! (ticket `fond-htn-12`, wave v26.9.17 FOND-HTN hardening).
//!
//! Every fixture lives in [`fixtures/fond-htn-micro/`] and carries a one-line
//! provenance comment (`hand-authored, <pattern>-pattern`) — no koala files
//! are vendored (koala is an external test oracle only). The suite pins the
//! end-to-end `parse -> ground -> translate -> fond_policy` pipeline of
//! [`ferroplan::hddl::solve_hddl`] against one micro fixture per FOND-HTN
//! pattern:
//!
//! | fixture | pattern | property pinned |
//! |---|---|---|
//! | `drop-retry` | Transport | oneof success/empty-branch; empty branch is a real, no-change outcome; retry loop solves only via the strong-cyclic fallback |
//! | `tray-dirty` | Childsnack | two overlapping (recovery-converging) branches; each dirt type routes to its own recovery method |
//! | `sense-then-branch` | Satellite | a sensed `oneof` outcome selects a *different* method for the same abstract task |
//! | `grow-loop` | Snake | a genuinely recursive method terminates within the wall budget (bounded unrolling) |
//! | `supervisor-fail` | Depots | failure outcome routes to the *second* method, which retries the root task |
//! | `plain-chain` | deterministic | baseline: every policy entry has exactly one outcome |
//! | `both-branches-deadend` | negative | both `oneof` branches dead-end; typed `Planner(NoPlan)`, and the translated graph proves the dead-ends |
//!
//! No-panic/no-hang contract: every `solve_hddl` call in this file passes a
//! `PlannerLimits` with a non-zero `max_wall_ms` — `solve_hddl`'s own
//! watchdog bounds the caller's wait regardless of which pipeline phase is
//! slow (see `HddlError::Timeout`).

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits, PolicyEntry, UniversalPlan};
use ferroplan::planning_types::PlanningType;
use ferroplan_hddl::translate::PlanningProblem as FlatProblem;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

/// Wall budget handed to every `solve_hddl` call here (milliseconds). These
/// fixtures are tiny; the budget exists for the no-panic/no-hang contract,
/// not because the work needs it.
const WALL_BUDGET_MS: u64 = 10_000;

fn limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: WALL_BUDGET_MS,
        ..PlannerLimits::default()
    }
}

const DROP_RETRY: (&str, &str) = (
    include_str!("fixtures/fond-htn-micro/drop-retry/domain.hddl"),
    include_str!("fixtures/fond-htn-micro/drop-retry/problem.hddl"),
);
const TRAY_DIRTY: (&str, &str) = (
    include_str!("fixtures/fond-htn-micro/tray-dirty/domain.hddl"),
    include_str!("fixtures/fond-htn-micro/tray-dirty/problem.hddl"),
);
const SENSE_THEN_BRANCH: (&str, &str) = (
    include_str!("fixtures/fond-htn-micro/sense-then-branch/domain.hddl"),
    include_str!("fixtures/fond-htn-micro/sense-then-branch/problem.hddl"),
);
const GROW_LOOP: (&str, &str) = (
    include_str!("fixtures/fond-htn-micro/grow-loop/domain.hddl"),
    include_str!("fixtures/fond-htn-micro/grow-loop/problem.hddl"),
);
const SUPERVISOR_FAIL: (&str, &str) = (
    include_str!("fixtures/fond-htn-micro/supervisor-fail/domain.hddl"),
    include_str!("fixtures/fond-htn-micro/supervisor-fail/problem.hddl"),
);
const PLAIN_CHAIN: (&str, &str) = (
    include_str!("fixtures/fond-htn-micro/plain-chain/domain.hddl"),
    include_str!("fixtures/fond-htn-micro/plain-chain/problem.hddl"),
);
const BOTH_BRANCHES_DEADEND: (&str, &str) = (
    include_str!("fixtures/fond-htn-micro/both-branches-deadend/domain.hddl"),
    include_str!("fixtures/fond-htn-micro/both-branches-deadend/problem.hddl"),
);

/// Rebuilds the flat (ground-IR) state space through the same public
/// `ferroplan_hddl` pipeline `solve_hddl` drives internally. Needed because
/// `UniversalPlan` carries only state *ids*: checking that a terminal policy
/// outcome actually satisfies the goal (outcome closure) requires the states'
/// fact sets and the translated goal, which is exactly `translate`'s output
/// (including the synthetic `htn:done` requirement folded into `goal.facts`).
fn flat_problem(domain: &str, problem: &str) -> FlatProblem {
    let domain = ferroplan_hddl::parser::parse_domain(domain).expect("fixture domain parses");
    let problem = ferroplan_hddl::parser::parse_problem(problem).expect("fixture problem parses");
    let ir = ferroplan_hddl::grounder::ground(&domain, &problem, &Default::default())
        .expect("fixture grounds");
    ferroplan_hddl::translate::translate(&ir, &Default::default()).expect("fixture translates")
}

/// Solves a fixture and asserts the no-hang contract on the way through.
/// Returns the plan, the flat state space, and the elapsed milliseconds.
fn solved(fixture: &(&str, &str)) -> (UniversalPlan, FlatProblem, u128) {
    let start = Instant::now();
    let plan =
        solve_hddl(fixture.0, fixture.1, &limits()).expect("fixture must solve within budget");
    let elapsed = start.elapsed().as_millis();
    assert!(
        elapsed < u128::from(WALL_BUDGET_MS),
        "no-hang contract: solve exceeded its {WALL_BUDGET_MS}ms budget ({elapsed}ms)"
    );
    (plan, flat_problem(fixture.0, fixture.1), elapsed)
}

/// A ground-fact string never contains `:` (real facts are `pred(args)`
/// with plain identifiers; every synthetic marker — `htn:done`,
/// `htn-frontier-canon:…`, `not:…`, `goal:reached` — does), so filtering on
/// `:` yields exactly the world facts of a composite state.
fn world_facts(facts: &BTreeSet<String>) -> BTreeSet<&str> {
    facts
        .iter()
        .map(String::as_str)
        .filter(|f| !f.contains(':'))
        .collect()
}

/// The result of the outcome-closure check, exposing lookups tests need for
/// fixture-specific assertions on top.
struct Closure<'a> {
    /// Chosen action per policy state.
    by_state: BTreeMap<&'a str, &'a PolicyEntry>,
    /// Flat-problem fact set per state id.
    states: BTreeMap<&'a str, &'a BTreeSet<String>>,
    goal: &'a BTreeSet<String>,
    initial: &'a str,
    /// Every state id reachable from the initial state by following *all*
    /// outcomes of the policy's chosen actions.
    reached: BTreeSet<&'a str>,
}

impl<'a> Closure<'a> {
    fn state_facts(&self, state: &str) -> &'a BTreeSet<String> {
        self.states[state]
    }

    fn entry(&self, state: &str) -> &'a PolicyEntry {
        self.by_state[state]
    }

    /// The unique policy entry whose chosen action's name contains `needle`
    /// (grounded action/method names like `:drop(d1)` / `:op-fast()` are
    /// unique per fixture here).
    fn entry_action_containing(&self, needle: &str) -> Option<&'a PolicyEntry> {
        self.by_state
            .values()
            .copied()
            .find(|e| e.action.contains(needle))
    }
}

/// Outcome-closure checker. From the flat problem's initial state, walk EVERY
/// outcome of the policy's chosen action at each reached state; assert:
/// 1. every outcome names a known flat state (no dangling ids),
/// 2. every outcome probability mass per entry sums to exactly 1_000_000,
/// 3. no outcome collapsing happened (every non-goal reached state keeps a
///    policy entry with a non-empty outcome set),
/// 4. every execution the policy admits eventually reaches a goal state —
///    i.e. every reached state either satisfies the translated goal or has a
///    chosen continuation, and at least one reached state is a goal state.
fn assert_outcome_closed<'a>(plan: &'a UniversalPlan, flat: &'a FlatProblem) -> Closure<'a> {
    assert!(plan.solved, "expected plan.solved");
    assert!(
        matches!(plan.planning_type, Some(PlanningType::Fond)),
        "solve_hddl must report PlanningType::Fond honestly, got {:?}",
        plan.planning_type
    );
    let states: BTreeMap<&str, &BTreeSet<String>> = flat
        .states
        .iter()
        .map(|s| (s.id.as_str(), &s.facts))
        .collect();
    let by_state: BTreeMap<&str, &PolicyEntry> =
        plan.policy.iter().map(|e| (e.state.as_str(), e)).collect();
    let goal = &flat.goal.facts;
    let initial: &str = flat.initial_states[0].as_str();

    let mut reached: BTreeSet<&str> = BTreeSet::new();
    let mut queue: VecDeque<&str> = VecDeque::from([initial]);
    let mut reached_goal = false;
    while let Some(state) = queue.pop_front() {
        if !reached.insert(state) {
            continue;
        }
        let facts = states
            .get(state)
            .unwrap_or_else(|| panic!("policy outcome names unknown flat state '{state}'"));
        if goal.is_subset(facts) {
            reached_goal = true;
            continue;
        }
        let entry = by_state.get(state).unwrap_or_else(|| {
            panic!(
                "state '{state}' is reachable under the policy but has no policy entry \
                 and is not a goal state — outcome closure violated"
            )
        });
        assert!(
            !entry.outcomes.is_empty(),
            "policy entry at '{state}' has zero outcomes (outcome collapsing?)"
        );
        let mass: u32 = entry.outcomes.iter().map(|o| o.probability_ppm).sum();
        assert_eq!(
            mass, 1_000_000,
            "outcome probability mass at '{state}' must sum to exactly 1_000_000"
        );
        for outcome in &entry.outcomes {
            queue.push_back(outcome.state.as_str());
        }
    }
    assert!(
        reached_goal,
        "policy never reaches a goal state from the initial state"
    );
    Closure {
        by_state,
        states,
        goal,
        initial,
        reached,
    }
}

/// Transport pattern: `drop`'s oneof is success vs an EMPTY branch. The empty
/// branch must survive as a real, no-change outcome (no collapsing with the
/// success branch), loop back into the initial composite state, and the
/// resulting retry loop must solve — which only the strong-cyclic fallback
/// can express (`fond_policy`'s least fixpoint cannot admit self-loops).
#[test]
fn drop_retry_solves_with_a_real_no_change_retry_outcome() {
    let (plan, flat, _elapsed) = solved(&DROP_RETRY);
    assert!(
        plan.notes.iter().any(|n| n.contains("strong-cyclic")),
        "a retry loop must route through the strong-cyclic fallback, notes: {:?}",
        plan.notes
    );
    let cc = assert_outcome_closed(&plan, &flat);

    let drop = cc
        .entry_action_containing(":drop(")
        .expect("policy must choose the drop action");
    assert!(drop.action.starts_with("htn:exec:"));
    // Branch count preserved: oneof success/empty-branch -> exactly 2 outcomes.
    assert_eq!(
        drop.outcomes.len(),
        2,
        "oneof branch count must be preserved (no outcome collapsing)"
    );

    let entry_world = world_facts(cc.state_facts(&drop.state));
    let no_change: Vec<_> = drop
        .outcomes
        .iter()
        .filter(|o| world_facts(cc.state_facts(&o.state)) == entry_world)
        .collect();
    let changed: Vec<_> = drop
        .outcomes
        .iter()
        .filter(|o| world_facts(cc.state_facts(&o.state)) != entry_world)
        .collect();
    assert_eq!(
        no_change.len(),
        1,
        "exactly one outcome must be the no-change (empty-branch) outcome"
    );
    assert_eq!(
        changed.len(),
        1,
        "exactly one outcome must be the success branch"
    );
    // The empty branch re-enters the initial composite state: the retry loop.
    assert_eq!(
        no_change[0].state, cc.initial,
        "empty branch must loop back to the initial composite state"
    );
    // The success branch genuinely changed the world (package delivered).
    assert!(
        cc.state_facts(&changed[0].state).contains("done"),
        "success outcome must add (done), got {:?}",
        cc.state_facts(&changed[0].state)
    );
}

/// Childsnack pattern: `place`'s two branches overlap (both delete
/// `(clean)`, both recover through a wash-like action back to the same
/// clean-and-serve continuation). Each landing state must keep its own
/// recovery method, and the whole policy must be outcome-closed to the goal.
#[test]
fn tray_dirty_overlapping_branches_each_recover_via_their_own_method() {
    let (plan, flat, _elapsed) = solved(&TRAY_DIRTY);
    let cc = assert_outcome_closed(&plan, &flat);

    let place = cc
        .entry_action_containing(":place")
        .expect("policy must choose the place action");
    assert_eq!(
        place.outcomes.len(),
        2,
        "two overlapping branches, no collapsing"
    );
    assert_ne!(
        place.outcomes[0].state, place.outcomes[1].state,
        "the two branches must land in distinct states"
    );
    for outcome in &place.outcomes {
        let facts = cc.state_facts(&outcome.state);
        let (dirt, method) = if facts.contains("dirty") {
            ("dirty", "serve-dirty")
        } else if facts.contains("grubby") {
            ("grubby", "serve-grubby")
        } else {
            panic!("place outcome is neither dirty nor grubby: {facts:?}")
        };
        assert!(
            !cc.goal.is_subset(facts),
            "landing in {dirt} must not already satisfy the goal"
        );
        assert!(
            cc.entry(&outcome.state).action.contains(method),
            "{dirt} branch must recover via {method}, got {}",
            cc.entry(&outcome.state).action
        );
    }
}

/// Satellite pattern: the sensed `oneof` (`sense-mode`) does not itself
/// progress the goal — it selects WHICH method decomposes the same abstract
/// `operate` task afterwards. Both sensed branches must stay alive and route
/// to their own scan method.
#[test]
fn sense_then_branch_decomposes_differently_per_sensed_outcome() {
    let (plan, flat, _elapsed) = solved(&SENSE_THEN_BRANCH);
    let cc = assert_outcome_closed(&plan, &flat);

    let sense = cc
        .entry_action_containing(":sense-mode")
        .expect("policy must choose the sensing action");
    assert_eq!(
        sense.outcomes.len(),
        2,
        "two sensed branches, no collapsing"
    );
    assert_ne!(sense.outcomes[0].state, sense.outcomes[1].state);
    let mut saw_fast = false;
    let mut saw_slow = false;
    for outcome in &sense.outcomes {
        let facts = cc.state_facts(&outcome.state);
        let chosen = &cc.entry(&outcome.state).action;
        if facts.contains("mode-fast") {
            assert!(
                chosen.contains("op-fast"),
                "fast-sensed state must decompose via op-fast, got {chosen}"
            );
            saw_fast = true;
        } else if facts.contains("mode-slow") {
            assert!(
                chosen.contains("op-slow"),
                "slow-sensed state must decompose via op-slow, got {chosen}"
            );
            saw_slow = true;
        } else {
            panic!("sensed outcome carries neither mode fact: {facts:?}")
        }
    }
    assert!(
        saw_fast && saw_slow,
        "both sensed branches must remain alive"
    );
}

/// Snake pattern: `build` decomposes via `grow`, which re-invokes `build` as
/// its own subtask — a genuinely recursive method. The recursion must unwind
/// through the `stop` base case and terminate within the wall budget, with
/// the reachable state set staying bounded (no runaway unrolling).
#[test]
fn grow_loop_recursive_method_terminates_within_budget() {
    let (plan, flat, elapsed) = solved(&GROW_LOOP);
    assert!(
        elapsed < u128::from(WALL_BUDGET_MS),
        "recursion must terminate within the wall budget, took {elapsed}ms"
    );
    let cc = assert_outcome_closed(&plan, &flat);
    assert!(
        cc.entry_action_containing(":grow").is_some(),
        "the recursive method must actually be used"
    );
    assert!(
        cc.entry_action_containing(":stop").is_some(),
        "the base-case method must actually be used"
    );
    assert!(
        cc.reached.len() >= 3 && cc.reached.len() <= 8,
        "bounded unrolling expected (a handful of states), got {}",
        cc.reached.len()
    );
}

/// Depots pattern: `attempt`'s failure outcome routes to the SECOND method
/// (`finish-retry`), which resets and re-invokes the root `ship` task —
/// fail-then-retry. The failure branch must reach the goal only through that
/// second method, and the retry must loop back through the initial composite
/// state (strong-cyclic territory, like drop-retry).
#[test]
fn supervisor_fail_routes_failure_through_the_second_method() {
    let (plan, flat, _elapsed) = solved(&SUPERVISOR_FAIL);
    assert!(
        plan.notes.iter().any(|n| n.contains("strong-cyclic")),
        "fail-then-retry must route through the strong-cyclic fallback, notes: {:?}",
        plan.notes
    );
    let cc = assert_outcome_closed(&plan, &flat);

    let attempt = cc
        .entry_action_containing(":attempt")
        .expect("policy must choose the attempt action");
    assert_eq!(
        attempt.outcomes.len(),
        2,
        "success/failure branches, no collapsing"
    );
    let failed = attempt
        .outcomes
        .iter()
        .find(|o| cc.state_facts(&o.state).contains("failed"))
        .expect("failure branch must be present");
    let ok = attempt
        .outcomes
        .iter()
        .find(|o| cc.state_facts(&o.state).contains("shipped"))
        .expect("success branch must be present");
    assert!(
        cc.entry(&failed.state).action.contains("finish-retry"),
        "failure branch must decompose via the second method finish-retry, got {}",
        cc.entry(&failed.state).action
    );
    assert!(
        cc.entry(&ok.state).action.contains("finish-ok"),
        "success branch must decompose via finish-ok, got {}",
        cc.entry(&ok.state).action
    );
    assert!(
        cc.by_state
            .values()
            .flat_map(|e| e.outcomes.iter())
            .any(|o| o.state == cc.initial),
        "the retry must loop back through the initial composite state"
    );
}

/// Deterministic baseline: no oneof anywhere, so every policy entry carries
/// exactly one outcome, and the plain strong fixpoint (`fond_policy` itself,
/// no strong-cyclic fallback) solves it.
#[test]
fn plain_chain_baseline_solves_with_single_outcome_entries() {
    let (plan, flat, _elapsed) = solved(&PLAIN_CHAIN);
    let cc = assert_outcome_closed(&plan, &flat);
    assert!(
        plan.notes.iter().any(|n| n.contains("strong FOND")),
        "acyclic baseline must solve with the plain strong fixpoint, notes: {:?}",
        plan.notes
    );
    assert!(
        !plan.notes.iter().any(|n| n.contains("strong-cyclic")),
        "baseline must not need the strong-cyclic fallback"
    );
    for entry in &plan.policy {
        assert_eq!(
            entry.outcomes.len(),
            1,
            "deterministic baseline: every entry must have exactly one outcome"
        );
    }
    assert_eq!(
        plan.policy.len(),
        4,
        "expected entries for the decompose step plus the three chain steps"
    );
    // Five states lie on the chain: the initial, the decomposed state, the
    // three step-landing states (the last of which is the goal state).
    assert_eq!(cc.reached.len(), 5, "policy must cover the whole chain");
}

/// Deliberately unsolvable: both `force-door` outcomes dead-end. Must report
/// the TYPED error `HddlError::Planner(PlannerError::NoPlan)` — and the
/// translated graph itself must prove the dead-ends (both branches exist,
/// both targets are terminal and non-goal), so the NoPlan is the input's
/// property, not a solver artifact.
#[test]
fn both_branches_deadend_reports_typed_noplan() {
    let err = solve_hddl(BOTH_BRANCHES_DEADEND.0, BOTH_BRANCHES_DEADEND.1, &limits())
        .expect_err("deliberately unsolvable fixture must not solve");
    assert!(
        matches!(err, HddlError::Planner(PlannerError::NoPlan)),
        "expected typed Planner(NoPlan), got {err:?}"
    );

    let flat = flat_problem(BOTH_BRANCHES_DEADEND.0, BOTH_BRANCHES_DEADEND.1);
    let force: Vec<_> = flat
        .transitions
        .iter()
        .filter(|t| t.action.contains(":force-door"))
        .collect();
    assert_eq!(
        force.len(),
        2,
        "both oneof branches must exist as transitions"
    );
    for edge in force {
        let target = flat
            .states
            .iter()
            .find(|s| s.id == edge.to)
            .expect("outcome target state exists");
        assert!(
            !flat.goal.facts.is_subset(&target.facts),
            "deadend branch target must not satisfy the goal"
        );
        let outgoing = flat
            .transitions
            .iter()
            .filter(|t| t.from == target.id)
            .count();
        assert_eq!(
            outgoing, 0,
            "deadend branch target must be a terminal state"
        );
    }
}

/// No-panic/no-hang contract over the WHOLE suite: every fixture wrapped in
/// the same wall budget via `solve_hddl`'s own API; each call must return
/// (Ok, or the typed NoPlan for the deliberately unsolvable fixture) well
/// inside the budget — never panic, hang, or surface a pipeline-level error.
#[test]
fn every_fixture_completes_within_its_wall_budget() {
    const ALL: &[(&str, &(&str, &str), bool)] = &[
        ("drop-retry", &DROP_RETRY, true),
        ("tray-dirty", &TRAY_DIRTY, true),
        ("sense-then-branch", &SENSE_THEN_BRANCH, true),
        ("grow-loop", &GROW_LOOP, true),
        ("supervisor-fail", &SUPERVISOR_FAIL, true),
        ("plain-chain", &PLAIN_CHAIN, true),
        ("both-branches-deadend", &BOTH_BRANCHES_DEADEND, false),
    ];
    for (name, fixture, solvable) in ALL {
        let start = Instant::now();
        let result = solve_hddl(fixture.0, fixture.1, &limits());
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < u128::from(WALL_BUDGET_MS),
            "{name}: exceeded its {}ms wall budget ({elapsed:?})",
            WALL_BUDGET_MS
        );
        match result {
            Ok(plan) => {
                assert!(plan.solved, "{name}: Ok plan must be marked solved");
                assert!(
                    solvable,
                    "{name}: solved, but fixture is deliberately unsolvable"
                );
            }
            Err(HddlError::Planner(PlannerError::NoPlan)) => {
                assert!(!solvable, "{name}: NoPlan, but fixture should solve");
            }
            Err(other) => panic!("{name}: unexpected error under wall budget: {other:?}"),
        }
    }
}

/// The wall budget is real: under a 1ms budget the watchdog must either
/// deliver the (fast) result or return the typed `HddlError::Timeout` —
/// never a hang, a worker panic, or a phase-level error.
#[test]
fn one_millisecond_budget_returns_typed_timeout_instead_of_hanging() {
    let tight = PlannerLimits {
        max_wall_ms: 1,
        ..PlannerLimits::default()
    };
    let result = solve_hddl(PLAIN_CHAIN.0, PLAIN_CHAIN.1, &tight);
    match result {
        Ok(_) | Err(HddlError::Timeout { .. }) => {}
        Err(other) => panic!("expected Ok or typed Timeout under a 1ms budget, got {other:?}"),
    }
}
