use crate::edge::Edge;
pub fn select<'a>(e: &'a [Edge], x: &[String]) -> Option<&'a Edge> {
    e.iter().find(|e| e.enabled && !x.contains(&e.id))
}
