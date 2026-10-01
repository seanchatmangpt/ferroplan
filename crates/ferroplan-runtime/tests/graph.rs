use ferroplan_runtime::{edge::Edge, graph::Graph};
#[test]
fn failure_removes_only_edge() {
    let mut g = Graph {
        edges: vec![
            Edge {
                id: "a".into(),
                provider: "p".into(),
                enabled: true,
            },
            Edge {
                id: "b".into(),
                provider: "q".into(),
                enabled: true,
            },
        ],
    };
    g.exclude("a");
    assert_eq!(g.lawful().count(), 1);
}
