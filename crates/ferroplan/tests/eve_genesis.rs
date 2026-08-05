use ferroplan::{
    Activator, CapabilityTarget, Eve, EveError, EveRequest, EveStage, GenesisWorld, HddlSurface,
    HumanPurpose, ManufactureTarget, PlanningRegime, PpddlSurface, MAX_PRIMARY_ACTIVATORS,
};

fn request(ppddl: bool) -> EveRequest {
    EveRequest {
        purpose: HumanPurpose {
            statement: "Deploy the service safely".to_string(),
            desired_consequence: "A replayable admitted deployment".to_string(),
            actor: Some("operator".to_string()),
            activators: vec![Activator {
                name: "environment".to_string(),
                value: "production".to_string(),
            }],
        },
        genesis: GenesisWorld {
            ontology_rdf: "@prefix fp: <urn:ferroplan:> .".to_string(),
            construct_query: "CONSTRUCT { ?s ?p ?o } WHERE { ?s ?p ?o }".to_string(),
            hddl: HddlSurface {
                domain: "(define (domain deploy) (:task deploy-service))".to_string(),
                problem: "(define (problem deploy-prod) (:domain deploy))".to_string(),
                root_task: "(deploy-service production)".to_string(),
            },
            ppddl: ppddl.then(|| PpddlSurface {
                domain: "(define (domain uncertain-deploy) (:requirements :probabilistic-effects))"
                    .to_string(),
                problem: "(define (problem uncertain-prod) (:domain uncertain-deploy))"
                    .to_string(),
            }),
        },
        manufacture: ManufactureTarget {
            name: "deploy-service.part".to_string(),
            template: "ggen://truex/part".to_string(),
            artifact_kind: ".part.wasm".to_string(),
            output: "target/parts/deploy-service.part.wasm".to_string(),
        },
        capability: CapabilityTarget {
            capability: "deploy-service".to_string(),
            route: "powl://deploy-service/v1".to_string(),
            authority_scopes: vec!["production:deploy".to_string()],
        },
    }
}

#[test]
fn deterministic_world_compiles_full_receipted_handoff() {
    let handoff = Eve::enter(request(false)).expect("valid deterministic handoff");

    assert_eq!(handoff.planning_regime, PlanningRegime::Deterministic);
    assert_eq!(handoff.protocol, "ferroplan.eve-genesis.v1");
    assert!(handoff.closure_id.starts_with("eve:"));
    assert!(handoff.ppddl.is_none());
    assert!(!handoff.stages.contains(&EveStage::GovernUncertaintyPpddl));
    assert_eq!(handoff.stages.first(), Some(&EveStage::GroundHumanPurpose));
    assert_eq!(handoff.stages.last(), Some(&EveStage::ReplayTruex));
    assert!(handoff.ggen.candidate_only);
    assert!(!handoff.mcp_plus.ambient_authority);
    assert!(handoff.mcp_plus.brce_required);
    assert!(handoff.truex.replay_required);
    assert_eq!(handoff.truex.expected_process_geometry, "POWL-v2");
    assert_eq!(handoff.truex.observed_path_format, "OCEL-2.0");
}

#[test]
fn probabilistic_world_inserts_ppddl_before_manufacture() {
    let handoff = Eve::enter(request(true)).expect("valid probabilistic handoff");

    assert_eq!(handoff.planning_regime, PlanningRegime::Probabilistic);
    assert!(handoff.ppddl.is_some());
    let ppddl = handoff
        .stages
        .iter()
        .position(|stage| *stage == EveStage::GovernUncertaintyPpddl)
        .unwrap();
    let ggen = handoff
        .stages
        .iter()
        .position(|stage| *stage == EveStage::ManufactureGgen)
        .unwrap();
    assert!(ppddl < ggen);
}

#[test]
fn identical_inputs_produce_identical_closure_identity() {
    let left = Eve::enter(request(false)).unwrap();
    let right = Eve::enter(request(false)).unwrap();
    assert_eq!(left.closure_id, right.closure_id);
}

#[test]
fn need9_refuses_and_returns_deterministic_split() {
    let mut input = request(false);
    input.purpose.activators = (0..=MAX_PRIMARY_ACTIVATORS)
        .map(|index| Activator {
            name: format!("condition-{index}"),
            value: "active".to_string(),
        })
        .collect();

    let error = Eve::enter(input).expect_err("ninth activator must split");
    match error {
        EveError::SplitRequired { directive } => {
            assert_eq!(directive.provided, 9);
            assert_eq!(directive.maximum, 8);
            assert_eq!(directive.groups.len(), 2);
            assert_eq!(directive.groups[0].len(), 8);
            assert_eq!(directive.groups[1].len(), 1);
        }
        other => panic!("unexpected refusal: {other:?}"),
    }
}

#[test]
fn missing_world_refuses_before_handoff() {
    let mut input = request(false);
    input.genesis.ontology_rdf.clear();

    assert_eq!(
        Eve::enter(input),
        Err(EveError::Missing {
            field: "genesis.ontology_rdf".to_string(),
        })
    );
}

#[test]
fn handoff_round_trips_as_structured_json() {
    let handoff = Eve::enter(request(true)).unwrap();
    let encoded = serde_json::to_string(&handoff).unwrap();
    let decoded = serde_json::from_str(&encoded).unwrap();
    assert_eq!(handoff, decoded);
}
