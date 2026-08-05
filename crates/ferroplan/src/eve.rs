//! Eve: the relational entry point into a Genesis-defined planning world.
//!
//! `ferroplan` already provides deterministic PDDL planning and bounded PPDDL
//! policy synthesis.  This module supplies the missing outer contract:
//!
//! ```text
//! human purpose -> Eve -> Genesis projection -> HDDL decomposition
//!               -> PPDDL policy (when required) -> ggen manufacture
//!               -> MCP+ capability handoff -> Truex consequence lifecycle
//! ```
//!
//! Eve does not improvise a plan or actuate a tool.  Eve deterministically
//! compiles a human purpose and a pre-existing lawful world into a typed handoff
//! for the formal layers that own decomposition, uncertainty, manufacture,
//! actuation, observation, conformance, receipt, and replay.

use std::hash::Hasher;

use serde::{Deserialize, Serialize};

use crate::hash::FxHasher;

/// A local Eve reflex may have at most eight primary activators.
///
/// A ninth activator is not silently flattened into a larger prompt.  It
/// requires the closure to be split into smaller lawful fields.
pub const MAX_PRIMARY_ACTIVATORS: usize = 8;

/// Human purpose before it is grounded in the Genesis world.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HumanPurpose {
    /// What the person is trying to accomplish, in their own language.
    pub statement: String,
    /// The consequence that would make the purpose complete.
    pub desired_consequence: String,
    /// Optional identity or role on whose authority the purpose is expressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    /// Bounded primary conditions that shape ingress.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activators: Vec<Activator>,
}

/// One bounded condition presented to Eve.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Activator {
    pub name: String,
    pub value: String,
}

/// The created world Eve makes relationally accessible.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenesisWorld {
    /// Canonical RDF/ontology source for the world.
    pub ontology_rdf: String,
    /// SPARQL CONSTRUCT query selecting the smallest relevant lawful subgraph.
    pub construct_query: String,
    /// Hierarchical task surface used to decompose the grounded purpose.
    pub hddl: HddlSurface,
    /// Optional bounded uncertainty surface.  Its presence selects PPDDL policy
    /// synthesis; its absence selects deterministic planning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ppddl: Option<PpddlSurface>,
}

/// HDDL source and the compound task that represents the grounded purpose.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HddlSurface {
    pub domain: String,
    pub problem: String,
    pub root_task: String,
}

/// PPDDL source used when the lawful world contains explicit uncertainty.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PpddlSurface {
    pub domain: String,
    pub problem: String,
}

/// Requested ggen projection.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ManufactureTarget {
    /// Stable target identity, such as `deploy-service.part`.
    pub name: String,
    /// ggen template or projection family.
    pub template: String,
    /// Artifact kind to manufacture, such as `.part.wasm`, manifest, adapter,
    /// workflow, test, or deployment material.
    pub artifact_kind: String,
    /// Destination understood by the ggen adapter.
    pub output: String,
}

/// MCP+ capability boundary for the manufactured result.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CapabilityTarget {
    /// Name exposed by MCP+ after manufacture and admission.
    pub capability: String,
    /// Route identity that binds the capability to process geometry.
    pub route: String,
    /// Authority scopes required before BRCE may actuate it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authority_scopes: Vec<String>,
}

/// Input to Eve's deterministic relational compiler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EveRequest {
    pub purpose: HumanPurpose,
    pub genesis: GenesisWorld,
    pub manufacture: ManufactureTarget,
    pub capability: CapabilityTarget,
}

/// Whether the grounded purpose enters deterministic planning or bounded
/// probabilistic policy synthesis.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PlanningRegime {
    Deterministic,
    Probabilistic,
}

/// One ordered lifecycle boundary emitted by Eve.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EveStage {
    /// Relate human language to the created world without granting authority.
    GroundHumanPurpose,
    /// Project the relevant lawful RDF subgraph through SPARQL CONSTRUCT.
    ProjectGenesis,
    /// Decompose the grounded goal through HDDL.
    DecomposeHddl,
    /// Synthesize a bounded PPDDL policy when uncertainty is explicit.
    GovernUncertaintyPpddl,
    /// Manufacture the required artifact through ggen.
    ManufactureGgen,
    /// Expose the admitted capability through MCP+.
    ExposeMcpPlus,
    /// Actuate only through the exclusive brokered DO boundary.
    ActuateBrce,
    /// Capture object-centric boundary evidence.
    ObserveOcel2,
    /// Derive conformance against the expected process geometry.
    ConformTruexKernel,
    /// Emit an admission or refusal receipt.
    AdmitReceipt,
    /// Replay the consequence and carry standing into the next closure.
    ReplayTruex,
}

/// Human purpose after Eve binds it to the selected Genesis world.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GroundedGoal {
    pub statement: String,
    pub desired_consequence: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub root_task: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activators: Vec<Activator>,
}

/// Exact Genesis projection requested by Eve.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenesisProjection {
    pub ontology_rdf: String,
    pub construct_query: String,
}

/// Exact hierarchical decomposition request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HddlDecompositionRequest {
    pub domain: String,
    pub problem: String,
    pub root_task: String,
}

/// Exact probabilistic policy request, present only for uncertain worlds.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PpddlPolicyRequest {
    pub domain: String,
    pub problem: String,
}

/// ggen manufacturing contract emitted by Eve.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GgenManufacturingRequest {
    pub target: ManufactureTarget,
    /// The closure whose projection is being manufactured.
    pub closure_id: String,
    /// ggen may construct a candidate; it may not self-admit execution.
    pub candidate_only: bool,
}

/// MCP+ handoff.  The capability remains non-authoritative until the listed
/// obligations close through Truex.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct McpPlusHandoff {
    pub target: CapabilityTarget,
    pub closure_id: String,
    pub ambient_authority: bool,
    pub brce_required: bool,
    pub receipt_obligations: Vec<String>,
}

/// Downstream consequence obligations owned by Truex rather than Eve.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TruexContinuation {
    pub expected_process_geometry: String,
    pub observed_path_format: String,
    pub conformance_engine: String,
    pub terminal_authority: String,
    pub replay_required: bool,
}

/// Complete, machine-readable handoff from Eve to the formal execution stack.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EveHandoff {
    pub protocol: String,
    /// Deterministic identity of the complete request.  This is an identity
    /// key, not a cryptographic receipt and not proof of execution.
    pub closure_id: String,
    pub planning_regime: PlanningRegime,
    pub stages: Vec<EveStage>,
    pub goal: GroundedGoal,
    pub genesis: GenesisProjection,
    pub hddl: HddlDecompositionRequest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ppddl: Option<PpddlPolicyRequest>,
    pub ggen: GgenManufacturingRequest,
    pub mcp_plus: McpPlusHandoff,
    pub truex: TruexContinuation,
}

/// A deterministic split directive produced by `Need9 => Split`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SplitDirective {
    pub provided: usize,
    pub maximum: usize,
    pub groups: Vec<Vec<Activator>>,
}

/// Eve refuses malformed or unbounded ingress instead of manufacturing a
/// plausible-looking handoff.
#[derive(thiserror::Error, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum EveError {
    #[error("missing required field `{field}`")]
    Missing { field: String },
    #[error("Need9 => Split: {directive:?}")]
    SplitRequired { directive: SplitDirective },
}

/// Stateless deterministic Eve compiler.
#[derive(Clone, Copy, Debug, Default)]
pub struct Eve;

impl Eve {
    /// Compile human purpose into a typed handoff across Genesis, HDDL, PPDDL,
    /// ggen, MCP+, and the downstream Truex consequence lifecycle.
    ///
    /// This method performs no actuation.  It creates the lawful relational
    /// contract that downstream adapters execute and receipt.
    pub fn enter(request: EveRequest) -> Result<EveHandoff, EveError> {
        validate_required("purpose.statement", &request.purpose.statement)?;
        validate_required(
            "purpose.desired_consequence",
            &request.purpose.desired_consequence,
        )?;
        validate_required("genesis.ontology_rdf", &request.genesis.ontology_rdf)?;
        validate_required(
            "genesis.construct_query",
            &request.genesis.construct_query,
        )?;
        validate_required("genesis.hddl.domain", &request.genesis.hddl.domain)?;
        validate_required("genesis.hddl.problem", &request.genesis.hddl.problem)?;
        validate_required("genesis.hddl.root_task", &request.genesis.hddl.root_task)?;
        validate_required("manufacture.name", &request.manufacture.name)?;
        validate_required("manufacture.template", &request.manufacture.template)?;
        validate_required(
            "manufacture.artifact_kind",
            &request.manufacture.artifact_kind,
        )?;
        validate_required("manufacture.output", &request.manufacture.output)?;
        validate_required("capability.capability", &request.capability.capability)?;
        validate_required("capability.route", &request.capability.route)?;

        if let Some(ppddl) = &request.genesis.ppddl {
            validate_required("genesis.ppddl.domain", &ppddl.domain)?;
            validate_required("genesis.ppddl.problem", &ppddl.problem)?;
        }

        if request.purpose.activators.len() > MAX_PRIMARY_ACTIVATORS {
            return Err(EveError::SplitRequired {
                directive: SplitDirective {
                    provided: request.purpose.activators.len(),
                    maximum: MAX_PRIMARY_ACTIVATORS,
                    groups: request
                        .purpose
                        .activators
                        .chunks(MAX_PRIMARY_ACTIVATORS)
                        .map(|chunk| chunk.to_vec())
                        .collect(),
                },
            });
        }

        let closure_id = closure_id(&request);
        let planning_regime = if request.genesis.ppddl.is_some() {
            PlanningRegime::Probabilistic
        } else {
            PlanningRegime::Deterministic
        };

        let mut stages = vec![
            EveStage::GroundHumanPurpose,
            EveStage::ProjectGenesis,
            EveStage::DecomposeHddl,
        ];
        if planning_regime == PlanningRegime::Probabilistic {
            stages.push(EveStage::GovernUncertaintyPpddl);
        }
        stages.extend([
            EveStage::ManufactureGgen,
            EveStage::ExposeMcpPlus,
            EveStage::ActuateBrce,
            EveStage::ObserveOcel2,
            EveStage::ConformTruexKernel,
            EveStage::AdmitReceipt,
            EveStage::ReplayTruex,
        ]);

        Ok(EveHandoff {
            protocol: "ferroplan.eve-genesis.v1".to_string(),
            closure_id: closure_id.clone(),
            planning_regime,
            stages,
            goal: GroundedGoal {
                statement: request.purpose.statement,
                desired_consequence: request.purpose.desired_consequence,
                actor: request.purpose.actor,
                root_task: request.genesis.hddl.root_task.clone(),
                activators: request.purpose.activators,
            },
            genesis: GenesisProjection {
                ontology_rdf: request.genesis.ontology_rdf,
                construct_query: request.genesis.construct_query,
            },
            hddl: HddlDecompositionRequest {
                domain: request.genesis.hddl.domain,
                problem: request.genesis.hddl.problem,
                root_task: request.genesis.hddl.root_task,
            },
            ppddl: request
                .genesis
                .ppddl
                .map(|surface| PpddlPolicyRequest {
                    domain: surface.domain,
                    problem: surface.problem,
                }),
            ggen: GgenManufacturingRequest {
                target: request.manufacture,
                closure_id: closure_id.clone(),
                candidate_only: true,
            },
            mcp_plus: McpPlusHandoff {
                target: request.capability,
                closure_id,
                ambient_authority: false,
                brce_required: true,
                receipt_obligations: vec![
                    "artifact-materialized".to_string(),
                    "boundary-evidence".to_string(),
                    "ocel2-observed-path".to_string(),
                    "powl-conformance".to_string(),
                    "receipt-admission-or-refusal".to_string(),
                    "replay".to_string(),
                ],
            },
            truex: TruexContinuation {
                expected_process_geometry: "POWL-v2".to_string(),
                observed_path_format: "OCEL-2.0".to_string(),
                conformance_engine: "Truex-Kernel/wasm4pm".to_string(),
                terminal_authority: "receipt-admission-or-refusal".to_string(),
                replay_required: true,
            },
        })
    }
}

fn validate_required(field: &str, value: &str) -> Result<(), EveError> {
    if value.trim().is_empty() {
        Err(EveError::Missing {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

fn closure_id(request: &EveRequest) -> String {
    let mut hasher = FxHasher::default();
    hash_field(&mut hasher, &request.purpose.statement);
    hash_field(&mut hasher, &request.purpose.desired_consequence);
    hash_field(
        &mut hasher,
        request.purpose.actor.as_deref().unwrap_or(""),
    );
    for activator in &request.purpose.activators {
        hash_field(&mut hasher, &activator.name);
        hash_field(&mut hasher, &activator.value);
    }
    hash_field(&mut hasher, &request.genesis.ontology_rdf);
    hash_field(&mut hasher, &request.genesis.construct_query);
    hash_field(&mut hasher, &request.genesis.hddl.domain);
    hash_field(&mut hasher, &request.genesis.hddl.problem);
    hash_field(&mut hasher, &request.genesis.hddl.root_task);
    if let Some(ppddl) = &request.genesis.ppddl {
        hasher.write_u8(1);
        hash_field(&mut hasher, &ppddl.domain);
        hash_field(&mut hasher, &ppddl.problem);
    } else {
        hasher.write_u8(0);
    }
    hash_field(&mut hasher, &request.manufacture.name);
    hash_field(&mut hasher, &request.manufacture.template);
    hash_field(&mut hasher, &request.manufacture.artifact_kind);
    hash_field(&mut hasher, &request.manufacture.output);
    hash_field(&mut hasher, &request.capability.capability);
    hash_field(&mut hasher, &request.capability.route);
    for scope in &request.capability.authority_scopes {
        hash_field(&mut hasher, scope);
    }
    format!("eve:{:016x}", hasher.finish())
}

fn hash_field(hasher: &mut FxHasher, value: &str) {
    hasher.write_usize(value.len());
    hasher.write(value.as_bytes());
}
