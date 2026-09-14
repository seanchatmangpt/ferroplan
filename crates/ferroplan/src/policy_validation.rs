//! Independent validation for FOND policies over the universal planning IR.
//!
//! The synthesizer and validator deliberately do not share fixpoint code. The
//! validator consumes only a [`PlanningProblem`] and returned [`UniversalPlan`],
//! reconstructs the selected-action graph, checks it against the declared
//! transition relation, and classifies the policy as a bounded-step `Strong`
//! policy, a fairness-dependent `StrongCyclic` policy, or `Invalid`.
//!
//! This module manufactures evidence only. Validation never grants execution or
//! actuation authority.

use crate::planning_runtime::{PlanningProblem, PolicyEntry, UniversalPlan};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

const PROBABILITY_SCALE: u64 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicyGuarantee {
    /// Every outcome of every selected action reaches the goal in a bounded
    /// number of policy steps; no fairness assumption is required.
    Strong,
    /// Every reachable state stays inside the policy and can reach a goal;
    /// progress therefore relies on the standard strong-cyclic fairness
    /// assumption for nondeterministic outcomes.
    StrongCyclic,
    /// The candidate is structurally invalid or does not satisfy either
    /// strong or strong-cyclic obligations from every initial state.
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicyIssue {
    UnknownInitialState { state: String },
    DuplicatePolicyState { state: String },
    UnknownPolicyState { state: String },
    PolicyOnGoalState { state: String },
    MissingPolicyEntry { state: String },
    UnknownAction { state: String, action: String },
    UnknownTransitionTarget { state: String, action: String, target: String },
    InvalidProbabilityMass { state: String, action: String, mass: u64 },
    OutcomeMismatch { state: String, action: String },
    UnsafeReachableState { state: String },
    NoGoalProgress { state: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyValidationReport {
    pub valid: bool,
    pub guarantee: PolicyGuarantee,
    /// All states reachable from the declared initial states while following
    /// the candidate policy. Sorted for deterministic replay and hashing.
    pub reachable_states: Vec<String>,
    /// Goal states reached by the policy graph. Sorted for deterministic replay.
    pub reachable_goals: Vec<String>,
    /// Structural or semantic falsifiers. Empty for an admitted policy.
    pub issues: Vec<PolicyIssue>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OutcomeKey {
    state: String,
    probability_ppm: u32,
    observation: Option<String>,
}

fn goal_holds(problem: &PlanningProblem, state_id: &str) -> bool {
    let Some(state) = problem.states.iter().find(|state| state.id == state_id) else {
        return false;
    };
    problem.goal.facts.is_subset(&state.facts)
        && problem
            .goal
            .numeric_min
            .iter()
            .all(|(name, value)| state.fluents.get(name).is_some_and(|actual| actual >= value))
        && problem
            .goal
            .numeric_max
            .iter()
            .all(|(name, value)| state.fluents.get(name).is_some_and(|actual| actual <= value))
}

fn transition_groups(
    problem: &PlanningProblem,
) -> BTreeMap<(String, String), BTreeSet<OutcomeKey>> {
    let mut groups = BTreeMap::<(String, String), BTreeSet<OutcomeKey>>::new();
    for edge in &problem.transitions {
        groups
            .entry((edge.from.clone(), edge.action.clone()))
            .or_default()
            .insert(OutcomeKey {
                state: edge.to.clone(),
                probability_ppm: edge.probability_ppm,
                observation: edge.observation.clone(),
            });
    }
    groups
}

fn policy_outcomes(entry: &PolicyEntry) -> BTreeSet<OutcomeKey> {
    entry
        .outcomes
        .iter()
        .map(|outcome| OutcomeKey {
            state: outcome.state.clone(),
            probability_ppm: outcome.probability_ppm,
            observation: outcome.observation.clone(),
        })
        .collect()
}

/// Validate a FOND policy independently from the synthesis implementation.
///
/// Admission is intentionally strict:
///
/// * every reachable non-goal state must select exactly one declared action;
/// * the policy's declared outcomes must exactly match that action's transition
///   outcomes, including probability and observation labels;
/// * no selected action may reach an unknown or explicitly unsafe state;
/// * each action's probability mass must total 1_000_000 ppm;
/// * the reachable selected-action graph must satisfy either the strong
///   least-fixpoint condition or the strong-cyclic fairness condition.
///
/// The returned report is evidence only and carries no actuation authority.
pub fn validate_fond_policy(
    problem: &PlanningProblem,
    plan: &UniversalPlan,
) -> PolicyValidationReport {
    let state_ids = problem
        .states
        .iter()
        .map(|state| state.id.clone())
        .collect::<BTreeSet<_>>();
    let transitions = transition_groups(problem);
    let mut issues = Vec::<PolicyIssue>::new();

    let mut entries = BTreeMap::<String, &PolicyEntry>::new();
    for entry in &plan.policy {
        if !state_ids.contains(&entry.state) {
            issues.push(PolicyIssue::UnknownPolicyState {
                state: entry.state.clone(),
            });
            continue;
        }
        if entries.insert(entry.state.clone(), entry).is_some() {
            issues.push(PolicyIssue::DuplicatePolicyState {
                state: entry.state.clone(),
            });
        }
        if goal_holds(problem, &entry.state) {
            issues.push(PolicyIssue::PolicyOnGoalState {
                state: entry.state.clone(),
            });
        }
    }

    let mut queue = VecDeque::<String>::new();
    let mut reachable = BTreeSet::<String>::new();
    for initial in &problem.initial_states {
        if !state_ids.contains(initial) {
            issues.push(PolicyIssue::UnknownInitialState {
                state: initial.clone(),
            });
            continue;
        }
        if reachable.insert(initial.clone()) {
            queue.push_back(initial.clone());
        }
    }

    while let Some(state) = queue.pop_front() {
        if problem.unsafe_states.contains(&state) {
            issues.push(PolicyIssue::UnsafeReachableState {
                state: state.clone(),
            });
        }
        if goal_holds(problem, &state) {
            continue;
        }

        let Some(entry) = entries.get(&state).copied() else {
            issues.push(PolicyIssue::MissingPolicyEntry {
                state: state.clone(),
            });
            continue;
        };
        let key = (state.clone(), entry.action.clone());
        let Some(expected) = transitions.get(&key) else {
            issues.push(PolicyIssue::UnknownAction {
                state: state.clone(),
                action: entry.action.clone(),
            });
            continue;
        };

        let mass = expected
            .iter()
            .map(|outcome| u64::from(outcome.probability_ppm))
            .sum::<u64>();
        if mass != PROBABILITY_SCALE {
            issues.push(PolicyIssue::InvalidProbabilityMass {
                state: state.clone(),
                action: entry.action.clone(),
                mass,
            });
        }

        if policy_outcomes(entry) != *expected {
            issues.push(PolicyIssue::OutcomeMismatch {
                state: state.clone(),
                action: entry.action.clone(),
            });
        }

        for outcome in expected {
            if !state_ids.contains(&outcome.state) {
                issues.push(PolicyIssue::UnknownTransitionTarget {
                    state: state.clone(),
                    action: entry.action.clone(),
                    target: outcome.state.clone(),
                });
                continue;
            }
            if reachable.insert(outcome.state.clone()) {
                queue.push_back(outcome.state.clone());
            }
        }
    }

    let reachable_goals = reachable
        .iter()
        .filter(|state| goal_holds(problem, state))
        .cloned()
        .collect::<BTreeSet<_>>();

    if !issues.is_empty() || reachable_goals.is_empty() {
        if reachable_goals.is_empty() {
            for state in reachable
                .iter()
                .filter(|state| !goal_holds(problem, state))
            {
                issues.push(PolicyIssue::NoGoalProgress {
                    state: state.clone(),
                });
            }
        }
        issues.sort_by_key(|issue| format!("{issue:?}"));
        issues.dedup();
        return PolicyValidationReport {
            valid: false,
            guarantee: PolicyGuarantee::Invalid,
            reachable_states: reachable.into_iter().collect(),
            reachable_goals: reachable_goals.into_iter().collect(),
            issues,
        };
    }

    // Independent strong-policy check: least fixed point grown backward from
    // reachable goal states, using only the candidate's selected action.
    let mut strong_winning = reachable_goals.clone();
    loop {
        let mut changed = false;
        for state in &reachable {
            if strong_winning.contains(state) || goal_holds(problem, state) {
                continue;
            }
            let Some(entry) = entries.get(state).copied() else {
                continue;
            };
            let key = (state.clone(), entry.action.clone());
            let Some(outcomes) = transitions.get(&key) else {
                continue;
            };
            if !outcomes.is_empty()
                && outcomes
                    .iter()
                    .all(|outcome| strong_winning.contains(&outcome.state))
            {
                strong_winning.insert(state.clone());
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let all_initials_strong = problem
        .initial_states
        .iter()
        .all(|state| strong_winning.contains(state));
    if all_initials_strong {
        return PolicyValidationReport {
            valid: true,
            guarantee: PolicyGuarantee::Strong,
            reachable_states: reachable.into_iter().collect(),
            reachable_goals: reachable_goals.into_iter().collect(),
            issues,
        };
    }

    // Independent strong-cyclic check: build the reverse graph induced by the
    // selected policy, then require every reachable state to have a path to a
    // reachable goal. In a finite closed policy graph this excludes non-goal
    // bottom SCCs; under the standard fair-outcome assumption the remaining
    // cycles eventually make progress to a goal.
    let mut predecessors = BTreeMap::<String, BTreeSet<String>>::new();
    for state in &reachable {
        if goal_holds(problem, state) {
            continue;
        }
        let Some(entry) = entries.get(state).copied() else {
            continue;
        };
        let key = (state.clone(), entry.action.clone());
        if let Some(outcomes) = transitions.get(&key) {
            for outcome in outcomes {
                if reachable.contains(&outcome.state) {
                    predecessors
                        .entry(outcome.state.clone())
                        .or_default()
                        .insert(state.clone());
                }
            }
        }
    }

    let mut can_reach_goal = reachable_goals.clone();
    let mut reverse_queue = VecDeque::from_iter(reachable_goals.iter().cloned());
    while let Some(state) = reverse_queue.pop_front() {
        for predecessor in predecessors.get(&state).into_iter().flatten() {
            if can_reach_goal.insert(predecessor.clone()) {
                reverse_queue.push_back(predecessor.clone());
            }
        }
    }

    let stranded = reachable
        .difference(&can_reach_goal)
        .cloned()
        .collect::<Vec<_>>();
    if stranded.is_empty() {
        PolicyValidationReport {
            valid: true,
            guarantee: PolicyGuarantee::StrongCyclic,
            reachable_states: reachable.into_iter().collect(),
            reachable_goals: reachable_goals.into_iter().collect(),
            issues,
        }
    } else {
        for state in stranded {
            issues.push(PolicyIssue::NoGoalProgress { state });
        }
        PolicyValidationReport {
            valid: false,
            guarantee: PolicyGuarantee::Invalid,
            reachable_states: reachable.into_iter().collect(),
            reachable_goals: reachable_goals.into_iter().collect(),
            issues,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planning_runtime::{Goal, PolicyOutcome, State, Transition};

    fn state(id: &str, facts: &[&str]) -> State {
        State {
            id: id.to_owned(),
            facts: facts.iter().map(|fact| (*fact).to_owned()).collect(),
            fluents: BTreeMap::new(),
        }
    }

    fn edge(action: &str, from: &str, to: &str, probability_ppm: u32) -> Transition {
        Transition {
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

    fn policy(state: &str, action: &str, outcomes: &[(&str, u32)]) -> PolicyEntry {
        PolicyEntry {
            state: state.to_owned(),
            action: action.to_owned(),
            outcomes: outcomes
                .iter()
                .map(|(target, probability_ppm)| PolicyOutcome {
                    state: (*target).to_owned(),
                    probability_ppm: *probability_ppm,
                    observation: None,
                })
                .collect(),
        }
    }

    fn base_problem(states: Vec<State>, transitions: Vec<Transition>) -> PlanningProblem {
        PlanningProblem {
            states,
            initial_states: vec!["s0".to_owned()],
            goal: Goal {
                facts: BTreeSet::from(["goal".to_owned()]),
                numeric_min: BTreeMap::new(),
                numeric_max: BTreeMap::new(),
            },
            transitions,
            ..PlanningProblem::default()
        }
    }

    #[test]
    fn admits_an_acyclic_strong_policy() {
        let problem = base_problem(
            vec![state("s0", &[]), state("s1", &[]), state("g", &["goal"])],
            vec![edge("advance", "s0", "s1", 1_000_000), edge("finish", "s1", "g", 1_000_000)],
        );
        let plan = UniversalPlan {
            solved: true,
            policy: vec![
                policy("s0", "advance", &[("s1", 1_000_000)]),
                policy("s1", "finish", &[("g", 1_000_000)]),
            ],
            ..UniversalPlan::default()
        };

        let report = validate_fond_policy(&problem, &plan);
        assert!(report.valid, "{report:?}");
        assert_eq!(report.guarantee, PolicyGuarantee::Strong);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn admits_a_retry_loop_only_as_strong_cyclic() {
        let problem = base_problem(
            vec![state("s0", &[]), state("g", &["goal"])],
            vec![
                edge("retry", "s0", "s0", 500_000),
                edge("retry", "s0", "g", 500_000),
            ],
        );
        let plan = UniversalPlan {
            solved: true,
            policy: vec![policy("s0", "retry", &[("s0", 500_000), ("g", 500_000)])],
            ..UniversalPlan::default()
        };

        let report = validate_fond_policy(&problem, &plan);
        assert!(report.valid, "{report:?}");
        assert_eq!(report.guarantee, PolicyGuarantee::StrongCyclic);
    }

    #[test]
    fn refuses_a_closed_non_goal_trap() {
        let problem = base_problem(
            vec![state("s0", &[]), state("trap", &[]), state("g", &["goal"])],
            vec![
                edge("choose", "s0", "trap", 500_000),
                edge("choose", "s0", "g", 500_000),
                edge("loop", "trap", "trap", 1_000_000),
            ],
        );
        let plan = UniversalPlan {
            solved: true,
            policy: vec![
                policy("s0", "choose", &[("trap", 500_000), ("g", 500_000)]),
                policy("trap", "loop", &[("trap", 1_000_000)]),
            ],
            ..UniversalPlan::default()
        };

        let report = validate_fond_policy(&problem, &plan);
        assert!(!report.valid);
        assert_eq!(report.guarantee, PolicyGuarantee::Invalid);
        assert!(report.issues.contains(&PolicyIssue::NoGoalProgress {
            state: "trap".to_owned(),
        }));
    }

    #[test]
    fn refuses_policy_outcome_drift() {
        let problem = base_problem(
            vec![state("s0", &[]), state("g", &["goal"])],
            vec![edge("finish", "s0", "g", 1_000_000)],
        );
        let plan = UniversalPlan {
            solved: true,
            policy: vec![policy("s0", "finish", &[("g", 900_000)])],
            ..UniversalPlan::default()
        };

        let report = validate_fond_policy(&problem, &plan);
        assert!(!report.valid);
        assert!(report.issues.contains(&PolicyIssue::OutcomeMismatch {
            state: "s0".to_owned(),
            action: "finish".to_owned(),
        }));
    }
}
