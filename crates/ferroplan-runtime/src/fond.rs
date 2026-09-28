use crate::{graph::Graph, policy};
pub fn reselect<'a>(g: &'a Graph, x: &[String]) -> Option<&'a str> {
    policy::select(&g.edges, x).map(|e| e.id.as_str())
}
