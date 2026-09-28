use crate::plan::Plan;
pub fn ranked(mut ps: Vec<Plan>) -> Vec<Plan> {
    ps.sort_by_key(|p| p.cost);
    ps
}
