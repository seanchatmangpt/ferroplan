use ferroplan_runtime::{coordinator::Coordinator, edge::Edge, graph::Graph};
#[test]
fn coordinator_reselects() {
    let mut c = Coordinator {
        graph: Graph {
            edges: vec![
                Edge {
                    id: "e0".into(),
                    provider: "p0".into(),
                    enabled: true,
                },
                Edge {
                    id: "e1".into(),
                    provider: "p1".into(),
                    enabled: true,
                },
            ],
        },
        excluded: vec![],
    };
    c.fail("e0");
    assert_eq!(c.next_edge(), Some("e1"));
}
