//! Drift court: the WASI JSON ABI implemented in the crate sources and the ABI
//! declared in `ontology/ferroplan-wasm.ttl` must describe the same surface.
//!
//! Checked, all as source text (std only, no new dependencies; runs on the
//! native host even though `wasi_abi` itself only compiles for
//! wasm32-wasip1):
//!
//! 1. every `"op" =>` arm in `fn dispatch` of `src/wasi_abi.rs` has a
//!    `wja:opName` in the ontology, and vice versa;
//! 2. every `"FP_*"` string literal in the WASI-surface sources (`ABI_SOURCES_REL`) has a `wja:codeName` in
//!    the ontology, and vice versa;
//! 3. `wja:opName` / `wja:codeName` are unique, and `wja:opOrder` /
//!    `wja:codeOrder` carry no duplicate numbers.
//!
//! The checker is a pure function over text, so the negative self-tests
//! below can mutate in-memory copies and prove the court actually refuses
//! (a court that never refuses carries no bits).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

const WASI_ABI_REL: &str = "src/wasi_abi.rs";
/// Sources that make up the WASI wire surface. `src/lib.rs` is the
/// wasm-bindgen browser surface (it additionally emits `FP_NO_PLAN`, which is
/// not a WASI ABI code) and is deliberately out of scope.
const ABI_SOURCES_REL: [&str; 3] = ["src/wasi_abi.rs", "src/dfcm_route.rs", "src/probe_guard.rs"];
const ONTOLOGY_REL: &str = "../../ontology/ferroplan-wasm.ttl";

fn crate_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn read(rel: &str) -> String {
    let p = crate_path(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()))
}

/// Quoted-string match arms (`"op" => ...`) inside `fn dispatch`.
fn dispatch_ops(src: &str) -> Result<BTreeSet<String>, String> {
    let start = src
        .find("fn dispatch(")
        .ok_or("wasi_abi.rs: `fn dispatch(` not found")?;
    let body = &src[start..];
    // Dispatch ends at the next column-0 item.
    let mut end = body.len();
    let mut off = 0;
    for (i, line) in body.lines().enumerate() {
        if i > 0 && (line.starts_with("fn ") || line.starts_with("pub ") || line.starts_with("///"))
        {
            end = off;
            break;
        }
        off += line.len() + 1;
    }
    let mut ops = BTreeSet::new();
    for line in body[..end].lines() {
        if let Some(rest) = line.trim_start().strip_prefix('"') {
            if let Some(q) = rest.find('"') {
                if rest[q + 1..].trim_start().starts_with("=>") {
                    ops.insert(rest[..q].to_string());
                }
            }
        }
    }
    Ok(ops)
}

/// Every complete `"FP_[A-Z0-9_]+"` literal on a non-comment line.
fn code_literals(src: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in src.lines() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        let mut rest = line;
        while let Some(i) = rest.find("\"FP_") {
            let tail = &rest[i + 1..];
            let Some(q) = tail.find('"') else { break };
            let lit = &tail[..q];
            if lit.len() > 3
                && lit
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
            {
                out.insert(lit.to_string());
            }
            rest = &tail[q + 1..];
        }
    }
    out
}

/// All values following `pred` on non-comment lines. `string` selects a
/// quoted literal versus a bare integer. A predicate may appear inline
/// (several statements per line).
fn values_after(ttl: &str, pred: &str, string: bool) -> Vec<String> {
    let mut out = Vec::new();
    for line in ttl.lines() {
        if line.trim_start().starts_with('#') {
            continue;
        }
        let mut rest = line;
        while let Some(i) = rest.find(pred) {
            let after = &rest[i + pred.len()..];
            // Reject longer predicates sharing the prefix (e.g. opNameX).
            if after.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
                rest = after;
                continue;
            }
            let after = after.trim_start();
            if string {
                if let Some(r) = after.strip_prefix('"') {
                    if let Some(q) = r.find('"') {
                        out.push(r[..q].to_string());
                    }
                }
            } else {
                let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
                if !digits.is_empty() {
                    out.push(digits);
                }
            }
            rest = after;
        }
    }
    out
}

fn duplicates(values: &[String]) -> Vec<String> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for v in values {
        *seen.entry(v.as_str()).or_default() += 1;
    }
    seen.into_iter()
        .filter(|(_, n)| *n > 1)
        .map(|(v, _)| v.to_string())
        .collect()
}

/// Pure checker: `wasi_abi` is the text of `src/wasi_abi.rs`, `sources` the
/// text of every WASI-surface source (including `wasi_abi`), `ttl` the ontology.
fn check(wasi_abi: &str, sources: &[String], ttl: &str) -> Result<(), String> {
    let mut problems: Vec<String> = Vec::new();

    let code_ops = dispatch_ops(wasi_abi)?;
    let onto_op_list = values_after(ttl, "wja:opName", true);
    let onto_ops: BTreeSet<String> = onto_op_list.iter().cloned().collect();
    if code_ops.is_empty() {
        problems.push("no dispatch arms parsed from src/wasi_abi.rs".into());
    }
    if onto_ops.is_empty() {
        problems.push("no wja:opName found in ontology".into());
    }
    for op in code_ops.difference(&onto_ops) {
        problems.push(format!(
            "op `{op}` dispatched in code but has no wja:opName"
        ));
    }
    for op in onto_ops.difference(&code_ops) {
        problems.push(format!("op `{op}` in ontology but not dispatched in code"));
    }

    let code_codes: BTreeSet<String> = sources.iter().flat_map(|s| code_literals(s)).collect();
    let onto_code_list = values_after(ttl, "wja:codeName", true);
    let onto_codes: BTreeSet<String> = onto_code_list.iter().cloned().collect();
    if code_codes.is_empty() {
        problems.push("no FP_* literals parsed from src".into());
    }
    if onto_codes.is_empty() {
        problems.push("no wja:codeName found in ontology".into());
    }
    for c in code_codes.difference(&onto_codes) {
        problems.push(format!(
            "error code {c} used in code but has no wja:codeName"
        ));
    }
    for c in onto_codes.difference(&code_codes) {
        problems.push(format!("error code {c} in ontology but unused in code"));
    }

    for (what, list) in [
        ("wja:opName", onto_op_list),
        ("wja:codeName", onto_code_list),
        ("wja:opOrder", values_after(ttl, "wja:opOrder", false)),
        ("wja:codeOrder", values_after(ttl, "wja:codeOrder", false)),
    ] {
        for d in duplicates(&list) {
            problems.push(format!("duplicate {what} value {d}"));
        }
    }

    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}

fn real_sources() -> Vec<String> {
    ABI_SOURCES_REL.iter().map(|rel| read(rel)).collect()
}

/// Remove every line containing `needle` (in-memory mutation).
fn drop_lines_containing(text: &str, needle: &str) -> String {
    text.lines()
        .filter(|l| !l.contains(needle))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn real_abi_and_ontology_agree() {
    let abi = read(WASI_ABI_REL);
    let ttl = read(ONTOLOGY_REL);
    if let Err(e) = check(&abi, &real_sources(), &ttl) {
        panic!("ABI/ontology drift:\n{e}");
    }
}

#[test]
fn extractors_parse_known_shapes() {
    let src = "fn dispatch(x: &[u8]) {\n    match op {\n        \"a\" => f(&req)?,\n        \"b\" => json!({}),\n        other => { return 1 }\n    }\n}\nfn next() { \"zz\" => 1 }\n";
    assert_eq!(
        dispatch_ops(src).unwrap(),
        ["a", "b"].map(String::from).into()
    );
    let ttl = "x:a a wja:Op ;\n    wja:opName \"a\" ;\n    wja:opOrder 1 .\n# wja:opName \"no\"\nx:e a wja:ErrorCode ; wja:codeName \"FP_X\" ; wja:codeOrder 7 .\n";
    assert_eq!(values_after(ttl, "wja:opName", true), ["a"]);
    assert_eq!(values_after(ttl, "wja:opOrder", false), ["1"]);
    assert_eq!(values_after(ttl, "wja:codeName", true), ["FP_X"]);
    assert_eq!(values_after(ttl, "wja:codeOrder", false), ["7"]);
    let rs = "// \"FP_COMMENT\"\nerr(\"FP_A\", \"msg\"); let x = \"FP_\"; err(\"FP_B_2\")\n";
    assert_eq!(
        code_literals(rs),
        ["FP_A", "FP_B_2"].map(String::from).into()
    );
}

#[test]
fn court_refuses_ontology_missing_an_op() {
    let abi = read(WASI_ABI_REL);
    let ttl = drop_lines_containing(&read(ONTOLOGY_REL), "wja:opName \"plan_production\"");
    let err = check(&abi, &real_sources(), &ttl).expect_err("removed op must be refused");
    assert!(err.contains("plan_production"), "{err}");
}

#[test]
fn court_refuses_ontology_with_extra_op() {
    let abi = read(WASI_ABI_REL);
    let ttl = format!(
        "{}\nfpw:op-ghost a wja:Op ; wja:opName \"ghost_op\" ; wja:opOrder 9999 .\n",
        read(ONTOLOGY_REL)
    );
    let err = check(&abi, &real_sources(), &ttl).expect_err("extra op must be refused");
    assert!(err.contains("ghost_op"), "{err}");
}

#[test]
fn court_refuses_code_missing_a_dispatch_arm() {
    let abi = drop_lines_containing(&read(WASI_ABI_REL), "\"readiness\" =>");
    let ttl = read(ONTOLOGY_REL);
    let err = check(&abi, &real_sources(), &ttl).expect_err("removed arm must be refused");
    assert!(err.contains("readiness"), "{err}");
}

#[test]
fn court_refuses_ontology_missing_an_error_code() {
    let abi = read(WASI_ABI_REL);
    let ttl = drop_lines_containing(&read(ONTOLOGY_REL), "wja:codeName \"FP_MODEL\"");
    let err = check(&abi, &real_sources(), &ttl).expect_err("removed code must be refused");
    assert!(err.contains("FP_MODEL"), "{err}");
}

#[test]
fn court_refuses_ontology_with_extra_error_code() {
    let abi = read(WASI_ABI_REL);
    let ttl = format!(
        "{}\nfpw:err-ghost a wja:ErrorCode ; wja:codeName \"FP_GHOST\" ; wja:codeOrder 9999 .\n",
        read(ONTOLOGY_REL)
    );
    let err = check(&abi, &real_sources(), &ttl).expect_err("extra code must be refused");
    assert!(err.contains("FP_GHOST"), "{err}");
}

#[test]
fn court_refuses_unlisted_code_in_source() {
    let abi = read(WASI_ABI_REL);
    let mut sources = real_sources();
    sources.push("fn x() { err_json(\"FP_SOURCE_ONLY\", \"m\"); }".to_string());
    let err = check(&abi, &sources, &read(ONTOLOGY_REL)).expect_err("unlisted code refused");
    assert!(err.contains("FP_SOURCE_ONLY"), "{err}");
}

#[test]
fn court_refuses_duplicate_op_order() {
    let abi = read(WASI_ABI_REL);
    let ttl = read(ONTOLOGY_REL).replacen("wja:opOrder 2 ;", "wja:opOrder 1 ;", 1);
    let err = check(&abi, &real_sources(), &ttl).expect_err("duplicate order must be refused");
    assert!(err.contains("duplicate wja:opOrder value 1"), "{err}");
}

#[test]
fn court_refuses_duplicate_code_order() {
    let abi = read(WASI_ABI_REL);
    let ttl = read(ONTOLOGY_REL).replacen("wja:codeOrder 2 .", "wja:codeOrder 1 .", 1);
    let err = check(&abi, &real_sources(), &ttl).expect_err("duplicate order must be refused");
    assert!(err.contains("duplicate wja:codeOrder value 1"), "{err}");
}
