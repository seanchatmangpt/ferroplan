//! Ground IR -> a flat FOND planning-problem IR: full breadth-first
//! enumeration of the reachable ground-fact-set state space, expanding every
//! applicable ground action's outcomes (including `when`-conditional
//! contributions) into real transitions with millionths-of-probability mass
//! summing to exactly 1_000_000 per (state, action) pair.
//!
//! The types here (`State`/`Goal`/`Transition`/`Task`/`Method`/
//! `PlanningProblem`) deliberately mirror `ferroplan::planning_runtime`'s
//! shapes field-for-field rather than importing them — see the crate-level
//! dependency-direction note in `Cargo.toml`/`lib.rs`. The adapter that maps
//! one onto the other lives in the `ferroplan` crate.

use crate::ast::{GoalDesc, Term};
use crate::grounder::{atom_key, GroundedIR};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

const PROBABILITY_SCALE: u32 = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateError {
    /// `ferroplan::planning_runtime::Goal` has no negative-fact field, so a
    /// `:goal` containing `(not ...)` cannot be represented — rejected here
    /// at the translation boundary rather than in the AST/grounder, per the
    /// design spec.
    UnsupportedNegativeGoal,
    UnboundVariable(String),
}

impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedNegativeGoal => {
                write!(f, "negative goal literals are not representable")
            }
            Self::UnboundVariable(v) => write!(f, "unbound variable '?{v}' in goal"),
        }
    }
}
impl std::error::Error for TranslateError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    pub id: String,
    pub facts: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Goal {
    pub facts: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub action: String,
    pub from: String,
    pub to: String,
    pub probability_ppm: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Task {
    pub id: String,
    pub primitive_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Method {
    pub id: String,
    pub task: String,
    pub subtasks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlanningProblem {
    pub states: Vec<State>,
    pub initial_states: Vec<String>,
    pub goal: Goal,
    pub transitions: Vec<Transition>,
    pub tasks: Vec<Task>,
    pub root_tasks: Vec<String>,
    pub methods: Vec<Method>,
}

fn flatten_goal_positive(
    goal: &GoalDesc,
    out: &mut BTreeSet<String>,
) -> Result<(), TranslateError> {
    match goal {
        GoalDesc::Empty => Ok(()),
        GoalDesc::Atom(a) => {
            let args = a
                .args
                .iter()
                .map(|t| match t {
                    Term::Const(c) => Ok(c.clone()),
                    Term::Var(v) => Err(TranslateError::UnboundVariable(v.clone())),
                })
                .collect::<Result<Vec<_>, _>>()?;
            out.insert(atom_key(&a.predicate, &args));
            Ok(())
        }
        GoalDesc::Not(_) => Err(TranslateError::UnsupportedNegativeGoal),
        GoalDesc::And(parts) => {
            for p in parts {
                flatten_goal_positive(p, out)?;
            }
            Ok(())
        }
    }
}

fn intern_state(
    facts: &BTreeSet<String>,
    state_ids: &mut BTreeMap<BTreeSet<String>, String>,
    states: &mut Vec<State>,
) -> String {
    if let Some(id) = state_ids.get(facts) {
        return id.clone();
    }
    let id = format!("s{}", state_ids.len());
    state_ids.insert(facts.clone(), id.clone());
    states.push(State {
        id: id.clone(),
        facts: facts.clone(),
    });
    id
}

pub fn translate(ir: &GroundedIR) -> Result<PlanningProblem, TranslateError> {
    let mut goal_facts = BTreeSet::new();
    flatten_goal_positive(&ir.goal, &mut goal_facts)?;
    let goal = Goal { facts: goal_facts };

    let mut state_ids: BTreeMap<BTreeSet<String>, String> = BTreeMap::new();
    let mut states: Vec<State> = Vec::new();
    let mut transitions: Vec<Transition> = Vec::new();

    let initial_id = intern_state(&ir.initial_facts, &mut state_ids, &mut states);
    let mut queue = VecDeque::from([ir.initial_facts.clone()]);
    let mut visited = BTreeSet::from([ir.initial_facts.clone()]);

    while let Some(facts) = queue.pop_front() {
        let from_id = state_ids[&facts].clone();
        for action in &ir.actions {
            if !action.pos_pre.is_subset(&facts) {
                continue;
            }
            if action.neg_pre.iter().any(|f| facts.contains(f)) {
                continue;
            }
            let branch_count = action.outcomes.len() as u32;
            if branch_count == 0 {
                continue;
            }
            let base = PROBABILITY_SCALE / branch_count;
            let remainder = PROBABILITY_SCALE % branch_count;
            for (i, branch) in action.outcomes.iter().enumerate() {
                let mut next = facts.clone();
                for d in &branch.del {
                    next.remove(d);
                }
                for a in &branch.add {
                    next.insert(a.clone());
                }
                for cond in &branch.conditional {
                    let holds = cond.pos_cond.is_subset(&facts)
                        && cond.neg_cond.iter().all(|f| !facts.contains(f));
                    if holds {
                        for d in &cond.del {
                            next.remove(d);
                        }
                        for a in &cond.add {
                            next.insert(a.clone());
                        }
                    }
                }
                let to_id = intern_state(&next, &mut state_ids, &mut states);
                let ppm = base + u32::from((i as u32) < remainder);
                transitions.push(Transition {
                    action: action.name.clone(),
                    from: from_id.clone(),
                    to: to_id,
                    probability_ppm: ppm,
                });
                if visited.insert(next.clone()) {
                    queue.push_back(next);
                }
            }
        }
    }

    let mut tasks = Vec::new();
    let mut seen_tasks = BTreeSet::new();
    for action in &ir.actions {
        if seen_tasks.insert(action.name.clone()) {
            tasks.push(Task {
                id: action.name.clone(),
                primitive_action: Some(action.name.clone()),
            });
        }
    }
    for m in &ir.methods {
        if seen_tasks.insert(m.task_name.clone()) {
            tasks.push(Task {
                id: m.task_name.clone(),
                primitive_action: None,
            });
        }
    }
    let methods = ir
        .methods
        .iter()
        .map(|m| Method {
            id: m.name.clone(),
            task: m.task_name.clone(),
            subtasks: m.subtasks.iter().map(|s| s.task_name.clone()).collect(),
        })
        .collect();

    Ok(PlanningProblem {
        states,
        initial_states: vec![initial_id],
        goal,
        transitions,
        tasks,
        root_tasks: ir.root_tasks.clone(),
        methods,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grounder::{ground, GroundingLimits};
    use crate::parser::{parse_domain, parse_problem};

    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_C_PROBLEM: &str = include_str!("../fixtures/c/problem.hddl");

    /// Fixture C's oneof action ("cross-bridge") lands either on the goal
    /// state directly or on a safe fallback from which a deterministic
    /// "walk" reaches the goal — a real (non-cyclic) two-outcome AND/OR
    /// structure, not a self-loop.
    #[test]
    fn translates_oneof_fixture_into_a_real_state_graph() {
        let domain = parse_domain(FIXTURE_C_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_C_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let problem = translate(&ir).unwrap();

        assert_eq!(problem.initial_states.len(), 1);
        assert!(problem.goal.facts.contains("at(l2)"));

        let initial = problem
            .states
            .iter()
            .find(|s| s.id == problem.initial_states[0])
            .unwrap();
        assert!(initial.facts.contains("at(l1)"));

        // The oneof action must appear with exactly two outcomes from the
        // initial state, splitting the full 1_000_000 probability mass.
        let cross_bridge_edges = problem
            .transitions
            .iter()
            .filter(|t| t.from == initial.id && t.action == "cross-bridge(l1,l2,l3)")
            .collect::<Vec<_>>();
        assert_eq!(cross_bridge_edges.len(), 2);
        let mass: u32 = cross_bridge_edges.iter().map(|t| t.probability_ppm).sum();
        assert_eq!(mass, 1_000_000);

        // One outcome must land directly on a goal state, the other on a
        // non-goal state from which "walk" is applicable and deterministic.
        let mut landed_on_goal = false;
        let mut landed_on_fallback = false;
        for edge in &cross_bridge_edges {
            let to_state = problem.states.iter().find(|s| s.id == edge.to).unwrap();
            if to_state.facts.contains("at(l2)") {
                landed_on_goal = true;
            }
            if to_state.facts.contains("at(l3)") {
                landed_on_fallback = true;
                let walk_edges = problem
                    .transitions
                    .iter()
                    .filter(|t| t.from == to_state.id && t.action == "walk(l3,l2)")
                    .collect::<Vec<_>>();
                assert_eq!(walk_edges.len(), 1);
                assert_eq!(walk_edges[0].probability_ppm, 1_000_000);
                let walk_to = problem
                    .states
                    .iter()
                    .find(|s| s.id == walk_edges[0].to)
                    .unwrap();
                assert!(walk_to.facts.contains("at(l2)"));
            }
        }
        assert!(landed_on_goal && landed_on_fallback);
    }

    #[test]
    fn rejects_negative_goal() {
        use crate::ast::AtomicFormula;
        let ir = GroundedIR {
            goal: GoalDesc::Not(Box::new(GoalDesc::Atom(AtomicFormula {
                predicate: "at".to_owned(),
                args: vec![Term::Const("l1".to_owned())],
            }))),
            ..GroundedIR::default()
        };
        let err = translate(&ir).unwrap_err();
        assert_eq!(err, TranslateError::UnsupportedNegativeGoal);
    }
}
