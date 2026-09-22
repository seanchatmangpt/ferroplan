//! Durable homes for the external oracle harness and the external corpora.
//!
//! The deep-lane tests used to hardcode `/tmp/fond-oracle` and
//! `/tmp/fond-review`; those homes were wiped on every reap, which is why
//! fond-htn-60/61/66 stand PARTIAL_ALIVE with EXTERNAL_ABSENT reruns. Since
//! FERROPLAN-26922-02 the locations resolve as:
//!
//! * oracle harness root — `$FERROPLAN_ORACLE_DIR`, defaulting to
//!   `~/.cache/ferroplan` (the runner script is `<root>/oracle-run.sh`);
//! * external corpus root — `$FERROPLAN_CORPUS_DIR`, defaulting to
//!   `~/.cache/ferroplan` (the IPC corpus is
//!   `<root>/HDDL-Parser/tests/ipc`);
//! * differential scratch/cache — `$FERROPLAN_RUN_DIR`, defaulting to
//!   `~/.cache/ferroplan/differential-fuzz-w61`.
//!
//! Every consumer of this module SKIPS BY NAME (loud note, exit 0) when its
//! external root is absent, so the deep lane can pass on a bare CI runner
//! while still running for real wherever the harness/corpus is installed.

#![allow(dead_code)] // shared helper: each consumer target uses a subset

use std::path::{Path, PathBuf};

fn cache_root() -> PathBuf {
    match std::env::var_os("HOME") {
        Some(home) => PathBuf::from(home).join(".cache").join("ferroplan"),
        None => PathBuf::from(".cache").join("ferroplan"),
    }
}

/// Root of the koala oracle harness (`oracle-run.sh` lives directly inside).
pub fn oracle_dir() -> PathBuf {
    match std::env::var_os("FERROPLAN_ORACLE_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => cache_root(),
    }
}

/// Root of the external corpora (`HDDL-Parser/tests/ipc` lives inside).
pub fn corpus_dir() -> PathBuf {
    match std::env::var_os("FERROPLAN_CORPUS_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => cache_root(),
    }
}

/// The IPC-2023 hierarchical-track corpus used by the grounding/caps reruns.
pub fn corpus_ipc_dir() -> PathBuf {
    corpus_dir().join("HDDL-Parser").join("tests").join("ipc")
}

/// Full path of the flock-serialized oracle runner script.
pub fn oracle_runner() -> PathBuf {
    oracle_dir().join("oracle-run.sh")
}

/// Scratch + verdict-cache directory for the differential fuzz driver.
pub fn differential_run_dir() -> PathBuf {
    match std::env::var_os("FERROPLAN_RUN_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => cache_root().join("differential-fuzz-w61"),
    }
}

/// Skip-by-name guard: loud note plus `false` when `path` is absent.
pub fn harness_present(what: &str, path: &Path) -> bool {
    if path.exists() {
        true
    } else {
        println!(
            "SKIP-BY-NAME: {what} not found at {} — populate it (or set \
             FERROPLAN_ORACLE_DIR / FERROPLAN_CORPUS_DIR, default \
             ~/.cache/ferroplan) and re-run; the deep lane skips cleanly \
             without it",
            path.display()
        );
        false
    }
}
