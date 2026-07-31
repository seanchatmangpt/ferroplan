//! Proof-carrying full planning across preserved deterministic and probabilistic rails.
//!
//! The generated contracts in [`contracts`] are owned by the v26.8.1 ggen ontology.
//! The handwritten modules contain bounded runtime logic. Planning never actuates.

mod contracts;
mod explain;
mod receipt;
mod session;
mod verify;

use serde::{Deserialize, Serialize};

pub use contracts::*;
pub use explain::explain_policy;
pub use receipt::{
    bind_policy_receipt, canonical_digest, verify_policy_chain, verify_policy_receipt,
    PolicyReceipt,
};
pub use session::{PolicySession, PolicySessionError};
pub use verify::verify_policy;

const ENVELOPE_SCHEMA: &str = "urn:chatman:ferroplan-full-planning-envelope:v1";
const AUTHORITY_BOUNDARY: &str = "planner-selects; external-authority-executes";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "rail", rename_all = "kebab-case")]
pub enum FullPlanningRequest {
    Deterministic {
        domain: String,
        problem: String,
        #[serde(default)]
        options: crate::Options,
    },
    Probabilistic {
        domain: String,
        problem: String,
        #[serde(default)]
        options: crate::ProbabilisticOptions,
        #[serde(default)]
        constraints: Vec<RiskConstraint>,
        #[serde(default)]
        unsafe_fact: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "rail", content = "result", rename_all = "kebab-case")]
pub enum FullPlanningResult {
    Deterministic {
        solution: crate::Solution,
        envelope: FullPlanningEnvelope,
    },
    Probabilistic {
        solution: crate::ProbabilisticSolution,
        verification: PolicyVerificationReport,
        receipt: PolicyReceipt,
        envelope: FullPlanningEnvelope,
    },
}

pub fn plan(request: FullPlanningRequest) -> Result<FullPlanningResult, String> {
    match request {
        FullPlanningRequest::Deterministic {
            domain,
            problem,
            options,
        } => {
            let solution = crate::solve(&domain, &problem, &options)
                .map_err(|error| error.to_string())?;
            let envelope = FullPlanningEnvelope {
                schema: ENVELOPE_SCHEMA.into(),
                rail: PlanningRail::Deterministic,
                observation_frontier_digest: canonical_digest(&problem),
                model_digest: canonical_digest(&(&domain, &problem, &options)),
                normalized_task_digest: None,
                solver: format!("{:?}/{:?}", options.mode, options.search),
                artifact_digest: canonical_digest(&solution),
                verifier_digest: None,
                authority_boundary: AUTHORITY_BOUNDARY.into(),
                predecessor: None,
            };
            Ok(FullPlanningResult::Deterministic { solution, envelope })
        }
        FullPlanningRequest::Probabilistic {
            domain,
            problem,
            options,
            constraints,
            unsafe_fact,
        } => {
            let solution = crate::solve_ppddl(&domain, &problem, &options)
                .map_err(|error| error.to_string())?;
            let verification = verify_policy(
                &domain,
                &problem,
                &options,
                &solution,
                &constraints,
                unsafe_fact.as_deref(),
            )
            .map_err(|error| error.to_string())?;
            let receipt = bind_policy_receipt(
                &domain,
                &problem,
                &options,
                &constraints,
                &solution,
                &verification,
                None,
            )
            .map_err(|error| error.to_string())?;
            let envelope = FullPlanningEnvelope {
                schema: ENVELOPE_SCHEMA.into(),
                rail: PlanningRail::Probabilistic,
                observation_frontier_digest: canonical_digest(&solution.initial_distribution),
                model_digest: receipt.model_digest.clone(),
                normalized_task_digest: Some(receipt.normalized_mdp_digest.clone()),
                solver: format!("{:?}", receipt.solver),
                artifact_digest: receipt.policy_digest.clone(),
                verifier_digest: Some(receipt.verifier_digest.clone()),
                authority_boundary: AUTHORITY_BOUNDARY.into(),
                predecessor: receipt.predecessor.clone(),
            };
            Ok(FullPlanningResult::Probabilistic {
                solution,
                verification,
                receipt,
                envelope,
            })
        }
    }
}
