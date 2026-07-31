use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    validate_ppddl_policy, PolicyDecision, ProbabilisticObjective, ProbabilisticOptions,
    ProbabilisticSolution,
};

use super::{
    canonical_digest, ConstraintVerdict, PolicyCounterexample, PolicyCounterexampleKind,
    PolicyVerificationReport, RiskConstraint, Standing, StandingReason, ValueInterval,
};

#[derive(Clone, Copy, Debug, Default)]
struct Evaluation {
    goal: f64,
    unsafe_reach: f64,
    reward: f64,
}

impl Evaluation {
    fn weighted_add(&mut self, probability: f64, other: Self, immediate_reward: f64, discount: f64) {
        self.goal += probability * other.goal;
        self.unsafe_reach += probability * other.unsafe_reach;
        self.reward += probability * (immediate_reward + discount * other.reward);
    }
}

fn is_unsafe(solution: &ProbabilisticSolution, state: usize, unsafe_fact: Option<&str>) -> bool {
    let Some(label) = unsafe_fact else {
        return false;
    };
    solution
        .states
        .iter()
        .find(|candidate| candidate.id == state)
        .map(|candidate| {
            candidate
                .facts
                .iter()
                .any(|fact| fact.eq_ignore_ascii_case(label))
        })
        .unwrap_or(false)
}

fn decision_map(solution: &ProbabilisticSolution) -> HashMap<(usize, Option<usize>), &PolicyDecision> {
    solution
        .policy
        .iter()
        .map(|decision| ((decision.state, decision.remaining), decision))
        .collect()
}

fn finite_eval(
    solution: &ProbabilisticSolution,
    state: usize,
    remaining: usize,
    unsafe_fact: Option<&str>,
    decisions: &HashMap<(usize, Option<usize>), &PolicyDecision>,
    memo: &mut HashMap<(usize, usize), Evaluation>,
) -> Evaluation {
    if let Some(value) = memo.get(&(state, remaining)) {
        return *value;
    }
    let goal = solution
        .states
        .iter()
        .find(|candidate| candidate.id == state)
        .map(|candidate| candidate.goal)
        .unwrap_or(false);
    if goal || remaining == 0 {
        let value = Evaluation {
            goal: if goal { 1.0 } else { 0.0 },
            unsafe_reach: if is_unsafe(solution, state, unsafe_fact) {
                1.0
            } else {
                0.0
            },
            reward: 0.0,
        };
        memo.insert((state, remaining), value);
        return value;
    }
    if is_unsafe(solution, state, unsafe_fact) {
        let value = Evaluation {
            goal: 0.0,
            unsafe_reach: 1.0,
            reward: 0.0,
        };
        memo.insert((state, remaining), value);
        return value;
    }
    let Some(decision) = decisions.get(&(state, Some(remaining))) else {
        return Evaluation::default();
    };
    let mut value = Evaluation::default();
    for outcome in &decision.outcomes {
        let next = if outcome.goal {
            Evaluation {
                goal: 1.0,
                unsafe_reach: if is_unsafe(solution, outcome.next_state, unsafe_fact) {
                    1.0
                } else {
                    0.0
                },
                reward: 0.0,
            }
        } else {
            finite_eval(
                solution,
                outcome.next_state,
                remaining - 1,
                unsafe_fact,
                decisions,
                memo,
            )
        };
        value.weighted_add(
            outcome.probability,
            next,
            outcome.reward,
            solution.discount,
        );
    }
    memo.insert((state, remaining), value);
    value
}

fn stationary_eval(
    solution: &ProbabilisticSolution,
    unsafe_fact: Option<&str>,
    epsilon: f64,
    max_iterations: usize,
) -> (HashMap<usize, Evaluation>, bool) {
    let decisions = decision_map(solution);
    let mut values = solution
        .states
        .iter()
        .map(|state| {
            (
                state.id,
                Evaluation {
                    goal: if state.goal { 1.0 } else { 0.0 },
                    unsafe_reach: if is_unsafe(solution, state.id, unsafe_fact) {
                        1.0
                    } else {
                        0.0
                    },
                    reward: 0.0,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let mut converged = false;
    for _ in 0..max_iterations {
        let previous = values.clone();
        let mut delta = 0.0f64;
        for state in &solution.states {
            if state.goal || is_unsafe(solution, state.id, unsafe_fact) {
                continue;
            }
            let Some(decision) = decisions.get(&(state.id, None)) else {
                continue;
            };
            let mut next_value = Evaluation::default();
            for outcome in &decision.outcomes {
                let successor = previous
                    .get(&outcome.next_state)
                    .copied()
                    .unwrap_or_default();
                next_value.weighted_add(
                    outcome.probability,
                    successor,
                    outcome.reward,
                    solution.discount,
                );
            }
            if let Some(current) = values.get_mut(&state.id) {
                delta = delta
                    .max((current.goal - next_value.goal).abs())
                    .max((current.unsafe_reach - next_value.unsafe_reach).abs())
                    .max((current.reward - next_value.reward).abs());
                *current = next_value;
            }
        }
        if delta <= epsilon {
            converged = true;
            break;
        }
    }
    (values, converged)
}

fn initial_evaluation(
    solution: &ProbabilisticSolution,
    options: &ProbabilisticOptions,
    unsafe_fact: Option<&str>,
) -> (Evaluation, bool) {
    if let Some(horizon) = solution.horizon {
        let decisions = decision_map(solution);
        let mut memo = HashMap::new();
        let mut total = Evaluation::default();
        for initial in &solution.initial_distribution {
            let value = finite_eval(
                solution,
                initial.state,
                horizon,
                unsafe_fact,
                &decisions,
                &mut memo,
            );
            total.goal += initial.probability * value.goal;
            total.unsafe_reach += initial.probability * value.unsafe_reach;
            total.reward += initial.probability * value.reward;
        }
        (total, true)
    } else {
        let (values, converged) = stationary_eval(
            solution,
            unsafe_fact,
            options.epsilon,
            options.max_iterations,
        );
        let mut total = Evaluation::default();
        for initial in &solution.initial_distribution {
            let value = values.get(&initial.state).copied().unwrap_or_default();
            total.goal += initial.probability * value.goal;
            total.unsafe_reach += initial.probability * value.unsafe_reach;
            total.reward += initial.probability * value.reward;
        }
        (total, converged)
    }
}

fn closure_counterexamples(
    solution: &ProbabilisticSolution,
    unsafe_fact: Option<&str>,
) -> Vec<PolicyCounterexample> {
    let state_ids = solution
        .states
        .iter()
        .map(|state| state.id)
        .collect::<HashSet<_>>();
    let decisions = decision_map(solution);
    let mut errors = Vec::new();
    let mut queue = VecDeque::new();
    for initial in &solution.initial_distribution {
        queue.push_back((initial.state, solution.horizon));
    }
    let mut seen = HashSet::new();
    while let Some((state, remaining)) = queue.pop_front() {
        if !state_ids.contains(&state) {
            errors.push(PolicyCounterexample {
                kind: PolicyCounterexampleKind::UnknownState,
                state: Some(state),
                message: format!("policy traversal reached unknown state {state}"),
            });
            continue;
        }
        if !seen.insert((state, remaining)) {
            continue;
        }
        let terminal = solution
            .states
            .iter()
            .find(|candidate| candidate.id == state)
            .map(|candidate| candidate.goal)
            .unwrap_or(false)
            || is_unsafe(solution, state, unsafe_fact)
            || remaining == Some(0);
        if terminal {
            continue;
        }
        let Some(decision) = decisions.get(&(state, remaining)) else {
            errors.push(PolicyCounterexample {
                kind: PolicyCounterexampleKind::MissingDecision,
                state: Some(state),
                message: format!("no policy decision for state {state} at {remaining:?}"),
            });
            continue;
        };
        let mass: f64 = decision
            .outcomes
            .iter()
            .map(|outcome| outcome.probability)
            .sum();
        if (mass - 1.0).abs() > 1e-9 {
            errors.push(PolicyCounterexample {
                kind: PolicyCounterexampleKind::ProbabilityMass,
                state: Some(state),
                message: format!("outcome mass for state {state} is {mass}"),
            });
        }
        for outcome in &decision.outcomes {
            if !state_ids.contains(&outcome.next_state) {
                errors.push(PolicyCounterexample {
                    kind: PolicyCounterexampleKind::UnknownSuccessor,
                    state: Some(state),
                    message: format!(
                        "state {state} action {} references unknown successor {}",
                        decision.action, outcome.next_state
                    ),
                });
            }
            queue.push_back((
                outcome.next_state,
                remaining.map(|value| value.saturating_sub(1)),
            ));
        }
    }
    errors
}

fn bellman_residual(solution: &ProbabilisticSolution) -> f64 {
    let decisions = decision_map(solution);
    let mut max_residual = 0.0f64;
    for decision in &solution.policy {
        let next_remaining = decision.remaining.map(|value| value.saturating_sub(1));
        let q = match solution.objective {
            ProbabilisticObjective::MaximizeGoalProbability
            | ProbabilisticObjective::MinimizeGoalProbability => decision
                .outcomes
                .iter()
                .map(|outcome| {
                    let next = if outcome.goal {
                        1.0
                    } else {
                        decisions
                            .get(&(outcome.next_state, next_remaining))
                            .map(|next| next.value)
                            .unwrap_or(0.0)
                    };
                    outcome.probability * next
                })
                .sum::<f64>(),
            _ => decision
                .outcomes
                .iter()
                .map(|outcome| {
                    let next = decisions
                        .get(&(outcome.next_state, next_remaining))
                        .map(|next| next.value)
                        .unwrap_or(0.0);
                    outcome.probability * (outcome.reward + solution.discount * next)
                })
                .sum::<f64>(),
        };
        max_residual = max_residual.max((decision.value - q).abs());
    }
    max_residual
}

pub fn verify_policy(
    domain: &str,
    problem: &str,
    options: &ProbabilisticOptions,
    solution: &ProbabilisticSolution,
    constraints: &[RiskConstraint],
    unsafe_fact: Option<&str>,
) -> Result<PolicyVerificationReport, crate::PpddlError> {
    for constraint in constraints {
        constraint
            .validate()
            .map_err(crate::PpddlError::InvalidOptions)?;
    }
    let structural = validate_ppddl_policy(domain, problem, options, solution)?;
    let mut counterexamples = closure_counterexamples(solution, unsafe_fact);
    counterexamples.extend(
        structural
            .errors
            .iter()
            .map(|message| PolicyCounterexample {
                kind: PolicyCounterexampleKind::StructuralMismatch,
                state: None,
                message: message.clone(),
            }),
    );
    let closure = counterexamples.iter().all(|error| {
        !matches!(
            error.kind,
            PolicyCounterexampleKind::UnknownState
                | PolicyCounterexampleKind::MissingDecision
                | PolicyCounterexampleKind::ProbabilityMass
                | PolicyCounterexampleKind::UnknownSuccessor
        )
    });
    let (evaluation, converged) = initial_evaluation(solution, options, unsafe_fact);
    let goal = ValueInterval::exact(evaluation.goal);
    let unsafe_reach = ValueInterval::exact(evaluation.unsafe_reach);
    let reward = ValueInterval::exact(evaluation.reward);
    let verdicts = constraints
        .iter()
        .cloned()
        .map(|constraint| {
            let (satisfied, observed) = match constraint {
                RiskConstraint::MinimumGoalProbability(threshold) => {
                    (goal.lower + options.epsilon >= threshold, goal)
                }
                RiskConstraint::MaximumUnsafeReachability(threshold) => {
                    (
                        unsafe_reach.upper <= threshold + options.epsilon,
                        unsafe_reach,
                    )
                }
                RiskConstraint::MinimumExpectedReward(threshold) => {
                    (reward.lower + options.epsilon >= threshold, reward)
                }
                RiskConstraint::MaximumExpectedCost(threshold) => {
                    let cost = ValueInterval::exact(-reward.lower);
                    (cost.upper <= threshold + options.epsilon, cost)
                }
            };
            ConstraintVerdict {
                constraint,
                satisfied,
                observed,
            }
        })
        .collect::<Vec<_>>();
    let constraints_ok = verdicts.iter().all(|verdict| verdict.satisfied);
    if !constraints_ok {
        counterexamples.push(PolicyCounterexample {
            kind: PolicyCounterexampleKind::UnsafeThreshold,
            state: None,
            message: "one or more hard risk constraints failed".into(),
        });
    }
    let residual = bellman_residual(solution);
    let valid = structural.valid && closure && converged && constraints_ok;
    let (standing, reason) = if valid {
        (Standing::PartialAlive, Some(StandingReason::NoReplay))
    } else if !converged {
        (
            Standing::Blocked,
            Some(StandingReason::ConvergenceLimit),
        )
    } else if !constraints_ok {
        (
            Standing::Blocked,
            Some(StandingReason::ConstraintViolation),
        )
    } else if !closure {
        (Standing::Blocked, Some(StandingReason::PolicyNotClosed))
    } else {
        (
            Standing::Blocked,
            Some(StandingReason::VerifierDisagreement),
        )
    };
    Ok(PolicyVerificationReport {
        standing,
        valid,
        model_digest: canonical_digest(&(domain, problem, options, constraints)),
        policy_digest: canonical_digest(solution),
        closure,
        bellman_residual: residual,
        goal_probability: goal,
        unsafe_probability: unsafe_reach,
        expected_reward: reward,
        constraints: verdicts,
        counterexamples,
        reason,
    })
}
