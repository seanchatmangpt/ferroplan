//! Python bindings — the engine's voice piped through pyo3, for anyone
//! working the console in a snake-tongue dialect.
//!
//! ```python
//! import ferroplan, json
//! sol = json.loads(ferroplan.plan(domain_pddl, problem_pddl))
//! print(sol["solved"], sol["plan"]["length"])
//! ```
//!
//! Build: `pip install maturin && maturin develop` in this crate, or
//! `maturin build --release` to cut a wheel and walk away.

use ferroplan_core::{solve, Mode, Options};
use pyo3::prelude::*;

/// Feed it a domain and a problem; it comes back with a `Solution` — JSON,
/// crack it open with `json.loads` — or `{"error": "..."}` if the parse or
/// the search went bad.
///
/// `mode` ∈ "auto" | "ff" | "pddl3" | "partition" | "temporal" (default
/// "auto" — let the engine read the shape). `threads` defaults to the
/// planner's own headcount; pass 1 to pin it down, one lane, deterministic.
#[pyfunction]
#[pyo3(signature = (domain, problem, mode=None, threads=None))]
fn plan(domain: &str, problem: &str, mode: Option<&str>, threads: Option<usize>) -> String {
    let mut opts = Options {
        mode: parse_mode(mode),
        ..Default::default()
    };
    if let Some(t) = threads {
        opts.threads = t;
    }
    match solve(domain, problem, &opts) {
        Ok(sol) => {
            serde_json::to_string(&sol).unwrap_or_else(|e| err_json(&format!("serialize: {e}")))
        }
        Err(e) => err_json(&e.to_string()),
    }
}

/// The build's serial number.
#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn parse_mode(m: Option<&str>) -> Mode {
    match m.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("ff") => Mode::Ff,
        Some("pddl3") => Mode::Pddl3,
        Some("partition") => Mode::Partition,
        Some("temporal") => Mode::Temporal,
        _ => Mode::Auto,
    }
}

fn err_json(msg: &str) -> String {
    serde_json::json!({ "error": msg }).to_string()
}

#[pymodule]
fn ferroplan(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(plan, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
