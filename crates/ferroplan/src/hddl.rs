//! HDDL entry point: parse -> ground -> translate (all via `ferroplan_hddl`)
//! -> adapt into [`crate::planning_runtime::PlanningProblem`] -> the
//! existing FOND solver (`solve_planning_type` with
//! [`PlanningType::Fond`], which dispatches to the existing `fond_policy`).
//!
//! This is the "explicit hddl-entry function" alternative rather than a new
//! `PlanningType::Hddl` variant: `PlanningType` already has a closed,
//! 18-member `ALL` array and a `required_capabilities`/`rail` match arm per
//! variant in `planning_types.rs` that every admitted paradigm must answer
//! for — HDDL is a *front-end* (a text syntax that compiles down to the
//! existing flat FOND model), not a new solving paradigm, so it does not
//! need its own entry in that constitution. The result of solving an HDDL
//! problem is still reported as `PlanningType::Fond` in the returned
//! `UniversalPlan`, honestly reflecting which solver actually ran.
//!
//! Dependency direction: `ferroplan` depends on `ferroplan-hddl` (declared in
//! `Cargo.toml`), never the reverse — `ferroplan-hddl` defines its own
//! ground-IR types in `translate.rs` rather than importing
//! `planning_runtime`'s, specifically so this edge does not create a cycle.
//! The `adapt_problem` function below is the one place that maps one shape
//! onto the other.

use crate::planning_runtime::{
    solve_planning_type, Goal, Method, PlannerError, PlannerLimits, PlanningProblem, State, Task,
    Transition, UniversalPlan, UniversalPlanningRequest,
};
use crate::planning_types::PlanningType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HddlError {
    Parse(String),
    Ground(String),
    Translate(String),
    Planner(PlannerError),
}

impl fmt::Display for HddlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(msg) => write!(f, "HDDL parse error: {msg}"),
            Self::Ground(msg) => write!(f, "HDDL grounding error: {msg}"),
            Self::Translate(msg) => write!(f, "HDDL translation error: {msg}"),
            Self::Planner(e) => write!(f, "planner error: {e}"),
        }
    }
}
impl std::error::Error for HddlError {}

impl From<PlannerError> for HddlError {
    fn from(e: PlannerError) -> Self {
        Self::Planner(e)
    }
}

/// Field-for-field adapter from `ferroplan_hddl::translate::PlanningProblem`
/// (that crate's own, dependency-free ground-IR shape) to this crate's
/// `planning_runtime::PlanningProblem`. The two shapes are intentionally
/// parallel, so this is a straight structural mapping, not a reinterpretation.
fn adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) -> PlanningProblem {
    PlanningProblem {
        states: p
            .states
            .into_iter()
            .map(|s| State {
                id: s.id,
                facts: s.facts,
                fluents: Default::default(),
            })
            .collect(),
        initial_states: p.initial_states,
        goal: Goal {
            facts: p.goal.facts,
            numeric_min: Default::default(),
            numeric_max: Default::default(),
        },
        unsafe_states: Default::default(),
        soft_goal_facts: Default::default(),
        transitions: p
            .transitions
            .into_iter()
            .map(|t| Transition {
                action: t.action,
                from: t.from,
                to: t.to,
                cost: 1,
                duration: 1,
                reward: 0,
                probability_ppm: t.probability_ppm,
                observation: None,
                requires: Default::default(),
            })
            .collect(),
        tasks: p
            .tasks
            .into_iter()
            .map(|t| Task {
                id: t.id,
                primitive_action: t.primitive_action,
                requires: Default::default(),
            })
            .collect(),
        root_tasks: p.root_tasks,
        methods: p
            .methods
            .into_iter()
            .map(|m| Method {
                id: m.id,
                task: m.task,
                subtasks: m.subtasks,
            })
            .collect(),
        workflow_edges: Default::default(),
        queues: Default::default(),
        agents: Default::default(),
        tools: Default::default(),
        rdf: Default::default(),
    }
}

/// Parse, ground, and translate an HDDL domain+problem pair, then run it
/// through the existing FOND solver.
pub fn solve_hddl(
    domain_src: &str,
    problem_src: &str,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, HddlError> {
    let domain = ferroplan_hddl::parser::parse_domain(domain_src)
        .map_err(|e| HddlError::Parse(e.to_string()))?;
    let problem = ferroplan_hddl::parser::parse_problem(problem_src)
        .map_err(|e| HddlError::Parse(e.to_string()))?;
    let ir = ferroplan_hddl::grounder::ground(&domain, &problem, &Default::default())
        .map_err(|e| HddlError::Ground(e.to_string()))?;
    let translated = ferroplan_hddl::translate::translate(&ir)
        .map_err(|e| HddlError::Translate(e.to_string()))?;
    let planning_problem = adapt_problem(translated);
    let request = UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: planning_problem,
        limits: limits.clone(),
    };
    Ok(solve_planning_type(&request)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_C_DOMAIN: &str = include_str!("../../ferroplan-hddl/fixtures/c/domain.hddl");
    const FIXTURE_C_PROBLEM: &str = include_str!("../../ferroplan-hddl/fixtures/c/problem.hddl");

    /// Real end-to-end run of the FOND `oneof` fixture: HDDL text -> parse ->
    /// ground -> translate -> the existing `fond_policy` (via
    /// `solve_planning_type`) -> a real, non-empty policy.
    #[test]
    fn solves_the_oneof_fixture_end_to_end() {
        let plan = solve_hddl(
            FIXTURE_C_DOMAIN,
            FIXTURE_C_PROBLEM,
            &PlannerLimits::default(),
        )
        .expect("HDDL FOND fixture solves end-to-end");
        assert!(plan.solved);
        assert_eq!(plan.planning_type, Some(PlanningType::Fond));
        assert!(
            !plan.policy.is_empty(),
            "expected at least one PolicyEntry, got none"
        );
        // The initial state's entry must choose the oneof action, since it is
        // the only action applicable there.
        let initial_entry = plan
            .policy
            .iter()
            .find(|e| e.action == "cross-bridge(l1,l2,l3)")
            .expect("policy chooses the oneof cross-bridge action somewhere");
        assert_eq!(initial_entry.outcomes.len(), 2);
        let mass: u32 = initial_entry
            .outcomes
            .iter()
            .map(|o| o.probability_ppm)
            .sum();
        assert_eq!(mass, 1_000_000);
    }

    #[test]
    fn rejects_malformed_hddl_with_a_parse_error() {
        let err = solve_hddl(
            "(define (domain broken",
            FIXTURE_C_PROBLEM,
            &PlannerLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(err, HddlError::Parse(_)));
    }
}
