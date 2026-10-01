//! Copy-drift court: local copies must stay byte-identical to their sources.
//!
//! 1. `ontology/contract.ttl` == `/ontology/ferroplan-host-contract.ttl`.
//! 2. `beam-host-templates/*.tmpl` and `qri-ontology.ttl` == the pack sources
//!    in `ggen-marketplace/packs/qri-qualification-profile-pack` (skipped with
//!    a printed reason when the pack is absent), EXCEPT names listed in
//!    `beam-host-templates/DIVERGENCE.md`: those must differ from the pack and be listed.
//!
//! Std only. Checkers are pure functions over bytes/text; mutation self-tests
//! prove the court refuses.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const PACK_REL: &str = "../../../ggen-marketplace/packs/qri-qualification-profile-pack";
const DIVERGENCE_POLL: Duration = Duration::from_secs(600);

fn crate_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Names (`*.tmpl` or `qri-ontology.ttl`) mentioned in DIVERGENCE.md text.
fn divergence_names(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !(c.is_alphanumeric() || matches!(c, '.' | '_' | '-')))
        .map(|t| t.trim_matches('.'))
        .filter(|t| t.ends_with(".tmpl") || *t == "qri-ontology.ttl")
        .map(str::to_owned)
        .collect()
}

/// One copy under test: name, local bytes, source bytes.
struct Pair {
    name: String,
    local: Vec<u8>,
    source: Vec<u8>,
}

/// Names whose local bytes differ from the source.
fn differing(pairs: &[Pair]) -> BTreeSet<String> {
    pairs
        .iter()
        .filter(|p| p.local != p.source)
        .map(|p| p.name.clone())
        .collect()
}

/// Pure court: differing set must equal the listed set (restricted to names
/// that are under test; listing an unknown name is itself a refusal).
fn check_copies(pairs: &[Pair], listed: &BTreeSet<String>) -> Result<(), String> {
    let known: BTreeSet<String> = pairs.iter().map(|p| p.name.clone()).collect();
    let diff = differing(pairs);
    let mut errs = Vec::new();
    for n in diff.difference(listed) {
        errs.push(format!(
            "{n}: differs from pack but not listed in DIVERGENCE.md"
        ));
    }
    for n in listed.difference(&diff) {
        if known.contains(n) {
            errs.push(format!(
                "{n}: listed in DIVERGENCE.md but identical to pack"
            ));
        } else {
            errs.push(format!(
                "{n}: listed in DIVERGENCE.md but is not a tracked copy"
            ));
        }
    }
    if errs.is_empty() {
        Ok(())
    } else {
        Err(errs.join("\n"))
    }
}

fn read_bytes(p: &Path) -> Vec<u8> {
    std::fs::read(p).unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()))
}

#[test]
fn contract_ttl_equals_root_ontology() {
    let local = read_bytes(&crate_path("ontology/contract.ttl"));
    let root = read_bytes(&crate_path("../../ontology/ferroplan-host-contract.ttl"));
    assert!(
        local == root,
        "ontology/contract.ttl drifted from /ontology/ferroplan-host-contract.ttl"
    );
}

#[test]
fn pack_copies_match_except_divergence() {
    let pack = crate_path(PACK_REL);
    if !pack.is_dir() {
        eprintln!("SKIP: pack source absent at {}", pack.display());
        return;
    }
    let mut pairs = Vec::new();
    let mut tmpls: Vec<PathBuf> = std::fs::read_dir(crate_path("beam-host-templates"))
        .expect("read beam-host-templates")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "tmpl"))
        .collect();
    tmpls.sort();
    for t in &tmpls {
        let name = t.file_name().unwrap().to_string_lossy().into_owned();
        let src = pack.join("templates/beam-host").join(&name);
        assert!(src.is_file(), "{name}: no pack source at {}", src.display());
        pairs.push(Pair {
            name,
            local: read_bytes(t),
            source: read_bytes(&src),
        });
    }
    pairs.push(Pair {
        name: "qri-ontology.ttl".into(),
        local: read_bytes(&crate_path("qri-ontology.ttl")),
        source: read_bytes(&pack.join("ontology.ttl")),
    });

    // Canonical home is beam-host-templates/DIVERGENCE.md; crate root accepted too.
    let div_candidates = [
        crate_path("beam-host-templates/DIVERGENCE.md"),
        crate_path("DIVERGENCE.md"),
    ];
    let found = || div_candidates.iter().find(|p| p.is_file()).cloned();
    // Only wait for DIVERGENCE.md when something actually differs.
    if !differing(&pairs).is_empty() {
        let t0 = Instant::now();
        while found().is_none() && t0.elapsed() < DIVERGENCE_POLL {
            std::thread::sleep(Duration::from_secs(5));
        }
    }
    let listed = match found() {
        Some(p) => divergence_names(&std::fs::read_to_string(&p).expect("read DIVERGENCE.md")),
        None => BTreeSet::new(),
    };
    if let Err(e) = check_copies(&pairs, &listed) {
        panic!("copy drift:\n{e}");
    }
}

// ---- mutation self-tests -------------------------------------------------

fn pair(name: &str, local: &str, source: &str) -> Pair {
    Pair {
        name: name.into(),
        local: local.into(),
        source: source.into(),
    }
}

fn set(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|s| s.to_string()).collect()
}

#[test]
fn divergence_names_parses_tokens() {
    let t = "- `host.ex.tmpl`: diverges\n* pool.ex.tmpl, and qri-ontology.ttl.\nother.txt";
    assert_eq!(
        divergence_names(t),
        set(&["host.ex.tmpl", "pool.ex.tmpl", "qri-ontology.ttl"])
    );
}

#[test]
fn identical_copies_pass() {
    assert!(check_copies(&[pair("a.tmpl", "x", "x")], &set(&[])).is_ok());
}

#[test]
fn mutated_copy_is_refused() {
    let r = check_copies(&[pair("a.tmpl", "x!", "x")], &set(&[]));
    assert!(r.unwrap_err().contains("a.tmpl: differs"));
}

#[test]
fn listed_divergence_must_actually_differ() {
    let r = check_copies(&[pair("a.tmpl", "x", "x")], &set(&["a.tmpl"]));
    assert!(r.unwrap_err().contains("identical to pack"));
}

#[test]
fn listed_divergence_that_differs_passes() {
    assert!(check_copies(&[pair("a.tmpl", "y", "x")], &set(&["a.tmpl"])).is_ok());
}

#[test]
fn listing_untracked_name_is_refused() {
    let r = check_copies(&[pair("a.tmpl", "x", "x")], &set(&["zzz.tmpl"]));
    assert!(r.unwrap_err().contains("not a tracked copy"));
}
