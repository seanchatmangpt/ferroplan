//! PPDDL and policy-session tools for the merged Ferroplan MCP server.

use std::collections::BTreeMap;
use std::sync::Arc;

use ferroplan::{
    bind_policy_receipt, explain_policy, parse_ppddl, simulate_ppddl, verify_policy,
    verify_policy_chain, PolicyReceipt, PolicySession, PolicyVerificationReport,
    ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution, RiskConstraint,
};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, ErrorData as McpError};
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::Mutex as AsyncMutex;

use crate::result::to_result;
use crate::Ferroplan;

include!(concat!(env!("OUT_DIR"), "/full_planning_ontology.rs"));

pub(crate) const RESOURCE_TOOLS: &[&str] = &[
    "parse_ppddl",
    "solve_ppddl",
    "validate_ppddl_policy",
    "simulate_ppddl",
    "explain_ppddl_policy",
    "policy_session_open",
    "policy_session_observe",
    "policy_session_decide",
    "policy_session_advance",
    "policy_session_set_goal",
    "policy_session_status",
    "policy_session_close",
    "bind_policy_receipt",
    "verify_policy_chain",
];

pub(crate) fn ontology_comment(name: &str) -> Option<&'static str> {
    Some(match name {
        "parse_ppddl" => PARSE_PPDDL_ONTOLOGY,
        "solve_ppddl" => SOLVE_PPDDL_ONTOLOGY,
        "validate_ppddl_policy" => VALIDATE_PPDDL_ONTOLOGY,
        "simulate_ppddl" => SIMULATE_PPDDL_ONTOLOGY,
        "explain_ppddl_policy" => EXPLAIN_PPDDL_ONTOLOGY,
        "policy_session_open" => POLICY_OPEN_ONTOLOGY,
        "policy_session_observe" => POLICY_OBSERVE_ONTOLOGY,
        "policy_session_decide" => POLICY_DECIDE_ONTOLOGY,
        "policy_session_advance" => POLICY_ADVANCE_ONTOLOGY,
        "policy_session_set_goal" => POLICY_SET_GOAL_ONTOLOGY,
        "policy_session_status" => POLICY_STATUS_ONTOLOGY,
        "policy_session_close" => POLICY_CLOSE_ONTOLOGY,
        "bind_policy_receipt" => BIND_POLICY_ONTOLOGY,
        "verify_policy_chain" => VERIFY_POLICY_CHAIN_ONTOLOGY,
        _ => return None,
    })
}

#[derive(Clone, Default)]
pub(crate) struct PolicySessionState {
    sessions: Arc<AsyncMutex<BTreeMap<String, Arc<AsyncMutex<PolicySession>>>>>,
}

impl PolicySessionState {
    async fn get(&self, id: &str) -> Result<Arc<AsyncMutex<PolicySession>>, String> {
        self.sessions
            .lock()
            .await
            .get(id)
            .cloned()
            .ok_or_else(|| format!("unknown policy session `{id}`"))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PpddlModelInput {
    domain: String,
    problem: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PpddlSolveInput {
    domain: String,
    problem: String,
    #[serde(default)]
    options: Option<ProbabilisticOptions>,
    #[serde(default)]
    constraints: Vec<RiskConstraint>,
    #[serde(default)]
    unsafe_fact: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PpddlPolicyInput {
    domain: String,
    problem: String,
    #[serde(default)]
    options: Option<ProbabilisticOptions>,
    solution: Value,
    #[serde(default)]
    constraints: Vec<RiskConstraint>,
    #[serde(default)]
    unsafe_fact: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PpddlSimulationInput {
    domain: String,
    problem: String,
    #[serde(default)]
    options: Option<ProbabilisticOptions>,
    episodes: usize,
    seed: u64,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ExplainInput {
    solution: Value,
    state: usize,
    #[serde(default)]
    remaining: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PolicyOpenInput {
    session_id: String,
    domain: String,
    problem: String,
    #[serde(default)]
    options: Option<ProbabilisticOptions>,
    #[serde(default)]
    constraints: Vec<RiskConstraint>,
    #[serde(default)]
    unsafe_fact: Option<String>,
    #[serde(default)]
    replace: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PolicySessionIdInput {
    session_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PolicyObservationInput {
    session_id: String,
    state: usize,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PolicyGoalInput {
    session_id: String,
    goal: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct BindPolicyInput {
    domain: String,
    problem: String,
    #[serde(default)]
    options: Option<ProbabilisticOptions>,
    #[serde(default)]
    constraints: Vec<RiskConstraint>,
    solution: Value,
    verifier: Value,
    #[serde(default)]
    predecessor: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct VerifyChainInput {
    receipts: Vec<PolicyReceipt>,
}

fn decode<T: serde::de::DeserializeOwned>(value: Value, label: &str) -> Result<T, String> {
    serde_json::from_value(value).map_err(|error| format!("invalid {label}: {error}"))
}

fn value<T: serde::Serialize>(value: &T) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|error| error.to_string())
}

#[tool_router(router = full_planning_router, vis = "pub")]
impl Ferroplan {
    #[tool(description = "Parse and normalize a PPDDL domain/problem without solving.")]
    fn parse_ppddl(
        &self,
        Parameters(input): Parameters<PpddlModelInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(value(&parse_ppddl(&input.domain, &input.problem)))
    }

    #[tool(description = "Synthesize and independently verify a bounded PPDDL policy.")]
    fn solve_ppddl(
        &self,
        Parameters(input): Parameters<PpddlSolveInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_solve_ppddl(input))
    }

    #[tool(description = "Recompile and verify a supplied PPDDL policy and hard constraints.")]
    fn validate_ppddl_policy(
        &self,
        Parameters(input): Parameters<PpddlPolicyInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_validate_ppddl_policy(input))
    }

    #[tool(description = "Run deterministic-seed simulation against the synthesized PPDDL policy.")]
    fn simulate_ppddl(
        &self,
        Parameters(input): Parameters<PpddlSimulationInput>,
    ) -> Result<CallToolResult, McpError> {
        let report = simulate_ppddl(
            &input.domain,
            &input.problem,
            &input.options.unwrap_or_default(),
            input.episodes,
            input.seed,
        )
        .map_err(|error| error.to_string());
        to_result(report.and_then(|report| value(&report)))
    }

    #[tool(description = "Project a deterministic explanation for one state in a supplied policy.")]
    fn explain_ppddl_policy(
        &self,
        Parameters(input): Parameters<ExplainInput>,
    ) -> Result<CallToolResult, McpError> {
        let solution = decode::<ProbabilisticSolution>(input.solution, "probabilistic solution");
        to_result(solution.and_then(|solution| {
            explain_policy(&solution, input.state, input.remaining).and_then(|report| value(&report))
        }))
    }

    #[tool(description = "Open a persistent observable-state PPDDL policy session.")]
    async fn policy_session_open(
        &self,
        Parameters(input): Parameters<PolicyOpenInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_policy_session_open(input).await)
    }

    #[tool(description = "Admit an explicitly observed state into a policy session.")]
    async fn policy_session_observe(
        &self,
        Parameters(input): Parameters<PolicyObservationInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_policy_session_observe(input).await)
    }

    #[tool(description = "Select the policy action for the session's admitted state.")]
    async fn policy_session_decide(
        &self,
        Parameters(input): Parameters<PolicySessionIdInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_policy_session_decide(input).await)
    }

    #[tool(description = "Advance a policy session to an admitted action outcome state.")]
    async fn policy_session_advance(
        &self,
        Parameters(input): Parameters<PolicyObservationInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_policy_session_advance(input).await)
    }

    #[tool(description = "Retarget a policy session to a new PPDDL goal expression.")]
    async fn policy_session_set_goal(
        &self,
        Parameters(input): Parameters<PolicyGoalInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_policy_session_set_goal(input).await)
    }

    #[tool(description = "Read the current policy-session state and conformance status.")]
    async fn policy_session_status(
        &self,
        Parameters(input): Parameters<PolicySessionIdInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_policy_session_status(input).await)
    }

    #[tool(description = "Close and remove a persistent policy session.")]
    async fn policy_session_close(
        &self,
        Parameters(input): Parameters<PolicySessionIdInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_policy_session_close(input).await)
    }

    #[tool(description = "Bind model, policy, verifier, constraints, and predecessor into a BLAKE3 receipt.")]
    fn bind_policy_receipt(
        &self,
        Parameters(input): Parameters<BindPolicyInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_bind_policy_receipt(input))
    }

    #[tool(description = "Verify a complete predecessor-linked policy-receipt chain.")]
    fn verify_policy_chain(
        &self,
        Parameters(input): Parameters<VerifyChainInput>,
    ) -> Result<CallToolResult, McpError> {
        to_result(Ok(json!({
            "schema": "urn:chatman:ferroplan-policy-chain-validation:v1",
            "valid": verify_policy_chain(&input.receipts),
            "receipt_count": input.receipts.len(),
        })))
    }
}

impl Ferroplan {
    fn do_solve_ppddl(&self, input: PpddlSolveInput) -> Result<Value, String> {
        let options = input.options.unwrap_or_default();
        let solution = ferroplan::solve_ppddl(&input.domain, &input.problem, &options)
            .map_err(|error| error.to_string())?;
        let verifier = verify_policy(
            &input.domain,
            &input.problem,
            &options,
            &solution,
            &input.constraints,
            input.unsafe_fact.as_deref(),
        )
        .map_err(|error| error.to_string())?;
        let receipt = bind_policy_receipt(
            &input.domain,
            &input.problem,
            &options,
            &input.constraints,
            &solution,
            &verifier,
            None,
        );
        Ok(json!({
            "schema": "urn:chatman:ferroplan-full-planning-result:v1",
            "solution": solution,
            "verification": verifier,
            "receipt": receipt,
        }))
    }

    fn do_validate_ppddl_policy(&self, input: PpddlPolicyInput) -> Result<Value, String> {
        let options = input.options.unwrap_or_default();
        let solution: ProbabilisticSolution = decode(input.solution, "probabilistic solution")?;
        let verifier = verify_policy(
            &input.domain,
            &input.problem,
            &options,
            &solution,
            &input.constraints,
            input.unsafe_fact.as_deref(),
        )
        .map_err(|error| error.to_string())?;
        value(&verifier)
    }

    async fn do_policy_session_open(&self, input: PolicyOpenInput) -> Result<Value, String> {
        if input.session_id.trim().is_empty() {
            return Err("session_id must not be empty".into());
        }
        let session = PolicySession::new(
            input.domain,
            input.problem,
            input.options.unwrap_or_default(),
            input.constraints,
            input.unsafe_fact,
        )
        .map_err(|error| error.to_string())?;
        let mut sessions = self.policy_session_state.sessions.lock().await;
        if sessions.contains_key(&input.session_id) && !input.replace {
            return Err(format!(
                "policy session `{}` already exists; set replace=true to replace it",
                input.session_id
            ));
        }
        let status = session.status();
        sessions.insert(input.session_id.clone(), Arc::new(AsyncMutex::new(session)));
        Ok(json!({
            "schema": "urn:chatman:ferroplan-policy-session-open:v1",
            "session_id": input.session_id,
            "status": status,
        }))
    }

    async fn do_policy_session_observe(
        &self,
        input: PolicyObservationInput,
    ) -> Result<Value, String> {
        let session = self.policy_session_state.get(&input.session_id).await?;
        let mut session = session.lock().await;
        session.observe(input.state).map_err(|error| error.to_string())?;
        Ok(json!({"session_id": input.session_id, "status": session.status()}))
    }

    async fn do_policy_session_decide(
        &self,
        input: PolicySessionIdInput,
    ) -> Result<Value, String> {
        let session = self.policy_session_state.get(&input.session_id).await?;
        let mut session = session.lock().await;
        let decision = session.decide().map_err(|error| error.to_string())?;
        Ok(json!({
            "session_id": input.session_id,
            "decision": decision,
            "status": session.status(),
        }))
    }

    async fn do_policy_session_advance(
        &self,
        input: PolicyObservationInput,
    ) -> Result<Value, String> {
        let session = self.policy_session_state.get(&input.session_id).await?;
        let mut session = session.lock().await;
        session.advance(input.state).map_err(|error| error.to_string())?;
        Ok(json!({"session_id": input.session_id, "status": session.status()}))
    }

    async fn do_policy_session_set_goal(&self, input: PolicyGoalInput) -> Result<Value, String> {
        let session = self.policy_session_state.get(&input.session_id).await?;
        let mut session = session.lock().await;
        session.set_goal(&input.goal).map_err(|error| error.to_string())?;
        Ok(json!({"session_id": input.session_id, "status": session.status()}))
    }

    async fn do_policy_session_status(
        &self,
        input: PolicySessionIdInput,
    ) -> Result<Value, String> {
        let session = self.policy_session_state.get(&input.session_id).await?;
        let session = session.lock().await;
        Ok(json!({"session_id": input.session_id, "status": session.status()}))
    }

    async fn do_policy_session_close(
        &self,
        input: PolicySessionIdInput,
    ) -> Result<Value, String> {
        let closed = self
            .policy_session_state
            .sessions
            .lock()
            .await
            .remove(&input.session_id)
            .is_some();
        Ok(json!({
            "schema": "urn:chatman:ferroplan-policy-session-close:v1",
            "session_id": input.session_id,
            "closed": closed,
        }))
    }

    fn do_bind_policy_receipt(&self, input: BindPolicyInput) -> Result<Value, String> {
        let options = input.options.unwrap_or_default();
        let solution: ProbabilisticSolution = decode(input.solution, "probabilistic solution")?;
        let verifier: PolicyVerificationReport = decode(input.verifier, "policy verifier report")?;
        value(&bind_policy_receipt(
            &input.domain,
            &input.problem,
            &options,
            &input.constraints,
            &solution,
            &verifier,
            input.predecessor,
        ))
    }
}
