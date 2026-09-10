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
use crate::grounder::{atom_key, evaluate_ground_goal, GroundEffectBranch, GroundedIR};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
#[cfg(test)]
use std::ops::Not;

const PROBABILITY_SCALE: u32 = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateError {
    /// `(not <non-atomic goal description>)` — e.g. `(not (and ...))`.
    /// `ferroplan::planning_runtime::Goal` has no negative-fact field, so a
    /// negative *atomic* literal is instead compiled here into a synthetic
    /// marker fact (see `negation_marker`) rather than needing one, but that
    /// compilation only knows how to negate a single ground atom — the same
    /// restriction the grounder already applies to `not` inside
    /// preconditions and `when`-conditions (see `grounder::flatten_goal`).
    UnsupportedNegativeGoal,
    /// `:goal` used `or`/`imply` at some point in the formula. Action
    /// *preconditions* fully support `or`/`imply` (see
    /// `grounder::GroundGoal`/`evaluate_ground_goal`), but the problem
    /// `:goal` still only supports positive/negative literals — extending
    /// past that means giving `ferroplan::planning_runtime::Goal` real
    /// disjunctive-goal semantics, which (like the negative-goal marker
    /// above) would change behavior for every planning family that shares
    /// that type, not just HDDL. Out of scope for this pass; refused loudly
    /// here rather than silently mis-translated.
    UnsupportedGoalConnective(String),
    UnboundVariable(String),
}

impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedNegativeGoal => {
                write!(f, "'not' of a non-atomic goal description is out of scope")
            }
            Self::UnsupportedGoalConnective(head) => write!(
                f,
                "'{head}' in ':goal' is out of scope (supported only in action preconditions)"
            ),
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

fn ground_atom_key(a: &crate::ast::AtomicFormula) -> Result<String, TranslateError> {
    let args = a
        .args
        .iter()
        .map(|t| match t {
            Term::Const(c) => Ok(c.clone()),
            Term::Var(v) => Err(TranslateError::UnboundVariable(v.clone())),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(atom_key(&a.predicate, &args))
}

/// The synthetic fact key standing in for "the ground atom `fact` is
/// currently absent". `fond_policy`/`Goal::holds` in `ferroplan`'s
/// `planning_runtime` only support goal membership as a positive-fact
/// subset test (`self.facts.is_subset(&state.facts)`) — there is no
/// negative-fact field to extend, and extending that shared `Goal` type
/// would change behavior for every planning family that consumes it
/// (Classical, CostOptimal, Fond, ...), not just HDDL. So a `(not P)` goal
/// literal is compiled entirely within this crate instead: once the real
/// BFS over ground-fact-set states below has produced its final, exact
/// per-state fact sets (used unmodified for actions' real precondition/
/// effect semantics), a `not:`-prefixed marker fact is inserted into
/// exactly those states where `fact` is genuinely absent — reachability-
/// filtering computed from the real per-state result, not a new `Goal`
/// shape. The `:` cannot appear in a real `atom_key` (predicate/object
/// names are plain HDDL identifiers), so no collision with a real fact is
/// possible.
fn negation_marker(fact: &str) -> String {
    format!("not:{fact}")
}

/// Splits a `:goal` description into positive ground atoms (`pos`) and
/// atoms whose absence the goal requires (`neg`). `Not` is only supported
/// directly around a single atom — `(not (and ...))` and similar remain
/// out of scope, the same restriction `grounder::flatten_goal` already
/// applies to preconditions and `when`-conditions.
fn flatten_goal(
    goal: &GoalDesc,
    pos: &mut BTreeSet<String>,
    neg: &mut BTreeSet<String>,
) -> Result<(), TranslateError> {
    match goal {
        GoalDesc::Empty => Ok(()),
        GoalDesc::Atom(a) => {
            pos.insert(ground_atom_key(a)?);
            Ok(())
        }
        GoalDesc::Not(inner) => match inner.as_ref() {
            GoalDesc::Atom(a) => {
                neg.insert(ground_atom_key(a)?);
                Ok(())
            }
            _ => Err(TranslateError::UnsupportedNegativeGoal),
        },
        GoalDesc::And(parts) => {
            for p in parts {
                flatten_goal(p, pos, neg)?;
            }
            Ok(())
        }
        GoalDesc::Or(_) => Err(TranslateError::UnsupportedGoalConnective("or".to_owned())),
        GoalDesc::Imply(_, _) => Err(TranslateError::UnsupportedGoalConnective(
            "imply".to_owned(),
        )),
    }
}

/// This action's real outcome probabilities, in ppm (parts-per-million out
/// of `PROBABILITY_SCALE`), one per `outcomes` entry in order, always
/// summing to exactly `PROBABILITY_SCALE`.
///
/// When every outcome carries a `probability_weight` (see
/// `grounder::GroundEffectBranch` — set from a `(:probabilistic ...)` effect
/// block, see `probabilistic::preprocess`) that parses to a nonnegative
/// `f64` with a positive total, the split is proportional to those real
/// weights via the largest-remainder method (assign each outcome
/// `floor(weight_i / total * PROBABILITY_SCALE)`, then hand out the leftover
/// ppm one at a time to the outcomes with the largest fractional remainder
/// so the exact-sum invariant holds even though flooring alone would
/// undercount). Otherwise (a plain `oneof`, or a mix of weighted/unweighted
/// outcomes, or unparseable/all-zero weights) every outcome gets an equal
/// share — the original, weight-oblivious behavior, preserved byte-for-byte
/// for every domain that doesn't use `:probabilistic`.
fn outcome_ppms(outcomes: &[GroundEffectBranch]) -> Vec<u32> {
    let n = outcomes.len();
    if n == 0 {
        return Vec::new();
    }

    let declared_weights: Option<Vec<f64>> = outcomes
        .iter()
        .map(|o| {
            o.probability_weight
                .as_deref()
                .and_then(|w| w.parse::<f64>().ok())
                .filter(|w| w.is_finite() && *w >= 0.0)
        })
        .collect();

    if let Some(weights) = declared_weights {
        let total: f64 = weights.iter().sum();
        if total > 0.0 {
            let scale = f64::from(PROBABILITY_SCALE);
            let raw: Vec<f64> = weights.iter().map(|w| w / total * scale).collect();
            let mut ppms: Vec<u32> = raw.iter().map(|r| r.floor() as u32).collect();
            let assigned: u32 = ppms.iter().sum();
            let mut leftover = PROBABILITY_SCALE - assigned;
            let mut order: Vec<usize> = (0..n).collect();
            order.sort_by(|&a, &b| {
                let fa = raw[a] - raw[a].floor();
                let fb = raw[b] - raw[b].floor();
                fb.partial_cmp(&fa).unwrap_or(std::cmp::Ordering::Equal)
            });
            for &i in &order {
                if leftover == 0 {
                    break;
                }
                ppms[i] += 1;
                leftover -= 1;
            }
            return ppms;
        }
    }

    let branch_count = n as u32;
    let base = PROBABILITY_SCALE / branch_count;
    let remainder = PROBABILITY_SCALE % branch_count;
    (0..n)
        .map(|i| base + u32::from((i as u32) < remainder))
        .collect()
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
    let mut goal_pos = BTreeSet::new();
    let mut goal_neg = BTreeSet::new();
    flatten_goal(&ir.goal, &mut goal_pos, &mut goal_neg)?;

    let mut state_ids: BTreeMap<BTreeSet<String>, String> = BTreeMap::new();
    let mut states: Vec<State> = Vec::new();
    let mut transitions: Vec<Transition> = Vec::new();

    let initial_id = intern_state(&ir.initial_facts, &mut state_ids, &mut states);
    let mut queue = VecDeque::from([ir.initial_facts.clone()]);
    let mut visited = BTreeSet::from([ir.initial_facts.clone()]);

    while let Some(facts) = queue.pop_front() {
        let from_id = state_ids[&facts].clone();
        for action in &ir.actions {
            // `evaluate_ground_goal` subsumes the old flat
            // "pos-subset-and-no-neg-overlap" check (that check is exactly
            // this evaluation on an And-of-Atom/Not(Atom) formula) while also
            // correctly handling `or`/`imply` preconditions, which cannot be
            // represented as a flat positive/negative literal set.
            if !evaluate_ground_goal(&action.precondition, &facts) {
                continue;
            }
            if action.outcomes.is_empty() {
                continue;
            }
            let ppms = outcome_ppms(&action.outcomes);
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
                let ppm = ppms[i];
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

    // Reachability-filtering for negative goal literals: the real BFS above
    // is already finished (every state's `facts` is its exact, final ground
    // fact set, computed purely from real action effects — untouched by
    // what follows). For each `(not P)` goal literal, add the synthetic
    // `negation_marker(P)` fact to exactly the states where `P` is genuinely
    // absent, so the shared, unmodified `Goal::holds` positive-subset test
    // downstream can express "P does not hold" as "this marker fact does".
    if !goal_neg.is_empty() {
        for state in &mut states {
            for neg_fact in &goal_neg {
                if !state.facts.contains(neg_fact) {
                    state.facts.insert(negation_marker(neg_fact));
                }
            }
        }
    }
    let mut goal_facts = goal_pos;
    for neg_fact in &goal_neg {
        goal_facts.insert(negation_marker(neg_fact));
    }
    let goal = Goal { facts: goal_facts };

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
    const FIXTURE_E_DOMAIN: &str = include_str!("../fixtures/e/domain.hddl");
    const FIXTURE_E_PROBLEM: &str = include_str!("../fixtures/e/problem.hddl");

    /// End-to-end parse -> ground -> translate over fixture E's
    /// `(:probabilistic 3 (and (heads)) 1 (and (tails)))` effect: the real
    /// resulting transition probabilities must land on the declared 3:1
    /// split (750_000/250_000 ppm), not an even 500_000/500_000 split —
    /// proving `outcome_ppms` actually consumes the weight, not just that
    /// grounding carries it.
    #[test]
    fn probabilistic_weights_translate_into_proportional_transition_probabilities() {
        let domain = parse_domain(FIXTURE_E_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_E_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let plan = translate(&ir).expect("fixture E translates");

        let initial = plan
            .states
            .iter()
            .find(|s| s.id == plan.initial_states[0])
            .unwrap();
        let toss_edges = plan
            .transitions
            .iter()
            .filter(|t| t.from == initial.id && t.action == "toss-coin")
            .collect::<Vec<_>>();
        assert_eq!(toss_edges.len(), 2);
        let mass: u32 = toss_edges.iter().map(|t| t.probability_ppm).sum();
        assert_eq!(mass, 1_000_000);

        let mut heads_ppm = None;
        let mut tails_ppm = None;
        for edge in &toss_edges {
            let to_state = plan.states.iter().find(|s| s.id == edge.to).unwrap();
            if to_state.facts.contains("heads") {
                heads_ppm = Some(edge.probability_ppm);
            }
            if to_state.facts.contains("tails") {
                tails_ppm = Some(edge.probability_ppm);
            }
        }
        // Weights are 3:1 -> 750_000/250_000, NOT an even 500_000/500_000
        // split -- this is the assertion that would fail if `translate`
        // still ignored `probability_weight` and split evenly.
        assert_eq!(heads_ppm, Some(750_000));
        assert_eq!(tails_ppm, Some(250_000));
    }

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

    /// `(not (at l1))` at the top of `:goal` (no `and` wrapper) must be
    /// accepted and compiled: no state exists yet, so this only exercises
    /// `flatten_goal` + the empty-BFS goal-fact synthesis, not reachability
    /// filtering across real states (see the fixture-driven tests below for
    /// that).
    #[test]
    fn accepts_bare_negative_goal_atom() {
        use crate::ast::AtomicFormula;
        let ir = GroundedIR {
            goal: GoalDesc::Not(Box::new(GoalDesc::Atom(AtomicFormula {
                predicate: "at".to_owned(),
                args: vec![Term::Const("l1".to_owned())],
            }))),
            initial_facts: BTreeSet::new(),
            ..GroundedIR::default()
        };
        let problem = translate(&ir).expect("bare negative goal atom translates");
        // The lone (empty) reachable state has no "at(l1)" fact, so the
        // negation marker must be present on it and required by the goal.
        assert!(problem.goal.facts.contains("not:at(l1)"));
        assert!(!problem.goal.facts.contains("at(l1)"));
        assert_eq!(problem.states.len(), 1);
        assert!(problem.states[0].facts.contains("not:at(l1)"));
    }

    /// `(not <non-atomic goal description>)` remains out of scope — the same
    /// restriction `grounder::flatten_goal` already applies to preconditions
    /// and `when`-conditions. This is the one shape `TranslateError::
    /// UnsupportedNegativeGoal` still names.
    #[test]
    fn rejects_not_of_a_non_atomic_goal_description() {
        use crate::ast::AtomicFormula;
        let ir = GroundedIR {
            goal: GoalDesc::Not(Box::new(GoalDesc::And(vec![GoalDesc::Atom(
                AtomicFormula {
                    predicate: "at".to_owned(),
                    args: vec![Term::Const("l1".to_owned())],
                },
            )]))),
            ..GroundedIR::default()
        };
        let err = translate(&ir).unwrap_err();
        assert_eq!(err, TranslateError::UnsupportedNegativeGoal);
    }

    /// End-to-end (parse -> ground -> translate) over fixture A (a simple
    /// pickup/drive/dropoff transport domain) with a compound goal `(and (at
    /// l2) (not (has-package)))`: reaching l2 without having dropped the
    /// package off first must NOT count as a goal state, proving the
    /// negation marker really does reachability-filter on the BFS's real,
    /// distinct per-state fact sets rather than trivially holding everywhere.
    #[test]
    fn negative_goal_literal_filters_real_reachable_states() {
        const DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
        // Fixture A's domain (pickup/drive/dropoff) with a compound goal
        // requiring `(at l2)` AND the ABSENCE of `has-package` — i.e. the
        // package must have been dropped off, not merely carried past l2.
        const PROBLEM: &str = "(define (problem transport-a-p1-neg)
  (:domain transport-a)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (and (at l2) (not (has-package)))))";

        let domain = parse_domain(DOMAIN).unwrap();
        let problem = parse_problem(PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let plan = translate(&ir).expect("negative goal literal translates");

        // Real reachable state with (at l2) AND (has-package) still set
        // (pickup -> drive, before dropoff) must exist and must NOT satisfy
        // the compound goal.
        let at_l2_with_package = plan
            .states
            .iter()
            .find(|s| s.facts.contains("at(l2)") && s.facts.contains("has-package"))
            .expect("a real state with at(l2) and has-package must be reachable");
        assert!(
            !plan.goal.facts.is_subset(&at_l2_with_package.facts),
            "a state that still holds the negated fact must not satisfy the goal"
        );
        assert!(at_l2_with_package.facts.contains("not:has-package").not());

        // Real reachable state with (at l2) and NOT (has-package) (after
        // dropoff) must exist and must satisfy the compound goal.
        let at_l2_dropped_off = plan
            .states
            .iter()
            .find(|s| s.facts.contains("at(l2)") && !s.facts.contains("has-package"))
            .expect("a real state with at(l2) and no has-package must be reachable");
        assert!(at_l2_dropped_off.facts.contains("not:has-package"));
        assert!(plan.goal.facts.is_subset(&at_l2_dropped_off.facts));
    }
}
