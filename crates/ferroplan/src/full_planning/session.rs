use crate::{PolicyDecision, ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution};

use super::{
    verify_policy, PolicySessionPhase, PolicySessionStatus, PolicyVerificationReport,
    RiskConstraint,
};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum PolicySessionError {
    #[error("policy synthesis failed: {0}")]
    Planning(String),
    #[error("session requires an explicit observation because the initial state is ambiguous")]
    ObservationIncomplete,
    #[error("state {0} is not part of the admitted policy model")]
    UnknownState(usize),
    #[error("state {state} is not an admitted outcome of action {action}")]
    OutcomeNotAdmitted { state: usize, action: String },
    #[error("policy has no decision for state {state} with remaining={remaining:?}")]
    PolicyNotClosed {
        state: usize,
        remaining: Option<usize>,
    },
    #[error("operation is invalid while session is in phase {0:?}")]
    InvalidPhase(PolicySessionPhase),
    #[error("cannot rewrite PPDDL problem section {0}")]
    Rewrite(String),
}

#[derive(Clone)]
pub struct PolicySession {
    domain: String,
    problem: String,
    options: ProbabilisticOptions,
    constraints: Vec<RiskConstraint>,
    unsafe_fact: Option<String>,
    solution: ProbabilisticSolution,
    verification: PolicyVerificationReport,
    current_state: Option<usize>,
    remaining: Option<usize>,
    selected: Option<PolicyDecision>,
    phase: PolicySessionPhase,
    predecessor_receipt: Option<String>,
}

impl PolicySession {
    pub fn new(
        domain: impl Into<String>,
        problem: impl Into<String>,
        options: ProbabilisticOptions,
        constraints: Vec<RiskConstraint>,
        unsafe_fact: Option<String>,
    ) -> Result<Self, PolicySessionError> {
        let domain = domain.into();
        let problem = problem.into();
        let solution = crate::solve_ppddl(&domain, &problem, &options)
            .map_err(|error| PolicySessionError::Planning(error.to_string()))?;
        let verification = verify_policy(
            &domain,
            &problem,
            &options,
            &solution,
            &constraints,
            unsafe_fact.as_deref(),
        )
        .map_err(|error| PolicySessionError::Planning(error.to_string()))?;
        let current_state = match solution.initial_distribution.as_slice() {
            [initial] if (initial.probability - 1.0).abs() <= 1e-12 => Some(initial.state),
            _ => None,
        };
        let phase = if current_state.is_some() {
            PolicySessionPhase::Ready
        } else {
            PolicySessionPhase::ObservationRequired
        };
        Ok(Self {
            domain,
            problem,
            options,
            constraints,
            unsafe_fact,
            remaining: solution.horizon,
            solution,
            verification,
            current_state,
            selected: None,
            phase,
            predecessor_receipt: None,
        })
    }

    pub fn solution(&self) -> &ProbabilisticSolution {
        &self.solution
    }

    pub fn verification(&self) -> &PolicyVerificationReport {
        &self.verification
    }

    pub fn status(&self) -> PolicySessionStatus {
        PolicySessionStatus {
            phase: self.phase,
            current_state: self.current_state,
            remaining: self.remaining,
            selected_action: self.selected.as_ref().map(display_action),
            policy_valid: self.verification.valid && self.policy_still_valid(),
            predecessor_receipt: self.predecessor_receipt.clone(),
        }
    }

    pub fn decide(&mut self) -> Result<Option<PolicyDecision>, PolicySessionError> {
        if !matches!(self.phase, PolicySessionPhase::Ready) {
            return Err(PolicySessionError::InvalidPhase(self.phase));
        }
        let state = self
            .current_state
            .ok_or(PolicySessionError::ObservationIncomplete)?;
        let terminal = self
            .solution
            .states
            .iter()
            .find(|candidate| candidate.id == state)
            .map(|candidate| candidate.goal)
            .ok_or(PolicySessionError::UnknownState(state))?
            || self.remaining == Some(0);
        if terminal {
            return Ok(None);
        }
        let decision = self
            .solution
            .policy
            .iter()
            .find(|candidate| candidate.state == state && candidate.remaining == self.remaining)
            .cloned()
            .ok_or(PolicySessionError::PolicyNotClosed {
                state,
                remaining: self.remaining,
            })?;
        self.selected = Some(decision.clone());
        self.phase = PolicySessionPhase::ActionSelected;
        Ok(Some(decision))
    }

    pub fn mark_awaiting_observation(&mut self) -> Result<(), PolicySessionError> {
        if !matches!(self.phase, PolicySessionPhase::ActionSelected) {
            return Err(PolicySessionError::InvalidPhase(self.phase));
        }
        self.phase = PolicySessionPhase::AwaitingObservation;
        Ok(())
    }

    pub fn advance(&mut self, next_state: usize) -> Result<(), PolicySessionError> {
        if !matches!(
            self.phase,
            PolicySessionPhase::ActionSelected | PolicySessionPhase::AwaitingObservation
        ) {
            return Err(PolicySessionError::InvalidPhase(self.phase));
        }
        let decision = self
            .selected
            .clone()
            .ok_or(PolicySessionError::InvalidPhase(self.phase))?;
        if !decision
            .outcomes
            .iter()
            .any(|outcome| outcome.next_state == next_state)
        {
            self.phase = PolicySessionPhase::Drifted;
            return Err(PolicySessionError::OutcomeNotAdmitted {
                state: next_state,
                action: display_action(&decision),
            });
        }
        self.current_state = Some(next_state);
        self.remaining = self.remaining.map(|value| value.saturating_sub(1));
        self.selected = None;
        self.phase = PolicySessionPhase::Ready;
        Ok(())
    }

    pub fn observe(&mut self, state: usize) -> Result<(), PolicySessionError> {
        if !self.solution.states.iter().any(|candidate| candidate.id == state) {
            self.phase = PolicySessionPhase::Drifted;
            return Err(PolicySessionError::UnknownState(state));
        }
        if matches!(
            self.phase,
            PolicySessionPhase::ActionSelected | PolicySessionPhase::AwaitingObservation
        ) {
            return self.advance(state);
        }
        self.current_state = Some(state);
        self.selected = None;
        self.phase = PolicySessionPhase::Ready;
        Ok(())
    }

    pub fn set_goal(&mut self, goal_expression: &str) -> Result<(), PolicySessionError> {
        self.problem = replace_section(&self.problem, ":goal", goal_expression)?;
        self.replan()
    }

    pub fn set_objective(
        &mut self,
        objective: ProbabilisticObjective,
    ) -> Result<(), PolicySessionError> {
        self.options.objective = objective;
        self.replan()
    }

    pub fn policy_still_valid(&self) -> bool {
        let Some(state) = self.current_state else {
            return false;
        };
        self.solution
            .states
            .iter()
            .find(|candidate| candidate.id == state)
            .map(|candidate| candidate.goal)
            .unwrap_or(false)
            || self.remaining == Some(0)
            || self
                .solution
                .policy
                .iter()
                .any(|decision| decision.state == state && decision.remaining == self.remaining)
    }

    pub fn replan(&mut self) -> Result<(), PolicySessionError> {
        let problem = match self.current_state {
            Some(state_id) => {
                let state = self
                    .solution
                    .states
                    .iter()
                    .find(|candidate| candidate.id == state_id)
                    .ok_or(PolicySessionError::UnknownState(state_id))?;
                let mut init = state.facts.join(" ");
                for (name, value) in &state.fluents {
                    init.push_str(&format!(" (= {name} {value})"));
                }
                replace_section(&self.problem, ":init", &init)?
            }
            None => self.problem.clone(),
        };
        let solution = crate::solve_ppddl(&self.domain, &problem, &self.options)
            .map_err(|error| PolicySessionError::Planning(error.to_string()))?;
        let verification = verify_policy(
            &self.domain,
            &problem,
            &self.options,
            &solution,
            &self.constraints,
            self.unsafe_fact.as_deref(),
        )
        .map_err(|error| PolicySessionError::Planning(error.to_string()))?;
        let current_state = match solution.initial_distribution.as_slice() {
            [initial] if (initial.probability - 1.0).abs() <= 1e-12 => Some(initial.state),
            _ => None,
        };
        self.problem = problem;
        self.solution = solution;
        self.verification = verification;
        self.current_state = current_state;
        self.remaining = self.solution.horizon;
        self.selected = None;
        self.phase = if self.current_state.is_some() {
            PolicySessionPhase::Ready
        } else {
            PolicySessionPhase::ObservationRequired
        };
        Ok(())
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }

    pub fn set_predecessor_receipt(&mut self, digest: Option<String>) {
        self.predecessor_receipt = digest;
    }

    pub fn close(&mut self) {
        self.selected = None;
        self.phase = PolicySessionPhase::Closed;
    }
}

fn display_action(decision: &PolicyDecision) -> String {
    std::iter::once(decision.action.clone())
        .chain(decision.args.iter().cloned())
        .collect::<Vec<_>>()
        .join(" ")
}

fn replace_section(
    problem: &str,
    section: &str,
    replacement_body: &str,
) -> Result<String, PolicySessionError> {
    let lower = problem.to_ascii_lowercase();
    let needle = format!("({section}");
    let start = lower
        .find(&needle)
        .ok_or_else(|| PolicySessionError::Rewrite(section.into()))?;
    let bytes = problem.as_bytes();
    let mut depth = 0i32;
    let mut end = None;
    for (offset, byte) in bytes[start..].iter().enumerate() {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + offset + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    let end = end.ok_or_else(|| PolicySessionError::Rewrite(section.into()))?;
    let replacement = format!("({section} {replacement_body})");
    Ok(format!("{}{}{}", &problem[..start], replacement, &problem[end..]))
}
