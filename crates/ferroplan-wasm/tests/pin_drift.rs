//! Pin-drift court: `registry/ARTIFACTS.sha256` must carry exactly one `wasm` line whose
//! sha256 and byte count equal the ontology pin (`wja:wasmSha256` / `wja:wasmBytes` in
//! `/ontology/ferroplan-wasm.ttl`).
//!
//! NOTE: the ENFORCED pin source is the ontology. ARTIFACTS.sha256 is a create-mode
//! projection that nothing else reads; this court is what detects hand edits to it.
//!
//! Std only. Parsers are pure functions over text; mutation self-tests prove refusal.

use std::path::PathBuf;

fn crate_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn is_sha256(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Parse the single `wasm <sha256> <bytes> <name>` line of ARTIFACTS.sha256.
fn parse_registry_pin(text: &str) -> Result<(String, u64), String> {
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| l.split_whitespace().next() == Some("wasm"))
        .collect();
    if lines.len() != 1 {
        return Err(format!(
            "expected exactly 1 wasm line, found {}",
            lines.len()
        ));
    }
    let f: Vec<&str> = lines[0].split_whitespace().collect();
    if f.len() != 4 {
        return Err(format!("wasm line needs 4 fields, found {}", f.len()));
    }
    if !is_sha256(f[1]) {
        return Err(format!("sha256 is not 64 lowercase hex: {}", f[1]));
    }
    let bytes: u64 = f[2]
        .parse()
        .map_err(|_| format!("bytes not an integer: {}", f[2]))?;
    Ok((f[1].to_owned(), bytes))
}

/// Extract `wja:wasmSha256 "<hex>"` and `wja:wasmBytes <int>` from ontology text.
fn parse_ontology_pin(text: &str) -> Result<(String, u64), String> {
    let mut sha = Vec::new();
    let mut bytes = Vec::new();
    for l in text.lines() {
        let t = l.trim();
        if let Some(r) = t.strip_prefix("wja:wasmSha256") {
            sha.push(
                r.trim()
                    .trim_end_matches([';', '.'])
                    .trim()
                    .trim_matches('"')
                    .to_owned(),
            );
        } else if let Some(r) = t.strip_prefix("wja:wasmBytes") {
            bytes.push(r.trim().trim_end_matches([';', '.']).trim().to_owned());
        }
    }
    if sha.len() != 1 || bytes.len() != 1 {
        return Err(format!(
            "ontology needs one sha and one bytes fact, found {}/{}",
            sha.len(),
            bytes.len()
        ));
    }
    if !is_sha256(&sha[0]) {
        return Err(format!("ontology sha256 malformed: {}", sha[0]));
    }
    let b = bytes[0]
        .parse()
        .map_err(|_| format!("ontology bytes not integer: {}", bytes[0]))?;
    Ok((sha.remove(0), b))
}

fn check(registry: &str, ontology: &str) -> Result<(), String> {
    let r = parse_registry_pin(registry)?;
    let o = parse_ontology_pin(ontology)?;
    if r != o {
        return Err(format!("registry pin {r:?} != ontology pin {o:?}"));
    }
    Ok(())
}

fn read(rel: &str) -> String {
    let p = crate_path(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

#[test]
fn registry_pin_matches_ontology() {
    let reg = read("registry/ARTIFACTS.sha256");
    let ont = read("../../ontology/ferroplan-wasm.ttl");
    check(&reg, &ont).unwrap_or_else(|e| panic!("pin drift: {e}"));
}

const ONT: &str = "x\n    wja:wasmSha256 \"088d9c3b0306e36123ddc1ee780ad7e9d4bd2ebb54f9726c43f40a2f6d718233\" ;\n    wja:wasmBytes 3521589 ;\n";
const REG: &str = "# c\nwasm 088d9c3b0306e36123ddc1ee780ad7e9d4bd2ebb54f9726c43f40a2f6d718233 3521589 ferroplan_wasm.wasm\n";

#[test]
fn baseline_fixture_passes() {
    check(REG, ONT).unwrap();
}

#[test]
fn mutation_edit_sha_refused() {
    let m = REG.replace("088d9c3b", "188d9c3b");
    assert!(check(&m, ONT).is_err());
}

#[test]
fn mutation_edit_bytes_refused() {
    let m = REG.replace("3521589", "3521590");
    assert!(check(&m, ONT).is_err());
}

#[test]
fn mutation_delete_line_refused() {
    assert!(check("# only comments\n", ONT).is_err());
}

#[test]
fn mutation_duplicate_line_refused() {
    let m = format!("{REG}{}", REG.lines().nth(1).unwrap());
    assert!(check(&m, ONT).is_err());
}

#[test]
fn mutation_malformed_refused() {
    assert!(check("wasm nothex 3521589 a.wasm\n", ONT).is_err());
    assert!(check(&REG.replace("3521589", "12x"), ONT).is_err());
}
