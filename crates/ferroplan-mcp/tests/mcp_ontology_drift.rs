//! Drift court: the MCP tool surface implemented in this crate's sources and
//! the `fp:McpTool` individuals in the Ferroplan ontologies must describe the
//! same surface.
//!
//! Checked, all as source text (std only):
//!
//! 1. the set of `#[tool(...)] fn <name>` handlers across the five live
//!    router files equals the set of `fp:McpTool` `rdfs:label`s;
//! 2. for the four stateless tools in `main.rs`, the fields of the request
//!    struct equal the ontology's `fp:hasInputField` labels (catches the
//!    historical `parse` `source`-vs-`pddl` drift);
//! 3. every `fp:rustConst` in the ontology is declared in the committed
//!    `src/generated/tool_ontology.rs`, in the module named by `fp:rustGroup`.
//!
//! Checkers are pure functions over text so the negative self-tests below can
//! mutate in-memory copies and prove the court refuses.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

const ROUTER_SOURCES: [&str; 5] = [
    "src/main.rs",
    "src/session.rs",
    "src/session_control.rs",
    "src/experience.rs",
    "src/admission.rs",
];
const ONTOLOGIES: [&str; 2] = [
    "../../plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl",
    "../../plugins/chatman-ecosystem/ontology/ferroplan-experience.ttl",
];
const GENERATED_REL: &str = "src/generated/tool_ontology.rs";
/// (tool name, request struct) for the stateless tools whose fields we pin.
const MAIN_STRUCTS: [(&str, &str); 4] = [
    ("solve", "SolveRequest"),
    ("parse", "ParseRequest"),
    ("validate", "ValidateRequest"),
    ("decompose", "DecomposeRequest"),
];

fn read(rel: &str) -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()))
}

fn ident_at(s: &str) -> String {
    s.chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// Names of `fn`s directly annotated with `#[tool(`. The attribute body is
/// skipped by paren matching (descriptions contain parens and brackets).
fn live_tools(src: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut rest = src;
    while let Some(i) = rest.find("#[tool(") {
        let after = &rest[i + "#[tool(".len()..];
        let (mut depth, mut in_str, mut esc, mut end) = (1usize, false, false, None);
        for (k, c) in after.char_indices() {
            if in_str {
                if esc {
                    esc = false;
                } else if c == '\\' {
                    esc = true;
                } else if c == '"' {
                    in_str = false;
                }
                continue;
            }
            match c {
                '"' => in_str = true,
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(k + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        let end = end.expect("unterminated #[tool( attribute");
        let tail = &after[end..];
        if let Some(f) = tail.find("fn ") {
            let name = ident_at(&tail[f + 3..]);
            if !name.is_empty() {
                out.insert(name);
            }
        }
        rest = &after[end..];
    }
    out
}

/// One parsed `fp:McpTool` individual: label, input-field labels, rust const/group.
#[derive(Default, Debug)]
struct OntTool {
    inputs: BTreeSet<String>,
    rust_const: Option<String>,
    rust_group: Option<String>,
}

fn quoted_after<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    let i = line.find(marker)? + marker.len();
    let rest = &line[i..];
    Some(&rest[..rest.find('"')?])
}

/// Line-oriented scan (the ontology keeps one `fp:McpTool` header per line and
/// its fields on following lines, up to the terminating `.`).
fn ontology_tools(ttl: &str) -> BTreeMap<String, OntTool> {
    let mut out = BTreeMap::new();
    let mut cur: Option<String> = None;
    for line in ttl.lines() {
        if line.starts_with("fp:Tool") && line.contains("a fp:McpTool") {
            let label = quoted_after(line, "rdfs:label \"").expect("McpTool without label");
            cur = Some(label.to_string());
            out.insert(label.to_string(), OntTool::default());
        } else if !line.starts_with(' ')
            && !line.starts_with("fp:Tool")
            && (line.trim().is_empty() || line.starts_with('#') || line.starts_with("fp:"))
        {
            cur = None;
        }
        if let Some(label) = &cur {
            let t = out.get_mut(label).unwrap();
            if let Some(c) = quoted_after(line, "fp:rustConst \"") {
                t.rust_const = Some(c.to_string());
            }
            if let Some(g) = quoted_after(line, "fp:rustGroup \"") {
                t.rust_group = Some(g.to_string());
            }
            if line.contains("fp:hasInputField") {
                if let Some(f) = quoted_after(line, "rdfs:label \"") {
                    t.inputs.insert(f.to_string());
                }
            }
        }
    }
    out
}

/// Field names of `struct <name> { ... }` (doc comments and attributes skipped).
fn struct_fields(src: &str, name: &str) -> Result<BTreeSet<String>, String> {
    let head = format!("struct {name} {{");
    let i = src.find(&head).ok_or(format!("struct {name} not found"))?;
    let body = &src[i + head.len()..];
    let body = &body[..body.find("\n}").ok_or("unterminated struct")?];
    Ok(body
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//") && !l.starts_with('#') && l.contains(':'))
        .map(|l| ident_at(l.trim_start_matches("pub ")))
        .collect())
}

fn check(
    live: &BTreeSet<String>,
    onto: &BTreeMap<String, OntTool>,
    main_src: &str,
    generated: &str,
) -> Result<(), String> {
    let names: BTreeSet<String> = onto.keys().cloned().collect();
    let only_live: Vec<_> = live.difference(&names).collect();
    let only_onto: Vec<_> = names.difference(live).collect();
    if !only_live.is_empty() || !only_onto.is_empty() {
        return Err(format!(
            "tool set drift: live-only {only_live:?}, ontology-only {only_onto:?}"
        ));
    }
    for (tool, strukt) in MAIN_STRUCTS {
        let fields = struct_fields(main_src, strukt)?;
        if fields != onto[tool].inputs {
            return Err(format!(
                "input drift for `{tool}`: {strukt} has {fields:?}, ontology has {:?}",
                onto[tool].inputs
            ));
        }
    }
    for (tool, t) in onto {
        match (&t.rust_const, &t.rust_group) {
            (None, None) => {}
            (Some(c), Some(g)) => {
                let modhead = format!("pub(crate) mod {g} {{");
                let m = generated
                    .find(&modhead)
                    .ok_or(format!("generated module `{g}` missing for `{tool}`"))?;
                let m_end = generated[m..]
                    .find("\n}")
                    .map_or(generated.len(), |e| m + e);
                if !generated[m..m_end].contains(&format!("pub(crate) const {c}:")) {
                    return Err(format!("const {c} for `{tool}` not in generated mod {g}"));
                }
            }
            _ => return Err(format!("`{tool}` has only one of rustConst/rustGroup")),
        }
    }
    Ok(())
}

fn inputs() -> (BTreeSet<String>, BTreeMap<String, OntTool>, String, String) {
    let mut live = BTreeSet::new();
    for f in ROUTER_SOURCES {
        live.extend(live_tools(&read(f)));
    }
    let mut onto = BTreeMap::new();
    for f in ONTOLOGIES {
        onto.extend(ontology_tools(&read(f)));
    }
    (live, onto, read("src/main.rs"), read(GENERATED_REL))
}

#[test]
fn live_tools_match_ontology() {
    let (live, onto, main_src, generated) = inputs();
    assert!(live.len() >= 40, "scanner found only {} tools", live.len());
    if let Err(e) = check(&live, &onto, &main_src, &generated) {
        panic!("{e}");
    }
}

#[test]
fn refuses_renamed_live_tool() {
    let (mut live, onto, main_src, generated) = inputs();
    live.remove("solve");
    live.insert("solve_renamed".into());
    assert!(check(&live, &onto, &main_src, &generated).is_err());
}

#[test]
fn refuses_ontology_tool_removed() {
    let (live, mut onto, main_src, generated) = inputs();
    onto.remove("session_list");
    assert!(check(&live, &onto, &main_src, &generated).is_err());
}

#[test]
fn refuses_parse_field_regression() {
    let (live, mut onto, main_src, generated) = inputs();
    let t = onto.get_mut("parse").unwrap();
    t.inputs.remove("pddl");
    t.inputs.insert("source".into());
    let err = check(&live, &onto, &main_src, &generated).unwrap_err();
    assert!(err.contains("parse"), "{err}");
}

#[test]
fn refuses_missing_generated_const() {
    let (live, onto, main_src, generated) = inputs();
    let mutated = generated.replace(
        "pub(crate) const SOLVE_ONTOLOGY:",
        "pub(crate) const SOLVE_X:",
    );
    assert!(check(&live, &onto, &main_src, &mutated).is_err());
}
