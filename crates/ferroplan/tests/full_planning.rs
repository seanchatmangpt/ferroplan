use ferroplan::{
    bind_policy_receipt, plan_full, verify_policy, verify_policy_chain, verify_policy_receipt,
    FullPlanningRequest, FullPlanningResult, PolicySession, PolicySessionError,
    PolicySessionPhase, ProbabilisticObjective, ProbabilisticOptions, RiskConstraint,
};

const DOMAIN: &str = r#"
(define (domain retry)
  (:requirements :strips :negative-preconditions :probabilistic-effects)
  (:predicates (done) (unsafe))
  (:action attempt
    :parameters ()
    :precondition (not (done))
    :effect (probabilistic 0.5 (done) 0.5 (and))))
"#;

const PROBLEM: &str = r#"
(define (problem retry-p)
  (:domain retry)
  (:init)
  (:goal (done)))
"#;

#[test]
fn full_planning_dispatches_probabilistic_and_receipts_it() {
    let result = plan_full(FullPlanningRequest::Probabilistic {
        domain: DOMAIN.into(),
        problem: PROBLEM.into(),
        options: ProbabilisticOptions {
            horizon: Some(2),
            objective: ProbabilisticObjective::MaximizeGoalProbability,
            ..Default::default()
        },
        constraints: vec![RiskConstraint::MinimumGoalProbability(0.75)],
        unsafe_fact: None,
    })
    .unwrap();
    let FullPlanningResult::Probabilistic {
        solution,
        verification,
        receipt,
    } = result
    else {
        panic!("wrong planning rail")
    };
    assert!(solution.solved);
    assert!(verification.valid, "{:?}", verification.counterexamples);
    assert!(verify_policy_receipt(&receipt));
}

#[test]
fn hard_constraint_blocks_probability_overclaim() {
    let options = ProbabilisticOptions {
        horizon: Some(1),
        objective: ProbabilisticObjective::MaximizeGoalProbability,
        ..Default::default()
    };
    let solution = ferroplan::solve_ppddl(DOMAIN, PROBLEM, &options).unwrap();
    let report = verify_policy(
        DOMAIN,
        PROBLEM,
        &options,
        &solution,
        &[RiskConstraint::MinimumGoalProbability(0.9)],
        None,
    )
    .unwrap();
    assert!(!report.valid);
    assert!(report.constraints.iter().any(|verdict| !verdict.satisfied));
}

#[test]
fn policy_session_decides_and_advances_only_to_admitted_successor() {
    let options = ProbabilisticOptions {
        horizon: Some(2),
        objective: ProbabilisticObjective::MaximizeGoalProbability,
        ..Default::default()
    };
    let mut session = PolicySession::new(DOMAIN, PROBLEM, options, vec![], None).unwrap();
    assert_eq!(session.status().phase, PolicySessionPhase::Ready);
    let decision = session.decide().unwrap().expect("non-terminal decision");
    let admitted = decision.outcomes[0].next_state;
    session.mark_awaiting_observation().unwrap();
    session.advance(admitted).unwrap();
    assert_eq!(session.status().phase, PolicySessionPhase::Ready);

    let mut session = session.fork();
    let decision = session.decide().unwrap();
    if decision.is_some() {
        let error = session.advance(usize::MAX).unwrap_err();
        assert!(matches!(error, PolicySessionError::OutcomeNotAdmitted { .. }));
        assert_eq!(session.status().phase, PolicySessionPhase::Drifted);
    }
}

#[test]
fn stochastic_initial_state_requires_observation() {
    let problem = r#"
(define (problem retry-p)
  (:domain retry)
  (:init (oneof (done) (and)))
  (:goal (done)))
"#;
    let session = PolicySession::new(
        DOMAIN,
        problem,
        ProbabilisticOptions::default(),
        vec![],
        None,
    )
    .unwrap();
    assert_eq!(session.status().phase, PolicySessionPhase::ObservationRequired);
}

#[test]
fn receipt_chain_binds_predecessors_and_refuses_tampering() {
    let options = ProbabilisticOptions {
        horizon: Some(2),
        ..Default::default()
    };
    let solution = ferroplan::solve_ppddl(DOMAIN, PROBLEM, &options).unwrap();
    let verification = verify_policy(DOMAIN, PROBLEM, &options, &solution, &[], None).unwrap();
    let first = bind_policy_receipt(
        DOMAIN,
        PROBLEM,
        &options,
        &[],
        &solution,
        &verification,
        None,
    );
    let second = bind_policy_receipt(
        DOMAIN,
        PROBLEM,
        &options,
        &[],
        &solution,
        &verification,
        Some(first.receipt_digest.clone()),
    );
    assert!(verify_policy_chain(&[first.clone(), second.clone()]));
    let mut tampered = second;
    tampered.policy_digest.push('0');
    assert!(!verify_policy_receipt(&tampered));
}
