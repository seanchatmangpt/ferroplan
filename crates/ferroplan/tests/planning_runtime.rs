use ferroplan::{
    solve_planning_type, Agent, PlannerError, PlanningMethod, PlanningProblem, PlanningTask,
    PlanningType, QueueState, RdfTriple, Tool, UniversalGoal, UniversalPlanningRequest,
    UniversalState, UniversalTransition, WorkflowEdge,
};
use std::collections::{BTreeMap, BTreeSet};

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

fn edge(action: &str, from: &str, to: &str) -> UniversalTransition {
    UniversalTransition {
        action: action.to_owned(),
        from: from.to_owned(),
        to: to.to_owned(),
        cost: 1,
        duration: 1,
        reward: 0,
        probability_ppm: 1_000_000,
        observation: None,
        requires: BTreeSet::new(),
    }
}

fn chain_problem() -> PlanningProblem {
    PlanningProblem {
        states: vec![state("s0", &[]), state("s1", &[]), state("g", &["done"])],
        initial_states: vec!["s0".to_owned()],
        goal: UniversalGoal {
            facts: set(&["done"]),
            ..UniversalGoal::default()
        },
        transitions: vec![edge("prepare", "s0", "s1"), edge("finish", "s1", "g")],
        ..PlanningProblem::default()
    }
}

fn solve(kind: PlanningType, problem: PlanningProblem) -> ferroplan::UniversalPlan {
    solve_planning_type(&UniversalPlanningRequest {
        planning_type: kind,
        problem,
        limits: Default::default(),
    })
    .unwrap()
}

#[test]
fn classical_cost_numeric_temporal_and_preferences_execute() {
    for kind in [
        PlanningType::Classical,
        PlanningType::CostOptimal,
        PlanningType::Temporal,
    ] {
        let result = solve(kind, chain_problem());
        assert!(result.solved);
        assert_eq!(result.steps.len(), 2);
    }

    let mut numeric = chain_problem();
    numeric.states[2].fluents.insert("quality".to_owned(), 10);
    numeric.goal.numeric_min.insert("quality".to_owned(), 10);
    assert!(solve(PlanningType::Numeric, numeric).solved);

    let mut preferences = chain_problem();
    preferences.soft_goal_facts.insert("bonus".to_owned(), 3);
    assert!(solve(PlanningType::Preferences, preferences).solved);
}

#[test]
fn probabilistic_policy_executes() {
    let mut problem = chain_problem();
    problem.transitions = vec![
        UniversalTransition {
            action: "try".to_owned(),
            from: "s0".to_owned(),
            to: "g".to_owned(),
            probability_ppm: 700_000,
            ..edge("try", "s0", "g")
        },
        UniversalTransition {
            action: "try".to_owned(),
            from: "s0".to_owned(),
            to: "s0".to_owned(),
            probability_ppm: 300_000,
            ..edge("try", "s0", "s0")
        },
    ];
    let result = solve(PlanningType::Probabilistic, problem);
    assert!(result.solved);
    assert_eq!(result.policy[0].action, "try");
}

#[test]
fn fond_strong_policy_executes() {
    let problem = PlanningProblem {
        states: vec![
            state("s0", &[]),
            state("g1", &["done"]),
            state("g2", &["done"]),
        ],
        initial_states: vec!["s0".to_owned()],
        goal: UniversalGoal {
            facts: set(&["done"]),
            ..Default::default()
        },
        transitions: vec![
            UniversalTransition {
                action: "commit".to_owned(),
                from: "s0".to_owned(),
                to: "g1".to_owned(),
                probability_ppm: 500_000,
                ..edge("commit", "s0", "g1")
            },
            UniversalTransition {
                action: "commit".to_owned(),
                from: "s0".to_owned(),
                to: "g2".to_owned(),
                probability_ppm: 500_000,
                ..edge("commit", "s0", "g2")
            },
        ],
        ..Default::default()
    };
    assert!(solve(PlanningType::Fond, problem).solved);
}

/// Strong-cyclic FOND probe: a single non-deterministic action "flip" from
/// `s0` either reaches the goal (`g`) or loops back to `s0` itself. No
/// ACYCLIC policy exists here (there is no way to make every outcome of any
/// action land in an already-solved state, because one outcome always maps
/// `s0` back onto `s0`), but a STRONG-CYCLIC policy trivially exists: keep
/// executing "flip" at `s0` until the goal outcome occurs, which happens with
/// probability 1 in the limit (Cimatti/Roveri strong-cyclic semantics allow a
/// policy that revisits states, as long as every state in the policy's fair
/// execution has a non-zero chance of eventually progressing to the goal).
///
/// `fond_policy`'s fixpoint (see `crates/ferroplan/src/planning_runtime.rs`)
/// only ever admits a state into `winning` when EVERY outcome of some action
/// is already in `winning` *before* this state is considered — i.e. it is a
/// monotonically growing (least-fixpoint) backward-induction pass, identical
/// in shape to the classical "Strong Planning" algorithm from Cimatti et al.
/// This cannot ever mark `s0` winning on its own: the `s0 -> s0` self-loop
/// outcome can never be in `winning` ahead of `s0` itself (that specific,
/// function-level contract is pinned directly against `fond_policy` in
/// `crates/ferroplan/src/planning_runtime.rs`'s own unit tests, since
/// `fond_policy` is private and unreachable from this integration-test
/// crate). `PlanningType::Fond`'s dispatch in `solve_planning_type` now
/// falls back to `fond_policy_strong_cyclic` -- the standard Cimatti et al.
/// two-phase (weak-reachability + greatest-fixpoint-prune) construction --
/// whenever `fond_policy` alone returns `NoPlan`, so the *overall* FOND
/// paradigm now does solve this domain end to end.
///
/// This test proves, by real execution through the public
/// `solve_planning_type` entry point (not by reading code alone), that
/// `PlanningType::Fond` now returns a solved policy on a domain that
/// REQUIRES a strong-cyclic (revisiting) policy.
#[test]
fn fond_via_solve_planning_type_now_solves_the_strong_cyclic_retry_loop() {
    let problem = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["done"])],
        initial_states: vec!["s0".to_owned()],
        goal: UniversalGoal {
            facts: set(&["done"]),
            ..Default::default()
        },
        transitions: vec![
            // "flip" from s0: either reach the goal, or land right back on
            // s0 (the retry loop). Under the strict mass rule (still kept
            // for Probabilistic and the deterministic types) this must sum
            // to 1_000_000; `PlanningType::Fond` is mass-blind and only
            // range-checks individual edges now.
            UniversalTransition {
                action: "flip".to_owned(),
                from: "s0".to_owned(),
                to: "g".to_owned(),
                probability_ppm: 500_000,
                ..edge("flip", "s0", "g")
            },
            UniversalTransition {
                action: "flip".to_owned(),
                from: "s0".to_owned(),
                to: "s0".to_owned(),
                probability_ppm: 500_000,
                ..edge("flip", "s0", "s0")
            },
        ],
        ..Default::default()
    };
    let result = solve_planning_type(&UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem,
        limits: Default::default(),
    });
    let plan = result.expect(
        "PlanningType::Fond should now solve the retry-loop domain via the \
         strong-cyclic fallback",
    );
    assert!(plan.solved);
    assert_eq!(plan.planning_type, Some(PlanningType::Fond));
    assert_eq!(plan.policy.len(), 1);
    let entry = &plan.policy[0];
    assert_eq!(entry.state, "s0");
    assert_eq!(entry.action, "flip");
    let mut outcomes = entry
        .outcomes
        .iter()
        .map(|outcome| outcome.state.clone())
        .collect::<Vec<_>>();
    outcomes.sort();
    assert_eq!(outcomes, vec!["g".to_owned(), "s0".to_owned()]);
}

/// The FOND/conformant/contingent solvers never read outcome probability
/// mass (their fixpoints are pure AND/OR reachability over the edge
/// topology), so `validate_problem` only range-checks individual edges for
/// these types instead of demanding a normalized 1_000_000 sum per
/// (state, action) group. A two-outcome nondeterministic action written
/// 1M/1M — valid nondeterminism, invalid as a probability distribution —
/// must therefore solve for every mass-blind planning type.
#[test]
fn mass_blind_planners_accept_unnormalized_outcome_masses() {
    let make = || PlanningProblem {
        states: vec![
            state("s0", &[]),
            state("g1", &["done"]),
            state("g2", &["done"]),
        ],
        initial_states: vec!["s0".to_owned()],
        goal: UniversalGoal {
            facts: set(&["done"]),
            ..Default::default()
        },
        transitions: vec![
            // The `edge` helper already emits 1_000_000; spelled out here to
            // make the (formerly mass-invalid) encoding under test explicit.
            UniversalTransition {
                probability_ppm: 1_000_000,
                ..edge("commit", "s0", "g1")
            },
            UniversalTransition {
                probability_ppm: 1_000_000,
                ..edge("commit", "s0", "g2")
            },
        ],
        ..Default::default()
    };
    for kind in [
        PlanningType::Fond,
        PlanningType::Conformant,
        PlanningType::Contingent,
    ] {
        assert!(
            solve(kind, make()).solved,
            "{kind} must accept a 1M/1M two-outcome action"
        );
    }
}

/// `PlanningType::Probabilistic` keeps the strict rule — each (state, action)
/// group's masses must sum to 1_000_000 — because value iteration literally
/// divides by the scale, so an unnormalized 900_000 group would silently
/// distort the policy. A mass deficit is still rejected with
/// `InvalidProbabilityMass` carrying the offending sum.
#[test]
fn probabilistic_still_rejects_a_mass_deficit() {
    let mut problem = chain_problem();
    problem.transitions = vec![
        UniversalTransition {
            probability_ppm: 700_000,
            ..edge("try", "s0", "g")
        },
        UniversalTransition {
            probability_ppm: 200_000,
            ..edge("try", "s0", "s0")
        },
    ];
    assert_eq!(
        solve_planning_type(&UniversalPlanningRequest {
            planning_type: PlanningType::Probabilistic,
            problem,
            limits: Default::default(),
        }),
        Err(PlannerError::InvalidProbabilityMass {
            state: "s0".to_owned(),
            action: "try".to_owned(),
            mass: 900_000,
        })
    );
}

/// The deterministic rails keep their existing validation untouched: the
/// plain single-outcome encoding (one edge per group at 1_000_000) still
/// solves exactly as before.
#[test]
fn deterministic_single_outcome_mass_rule_is_unchanged() {
    assert!(solve(PlanningType::Classical, chain_problem()).solved);
}

#[test]
fn conformant_and_contingent_belief_planners_execute() {
    let base = PlanningProblem {
        states: vec![
            state("a", &[]),
            state("b", &[]),
            state("ga", &["done"]),
            state("gb", &["done"]),
        ],
        initial_states: vec!["a".to_owned(), "b".to_owned()],
        goal: UniversalGoal {
            facts: set(&["done"]),
            ..Default::default()
        },
        transitions: vec![edge("resolve", "a", "ga"), edge("resolve", "b", "gb")],
        ..Default::default()
    };
    assert!(solve(PlanningType::Conformant, base.clone()).solved);

    let mut contingent = base;
    contingent.transitions[0].observation = Some("saw-a".to_owned());
    contingent.transitions[1].observation = Some("saw-b".to_owned());
    assert!(solve(PlanningType::Contingent, contingent).solved);
}

fn hierarchy_problem() -> PlanningProblem {
    PlanningProblem {
        tasks: vec![
            PlanningTask {
                id: "root".to_owned(),
                primitive_action: None,
                requires: set(&[]),
            },
            PlanningTask {
                id: "inspect".to_owned(),
                primitive_action: Some("inspect-repo".to_owned()),
                requires: set(&["read"]),
            },
            PlanningTask {
                id: "verify".to_owned(),
                primitive_action: Some("run-tests".to_owned()),
                requires: set(&["test"]),
            },
        ],
        root_tasks: vec!["root".to_owned()],
        methods: vec![PlanningMethod {
            id: "root-method".to_owned(),
            task: "root".to_owned(),
            subtasks: vec!["inspect".to_owned(), "verify".to_owned()],
        }],
        ..Default::default()
    }
}

#[test]
fn hierarchical_and_resolution_adaptive_planners_execute() {
    for kind in [PlanningType::Hierarchical, PlanningType::ResolutionAdaptive] {
        let result = solve(kind, hierarchy_problem());
        assert_eq!(result.decomposition, ["inspect-repo", "run-tests"]);
    }
}

/// Compound task with no primitive action (`requires` unused by the
/// hierarchical planner; kept empty in these fixtures).
fn htn_task(id: &str, primitive_action: Option<&str>) -> PlanningTask {
    PlanningTask {
        id: id.to_owned(),
        primitive_action: primitive_action.map(|action| action.to_owned()),
        requires: BTreeSet::new(),
    }
}

fn htn_method(id: &str, task: &str, subtasks: &[&str]) -> PlanningMethod {
    PlanningMethod {
        id: id.to_owned(),
        task: task.to_owned(),
        subtasks: subtasks
            .iter()
            .map(|subtask| (*subtask).to_owned())
            .collect(),
    }
}

fn solve_hierarchical_error(problem: PlanningProblem) -> PlannerError {
    solve_planning_type(&UniversalPlanningRequest {
        planning_type: PlanningType::Hierarchical,
        problem,
        limits: Default::default(),
    })
    .unwrap_err()
}

/// (a) Two-method counterexample: the FIRST declared method of "top" leads
/// to a dead-end compound task with no method of its own; the second method
/// decomposes to a primitive.  The planner must backtrack past the dead end
/// and return m-second's decomposition (pre-backtracking this returned
/// `NoMethod { task: "dead-end" }`).
#[test]
fn hierarchical_backtracks_past_dead_end_first_method_to_second_method() {
    let problem = PlanningProblem {
        tasks: vec![
            htn_task("top", None),
            htn_task("dead-end", None),
            htn_task("act", Some("act-primitive")),
        ],
        root_tasks: vec!["top".to_owned()],
        methods: vec![
            htn_method("m-first", "top", &["dead-end"]),
            htn_method("m-second", "top", &["act"]),
        ],
        ..Default::default()
    };
    let result = solve(PlanningType::Hierarchical, problem);
    assert!(result.solved);
    assert_eq!(result.decomposition, ["act-primitive"]);
    // Honesty marker: `solved` means structurally decomposed, not
    // state-level goal satisfaction (this planner has no state semantics).
    assert!(result
        .notes
        .iter()
        .any(|note| note.contains("method backtracking") && note.contains("no state semantics")));
}

/// (b) Three-level hierarchy requiring backtracking at TWO distinct choice
/// points: level 1 ("top": m-top-first leads to mid-a, whose only method
/// dead-ends) and level 2 ("mid-b": its first method references a task id
/// that does not exist at all, `UnknownTask`).  Only the third choice,
/// m-mid-b-second, reaches the primitive.
#[test]
fn hierarchical_backtracks_twice_across_three_levels() {
    let problem = PlanningProblem {
        tasks: vec![
            htn_task("top", None),
            htn_task("mid-a", None),
            htn_task("mid-b", None),
            htn_task("dead-end", None),
            htn_task("leaf", Some("leaf-action")),
        ],
        root_tasks: vec!["top".to_owned()],
        methods: vec![
            htn_method("m-top-first", "top", &["mid-a"]),
            htn_method("m-top-second", "top", &["mid-b"]),
            htn_method("m-mid-a", "mid-a", &["dead-end"]),
            htn_method("m-mid-b-first", "mid-b", &["ghost"]),
            htn_method("m-mid-b-second", "mid-b", &["leaf"]),
        ],
        ..Default::default()
    };
    let result = solve(PlanningType::Hierarchical, problem);
    assert!(result.solved);
    assert_eq!(result.decomposition, ["leaf-action"]);
}

/// (c) When EVERY method of the root fails, `NoMethod` is still returned.
/// The documented consolidation choice pins the error to the LAST method
/// tried (declaration order), so the task named is the second dead end.
#[test]
fn hierarchical_returns_no_method_when_every_method_fails() {
    let problem = PlanningProblem {
        tasks: vec![
            htn_task("top", None),
            htn_task("dead-end-one", None),
            htn_task("dead-end-two", None),
        ],
        root_tasks: vec!["top".to_owned()],
        methods: vec![
            htn_method("m-first", "top", &["dead-end-one"]),
            htn_method("m-second", "top", &["dead-end-two"]),
        ],
        ..Default::default()
    };
    assert_eq!(
        solve_hierarchical_error(problem),
        PlannerError::NoMethod {
            task: "dead-end-two".to_owned()
        }
    );
}

/// (d) A method whose subtasks recurse back into the task currently being
/// expanded trips the per-path cycle detector — but that failure is specific
/// to that method, not to the task: the planner must backtrack to the next
/// cycle-free method instead of aborting the whole expansion.
#[test]
fn hierarchical_cycle_via_one_method_backtracks_to_the_next() {
    let problem = PlanningProblem {
        tasks: vec![
            htn_task("top", None),
            htn_task("loop", None),
            htn_task("act", Some("act-primitive")),
        ],
        root_tasks: vec!["top".to_owned()],
        methods: vec![
            htn_method("m-top-cyclic", "top", &["loop", "act"]),
            htn_method("m-top-acyclic", "top", &["act"]),
            htn_method("m-loop", "loop", &["top"]),
        ],
        ..Default::default()
    };
    let result = solve(PlanningType::Hierarchical, problem);
    assert!(result.solved);
    assert_eq!(result.decomposition, ["act-primitive"]);
}

/// Cycle detection itself is still sound: a compound task whose ONLY method
/// recurses into itself still reports `HierarchyCycle` (backtracking found
/// no alternative method).
#[test]
fn hierarchical_pure_self_cycle_still_reports_hierarchy_cycle() {
    let problem = PlanningProblem {
        tasks: vec![htn_task("top", None)],
        root_tasks: vec!["top".to_owned()],
        methods: vec![htn_method("m-top", "top", &["top"])],
        ..Default::default()
    };
    assert_eq!(
        solve_hierarchical_error(problem),
        PlannerError::HierarchyCycle {
            task: "top".to_owned()
        }
    );
}

#[test]
fn partial_order_and_workflow_planners_execute_and_refuse_cycles() {
    let mut problem = hierarchy_problem();
    problem.root_tasks.clear();
    problem.methods.clear();
    problem.workflow_edges = vec![WorkflowEdge {
        before: "inspect".to_owned(),
        after: "verify".to_owned(),
    }];
    for kind in [PlanningType::PartialOrder, PlanningType::Workflow] {
        let result = solve(kind, problem.clone());
        assert!(result.solved);
        assert!(
            result
                .decomposition
                .iter()
                .position(|x| x == "inspect")
                .unwrap()
                < result
                    .decomposition
                    .iter()
                    .position(|x| x == "verify")
                    .unwrap()
        );
    }
    problem.workflow_edges.push(WorkflowEdge {
        before: "verify".to_owned(),
        after: "inspect".to_owned(),
    });
    let error = solve_planning_type(&UniversalPlanningRequest {
        planning_type: PlanningType::Workflow,
        problem,
        limits: Default::default(),
    })
    .unwrap_err();
    assert_eq!(error, PlannerError::WorkflowCycle);
}

#[test]
fn flow_constrained_planner_enforces_wip() {
    let mut open = chain_problem();
    open.queues = vec![QueueState {
        id: "verify".to_owned(),
        current_wip: 0,
        max_wip: 1,
    }];
    assert!(solve(PlanningType::FlowConstrained, open).solved);

    let mut full = chain_problem();
    full.queues = vec![QueueState {
        id: "verify".to_owned(),
        current_wip: 1,
        max_wip: 1,
    }];
    assert!(matches!(
        solve_planning_type(&UniversalPlanningRequest {
            planning_type: PlanningType::FlowConstrained,
            problem: full,
            limits: Default::default(),
        }),
        Err(PlannerError::WipBoundExceeded { .. })
    ));
}

#[test]
fn multi_agent_and_a2a_planners_assign_capable_owners() {
    let mut multi = chain_problem();
    multi.transitions[0].requires = set(&["read"]);
    multi.transitions[1].requires = set(&["test"]);
    multi.agents = vec![
        Agent {
            id: "reader".to_owned(),
            capabilities: set(&["read"]),
            capacity: 1,
            current_wip: 0,
        },
        Agent {
            id: "tester".to_owned(),
            capabilities: set(&["test"]),
            capacity: 1,
            current_wip: 0,
        },
    ];
    let result = solve(PlanningType::MultiAgent, multi);
    assert_eq!(result.steps[0].agent.as_deref(), Some("reader"));
    assert_eq!(result.steps[1].agent.as_deref(), Some("tester"));

    let mut delegated = hierarchy_problem();
    delegated.agents = vec![
        Agent {
            id: "reader".to_owned(),
            capabilities: set(&["read"]),
            capacity: 1,
            current_wip: 0,
        },
        Agent {
            id: "tester".to_owned(),
            capabilities: set(&["test"]),
            capacity: 1,
            current_wip: 0,
        },
    ];
    assert!(solve(PlanningType::A2aDelegated, delegated)
        .steps
        .iter()
        .all(|step| step.agent.is_some()));
}

#[test]
fn mcp_planner_binds_only_authorized_verified_receipted_tools() {
    let mut problem = hierarchy_problem();
    problem.tools = vec![
        Tool {
            id: "repo-read".to_owned(),
            capabilities: set(&["read"]),
            authority_bound: true,
            verifier_bound: true,
            receipt_bound: true,
        },
        Tool {
            id: "test-runner".to_owned(),
            capabilities: set(&["test"]),
            authority_bound: true,
            verifier_bound: true,
            receipt_bound: true,
        },
    ];
    let result = solve(PlanningType::McpBound, problem.clone());
    assert!(result.steps.iter().all(|step| step.tool.is_some()));

    problem.tools[0].authority_bound = false;
    assert!(matches!(
        solve_planning_type(&UniversalPlanningRequest {
            planning_type: PlanningType::McpBound,
            problem,
            limits: Default::default(),
        }),
        Err(PlannerError::AuthorityUnbound { .. })
    ));
}

#[test]
fn rdf_planner_constructs_and_solves_a_bounded_projection() {
    let rdf = vec![
        RdfTriple {
            subject: "s0".to_owned(),
            predicate: "state".to_owned(),
            object: "true".to_owned(),
        },
        RdfTriple {
            subject: "s0".to_owned(),
            predicate: "initial".to_owned(),
            object: "true".to_owned(),
        },
        RdfTriple {
            subject: "g".to_owned(),
            predicate: "state".to_owned(),
            object: "true".to_owned(),
        },
        RdfTriple {
            subject: "g".to_owned(),
            predicate: "goal".to_owned(),
            object: "true".to_owned(),
        },
        RdfTriple {
            subject: "move".to_owned(),
            predicate: "from".to_owned(),
            object: "s0".to_owned(),
        },
        RdfTriple {
            subject: "move".to_owned(),
            predicate: "to".to_owned(),
            object: "g".to_owned(),
        },
        RdfTriple {
            subject: "move".to_owned(),
            predicate: "action".to_owned(),
            object: "move".to_owned(),
        },
    ];
    let result = solve(
        PlanningType::RdfDerived,
        PlanningProblem {
            rdf,
            ..Default::default()
        },
    );
    assert!(result.solved);
    assert_eq!(result.steps[0].action, "move");
}

#[test]
fn every_planning_type_has_an_executing_fixture() {
    let fixtures = [
        (PlanningType::Classical, chain_problem()),
        (PlanningType::CostOptimal, chain_problem()),
        (PlanningType::Numeric, chain_problem()),
        (PlanningType::Temporal, chain_problem()),
        (PlanningType::Preferences, chain_problem()),
        (PlanningType::Hierarchical, hierarchy_problem()),
        (PlanningType::PartialOrder, hierarchy_problem()),
        (PlanningType::Workflow, hierarchy_problem()),
        (PlanningType::FlowConstrained, chain_problem()),
        (PlanningType::ResolutionAdaptive, hierarchy_problem()),
    ];
    for (kind, problem) in fixtures {
        let result = solve(kind, problem);
        assert_eq!(result.planning_type, Some(kind));
        assert!(result.solved);
    }
}
