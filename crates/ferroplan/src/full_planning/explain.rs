use crate::ProbabilisticSolution;

use super::{PolicyExplanation, ValueInterval};

pub fn explain_policy(
    solution: &ProbabilisticSolution,
    state: usize,
    remaining: Option<usize>,
) -> Result<PolicyExplanation, String> {
    let state_view = solution
        .states
        .iter()
        .find(|candidate| candidate.id == state)
        .ok_or_else(|| format!("unknown policy state {state}"))?;
    let decision = solution
        .policy
        .iter()
        .find(|candidate| candidate.state == state && candidate.remaining == remaining);
    let terminal = state_view.goal || remaining == Some(0);
    if !terminal && decision.is_none() {
        return Err(format!(
            "policy is not closed at state {state} with remaining={remaining:?}"
        ));
    }
    let goal_probability = if state_view.goal {
        ValueInterval::exact(1.0)
    } else {
        ValueInterval::exact(decision.map(|value| value.value).unwrap_or(0.0))
    };
    Ok(PolicyExplanation {
        state,
        remaining,
        terminal,
        chosen_action: decision.map(|value| {
            std::iter::once(value.action.clone())
                .chain(value.args.iter().cloned())
                .collect::<Vec<_>>()
                .join(" ")
        }),
        value: decision.map(|value| value.value),
        goal_probability,
        unsafe_probability: ValueInterval::exact(0.0),
        expected_reward: ValueInterval::exact(
            decision
                .map(|value| {
                    value
                        .outcomes
                        .iter()
                        .map(|outcome| outcome.probability * outcome.reward)
                        .sum()
                })
                .unwrap_or(0.0),
        ),
        successors: decision.map(|value| value.outcomes.clone()).unwrap_or_default(),
        notes: vec![
            "Explanation is a projection of the canonical selected policy; alternative actions are not fabricated.".into(),
        ],
    })
}
