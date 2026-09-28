use crate::graph::Graph;
pub fn exclude_failed(g: &mut Graph, edge: &str) -> usize {
    g.exclude(edge);
    g.lawful().count()
}
