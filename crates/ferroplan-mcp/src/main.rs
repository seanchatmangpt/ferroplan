//! `ferroplan-mcp` — one MCP server for deterministic planning, PPDDL policy
//! synthesis, persistent planning sessions, bounded allocation, and receipts.
//!
//! The server exposes four independently bounded tool groups:
//!
//! - deterministic stateless planning;
//! - persistent deterministic Session minds and CMCA allocation;
//! - canonical admission receipts;
//! - probabilistic planning, policy sessions, verification, simulation, and
//!   policy receipts.
//!
//! Tool descriptions are projected from `ferroplan-domain.ttl` at build time.
//! Planning and verification remain separate from actuation authority.

mod admission;
mod full_planning;
mod result;
mod session;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, ErrorData as McpError, ListResourcesResult, ReadResourceRequestParams,
    ReadResourceResult, Resource, ResourceContents, ServerCapabilities, ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::{tool, tool_handler, tool_router, RoleServer, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;

use crate::result::to_result;

include!(concat!(env!("OUT_DIR"), "/main_ontology.rs"));

const MAIN_RESOURCE_TOOLS: &[&str] = &["solve", "parse", "validate", "decompose"];

fn main_ontology_comment(name: &str) -> Option<&'static str> {
    Some(match name {
        "solve" => SOLVE_ONTOLOGY,
        "parse" => PARSE_ONTOLOGY,
        "validate" => VALIDATE_ONTOLOGY,
        "decompose" => DECOMPOSE_ONTOLOGY,
        _ => return None,
    })
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SolveRequest {
    domain: String,
    problem: String,
    #[serde(default)]
    options: Option<ferroplan::Options>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ParseRequest {
    pddl: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ValidateRequest {
    domain: String,
    problem: String,
    plan: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DecomposeRequest {
    domain: String,
    problem: String,
    #[serde(default)]
    options: Option<ferroplan::Options>,
}

#[derive(Clone)]
struct Ferroplan {
    tool_router: ToolRouter<Self>,
    session_state: session::SessionState,
    policy_session_state: full_planning::PolicySessionState,
}

impl Ferroplan {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router()
                + Self::session_router()
                + Self::admission_router()
                + Self::full_planning_router(),
            session_state: session::SessionState::default(),
            policy_session_state: full_planning::PolicySessionState::default(),
        }
    }
}

#[tool_router]
impl Ferroplan {
    #[tool(
        description = "Plan a deterministic PDDL domain + problem and return a structured Solution."
    )]
    fn solve(&self, Parameters(req): Parameters<SolveRequest>) -> Result<CallToolResult, McpError> {
        to_result(self.do_solve(req))
    }

    #[tool(description = "Syntax-check one PDDL source and return a structure summary.")]
    fn parse(&self, Parameters(req): Parameters<ParseRequest>) -> Result<CallToolResult, McpError> {
        to_result(as_value(&ferroplan::parse(&req.pddl)))
    }

    #[tool(description = "Independently validate a deterministic or temporal plan.")]
    fn validate(
        &self,
        Parameters(req): Parameters<ValidateRequest>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_validate(req))
    }

    #[tool(description = "Decompose and solve a temporal goal as ordered contracts.")]
    fn decompose(
        &self,
        Parameters(req): Parameters<DecomposeRequest>,
    ) -> Result<CallToolResult, McpError> {
        to_result(self.do_decompose(req))
    }
}

impl Ferroplan {
    fn do_solve(&self, req: SolveRequest) -> Result<serde_json::Value, String> {
        let opts = req.options.unwrap_or_default();
        let sol = ferroplan::solve(&req.domain, &req.problem, &opts).map_err(|e| e.to_string())?;
        as_value(&sol)
    }

    fn do_validate(&self, req: ValidateRequest) -> Result<serde_json::Value, String> {
        let (valid, reason) =
            match ferroplan::plan::validate_plan(&req.domain, &req.problem, &req.plan)? {
                ferroplan::plan::Validity::Valid => (true, None),
                ferroplan::plan::Validity::Invalid(why) => (false, Some(why)),
            };
        Ok(json!({
            "schema": "urn:ferroplan:plan-validation:v1",
            "valid": valid,
            "reason": reason,
        }))
    }

    fn do_decompose(&self, req: DecomposeRequest) -> Result<serde_json::Value, String> {
        let opts = req.options.unwrap_or_default();
        let dec =
            ferroplan::decompose(&req.domain, &req.problem, &opts).map_err(|e| e.to_string())?;
        as_value(&dec)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Ferroplan {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
        )
        .with_server_info(rmcp::model::Implementation::new(
            "ferroplan",
            env!("CARGO_PKG_VERSION"),
        ))
        .with_instructions(
            "Use solve/parse/validate/decompose for deterministic PDDL. Use parse_ppddl/\
             solve_ppddl/validate_ppddl_policy/simulate_ppddl/explain_ppddl_policy for \
             probabilistic planning. Policy sessions require explicit observations and refuse \
             impossible outcomes. Receipt tools bind evidence but never authorize actuation. \
             Read ferroplan://tools/<name> for ontology-sourced semantics.",
        )
    }

    async fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let resources = all_tool_names()
            .into_iter()
            .map(|name| {
                Resource::new(
                    format!("ferroplan://tools/{name}"),
                    format!("{name} (semantic summary)"),
                )
                .with_description(format!(
                    "Ontology-sourced semantics for the `{name}` tool, from ferroplan-domain.ttl."
                ))
                .with_mime_type("application/json")
            })
            .collect();
        Ok(ListResourcesResult {
            resources,
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let name = request
            .uri
            .strip_prefix("ferroplan://tools/")
            .ok_or_else(|| McpError::resource_not_found(request.uri.clone(), None))?;
        let ontology_comment = main_ontology_comment(name)
            .or_else(|| session::ontology_comment(name))
            .or_else(|| admission::ontology_comment(name))
            .or_else(|| full_planning::ontology_comment(name))
            .ok_or_else(|| McpError::resource_not_found(request.uri.clone(), None))?;
        let body = serde_json::json!({
            "tool": name,
            "source": "plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl",
            "rdfs_comment": ontology_comment,
        });
        Ok(ReadResourceResult::new(vec![ResourceContents::text(
            serde_json::to_string_pretty(&body).unwrap_or_default(),
            request.uri,
        )]))
    }
}

fn all_tool_names() -> Vec<&'static str> {
    MAIN_RESOURCE_TOOLS
        .iter()
        .chain(session::RESOURCE_TOOLS)
        .chain(admission::RESOURCE_TOOLS)
        .chain(full_planning::RESOURCE_TOOLS)
        .copied()
        .collect()
}

#[allow(dead_code)]
fn pretty<T: serde::Serialize>(v: &T) -> Result<String, String> {
    serde_json::to_string_pretty(v).map_err(|e| e.to_string())
}

fn as_value<T: serde::Serialize>(v: &T) -> Result<serde_json::Value, String> {
    serde_json::to_value(v).map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = Ferroplan::new()
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|e| {
            eprintln!("serving error: {e}");
            e
        })?;
    service.waiting().await?;
    Ok(())
}
