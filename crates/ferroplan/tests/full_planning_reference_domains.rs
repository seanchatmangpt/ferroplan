use ferroplan::{
    simulate_ppddl, verify_policy, ProbabilisticObjective, ProbabilisticOptions, RiskConstraint,
};

const REPOSITORY_DOMAIN: &str =
    include_str!("../../../examples/repository_uncertainty/domain.pddl");
const REPOSITORY_PROBLEM: &str =
    include_str!("../../../examples/repository_uncertainty/problem.pddl");
const TCPS_DOMAIN: &str = include_str!("../../../examples/tcps_application_pipeline/domain.pddl");
const TCPS_PROBLEM: &str =
    include_str!("../../../examples/tcps_application_pipeline/problem.pddl");

#[test]
fn repository_uncertainty_policy_is_closed_and_replayable() {
    let options = ProbabilisticOptions {
        horizon: Some(5),
        objective: ProbabilisticObjective::MaximizeGoalProbability,
        ..Default::default()
    };
    let solution = ferroplan::solve_ppddl(REPOSITORY_DOMAIN, REPOSITORY_PROBLEM, &options).unwrap();
    let report = verify_policy(
        REPOSITORY_DOMAIN,
        REPOSITORY_PROBLEM,
        &options,
        &solution,
        &[RiskConstraint::MinimumGoalProbability(1.0)],
        None,
    )
    .unwrap();
    assert!(report.valid, "{:?}", report.counterexamples);
    let first = simulate_ppddl(REPOSITORY_DOMAIN, REPOSITORY_PROBLEM, &options, 64, 17).unwrap();
    let second = simulate_ppddl(REPOSITORY_DOMAIN, REPOSITORY_PROBLEM, &options, 64, 17).unwrap();
    assert_eq!(serde_json::to_value(first).unwrap(), serde_json::to_value(second).unwrap());
}

#[test]
fn tcps_pipeline_synthesizes_a_policy_and_exposes_unsafe_risk() {
    let options = ProbabilisticOptions {
        horizon: Some(8),
        objective: ProbabilisticObjective::MaximizeGoalProbability,
        ..Default::default()
    };
    let solution = ferroplan::solve_ppddl(TCPS_DOMAIN, TCPS_PROBLEM, &options).unwrap();
    assert!(solution.solved);
    assert!(!solution.policy.is_empty());
    let report = verify_policy(
        TCPS_DOMAIN,
        TCPS_PROBLEM,
        &options,
        &solution,
        &[RiskConstraint::MaximumUnsafeReachability(1.0)],
        Some("(INCOMPATIBLE)"),
    )
    .unwrap();
    assert!(report.unsafe_probability.lower >= 0.0);
    assert!(report.unsafe_probability.upper <= 1.0);
}
