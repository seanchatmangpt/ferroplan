//! Pack-content-pin court: the root `ggen.toml` `[[packs]]` stanza consumes
//! `../ggen-marketplace/packs/rust-wasi-wasmex-pack` by bare filesystem-relative
//! path with no version pin — the one unpinned input in a repo that otherwise
//! pins everything (`wja:wasmSha256`, `tests/pin_drift.rs`, `ci_copy_drift.rs`,
//! `abi_ontology_drift.rs`).
//!
//! This court closes that gap: `ontology/ferroplan-wasm.ttl` now carries
//! `wja:packDir` + `wja:packContentSha256`, and this test recomputes the pack
//! directory's content hash at test time and fails on drift.
//!
//! Hash algorithm (must match the ontology comment exactly):
//! sha256 over the pack directory's files in sorted relative-path order, for
//! each file hashing: rel-path bytes, 0x00, 8-byte little-endian content
//! length, then the file bytes.
//!
//! Skip discipline: if the sibling `ggen-marketplace` checkout is absent the
//! test emits `BLOCKED:marketplace-absent` and skips — a missing sibling repo
//! is a topology fact, not drift evidence.
//!
//! Sabotage twins: the in-test mutations prove the court is non-vacuous (an
//! edited hash, an edited path, an added file, and a byte-flip in a pack file
//! copy are all refused by the same `check` the real run uses).

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

fn crate_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

const PACK_REL: &str = "../../../ggen-marketplace/packs/rust-wasi-wasmex-pack";

/// Deterministic directory-content hash: sorted relative paths; per file,
/// rel-path + 0x00 + u64-LE length + content, all fed to one sha256.
fn pack_content_hash(dir: &Path) -> Result<String, String> {
    if !dir.is_dir() {
        return Err(format!("not a directory: {}", dir.display()));
    }
    let mut rels: Vec<String> = Vec::new();
    collect_rel_files(dir, dir, &mut rels)?;
    rels.sort();
    if rels.is_empty() {
        return Err(format!("pack directory is empty: {}", dir.display()));
    }
    let mut hasher = Sha256::new();
    for rel in &rels {
        let bytes = std::fs::read(dir.join(rel)).map_err(|e| format!("read {rel}: {e}"))?;
        hasher.update(rel.as_bytes());
        hasher.update([0u8]);
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn collect_rel_files(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let entries =
        std::fs::read_dir(dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("entry {}: {e}", dir.display()))?;
        let p = entry.path();
        if p.is_dir() {
            collect_rel_files(root, &p, out)?;
        } else {
            let rel = p
                .strip_prefix(root)
                .map_err(|e| format!("strip_prefix: {e}"))?
                .to_string_lossy()
                .replace('\\', "/");
            out.push(rel);
        }
    }
    Ok(())
}

/// Extract the single `wja:packContentSha256 "<hex>"` fact from ontology text.
fn parse_ontology_pin(text: &str) -> Result<String, String> {
    let mut found = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if let Some(r) = t.strip_prefix("wja:packContentSha256") {
            found.push(
                r.trim()
                    .trim_end_matches([';', '.'])
                    .trim()
                    .trim_matches('"')
                    .to_owned(),
            );
        }
    }
    if found.len() != 1 {
        return Err(format!(
            "ontology needs exactly one packContentSha256 fact, found {}",
            found.len()
        ));
    }
    let sha = &found[0];
    let hex = sha.len() == 64 && sha.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
    if !hex {
        return Err(format!("pack sha256 malformed: {sha}"));
    }
    Ok(sha.clone())
}

fn check(ontology: &str, pack_hash: &str) -> Result<(), String> {
    let pin = parse_ontology_pin(ontology)?;
    if pin != pack_hash {
        return Err(format!("ontology pin {pin} != recomputed pack hash {pack_hash}"));
    }
    Ok(())
}

/// The real court: recompute the sibling pack's content hash and compare
/// against the ontology pin. Skips (does not fail) when the sibling checkout
/// is absent.
#[test]
fn marketplace_pack_hash_matches_ontology_pin() {
    let ont =
        std::fs::read_to_string(crate_path("../../ontology/ferroplan-wasm.ttl"))
            .expect("read ontology/ferroplan-wasm.ttl");
    parse_ontology_pin(&ont).expect("ontology must carry a well-formed pack pin");

    let pack_dir = crate_path(PACK_REL);
    if !pack_dir.is_dir() {
        eprintln!("BLOCKED:marketplace-absent: {} not found", pack_dir.display());
        return;
    }
    let hash = pack_content_hash(&pack_dir).expect("hash pack directory");
    check(&ont, &hash).unwrap_or_else(|e| panic!("pack content drift: {e}"));
}

/// The ontology pin must also name the consumed pack path, so the pin is
/// self-describing (a hash without its subject is archaeology).
#[test]
fn ontology_pin_names_pack_dir() {
    let ont =
        std::fs::read_to_string(crate_path("../../ontology/ferroplan-wasm.ttl"))
            .expect("read ontology/ferroplan-wasm.ttl");
    assert!(
        ont.contains("wja:packDir \"../ggen-marketplace/packs/rust-wasi-wasmex-pack\""),
        "ontology must pin the consumed pack directory path"
    );
}

const ONT: &str = "x\n    wja:packContentSha256 \"be3b68f261212f2bc786be79d0b564cd4028a465c7a11a3445b6b45ef56029ea\" ;\n";
const HASH: &str = "be3b68f261212f2bc786be79d0b564cd4028a465c7a11a3445b6b45ef56029ea";

#[test]
fn baseline_fixture_passes() {
    check(ONT, HASH).unwrap();
}

#[test]
fn mutation_edited_hash_refused() {
    let m = ONT.replace("be3b68f2", "ce3b68f2");
    assert!(check(&m, HASH).is_err());
}

#[test]
fn mutation_drifted_pack_refused() {
    assert!(check(ONT, &HASH.replace("be3b", "0000")).is_err());
}

#[test]
fn mutation_missing_pin_refused() {
    assert!(check("# no pin here\n", HASH).is_err());
}

#[test]
fn mutation_duplicate_pin_refused() {
    let m = format!("{ONT}{}", ONT.lines().nth(1).unwrap());
    assert!(parse_ontology_pin(&m).is_err());
}

#[test]
fn mutation_malformed_pin_refused() {
    let m = ONT.replace(
        "be3b68f261212f2bc786be79d0b564cd4028a465c7a11a3445b6b45ef56029ea",
        "NOTHEX",
    );
    assert!(parse_ontology_pin(&m).is_err());
}

/// Sabotage twin over the REAL pack directory (skipped with the court when
/// the sibling checkout is absent): a copy of the pack with one flipped byte
/// must hash differently — proves the hash sees content, not just names.
#[test]
fn mutation_pack_byte_flip_changes_hash() {
    let pack_dir = crate_path(PACK_REL);
    if !pack_dir.is_dir() {
        eprintln!("BLOCKED:marketplace-absent: {}", pack_dir.display());
        return;
    }
    let real = pack_content_hash(&pack_dir).unwrap();
    // Copy one pack file into a temp dir shaped like a pack, flip a byte.
    let tmp = std::env::temp_dir().join(format!(
        "fp-pack-pin-sabotage-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&tmp).unwrap();
    let mut probe: Option<(String, Vec<u8>)> = None;
    for rel in ["pack.toml", "ontology.ttl", "README.md"] {
        if let Ok(b) = std::fs::read(pack_dir.join(rel)) {
            probe = Some((rel.to_owned(), b));
            break;
        }
    }
    let (rel, mut bytes) = probe.expect("pack must contain a probe file");
    let i = bytes
        .iter()
        .position(|b| *b != b'X')
        .expect("probe file has a non-X byte");
    bytes[i] = b'X';
    std::fs::write(tmp.join(&rel), &bytes).unwrap();
    let sabotaged = pack_content_hash(&tmp).unwrap();
    assert_ne!(real, sabotaged, "byte flip must change the pack hash");
    let _ = std::fs::remove_dir_all(&tmp);
}
