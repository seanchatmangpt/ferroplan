use ferroplan_runtime::{edge::Edge, graph::Graph, health::Health, reconcile};
#[test]
fn open_provider_removed() {
    let mut g = Graph {
        edges: vec![Edge {
            id: "e".into(),
            provider: "p".into(),
            enabled: true,
        }],
    };
    reconcile::reconcile(&mut g, &[("e".into(), Health::Open)]);
    assert_eq!(g.lawful().count(), 0);
}
