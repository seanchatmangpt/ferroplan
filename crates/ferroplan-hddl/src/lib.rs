//! ferroplan-hddl — HDDL front-end for ferroplan.
//!
//! Pipeline: `parser::parse_domain`/`parse_problem` (HDDL text -> `ast`) ->
//! `grounder::ground` (`ast::Domain`+`ast::Problem` -> `grounder::GroundedIR`,
//! full typed-object grounding of actions/methods) -> `translate::translate`
//! (`GroundedIR` -> a flat FOND `translate::PlanningProblem`: real BFS over
//! the reachable ground-fact-set state space).
//!
//! Scope (see `ast.rs` for the precise boundary): typed parameters/objects, a
//! single-level type hierarchy, predicates, primitive actions with
//! conjunctive precondition/effect, `oneof` non-deterministic effects and
//! `when` conditional effects, abstract tasks, methods with totally- or
//! partially-ordered subtask networks, ground init facts, a `:goal`
//! conjunction of positive literals, and the problem's initial `:htn`
//! network.
//!
//! Dependency direction: this crate has zero dependency on `ferroplan` — see
//! the note in `Cargo.toml` and `translate.rs`. `ferroplan` depends on this
//! crate and adapts `translate::PlanningProblem` into its own
//! `planning_runtime::PlanningProblem` before handing it to the existing FOND
//! solver.

pub mod ast;
pub mod grounder;
pub mod parser;
pub mod translate;
pub mod validate;
