//! Meta-drift court: the ontology wasm pin has TWO independent extraction
//! mechanisms — the `justfile` `wasm-build-pin` grep-regex (shell, head -1) and
//! `tests/pin_drift.rs`'s strict single-fact parser. If they ever disagree, one
//! of the two courts is pinning a different subject than the other. This test
//! reimplements the justfile's matching logic as a pure function and asserts
//! both mechanisms extract identical (sha256, bytes) from the REAL
//! `ontology/ferroplan-wasm.ttl`.
//!
//! It also pins the test-directory inventory: the pinned list of court files
//! below must equal the actual `crates/ferroplan-wasm/tests/` contents in both
//! directions — an unlisted new court or a deleted pinned court fails here, so
//! the court surface itself cannot drift silently.
//!
//! Std only. All extractors are pure functions over text; mutation self-tests
//! prove the court is non-vacuous.

use std::path::PathBuf;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

// ---------------------------------------------------------------------------
// Mechanism 1: the justfile `wasm-build-pin` extraction, reimplemented.
// justfile does:
//   grep -o 'wja:wasmSha256 "[0-9a-f]*"' ontology/ferroplan-wasm.ttl | head -1 | cut -d'"' -f2
//   grep -o 'wja:wasmBytes [0-9]*'      ontology/ferroplan-wasm.ttl | head -1 | cut -d' ' -f2
// grep -o takes non-overlapping matches left-to-right; head -1 = first match.
// ---------------------------------------------------------------------------

/// Equivalent of `grep -o 'wja:wasmSha256 "[0-9a-f]*"' | head -1 | cut -d'"' -f2`.
fn justfile_style_sha(text: &str) -> Option<String> {
    let mut rest = text;
    while let Some(i) = rest.find("wja:wasmSha256 \"") {
        let after = &rest[i + "wja:wasmSha256 \"".len()..];
        let hex: String = after.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
        // The grep pattern only matches chars inside the quotes up to the next
        // quote; the matched run must be [0-9a-f] only and quote-terminated to
        // equal the full grep match semantics.
        let closes = after[hex.len()..].starts_with('"');
        if closes
            && !hex.is_empty()
            && hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
        {
            return Some(hex);
        }
        rest = after;
    }
    None
}

/// Equivalent of `grep -o 'wja:wasmBytes [0-9]*' | head -1 | cut -d' ' -f2`.
fn justfile_style_bytes(text: &str) -> Option<String> {
    let mut rest = text;
    while let Some(i) = rest.find("wja:wasmBytes ") {
        let after = &rest[i + "wja:wasmBytes ".len()..];
        let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            return Some(digits);
        }
        rest = after;
    }
    None
}

// ---------------------------------------------------------------------------
// Mechanism 2: the pin_drift.rs ontology parser (strict: exactly one fact).
// ---------------------------------------------------------------------------

fn is_sha256(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Same shape as `tests/pin_drift.rs::parse_ontology_pin`.
fn pin_drift_style_parse(text: &str) -> Result<(String, u64), String> {
    let mut sha = Vec::new();
    let mut bytes = Vec::new();
    for line in text.lines() {
        let t = line.trim();
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

/// The court: both mechanisms must extract the identical pin, and the strict
/// parser must succeed. Err(message) on any disagreement.
fn check_mechanisms(ontology: &str) -> Result<(String, u64), String> {
    let strict = pin_drift_style_parse(ontology)?;
    let jf_sha = justfile_style_sha(ontology)
        .ok_or_else(|| "justfile-style extractor found no wja:wasmSha256".to_owned())?;
    let jf_bytes = justfile_style_bytes(ontology)
        .ok_or_else(|| "justfile-style extractor found no wja:wasmBytes".to_owned())?;
    let jf_bytes: u64 = jf_bytes
        .parse()
        .map_err(|_| format!("justfile-style bytes not integer: {jf_bytes}"))?;
    if jf_sha != strict.0 || jf_bytes != strict.1 {
        return Err(format!(
            "MECHANISM_DRIFT: justfile pin ({jf_sha}, {jf_bytes}) != pin_drift pin ({}, {})",
            strict.0, strict.1
        ));
    }
    Ok(strict)
}

// ---------------------------------------------------------------------------
// Court-file inventory: the pinned list must equal the actual tests/ contents.
// ---------------------------------------------------------------------------

/// Every Rust court file that must exist in `crates/ferroplan-wasm/tests/`.
/// Adding or removing a court file requires updating this list IN THE SAME
/// dependency-closed change — that is the point of the pin.
const PINNED_COURT_FILES: &[&str] = &[
    "abi_ontology_drift.rs",
    "browser.rs",
    "ci_copy_drift.rs",
    "concurrency_stress.rs",
    "copy_drift.rs",
    "dfcm_browser_parity.rs",
    "free_arity_drift.rs",
    "meta_drift.rs",
    "pack_content_pin.rs",
    "pin_drift.rs",
];

/// Ok(()) when the directory listing equals the pinned list exactly.
fn check_inventory(actual: &mut Vec<String>) -> Result<(), String> {
    let mut pinned: Vec<String> = PINNED_COURT_FILES.iter().map(|s| s.to_string()).collect();
    actual.sort();
    pinned.sort();
    if *actual == pinned {
        return Ok(());
    }
    let unlisted: Vec<&String> = actual.iter().filter(|f| !pinned.contains(f)).collect();
    let missing: Vec<&String> = pinned.iter().filter(|f| !actual.contains(f)).collect();
    Err(format!(
        "COURT_INVENTORY_DRIFT: unlisted test files {unlisted:?}; pinned-but-missing {missing:?}"
    ))
}

// ---------------------------------------------------------------------------
// Real-subject courts
// ---------------------------------------------------------------------------

#[test]
fn both_pin_mechanisms_agree_on_real_ontology() {
    let ont = std::fs::read_to_string(repo_path("ontology/ferroplan-wasm.ttl"))
        .expect("read ontology/ferroplan-wasm.ttl");
    let pin = check_mechanisms(&ont).unwrap_or_else(|e| panic!("meta drift: {e}"));
    // Cross-check against the committed registry projection, as pin_drift does.
    let reg = std::fs::read_to_string(repo_path("crates/ferroplan-wasm/registry/ARTIFACTS.sha256"))
        .expect("read registry/ARTIFACTS.sha256");
    let wasm_line = reg
        .lines()
        .map(str::trim)
        .find(|l| l.split_whitespace().next() == Some("wasm"))
        .expect("registry carries a wasm line");
    let f: Vec<&str> = wasm_line.split_whitespace().collect();
    assert_eq!(f[1], pin.0, "registry sha != ontology pin");
    assert_eq!(f[2], pin.1.to_string(), "registry bytes != ontology pin");
}

#[test]
fn test_dir_inventory_matches_pinned_court_list() {
    let dir = repo_path("crates/ferroplan-wasm/tests");
    let mut actual: Vec<String> = std::fs::read_dir(&dir)
        .expect("read tests dir")
        .map(|e| e.expect("dir entry").file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".rs"))
        .collect();
    check_inventory(&mut actual).unwrap_or_else(|e| panic!("{e}"));
}

// ---------------------------------------------------------------------------
// Fixtures + mutation self-tests (non-vacuity)
// ---------------------------------------------------------------------------

const ONT: &str = "x\n    wja:wasmSha256 \"088d9c3b0306e36123ddc1ee780ad7e9d4bd2ebb54f9726c43f40a2f6d718233\" ;\n    wja:wasmBytes 3521589 ;\n";

#[test]
fn baseline_fixture_passes() {
    assert_eq!(
        check_mechanisms(ONT).unwrap(),
        (
            "088d9c3b0306e36123ddc1ee780ad7e9d4bd2ebb54f9726c43f40a2f6d718233".to_owned(),
            3521589
        )
    );
}

#[test]
fn mutation_duplicated_sha_refused() {
    // Strict parser refuses two facts; court must fail even though the
    // justfile-style head-1 extractor would happily take the first.
    let m = format!("{ONT}    wja:wasmSha256 \"deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef\" ;\n");
    assert!(check_mechanisms(&m).is_err());
}

#[test]
fn mutation_edited_sha_still_agrees_but_registry_refused() {
    // A consistent edit of BOTH mechanisms agrees mechanically; the real court
    // above additionally binds to ARTIFACTS.sha256, which catches it there.
    let m = ONT.replace("088d9c3b", "188d9c3b");
    assert!(check_mechanisms(&m).is_ok(), "mechanisms stay consistent");
}

#[test]
fn mutation_missing_sha_refused() {
    let m = ONT.replace("wja:wasmSha256", "wja:otherProp");
    assert!(check_mechanisms(&m).is_err());
}

#[test]
fn mutation_missing_bytes_refused() {
    let m = ONT.replace("wja:wasmBytes", "wja:otherProp");
    assert!(check_mechanisms(&m).is_err());
}

#[test]
fn mutation_justfile_extractor_greedy_prefix_refused() {
    // If a second prefix-bearing line appears where the strict parser would
    // error, the court fails — no silent head-1 divergence.
    let m = format!("{ONT}    wja:wasmBytes 999 ;\n");
    assert!(check_mechanisms(&m).is_err());
}

#[test]
fn inventory_mutation_unlisted_file_refused() {
    let mut actual: Vec<String> = PINNED_COURT_FILES
        .iter()
        .map(|s| s.to_string())
        .chain(["sneaky_court.rs".to_owned()])
        .collect();
    assert!(check_inventory(&mut actual).is_err());
}

#[test]
fn inventory_mutation_missing_file_refused() {
    let mut actual: Vec<String> = PINNED_COURT_FILES
        .iter()
        .filter(|s| **s != "pin_drift.rs")
        .map(|s| s.to_string())
        .collect();
    assert!(check_inventory(&mut actual).is_err());
}
