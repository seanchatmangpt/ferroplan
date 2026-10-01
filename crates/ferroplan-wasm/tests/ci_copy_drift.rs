//! CI-ontology copy-drift court: `scripts/ci_gen/ontology/ferroplan-ci.ttl` must stay
//! byte-identical to `ontology/ferroplan-ci.ttl` (ci_gen.sh --check also refuses).
//! Std only; the checker is a pure function with a mutation self-test.

use std::path::PathBuf;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

/// Err(typed message) when the copy differs from the source.
fn check_copy(source: &[u8], copy: &[u8]) -> Result<(), String> {
    if source == copy {
        Ok(())
    } else {
        Err(format!(
            "CI_ONTOLOGY_COPY_DRIFT: copy ({} bytes) != source ({} bytes)",
            copy.len(),
            source.len()
        ))
    }
}

#[test]
fn ci_ontology_copy_is_byte_identical() {
    let src = std::fs::read(repo_path("ontology/ferroplan-ci.ttl")).expect("read source");
    let cpy =
        std::fs::read(repo_path("scripts/ci_gen/ontology/ferroplan-ci.ttl")).expect("read copy");
    check_copy(&src, &cpy).unwrap();
}

#[test]
fn mutated_copy_is_refused() {
    let src = std::fs::read(repo_path("ontology/ferroplan-ci.ttl")).expect("read source");
    let mut m = src.clone();
    m.push(b'\n');
    assert!(check_copy(&src, &m)
        .unwrap_err()
        .starts_with("CI_ONTOLOGY_COPY_DRIFT"));
    assert!(check_copy(&src, &src[..src.len() - 1]).is_err());
}
