//! Guard for `domains/solve_x.hddl` / `domains/solve_x.problem.hddl` -- the HDDL form of the SOLVE
//! decomposition (preserve -> fence -> calculus -> exclusions -> falsifier -> extension ->
//! operationalize). Both files are read from disk and run through the real pipeline: parse and
//! validation from `ferroplan_hddl`, then `ferroplan::solve_hddl` (parse -> ground -> translate ->
//! solve). The falsifier mutates a precondition in a scratch copy of the domain text and requires
//! the same pipeline to refuse. No doubles.

use ferroplan::planning_runtime::PlannerLimits;
use ferroplan::solve_hddl;
use ferroplan_hddl::parser::{parse_domain, parse_problem};
use ferroplan_hddl::validate::{validate_domain, validate_problem};

const DOMAIN_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../domains/solve_x.hddl");
const PROBLEM_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../domains/solve_x.problem.hddl"
);

const ORDER: [&str; 7] = [
    "preserve",
    "fence",
    "calculus",
    "exclusions",
    "falsifier",
    "extension",
    "operationalize",
];

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

#[test]
fn solve_x_pair_parses_and_validates() {
    let (d, p) = (read(DOMAIN_PATH), read(PROBLEM_PATH));
    let domain = parse_domain(&d).expect("domain parses");
    let problem = parse_problem(&p).expect("problem parses");
    validate_domain(&domain).expect("domain validates");
    validate_problem(&domain, &problem).expect("problem validates");
    assert_eq!(domain.name.to_string(), "solve-x");
    assert_eq!(domain.actions.len(), 7);
    assert_eq!(domain.methods.len(), 1);
}

/// The executed primitive phases of a FOND-HTN policy, in policy order: every `htn:exec:<id>:<action>`
/// entry, reduced to its action name.
fn executed_phases(plan: &ferroplan::planning_runtime::UniversalPlan) -> Vec<String> {
    plan.policy
        .iter()
        .filter_map(|e| e.action.strip_prefix("htn:exec:"))
        .map(|rest| rest.rsplit(':').next().unwrap_or(rest).to_ascii_lowercase())
        .collect()
}

#[test]
fn solve_x_solves_with_the_seven_phases_in_order() {
    let plan = solve_hddl(
        &read(DOMAIN_PATH),
        &read(PROBLEM_PATH),
        &PlannerLimits::default(),
    )
    .expect("solve-x is solvable");
    assert!(plan.solved, "plan not solved: {plan:?}");
    assert_eq!(executed_phases(&plan), ORDER, "policy: {:?}", plan.policy);
    assert!(
        plan.policy
            .iter()
            .any(|e| e.action.starts_with("htn:decompose:") && e.action.ends_with(":m-solve")),
        "m-solve decomposition missing: {:?}",
        plan.policy
    );
}

#[test]
fn breaking_a_precondition_makes_solve_x_unsolvable() {
    let domain = read(DOMAIN_PATH);
    let broken = domain.replace(
        ":precondition (phase-calculus)",
        ":precondition (phase-preserve)",
    );
    assert_ne!(broken, domain, "mutation must have applied");
    match solve_hddl(&broken, &read(PROBLEM_PATH), &PlannerLimits::default()) {
        Err(_) => {}
        Ok(plan) => assert!(
            !plan.solved || executed_phases(&plan) != ORDER,
            "mutated domain still solved in the full order: {plan:?}"
        ),
    }
}
