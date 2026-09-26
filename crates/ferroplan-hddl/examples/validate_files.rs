//! HDDL file gate: parse a domain + problem pair from disk, run the static
//! well-formedness checks, and ground the domain's methods over the
//! problem's objects. Exit 0 with a one-line summary when the pair is
//! admissible; exit 2 with the failing stage and message otherwise
//! (including a missing/unreadable file or a wrong argument count).
//!
//! This is the check a generator of HDDL (e.g. a doctrine -> HDDL renderer)
//! runs on its output before handing it to a planner. It proves parse,
//! validation and method grounding only; it does not solve.
//!
//! Run: `cargo run -q -p ferroplan-hddl --example validate_files -- \
//!   <domain.hddl> <problem.hddl>`

use ferroplan_hddl::grounder::{
    build_type_closure, ground_methods, index_objects_by_type, GroundingLimits,
};
use ferroplan_hddl::parser::{parse_domain, parse_problem};
use ferroplan_hddl::validate::{validate_domain, validate_problem};

/// What an admitted pair looks like, for the success line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub domain: String,
    pub problem: String,
    pub actions: usize,
    pub tasks: usize,
    pub methods: usize,
    pub ground_methods: usize,
}

/// Parse, validate, and ground methods. `Err` names the failing stage.
pub fn validate_pair(domain_src: &str, problem_src: &str) -> Result<Summary, String> {
    let domain = parse_domain(domain_src).map_err(|e| format!("domain parse: {e}"))?;
    let problem = parse_problem(problem_src).map_err(|e| format!("problem parse: {e}"))?;
    validate_domain(&domain).map_err(|e| format!("domain validation: {e}"))?;
    validate_problem(&domain, &problem).map_err(|e| format!("problem validation: {e}"))?;
    let closure = build_type_closure(&domain).map_err(|e| format!("type closure: {e}"))?;
    let objects = index_objects_by_type(&domain, &problem, &closure);
    let grounded = ground_methods(&domain, &objects, &GroundingLimits::default())
        .map_err(|e| format!("method grounding: {e}"))?;
    Ok(Summary {
        domain: domain.name.to_string(),
        problem: problem.name.to_string(),
        actions: domain.actions.len(),
        tasks: domain.tasks.len(),
        methods: domain.methods.len(),
        ground_methods: grounded.len(),
    })
}

/// Read both files and validate; `Err` names the file or the stage.
pub fn validate_paths(domain_path: &str, problem_path: &str) -> Result<Summary, String> {
    let domain_src =
        std::fs::read_to_string(domain_path).map_err(|e| format!("read {domain_path}: {e}"))?;
    let problem_src =
        std::fs::read_to_string(problem_path).map_err(|e| format!("read {problem_path}: {e}"))?;
    validate_pair(&domain_src, &problem_src)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("usage: validate_files <domain.hddl> <problem.hddl>");
        std::process::exit(2);
    }
    match validate_paths(&args[0], &args[1]) {
        Ok(s) => {
            println!(
                "ok domain={} problem={} actions={} tasks={} methods={} ground_methods={}",
                s.domain, s.problem, s.actions, s.tasks, s.methods, s.ground_methods
            );
        }
        Err(message) => {
            eprintln!("refused: {message}");
            std::process::exit(2);
        }
    }
}
