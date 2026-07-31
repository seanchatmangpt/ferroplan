//! Proof that the merged server exposes every deterministic, probabilistic,
//! session, allocation, and receipt tool exactly once.

use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

fn raw_drive(messages: &[Value]) -> Vec<Value> {
    let bin = env!("CARGO_BIN_EXE_ferroplan-mcp");
    let mut child = Command::new(bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn ferroplan-mcp");
    {
        let stdin = child.stdin.as_mut().expect("stdin");
        for message in messages {
            writeln!(stdin, "{message}").expect("write message");
        }
    }
    let out = child.wait_with_output().expect("wait");
    assert!(out.status.success(), "server exited with {:?}", out.status);
    String::from_utf8(out.stdout)
        .expect("utf8")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("each response is one JSON line"))
        .collect()
}

fn handshake_initialize() -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": "__handshake__",
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "merged-server-test", "version": "0"}
        }
    })
}

fn handshake_initialized() -> Value {
    json!({"jsonrpc": "2.0", "method": "notifications/initialized"})
}

fn drive(messages: &[Value]) -> Vec<Value> {
    let mut all = vec![handshake_initialize(), handshake_initialized()];
    all.extend_from_slice(messages);
    let mut response = raw_drive(&all);
    assert!(!response.is_empty(), "expected at least the handshake response");
    response.remove(0);
    response
}

fn find_response(response: &[Value], id: i64) -> Value {
    response
        .iter()
        .find(|value| value["id"] == json!(id))
        .unwrap_or_else(|| panic!("no response with id {id} in {response:?}"))
        .clone()
}

const ALL_TOOLS: &[&str] = &[
    "solve",
    "parse",
    "validate",
    "decompose",
    "session_open",
    "session_observe",
    "session_set_goal",
    "session_think",
    "session_advance",
    "session_status",
    "session_close",
    "cmca_allocate",
    "cmca_allocate_recursive",
    "canonical_digest",
    "bind_allocation_receipt",
    "bind_plan_receipt",
    "verify_receipt",
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

#[test]
fn initialize_advertises_the_complete_tool_surface() {
    let response = raw_drive(&[
        json!({
            "jsonrpc":"2.0","id":1,"method":"initialize",
            "params":{
                "protocolVersion":"2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "merged-server-test", "version": "0"}
            }
        }),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
    ]);
    assert_eq!(response.len(), 2, "notification must not produce a response");
    assert_eq!(response[0]["result"]["serverInfo"]["name"], "ferroplan");

    let mut names: Vec<&str> = response[1]["result"]["tools"]
        .as_array()
        .expect("tools array")
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    names.sort_unstable();

    let mut expected: Vec<&str> = ALL_TOOLS.to_vec();
    expected.sort_unstable();
    assert_eq!(names.len(), ALL_TOOLS.len(), "tool count drift: {names:?}");
    assert_eq!(names, expected, "merged server tool set drifted");
}

#[test]
fn resources_list_matches_the_complete_tool_surface() {
    let response = drive(&[
        json!({"jsonrpc":"2.0","id":1,"method":"resources/list"}),
        json!({"jsonrpc":"2.0","id":2,"method":"resources/read",
               "params":{"uri":"ferroplan://tools/solve_ppddl"}}),
    ]);

    let list = find_response(&response, 1);
    let resources = list["result"]["resources"]
        .as_array()
        .expect("resources array");
    assert_eq!(resources.len(), ALL_TOOLS.len());

    let mut uris: Vec<String> = resources
        .iter()
        .map(|resource| resource["uri"].as_str().unwrap().to_owned())
        .collect();
    uris.sort_unstable();
    let mut expected: Vec<String> = ALL_TOOLS
        .iter()
        .map(|name| format!("ferroplan://tools/{name}"))
        .collect();
    expected.sort_unstable();
    assert_eq!(uris, expected);

    let read = find_response(&response, 2);
    let contents = read["result"]["contents"]
        .as_array()
        .expect("contents array");
    assert_eq!(contents.len(), 1);
    let text = contents[0]["text"].as_str().expect("resource text");
    let body: Value = serde_json::from_str(text).expect("resource body is JSON");
    assert_eq!(body["tool"], "solve_ppddl");
    let comment = body["rdfs_comment"].as_str().expect("rdfs_comment string");
    assert!(comment.len() > 10, "ontology comment is not useful: {comment:?}");
}

#[test]
fn no_tool_input_schema_uses_a_boolean_subschema() {
    let response = drive(&[json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"})]);
    let tools = find_response(&response, 1)["result"]["tools"]
        .as_array()
        .expect("tools array")
        .clone();
    assert_eq!(tools.len(), ALL_TOOLS.len());

    let offenders: Vec<String> = tools
        .iter()
        .flat_map(|tool| {
            let name = tool["name"].as_str().unwrap_or("<unnamed>").to_owned();
            tool["inputSchema"]["properties"]
                .as_object()
                .into_iter()
                .flatten()
                .filter(|(_, value)| value.is_boolean())
                .map(move |(key, value)| format!("{name}.{key} = {value}"))
                .collect::<Vec<_>>()
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "boolean subschemas are rejected by MCP clients; use object schemas: {offenders:#?}"
    );
}
