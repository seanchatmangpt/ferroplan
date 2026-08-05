//! Planning-type constitution for Ferroplan.
//!
//! This module does not claim that every planning paradigm is solved by the
//! native search kernel. It provides one exhaustive, serializable contract for
//! selecting the correct execution boundary without asking an LLM to reinterpret
//! the word "planning" for each request.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

/// The planning paradigms that may be requested from the Ferroplan ecosystem.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum PlanningType {
    Classical,
    CostOptimal,
    Numeric,
    Temporal,
    Preferences,
    Probabilistic,
    Fond,
    Conformant,
    Contingent,
    Hierarchical,
    PartialOrder,
    Workflow,
    FlowConstrained,
    ResolutionAdaptive,
    MultiAgent,
    RdfDerived,
    A2aDelegated,
    McpBound,
}

impl PlanningType {
    /// All admitted planning types in stable serialization order.
    pub const ALL: [Self; 18] = [
        Self::Classical,
        Self::CostOptimal,
        Self::Numeric,
        Self::Temporal,
        Self::Preferences,
        Self::Probabilistic,
        Self::Fond,
        Self::Conformant,
        Self::Contingent,
        Self::Hierarchical,
        Self::PartialOrder,
        Self::Workflow,
        Self::FlowConstrained,
        Self::ResolutionAdaptive,
        Self::MultiAgent,
        Self::RdfDerived,
        Self::A2aDelegated,
        Self::McpBound,
    ];

    /// Stable ASCII token for CLI, MCP, RDF, and receipt projections.
    pub const fn token(self) -> &'static str {
        match self {
            Self::Classical => "classical",
            Self::CostOptimal => "cost_optimal",
            Self::Numeric => "numeric",
            Self::Temporal => "temporal",
            Self::Preferences => "preferences",
            Self::Probabilistic => "probabilistic",
            Self::Fond => "fond",
            Self::Conformant => "conformant",
            Self::Contingent => "contingent",
            Self::Hierarchical => "hierarchical",
            Self::PartialOrder => "partial_order",
            Self::Workflow => "workflow",
            Self::FlowConstrained => "flow_constrained",
            Self::ResolutionAdaptive => "resolution_adaptive",
            Self::MultiAgent => "multi_agent",
            Self::RdfDerived => "rdf_derived",
            Self::A2aDelegated => "a2a_delegated",
            Self::McpBound => "mcp_bound",
        }
    }

    /// The lawful runtime boundary for this planning type at the current core.
    pub const fn rail(self) -> PlanningRail {
        match self {
            Self::Classical
            | Self::CostOptimal
            | Self::Numeric
            | Self::Temporal
            | Self::Preferences => PlanningRail::NativeDeterministic,
            Self::Probabilistic => PlanningRail::NativeProbabilistic,
            Self::PartialOrder | Self::Workflow => PlanningRail::Composed,
            Self::FlowConstrained | Self::ResolutionAdaptive => PlanningRail::PolicyOverlay,
            Self::RdfDerived => PlanningRail::GraphProjection,
            Self::A2aDelegated => PlanningRail::Delegation,
            Self::McpBound => PlanningRail::CapabilityBinding,
            Self::Fond
            | Self::Conformant
            | Self::Contingent
            | Self::Hierarchical
            | Self::MultiAgent => PlanningRail::ExternalPlanner,
        }
    }

    /// Capabilities that must be present before the type can be admitted.
    pub fn required_capabilities(self) -> BTreeSet<PlanningCapability> {
        use PlanningCapability as C;
        let capabilities: &[C] = match self {
            Self::Classical => &[C::DeterministicState, C::SequentialPlan],
            Self::CostOptimal => &[C::DeterministicState, C::ActionCosts, C::OptimalityProof],
            Self::Numeric => &[C::DeterministicState, C::NumericFluents],
            Self::Temporal => &[C::DeterministicState, C::DurativeActions, C::TemporalValidation],
            Self::Preferences => &[C::DeterministicState, C::SoftGoals],
            Self::Probabilistic => &[C::StochasticTransitions, C::Policy, C::PolicyValidation],
            Self::Fond => &[C::NondeterministicTransitions, C::Policy, C::StrongCyclicValidation],
            Self::Conformant => &[C::BeliefState, C::OpenLoopPlan],
            Self::Contingent => &[C::BeliefState, C::ObservationBranching, C::Policy],
            Self::Hierarchical => &[C::CompoundTasks, C::Methods, C::PrimitiveClosure],
            Self::PartialOrder => &[C::PartialOrder, C::PrimitiveClosure],
            Self::Workflow => &[C::PartialOrder, C::ReceiptJoin, C::PrimitiveClosure],
            Self::FlowConstrained => &[C::QueueState, C::WipBounds, C::Policy],
            Self::ResolutionAdaptive => &[C::ResolutionObligations, C::PrimitiveClosure],
            Self::MultiAgent => &[C::AgentCapabilities, C::CoordinationPolicy],
            Self::RdfDerived => &[C::AdmittedGraph, C::DeterministicProjection],
            Self::A2aDelegated => &[C::AgentCapabilities, C::DelegationEnvelope],
            Self::McpBound => &[C::ToolCapabilities, C::AuthorityBinding, C::PrimitiveClosure],
        };
        capabilities.iter().copied().collect()
    }
}

impl fmt::Display for PlanningType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.token())
    }
}

/// Runtime boundary selected for a planning request.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum PlanningRail {
    NativeDeterministic,
    NativeProbabilistic,
    Composed,
    PolicyOverlay,
    GraphProjection,
    Delegation,
    CapabilityBinding,
    ExternalPlanner,
}

/// Atomic semantic requirements used to prove that a planning request is closed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum PlanningCapability {
    DeterministicState,
    SequentialPlan,
    ActionCosts,
    OptimalityProof,
    NumericFluents,
    DurativeActions,
    TemporalValidation,
    SoftGoals,
    StochasticTransitions,
    NondeterministicTransitions,
    Policy,
    PolicyValidation,
    StrongCyclicValidation,
    BeliefState,
    OpenLoopPlan,
    ObservationBranching,
    CompoundTasks,
    Methods,
    PartialOrder,
    ReceiptJoin,
    QueueState,
    WipBounds,
    ResolutionObligations,
    AgentCapabilities,
    CoordinationPolicy,
    AdmittedGraph,
    DeterministicProjection,
    DelegationEnvelope,
    ToolCapabilities,
    AuthorityBinding,
    PrimitiveClosure,
}

/// A bounded request to route through the appropriate planning rail.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PlanningRequest {
    pub subject: String,
    pub planning_type: PlanningType,
    #[serde(default)]
    pub available_capabilities: BTreeSet<PlanningCapability>,
    #[serde(default)]
    pub authority_bound: bool,
    #[serde(default)]
    pub verifier_bound: bool,
    #[serde(default)]
    pub receipt_bound: bool,
}

/// Successful routing result. This is a selection result, never an actuation grant.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PlanningRoute {
    pub subject: String,
    pub planning_type: PlanningType,
    pub rail: PlanningRail,
    pub required_capabilities: BTreeSet<PlanningCapability>,
}

/// Typed refusal when the request is not sufficiently resolved for routing.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "code", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlanningRouteError {
    EmptySubject,
    MissingCapabilities {
        missing: BTreeSet<PlanningCapability>,
    },
    AuthorityUnbound,
    VerifierUnbound,
    ReceiptUnbound,
}

impl fmt::Display for PlanningRouteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySubject => formatter.write_str("REFUSED:EMPTY_SUBJECT"),
            Self::MissingCapabilities { missing } => {
                write!(formatter, "REFUSED:MISSING_PLANNING_CAPABILITIES:{missing:?}")
            }
            Self::AuthorityUnbound => formatter.write_str("REFUSED:AUTHORITY_UNBOUND"),
            Self::VerifierUnbound => formatter.write_str("REFUSED:VERIFIER_UNBOUND"),
            Self::ReceiptUnbound => formatter.write_str("REFUSED:RECEIPT_UNBOUND"),
        }
    }
}

impl std::error::Error for PlanningRouteError {}

/// Route a request after proving semantic, authority, verification, and receipt closure.
///
/// This function performs no planning and has no actuation authority.
pub fn route_planning_request(
    request: &PlanningRequest,
) -> Result<PlanningRoute, PlanningRouteError> {
    if request.subject.trim().is_empty() {
        return Err(PlanningRouteError::EmptySubject);
    }

    let required = request.planning_type.required_capabilities();
    let missing = required
        .difference(&request.available_capabilities)
        .copied()
        .collect::<BTreeSet<_>>();
    if !missing.is_empty() {
        return Err(PlanningRouteError::MissingCapabilities { missing });
    }
    if !request.authority_bound {
        return Err(PlanningRouteError::AuthorityUnbound);
    }
    if !request.verifier_bound {
        return Err(PlanningRouteError::VerifierUnbound);
    }
    if !request.receipt_bound {
        return Err(PlanningRouteError::ReceiptUnbound);
    }

    Ok(PlanningRoute {
        subject: request.subject.clone(),
        planning_type: request.planning_type,
        rail: request.planning_type.rail(),
        required_capabilities: required,
    })
}
