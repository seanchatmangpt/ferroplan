//! # ferroplan
//!
//! Night-shift dispatcher for [PDDL](https://en.wikipedia.org/wiki/Planning_Domain_Definition_Language)
//! problems — a deterministic core running cold and fast through the search
//! space, built for an era that stopped trusting hand-tuned intuition. Under
//! the hood: a delete-relaxation FF heuristic riding a data-oriented (bitset,
//! structure-of-arrays) task grid, hill-climbing until it stalls, best-first
//! standing by to pick up the slack, grounding and heuristic work sharded
//! across threads.
//!
//! Coverage across the wire: STRIPS, typing, ADL (conditional/`forall`
//! effects, equality), numeric fluents, derived axioms, **PDDL3** soft-goal
//! preferences and metric, **PDDL2.1 temporal** durative actions, and
//! **PPDDL 1.0** probabilistic planning — explicit-MDP policy synthesis,
//! rewards, simulation, validation, the whole chain. An SGPlan-style
//! **partition-and-resolve** mode runs the split jobs too.
//!
//! [`eve`] is the relational contract layer above the engine: human intent
//! gets grounded against a Genesis ontology, run through SPARQL, cut into
//! HDDL pieces, kept honest by PPDDL when the world won't commit to a single
//! outcome, then handed off to ggen/MCP+ — Truex receipt and replay chain
//! unbroken the whole way down.
//!
//! ## The public API (all `serde`-serializable)
//!
//! - [`solve`] — hand it a domain and a problem, it hands back a [`Solution`].
//! - [`solve_ppddl`] — cuts a bounded stochastic policy out of PPDDL.
//! - [`simulate_ppddl`] / [`validate_ppddl_policy`] — receipts from the probabilistic run.
//! - [`decompose`] — breaks a temporal goal into ordered [`Contract`]s and works them.
//! - [`Eve::enter`] — compiles human intent into the Genesis/HDDL/PPDDL/ggen/MCP+
//!   handoff chain, no actuation authority granted along the way.
//! - [`route_planning_request`] — picks the lawful rail for a typed planning request.
//! - [`solve_planning_type`] — runs every admitted planning family over the bounded universal model.
//! - [`parse`] / [`parse_ppddl`] — quick read on syntax and structure, no full commitment.
//! - [`Session`] — ground once, replan as many times as the world keeps shifting.
//! - [`plan::validate_plan`] — a second set of eyes on a deterministic plan.
//!
//! ## Quick start
//! ```no_run
//! let domain = std::fs::read_to_string("domain.pddl").unwrap();
//! let problem = std::fs::read_to_string("problem.pddl").unwrap();
//!
//! let solution = ferroplan::solve(&domain, &problem, &ferroplan::Options::default()).unwrap();
//! if let Some(plan) = solution.plan {
//!     for step in &plan.steps { println!("{}", step.action); }
//! }
//! ```

// engine (data-oriented core)
pub mod bitset;
pub mod clock;
pub mod derived;
pub mod features;
pub mod ground;
pub mod hash;
pub mod heuristic;
pub mod invariants;
pub mod lama;
pub mod landmarks;
pub mod lexer;
pub mod novelty;
pub mod orbits;
pub mod output;
pub mod packed;
pub mod par;
pub mod parser;
pub mod resource;
pub mod search;
pub mod types;

// modes (built on the engine)
pub mod constraints;
pub mod costs;
pub mod espc;
pub mod partition;
pub mod pddl3;
pub mod plan;
pub mod portfolio;
pub mod report;
pub mod resolve;
pub mod selection;
pub mod temporal;
pub mod trace;
pub mod tresolve;
pub mod tsched;
pub mod verify;
pub mod viz;

// orchestration + smart public API
pub mod api;
pub mod planner;
pub mod session;

pub use api::{
    decompose, parse, solve, Contract, Decomposition, DomainSummary, Metric, Mode, Options,
    ParseReport, Plan, ProblemSummary, Search, Solution, SolveError, Statistics, Step,
};
pub use planner::{run_ff, run_planner};
pub use session::Session;
pub use trace::{trace, StateSnapshot};
pub use types::ParseError;
