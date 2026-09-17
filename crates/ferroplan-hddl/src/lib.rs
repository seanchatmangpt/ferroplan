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
//! conjunctive precondition/effect, `when` conditional effects, `oneof`
//! non-deterministic effects (precise surface below), abstract tasks,
//! methods with totally- or partially-ordered subtask networks, ground init
//! facts, a `:goal` conjunction of positive and/or negative ground literals
//! (`(not (pred ...))`; negating anything but a single atom, e.g. `(not (and
//! ...))`, stays out of scope — see `translate.rs`), and the problem's
//! initial `:htn` network.
//!
//! The supported `oneof` surface, aligned exactly with the koala-planner
//! reference (its hddl.y grammar admits `oneof` in one position only):
//! `oneof` appears at most once per action, as the ENTIRE top-level
//! `:effect` — `:effect (oneof e1 ... ek)`, k >= 1, where k == 1 normalizes
//! to the bare deterministic branch (koala hddl.y:327-330) and k == 0 is
//! refused. Branch effects may be plain literal effects, `and`-conjunctions,
//! or the empty effect `()` (which survives grounding and translation as a
//! genuine no-change outcome). Deliberate deviation from koala, documented
//! at `parser::parse_effect`'s `when` arm: a `when` inside a `oneof` branch
//! is refused with `ParseError::MalformedOneof` — koala parses that shape
//! but silently drops the conditional effect downstream (a known koala
//! TODO), so ferroplan refuses loudly instead. Nesting a `oneof` under
//! `and`/`when`/another `oneof`, or using `oneof` in any
//! precondition/method-condition/`:goal` position, is likewise refused with
//! `ParseError::MalformedOneof`. Branches need NOT be mutually exclusive —
//! overlapping branches are kept verbatim, and no
//! exclusivity/disjointness validation exists. Semantics: one
//! non-deterministic choice point per action; each branch grounds to
//! exactly one outcome of the translated explicit graph, applied
//! atomically; the action's single `:precondition` gates all branches.
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
