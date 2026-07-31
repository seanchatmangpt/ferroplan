//! Generates `const *_ONTOLOGY: &str = "...";` resource-content constants for
//! each MCP tool group, extracted at compile time from
//! `plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl`.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SESSION_TOOLS: &[(&str, &str)] = &[
    ("OPEN_ONTOLOGY", "session_open"),
    ("OBSERVE_ONTOLOGY", "session_observe"),
    ("SET_GOAL_ONTOLOGY", "session_set_goal"),
    ("THINK_ONTOLOGY", "session_think"),
    ("ADVANCE_ONTOLOGY", "session_advance"),
    ("STATUS_ONTOLOGY", "session_status"),
    ("CLOSE_ONTOLOGY", "session_close"),
    ("CMCA_ONTOLOGY", "cmca_allocate"),
    ("CMCA_RECURSIVE_ONTOLOGY", "cmca_allocate_recursive"),
];

const ADMISSION_TOOLS: &[(&str, &str)] = &[
    ("DIGEST_ONTOLOGY", "canonical_digest"),
    ("BIND_ALLOC_ONTOLOGY", "bind_allocation_receipt"),
    ("BIND_PLAN_ONTOLOGY", "bind_plan_receipt"),
    ("VERIFY_ONTOLOGY", "verify_receipt"),
];

const MAIN_TOOLS: &[(&str, &str)] = &[
    ("SOLVE_ONTOLOGY", "solve"),
    ("PARSE_ONTOLOGY", "parse"),
    ("VALIDATE_ONTOLOGY", "validate"),
    ("DECOMPOSE_ONTOLOGY", "decompose"),
];

const FULL_PLANNING_TOOLS: &[(&str, &str)] = &[
    ("PARSE_PPDDL_ONTOLOGY", "parse_ppddl"),
    ("SOLVE_PPDDL_ONTOLOGY", "solve_ppddl"),
    ("VALIDATE_PPDDL_ONTOLOGY", "validate_ppddl_policy"),
    ("SIMULATE_PPDDL_ONTOLOGY", "simulate_ppddl"),
    ("EXPLAIN_PPDDL_ONTOLOGY", "explain_ppddl_policy"),
    ("POLICY_OPEN_ONTOLOGY", "policy_session_open"),
    ("POLICY_OBSERVE_ONTOLOGY", "policy_session_observe"),
    ("POLICY_DECIDE_ONTOLOGY", "policy_session_decide"),
    ("POLICY_ADVANCE_ONTOLOGY", "policy_session_advance"),
    ("POLICY_SET_GOAL_ONTOLOGY", "policy_session_set_goal"),
    ("POLICY_STATUS_ONTOLOGY", "policy_session_status"),
    ("POLICY_CLOSE_ONTOLOGY", "policy_session_close"),
    ("BIND_POLICY_ONTOLOGY", "bind_policy_receipt"),
    ("VERIFY_POLICY_CHAIN_ONTOLOGY", "verify_policy_chain"),
];

const FALLBACK: &str = "(ontology extraction fallback: no rdfs:comment could be located for this tool's fp:McpTool individual at build time; see build.rs.)";

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by cargo");
    let ttl_relative = "../../plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl";
    let ttl_path = Path::new(&manifest_dir).join(ttl_relative);

    println!("cargo:rerun-if-changed={ttl_relative}");
    println!("cargo:rerun-if-changed=build.rs");

    let ttl = fs::read_to_string(&ttl_path).unwrap_or_else(|error| {
        panic!(
            "ferroplan-mcp/build.rs: failed to read ontology at {} (resolved from CARGO_MANIFEST_DIR={manifest_dir}, relative path {ttl_relative}): {error}",
            ttl_path.display()
        )
    });

    let comments = extract_tool_comments(&ttl);
    let mut fallbacks_used = Vec::new();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo"));

    for (file, tools) in [
        ("session_ontology.rs", SESSION_TOOLS),
        ("admission_ontology.rs", ADMISSION_TOOLS),
        ("main_ontology.rs", MAIN_TOOLS),
        ("full_planning_ontology.rs", FULL_PLANNING_TOOLS),
    ] {
        write_group(
            &out_dir.join(file),
            tools,
            &comments,
            &mut fallbacks_used,
        );
    }

    if !fallbacks_used.is_empty() {
        println!(
            "cargo:warning=ferroplan-mcp/build.rs: fell back to a placeholder ontology comment for tool(s): {}",
            fallbacks_used.join(", ")
        );
    }
}

fn extract_tool_comments(ttl: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let lines: Vec<&str> = ttl.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("fp:Tool") && line.contains("a fp:McpTool") {
            if let Some(label) = extract_quoted(line, "rdfs:label \"") {
                let mut j = i;
                let mut found = None;
                while j < lines.len() && j < i + 8 {
                    if let Some(comment) = extract_quoted(lines[j], "rdfs:comment \"") {
                        found = Some(comment);
                        break;
                    }
                    if j > i && lines[j].starts_with("fp:") {
                        break;
                    }
                    j += 1;
                }
                if let Some(comment) = found {
                    out.insert(label, comment);
                }
            }
        }
        i += 1;
    }
    out
}

fn extract_quoted(line: &str, marker: &str) -> Option<String> {
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let mut out = String::new();
    let mut chars = rest.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(&next) = chars.peek() {
                    out.push(next);
                    chars.next();
                }
            }
            '"' => return Some(out),
            _ => out.push(c),
        }
    }
    None
}

fn write_group(
    dest: &Path,
    tools: &[(&str, &str)],
    comments: &BTreeMap<String, String>,
    fallbacks_used: &mut Vec<String>,
) {
    let mut generated = String::new();
    generated.push_str("// @generated by ferroplan-mcp/build.rs from ferroplan-domain.ttl. Do not edit; do not commit (OUT_DIR only).\n");
    for (const_name, tool_label) in tools {
        let text = match comments.get(*tool_label) {
            Some(comment) => comment.clone(),
            None => {
                fallbacks_used.push((*tool_label).to_string());
                FALLBACK.to_string()
            }
        };
        generated.push_str(&format!(
            "const {const_name}: &str = {};\n",
            rust_string_literal(&text)
        ));
    }
    fs::write(dest, generated).unwrap_or_else(|error| {
        panic!(
            "ferroplan-mcp/build.rs: failed to write generated ontology file {}: {error}",
            dest.display()
        )
    });
}

fn rust_string_literal(s: &str) -> String {
    format!("{s:?}")
}
