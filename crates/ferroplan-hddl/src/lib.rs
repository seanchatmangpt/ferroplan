//! ferroplan-hddl — HDDL front-end for ferroplan.
//!
//! Pipeline: `probabilistic::preprocess` (rewrites koala-planner-style
//! `(:probabilistic w1 e1 w2 e2 ...)` effect blocks into standard `oneof`
//! blocks plus a side-channel weight map, as a pure text pass before
//! tokenizing) -> `parser::parse_domain`/`parse_problem` (HDDL text -> `ast`)
//! -> `grounder::ground` (`ast::Domain`+`ast::Problem` ->
//! `grounder::GroundedIR`, full typed-object grounding of actions/methods,
//! carrying declared weights onto each `GroundEffectBranch`) ->
//! `translate::translate` (`GroundedIR` -> a flat FOND
//! `translate::PlanningProblem`: real BFS over the reachable ground-fact-set
//! state space, splitting each action's outcome probabilities proportionally
//! to declared weights when present, evenly otherwise).
//!
//! Scope (see `ast.rs` for the precise boundary): typed parameters/objects, a
//! single-level type hierarchy, predicates, primitive actions with
//! conjunctive precondition/effect, `oneof` non-deterministic effects and
//! `when` conditional effects, abstract tasks, methods with totally- or
//! partially-ordered subtask networks, ground init facts, a `:goal`
//! conjunction of positive and/or negative ground literals (`(not (pred
//! ...))`; negating anything but a single atom, e.g. `(not (and ...))`,
//! stays out of scope — see `translate.rs`), and the problem's initial
//! `:htn` network.
//!
//! Dependency direction: this crate has zero dependency on `ferroplan` — see
//! the note in `Cargo.toml` and `translate.rs`. `ferroplan` depends on this
//! crate and adapts `translate::PlanningProblem` into its own
//! `planning_runtime::PlanningProblem` before handing it to the existing FOND
//! solver.

// No legitimate `unsafe` usage anywhere in this crate (verified: zero hits
// for `unsafe` across `src/*.rs`) -- this crate parses/grounds/translates
// untrusted HDDL text, so forbid it at the lint level rather than merely
// discouraging it.
#![forbid(unsafe_code)]

pub mod ast;
pub mod grounder;
pub mod parser;
pub mod probabilistic;
pub mod translate;
pub mod validate;
