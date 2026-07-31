//! Public full-planning structures built around ggen-owned enum contracts.
//!
//! The files under `full_planning/generated/` are overwritten by `ggen sync run`.
//! This file owns the handwritten structures that compose those generated terms.

use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

include!("generated/planning_rail.rs");
include!("generated/policy_search.rs");
include!("generated/standing.rs");
include!("generated/risk_constraint.rs");

impl Default for PolicySearch {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StandingReason {
    NoReplay,
    ConvergenceLimit,
    DependencyMissing,
    MockedValidator,
    ConstraintViolation,
    PolicyNotClosed,
    ObservationIncomplete,
    OutcomeNotAdmitted,
    VerifierDisagreement,
    ResourceLimit,
}

impl RiskConstraint {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::MinimumGoalProbability(value)
            | Self::MaximumUnsafeReachability(value) => {
                if !value.is_finite() || !(0.0..=1.0).contains(value) {
                    return Err("probability thresholds must be finite and in [0,1]".into());
                }
            }
            Self::MinimumExpectedReward(value) | Self::MaximumExpectedCost(value) => {
                if !value.is_finite() {
                    return Err("reward/cost thresholds must be finite".into());
                }
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ValueInterval {
    pub lower: f64,
    pub upper: f64,
}

impl ValueInterval {
    pub fn exact(value: f64) -> Self {
        Self {
            lower: value,
            upper: value,
        }
    }

    pub fn width(self) -> f64 {
        self.upper - self.lower
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ConstraintVerdict {
    pub constraint: RiskConstraint,
    pub satisfied: bool,
    pub observed: ValueInterval,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum PolicyCounterexampleKind {
    UnknownState,
    MissingDecision,
    ProbabilityMass,
    UnknownSuccessor,
    BellmanResidual,
    UnsafeThreshold,
    RewardThreshold,
    StructuralMismatch,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PolicyCounterexample {
    pub kind: PolicyCounterexampleKind,
    pub state: Option<usize>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PolicyVerificationReport {
    pub standing: Standing,
    pub valid: bool,
    pub model_digest: String,
    pub policy_digest: String,
    pub closure: bool,
    pub bellman_residual: f64,
    pub goal_probability: ValueInterval,
    pub unsafe_probability: ValueInterval,
    pub expected_reward: ValueInterval,
    pub constraints: Vec<ConstraintVerdict>,
    pub counterexamples: Vec<PolicyCounterexample>,
    pub reason: Option<StandingReason>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicySessionPhase {
    Created,
    Ready,
    ActionSelected,
    AwaitingObservation,
    ObservationRequired,
    Drifted,
    Blocked,
    Closed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PolicySessionStatus {
    pub phase: PolicySessionPhase,
    pub current_state: Option<usize>,
    pub remaining: Option<usize>,
    pub selected_action: Option<String>,
    pub policy_valid: bool,
    pub predecessor_receipt: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PolicyExplanationOutcome {
    pub probability: f64,
    pub next_state: usize,
    pub reward: f64,
    pub goal: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PolicyExplanation {
    pub state: usize,
    pub remaining: Option<usize>,
    pub terminal: bool,
    pub chosen_action: Option<String>,
    pub value: Option<f64>,
    pub goal_probability: ValueInterval,
    pub unsafe_probability: ValueInterval,
    pub expected_reward: ValueInterval,
    pub successors: Vec<PolicyExplanationOutcome>,
    pub notes: Vec<String>,
}
