//! Canonical flat-FOND solver suite.
//!
//! Each scenario is a hand-authored, tiny, explicit-state `PlanningProblem`
//! encoding one canonical FOND benchmark family, pinned against the public
//! dispatcher `solve_planning_type(PlanningType::Fond)` with its literature
//! verdict: **strong** (acyclic policy exists), **strong-cyclic-only** (a
//! policy exists but every one contains a retry cycle), or **unsolvable**
//! (typed `PlannerError::NoPlan`, never a bogus policy).
//!
//! Verdict taxonomy source: A. Cimatti, M. Pistore, M. Roveri, P. Traverso,
//! "Weak, Strong, and Strong Cyclic Planning via Symbolic Model Checking,"
//! Artificial Intelligence 147(1-2), 2003.
//!
//! Branch note: on this branch `validate_problem` still enforces
//! per-(state,action) probability mass == 1_000_000 (the relax lives on
//! `fix/fond-minors`, unmerged), so every two-outcome action is encoded as
//! 500000/500000.
//!
//! Provenance (wave-context rule (b)): every fixture below is a hand-authored
//! explicit-state abstraction of its canonical domain — the pinned property is
//! the literature verdict, not byte-level fidelity to a PDDL file. Per-domain
//! citations live on each fixture function.

use ferroplan::{
    solve_planning_type, PlannerError, PlannerLimits, PlanningProblem, PlanningType, UniversalGoal,
    UniversalPlan, UniversalPlanningRequest, UniversalPolicyEntry, UniversalState,
    UniversalTransition,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

const DETERMINISTIC_PPM: u32 = 1_000_000;
const HALF_PPM: u32 = 500_000;
const PROBABILITY_SCALE: u64 = 1_000_000;

fn set(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn state(id: &str, facts: &[&str]) -> UniversalState {
    UniversalState {
        id: id.to_owned(),
        facts: set(facts),
        fluents: BTreeMap::new(),
    }
}

fn edge(action: &str, from: &str, to: &str, probability_ppm: u32) -> UniversalTransition {
    UniversalTransition {
        action: action.to_owned(),
        from: from.to_owned(),
        to: to.to_owned(),
        cost: 1,
        duration: 1,
        reward: 0,
        probability_ppm,
        observation: None,
        requires: BTreeSet::new(),
    }
}

/// One two-outcome nondeterministic action, encoded 500000/500000 per the
/// branch's `validate_problem` mass==1M enforcement.
fn fork(action: &str, from: &str, outcome_a: &str, outcome_b: &str) -> Vec<UniversalTransition> {
    vec![
        edge(action, from, outcome_a, HALF_PPM),
        edge(action, from, outcome_b, HALF_PPM),
    ]
}

fn problem(
    states: Vec<UniversalState>,
    initial: &str,
    goal_facts: &[&str],
    unsafe_states: &[&str],
    transitions: Vec<UniversalTransition>,
) -> PlanningProblem {
    PlanningProblem {
        states,
        initial_states: vec![initial.to_owned()],
        goal: UniversalGoal {
            facts: set(goal_facts),
            ..UniversalGoal::default()
        },
        unsafe_states: set(unsafe_states),
        transitions,
        ..PlanningProblem::default()
    }
}

fn solve_fond(problem: PlanningProblem) -> Result<UniversalPlan, PlannerError> {
    solve_planning_type(&UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem,
        limits: PlannerLimits::default(),
    })
}

fn is_goal(problem: &PlanningProblem, state: &UniversalState) -> bool {
    problem
        .goal
        .facts
        .iter()
        .all(|fact| state.facts.contains(fact))
}

/// (state, action) -> [(outcome state, probability ppm)] straight from the
/// encoded transitions.
fn outcome_groups(problem: &PlanningProblem) -> BTreeMap<(&str, &str), Vec<(&str, u32)>> {
    let mut groups = BTreeMap::<(&str, &str), Vec<(&str, u32)>>::new();
    for transition in &problem.transitions {
        groups
            .entry((transition.from.as_str(), transition.action.as_str()))
            .or_default()
            .push((transition.to.as_str(), transition.probability_ppm));
    }
    groups
}

/// Assert the dispatcher solved the problem and the returned policy is
/// **outcome-closed**: exactly one entry per state, each entry's outcomes a
/// multiset-equal cover of that (state, action) group with probability mass
/// summing to the full scale, every policy-reachable non-goal state covered,
/// and the goal reachable under the policy from every reachable state.
/// Returns the set of states reachable under the policy.
fn assert_solved_and_closed(
    problem: &PlanningProblem,
    plan: &UniversalPlan,
    context: &str,
) -> BTreeSet<String> {
    assert!(
        plan.solved,
        "{context}: dispatcher reported an unsolved plan"
    );
    assert!(
        plan.planning_type == Some(PlanningType::Fond),
        "{context}: plan not stamped as Fond"
    );

    let states = problem
        .states
        .iter()
        .map(|state| (state.id.as_str(), state))
        .collect::<BTreeMap<_, _>>();
    let groups = outcome_groups(problem);

    let mut by_state = BTreeMap::<&str, &UniversalPolicyEntry>::new();
    for entry in &plan.policy {
        let previous = by_state.insert(entry.state.as_str(), entry);
        assert!(
            previous.is_none(),
            "{context}: duplicate policy entries for state {}",
            entry.state
        );
    }

    for entry in &plan.policy {
        let expected = groups
            .get(&(entry.state.as_str(), entry.action.as_str()))
            .unwrap_or_else(|| {
                panic!(
                    "{context}: policy names unknown (state, action) pair ({}, {})",
                    entry.state, entry.action
                )
            });
        let expected_counts = expected.iter().copied().fold(
            BTreeMap::<(&str, u32), usize>::new(),
            |mut counts, key| {
                *counts.entry(key).or_default() += 1;
                counts
            },
        );
        let actual_counts = entry
            .outcomes
            .iter()
            .map(|outcome| (outcome.state.as_str(), outcome.probability_ppm))
            .fold(BTreeMap::<(&str, u32), usize>::new(), |mut counts, key| {
                *counts.entry(key).or_default() += 1;
                counts
            });
        assert_eq!(
            actual_counts, expected_counts,
            "{context}: policy outcomes for ({}, {}) do not exactly cover the encoded action group",
            entry.state, entry.action
        );
        let mass: u64 = entry
            .outcomes
            .iter()
            .map(|outcome| u64::from(outcome.probability_ppm))
            .sum();
        assert_eq!(
            mass, PROBABILITY_SCALE,
            "{context}: policy outcomes for ({}, {}) do not carry full probability mass",
            entry.state, entry.action
        );
    }

    // Forward reachability under the policy from every initial state.
    let mut reachable = BTreeSet::<String>::new();
    let mut queue = VecDeque::<String>::new();
    for initial in &problem.initial_states {
        queue.push_back(initial.clone());
    }
    while let Some(current) = queue.pop_front() {
        if !reachable.insert(current.clone()) {
            continue;
        }
        if is_goal(problem, states[current.as_str()]) {
            continue;
        }
        let entry = by_state.get(current.as_str()).unwrap_or_else(|| {
            panic!("{context}: policy has no entry for policy-reachable non-goal state {current}")
        });
        for outcome in &entry.outcomes {
            queue.push_back(outcome.state.clone());
        }
    }

    // Under the policy, the goal must be reachable from every reachable state
    // (strong-cyclic closure under the standard fairness assumption).
    for start in &reachable {
        if is_goal(problem, states[start.as_str()]) {
            continue;
        }
        let mut seen = BTreeSet::from([start.clone()]);
        let mut search = VecDeque::from([start.clone()]);
        let mut reaches_goal = false;
        while let Some(current) = search.pop_front() {
            if is_goal(problem, states[current.as_str()]) {
                reaches_goal = true;
                break;
            }
            if let Some(entry) = by_state.get(current.as_str()) {
                for outcome in &entry.outcomes {
                    if seen.insert(outcome.state.clone()) {
                        search.push_back(outcome.state.clone());
                    }
                }
            }
        }
        assert!(
            reaches_goal,
            "{context}: state {start} cannot reach the goal under the policy"
        );
    }

    reachable
}

/// White/gray/black DFS cycle detection over the policy's reachable fragment.
fn fragment_has_cycle(
    roots: impl IntoIterator<Item = String>,
    adjacency: &BTreeMap<String, Vec<String>>,
) -> bool {
    const WHITE: u8 = 0;
    const GRAY: u8 = 1;
    const BLACK: u8 = 2;
    fn visit(
        node: &str,
        adjacency: &BTreeMap<String, Vec<String>>,
        color: &mut BTreeMap<String, u8>,
    ) -> bool {
        color.insert(node.to_owned(), GRAY);
        for next in adjacency.get(node).into_iter().flatten() {
            let next_color = color.get(next).copied().unwrap_or(WHITE);
            if next_color == GRAY {
                return true;
            }
            if next_color == WHITE && visit(next, adjacency, color) {
                return true;
            }
        }
        color.insert(node.to_owned(), BLACK);
        false
    }
    let mut color = BTreeMap::<String, u8>::new();
    for root in roots {
        if color.get(&root).copied().unwrap_or(WHITE) == WHITE
            && visit(&root, adjacency, &mut color)
        {
            return true;
        }
    }
    false
}

fn policy_adjacency(plan: &UniversalPlan) -> BTreeMap<String, Vec<String>> {
    plan.policy
        .iter()
        .map(|entry| {
            (
                entry.state.clone(),
                entry
                    .outcomes
                    .iter()
                    .map(|outcome| outcome.state.clone())
                    .collect(),
            )
        })
        .collect()
}

/// Cyclic-only verdict: solved, outcome-closed, AND the reachable policy
/// fragment contains at least one cycle — which simultaneously proves the
/// dispatcher's acyclic strong fixpoint (`fond_policy`) returned NoPlan and
/// the strong-cyclic fallback produced this policy (an acyclic fixpoint can
/// never emit a cyclic policy).
fn assert_cyclic_only(problem: &PlanningProblem, plan: &UniversalPlan, context: &str) {
    let reachable = assert_solved_and_closed(problem, plan, context);
    assert!(
        fragment_has_cycle(reachable, &policy_adjacency(plan)),
        "{context}: expected a retry cycle in the reachable policy fragment \
         (cyclic-only scenario must exercise the strong-cyclic fallback)"
    );
}

/// Strong verdict: solved, outcome-closed, acyclic under the policy (the
/// reachable fragment is a DAG whose every path terminates in the goal), and
/// the encoded problem is genuinely nondeterministic (some action group has
/// more than one outcome — otherwise this would not pin FOND behavior at all).
fn assert_strong(problem: &PlanningProblem, plan: &UniversalPlan, context: &str) {
    let reachable = assert_solved_and_closed(problem, plan, context);
    assert!(
        !fragment_has_cycle(reachable, &policy_adjacency(plan)),
        "{context}: strong policy must be acyclic (DAG-to-goal) under the policy"
    );
    let widest_group = outcome_groups(problem)
        .values()
        .map(Vec::len)
        .max()
        .unwrap_or(0);
    assert!(
        widest_group >= 2,
        "{context}: scenario encodes no nondeterministic action; it does not pin FOND behavior"
    );
}

fn assert_typed_no_plan(result: Result<UniversalPlan, PlannerError>, context: &str) {
    match result {
        Ok(plan) => panic!(
            "{context}: expected Err(NoPlan), got a policy with {} entries (solved={})",
            plan.policy.len(),
            plan.solved
        ),
        Err(PlannerError::NoPlan) => {}
        Err(other) => panic!("{context}: expected Err(NoPlan), got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Self-loop classic: `flip` s0 -> {s0, goal}
// ---------------------------------------------------------------------------

/// Minimal strong-cyclic retry loop (hand-authored after the retry-loop
/// examples of Cimatti et al., AIJ 2003, section on strong vs. strong-cyclic).
/// The acyclic strong fixpoint cannot admit `s0` (its own retry outcome is not
/// yet winning when it would need to be), so this must be solved by the
/// dispatcher's strong-cyclic fallback with an outcome-closed policy.
fn flip_problem() -> PlanningProblem {
    let mut transitions = fork("flip", "s0", "s0", "g");
    transitions.sort_by(|a, b| a.to.cmp(&b.to));
    problem(
        vec![state("s0", &["start"]), state("g", &["goal"])],
        "s0",
        &["goal"],
        &[],
        transitions,
    )
}

#[test]
fn flip_self_loop_solves_strong_cyclic_with_closed_policy() {
    let problem = flip_problem();
    let plan = solve_fond(problem.clone()).expect("flip self-loop must be strong-cyclic solvable");
    assert_cyclic_only(&problem, &plan, "flip");
    let entry = plan
        .policy
        .iter()
        .find(|entry| entry.state == "s0")
        .expect("flip policy must cover s0");
    assert_eq!(entry.action, "flip");
    assert_eq!(plan.policy.len(), 1, "goal state needs no policy entry");
}

/// Dead-end variant: `flip` may reach an absorbing non-goal state instead of
/// the goal; no policy exists, and the dispatcher must return the typed
/// `PlannerError::NoPlan`, never a bogus policy.
fn flip_dead_end_problem() -> PlanningProblem {
    let mut transitions = fork("flip", "s0", "s0", "dead");
    transitions.sort_by(|a, b| a.to.cmp(&b.to));
    problem(
        vec![
            state("s0", &["start"]),
            state("g", &["goal"]),
            state("dead", &["absorbed"]),
        ],
        "s0",
        &["goal"],
        &["dead"],
        transitions,
    )
}

#[test]
fn flip_dead_end_variant_returns_typed_no_plan() {
    assert_typed_no_plan(solve_fond(flip_dead_end_problem()), "flip dead-end variant");
}

// ---------------------------------------------------------------------------
// faults — strong
// ---------------------------------------------------------------------------

/// Cascading-fault recovery with nondeterministic outcomes everywhere, yet a
/// strictly acyclic recovery policy exists (hand-authored after the
/// fault-tolerance motivation of Cimatti et al., AIJ 2003). The dispatcher's
/// strong fixpoint must solve it directly; the policy must be a DAG-to-goal.
fn faults_problem() -> PlanningProblem {
    let mut transitions = Vec::new();
    transitions.extend(fork("engage", "ok", "f1", "g"));
    transitions.extend(fork("escalate-repair", "f1", "f2", "g"));
    transitions.push(edge("final-repair", "f2", "g", DETERMINISTIC_PPM));
    problem(
        vec![
            state("ok", &["running"]),
            state("f1", &["fault"]),
            state("f2", &["fault", "deep"]),
            state("g", &["goal"]),
        ],
        "ok",
        &["goal"],
        &[],
        transitions,
    )
}

#[test]
fn faults_strong_policy_is_dag_to_goal() {
    let problem = faults_problem();
    let plan = solve_fond(problem.clone()).expect("faults must admit a strong policy");
    assert_strong(&problem, &plan, "faults");
    let covered = plan
        .policy
        .iter()
        .map(|entry| entry.state.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        covered,
        BTreeSet::from(["ok", "f1", "f2"]),
        "every non-goal state on the recovery chain needs a policy entry"
    );
}

// ---------------------------------------------------------------------------
// boolean — strong
// ---------------------------------------------------------------------------

/// Boolean latch abstraction (hand-authored): writing two bits has a
/// nondeterministic effect (either bit may land first); a follow-up write per
/// residue state forces the full assignment acyclically. Strong FOND verdict
/// per the strong-planning taxonomy of Cimatti et al., AIJ 2003.
fn boolean_problem() -> PlanningProblem {
    let mut transitions = fork("write", "b00", "b10", "b01");
    transitions.push(edge("set-bit-b", "b10", "b11", DETERMINISTIC_PPM));
    transitions.push(edge("set-bit-a", "b01", "b11", DETERMINISTIC_PPM));
    problem(
        vec![
            state("b00", &["start"]),
            state("b10", &["bit-a"]),
            state("b01", &["bit-b"]),
            state("b11", &["bit-a", "bit-b", "goal"]),
        ],
        "b00",
        &["goal"],
        &[],
        transitions,
    )
}

#[test]
fn boolean_strong_policy_is_dag_to_goal() {
    let problem = boolean_problem();
    let plan = solve_fond(problem.clone()).expect("boolean must admit a strong policy");
    assert_strong(&problem, &plan, "boolean");
}

// ---------------------------------------------------------------------------
// coffee — cyclic-only
// ---------------------------------------------------------------------------

/// Coffee-robot family (Boutilier, Dean, Hanks, "Decision-Theoretic Planning,"
/// AIJ 1999 — the coffee problem as the canonical serve/retry domain),
/// hand-authored explicit-state abstraction: grinding is deterministic, but
/// brewing repeatedly fails until a lucky run succeeds. Every policy contains
/// the brew retry cycle, so this is strong-cyclic-only.
fn coffee_problem() -> PlanningProblem {
    let mut transitions = vec![edge("grind", "no-coffee", "ground", DETERMINISTIC_PPM)];
    transitions.extend(fork("brew", "ground", "ground", "served"));
    problem(
        vec![
            state("no-coffee", &["start"]),
            state("ground", &["ground"]),
            state("served", &["coffee", "goal"]),
        ],
        "no-coffee",
        &["goal"],
        &[],
        transitions,
    )
}

#[test]
fn coffee_retry_loop_solves_cyclic_only() {
    let problem = coffee_problem();
    let plan = solve_fond(problem.clone()).expect("coffee must be strong-cyclic solvable");
    assert_cyclic_only(&problem, &plan, "coffee");
}

// ---------------------------------------------------------------------------
// wall — cyclic-only
// ---------------------------------------------------------------------------

/// Wall traversal (hand-authored after the wall-crossing examples shipped with
/// the strong/strong-cyclic model-checking planners of Cimatti et al., AIJ
/// 2003): crossing may bounce the agent back to the same spot until a run
/// succeeds. Every policy retries in place — strong-cyclic-only.
fn wall_problem() -> PlanningProblem {
    let transitions = fork("cross", "before", "before", "past");
    problem(
        vec![
            state("before", &["before-wall"]),
            state("past", &["past-wall", "goal"]),
        ],
        "before",
        &["goal"],
        &[],
        transitions,
    )
}

#[test]
fn wall_bounce_retry_solves_cyclic_only() {
    let problem = wall_problem();
    let plan = solve_fond(problem.clone()).expect("wall must be strong-cyclic solvable");
    assert_cyclic_only(&problem, &plan, "wall");
}

// ---------------------------------------------------------------------------
// islands — cyclic-only
// ---------------------------------------------------------------------------

/// Islands benchmark family (canonical FOND benchmark set of Mattmüller,
/// Ortlieb, Helmert, Bercher, "Pattern Database Heuristics for Fully
/// Observable Non-deterministic Planning," ICAPS 2008), hand-authored
/// explicit-state abstraction: every bridge to the goal island may deposit the
/// agent back on the shore until a lucky crossing. Both bridges carry a retry
/// cycle, so no strong policy exists; a strong-cyclic one does.
fn islands_problem() -> PlanningProblem {
    let mut transitions = Vec::new();
    transitions.extend(fork("bridge-a", "shore", "shore", "isle-goal"));
    transitions.extend(fork("bridge-b", "shore", "shore", "isle-mid"));
    transitions.push(edge("hop", "isle-mid", "isle-goal", DETERMINISTIC_PPM));
    problem(
        vec![
            state("shore", &["mainland"]),
            state("isle-mid", &["near-island"]),
            state("isle-goal", &["on-island", "goal"]),
        ],
        "shore",
        &["goal"],
        &[],
        transitions,
    )
}

#[test]
fn islands_bridge_retry_solves_cyclic_only() {
    let problem = islands_problem();
    let plan = solve_fond(problem.clone()).expect("islands must be strong-cyclic solvable");
    assert_cyclic_only(&problem, &plan, "islands");
}

// ---------------------------------------------------------------------------
// tireworld-s — cyclic-only
// ---------------------------------------------------------------------------

/// tireworld (PPDDL IPC 2004; canonical strong-cyclic FOND benchmark of
/// Mattmüller et al., ICAPS 2008), hand-authored explicit-state abstraction:
/// driving from the flat depot cell `a` over the slope to `b` may blow a tire,
/// leaving the car flat at `a`, where the spare depot permits a change and a
/// retry (depot abstraction: the cell's spare supply is not consumed by a
/// single change, so no absorbing dead end is reachable). Every policy cycles
/// at `a` — strong-cyclic-only.
fn tireworld_s_problem() -> PlanningProblem {
    let mut transitions = fork("drive-a-b", "a-ok", "b-ok", "a-flat");
    transitions.push(edge("change-tire-a", "a-flat", "a-ok", DETERMINISTIC_PPM));
    transitions.push(edge("drive-b-c", "b-ok", "c-ok", DETERMINISTIC_PPM));
    problem(
        vec![
            state("a-ok", &["at-a", "tire-ok"]),
            state("a-flat", &["at-a", "tire-flat"]),
            state("b-ok", &["at-b", "tire-ok"]),
            state("c-ok", &["at-c", "tire-ok", "goal"]),
        ],
        "a-ok",
        &["goal"],
        &[],
        transitions,
    )
}

#[test]
fn tireworld_s_depot_retry_solves_cyclic_only() {
    let problem = tireworld_s_problem();
    let plan = solve_fond(problem.clone()).expect("tireworld-s must be strong-cyclic solvable");
    assert_cyclic_only(&problem, &plan, "tireworld-s");
}

// ---------------------------------------------------------------------------
// triangle-tireworld — cyclic-only with dead-end avoidance
// ---------------------------------------------------------------------------

/// triangle-tireworld (PPDDL IPC 2004; canonical strong-cyclic FOND benchmark
/// of Mattmüller et al., ICAPS 2008), hand-authored explicit-state abstraction
/// preserving its defining property: one route's failure outcome is
/// unrecoverable (cell `c` has no spare depot, so going flat there is an
/// absorbing failure) while the other route's is not. A strong-cyclic policy
/// exists ONLY by avoiding the C route entirely; the solver's greatest-fixpoint
/// prune must exclude `c-ok` (its only action can reach the absorbing
/// `c-flat`, which is not even weakly goal-reachable), forcing the A route
/// through depot cell `b`. Cycles at `a` and `b` make it cyclic-only.
fn triangle_tireworld_problem() -> PlanningProblem {
    let mut transitions = Vec::new();
    transitions.extend(fork("drive-a-b", "a-ok", "b-ok", "a-flat"));
    transitions.extend(fork("drive-a-c", "a-ok", "c-ok", "a-flat"));
    transitions.push(edge("change-tire-a", "a-flat", "a-ok", DETERMINISTIC_PPM));
    transitions.extend(fork("drive-b-f", "b-ok", "f-ok", "b-flat"));
    transitions.push(edge("change-tire-b", "b-flat", "b-ok", DETERMINISTIC_PPM));
    transitions.extend(fork("drive-c-f", "c-ok", "f-ok", "c-flat"));
    problem(
        vec![
            state("a-ok", &["at-a", "tire-ok"]),
            state("a-flat", &["at-a", "tire-flat"]),
            state("b-ok", &["at-b", "tire-ok"]),
            state("b-flat", &["at-b", "tire-flat"]),
            state("c-ok", &["at-c", "tire-ok"]),
            state("c-flat", &["at-c", "tire-flat", "stranded"]),
            state("f-ok", &["at-f", "tire-ok", "goal"]),
        ],
        "a-ok",
        &["goal"],
        &["c-flat"],
        transitions,
    )
}

#[test]
fn triangle_tireworld_avoids_dead_end_route_solves_cyclic_only() {
    let problem = triangle_tireworld_problem();
    let plan =
        solve_fond(problem.clone()).expect("triangle-tireworld must be strong-cyclic solvable");
    assert_cyclic_only(&problem, &plan, "triangle-tireworld");
    let actions = plan
        .policy
        .iter()
        .map(|entry| entry.action.as_str())
        .collect::<BTreeSet<_>>();
    assert!(
        !actions.contains("drive-a-c") && !actions.contains("drive-c-f"),
        "strong-cyclic policy must avoid the dead-end C route entirely"
    );
    assert!(
        plan.policy.iter().all(|entry| entry.state != "c-ok"),
        "the unsalvageable state c-ok must carry no policy entry"
    );
    assert_eq!(
        plan.policy
            .iter()
            .find(|entry| entry.state == "a-ok")
            .map(|entry| entry.action.as_str()),
        Some("drive-a-b"),
        "policy must commit to the depot route through b"
    );
}

// ---------------------------------------------------------------------------
// tireworld-unsolvable — absorbing failure, no spare
// ---------------------------------------------------------------------------

/// tireworld family, unsolvable variant (hand-authored): the only crossing may
/// blow the tire with no spare depot anywhere, and going flat is an absorbing
/// failure (self-loop, marked unsafe). No action's outcome set avoids it, so
/// both the strong fixpoint and the strong-cyclic fixpoint must reject the
/// problem with the typed `PlannerError::NoPlan` — never a bogus policy.
fn tireworld_unsolvable_problem() -> PlanningProblem {
    let mut transitions = fork("drive-a-c", "a-ok", "c-ok", "a-flat");
    transitions.push(edge("stall", "a-flat", "a-flat", DETERMINISTIC_PPM));
    problem(
        vec![
            state("a-ok", &["at-a", "tire-ok"]),
            state("a-flat", &["at-a", "tire-flat", "stranded"]),
            state("c-ok", &["at-c", "tire-ok", "goal"]),
        ],
        "a-ok",
        &["goal"],
        &["a-flat"],
        transitions,
    )
}

#[test]
fn tireworld_unsolvable_absorbing_failure_returns_typed_no_plan() {
    assert_typed_no_plan(
        solve_fond(tireworld_unsolvable_problem()),
        "tireworld-unsolvable",
    );
}
