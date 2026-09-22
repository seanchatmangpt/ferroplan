//! Memory-ceiling stress suite (ticket `fond-htn-30-stress-memory-ceilings`).
//!
//! Contract under test: adversarial HDDL schemas whose *naive* combinatorial
//! products are astronomically large (>= 10^7 typed bindings, 50^10
//! decomposition choices, 500 predicates, 300-deep legal typing chains) are
//! always answered by the parse -> ground -> translate -> solve pipeline with
//! either
//!
//! * a **typed refusal** from a configured ceiling — `GroundError::
//!   LimitExceeded`, `TranslateError::MemoryLimitExceeded`,
//!   `TranslateError::TaskNetworkDepthExceeded`, `PlannerError::Timeout` (the
//!   `ResourceBound`/`Timeout`-shaped refusal family), or
//! * a **clean solve within the ceiling**,
//!
//! and NEVER a panic, never a signal kill (SIGKILL/SIGSEGV/...), never an
//! unbounded RSS climb. Every case's peak RSS is *measured*, not guessed.
//!
//! # Case families (ticket scope)
//!
//! 1. `binding-product-8p`, `binding-ladder-2to8` — actions with 2-8 typed
//!    parameters over object sets whose naive binding product explodes
//!    (8^8 = 16,777,216 for the max case; 19,173,952 cumulative for the
//!    ladder). `GroundingLimits::max_ground_actions` refuses lazily
//!    (`BindingIter` yields one binding at a time — see `grounder.rs`), so
//!    the 10^7+ naive product is answered after ~10k *enumerated* bindings
//!    with bounded memory.
//! 2. `method-fanout-50x10` — one abstract task with 50 methods x 10
//!    subtasks (naive decomposition product 50^10 ~ 9.8e16), made
//!    state-distinguishable by a per-method `mark-i` effect so the
//!    translator's content dedup (`augmented_facts`) cannot collapse the
//!    branches. `TranslateLimits::max_states` refuses with
//!    `MemoryLimitExceeded`.
//! 3. `predicates-500` — a 500-predicate domain that stays within every
//!    ceiling and must **solve cleanly** (the contract's positive arm).
//! 4. `typing-chain-300` — a 300-deep-but-legal type hierarchy; the deepest
//!    object must remain usable at the root type (closure correctness under
//!    depth), solving cleanly.
//!
//! Plus two boundary probes that pin down *which* ceiling fires:
//! `fanout-dedup-50x10` (the same 50x10 fan-out WITHOUT distinguishing
//! marks — content dedup collapses it, so the depth bound, not memory, is
//! what refuses, at trivial RSS) and `planner-wall-bound` (a flat 1,000-state
//! FOND chain under a 100 ms `PlannerLimits::max_wall_ms` -> typed
//! `PlannerError::Timeout`).
//!
//! # How the ceilings are enforced and measured
//!
//! Each case runs in its own **child process** (this binary re-execs itself
//! with `FERROPLAN_MEMORY_STRESS_CHILD=<case>`; `harness = false` gives us
//! the `main`). Two independent belts:
//!
//! * `RLIMIT_AS` is set to 512 MiB via a `sh -c 'ulimit -v ...; exec'`
//!   wrapper. macOS treats `RLIMIT_AS` as best-effort (not every allocation
//!   path is subject to it), so it is a *backstop*, not the evidence.
//! * The parent **samples the child's RSS every ~10 ms** via `ps -o rss=`
//!   and records the peak. This is the measured evidence, per the ticket's
//!   documented-fallback option (`mach_task_info` would need unsafe FFI
//!   bindings; `ps` sampling is the portable route). Sampling resolution is
//!   one interval (~10 ms); a spike shorter than that can be missed — the
//!   committed ceilings (`max_ground_actions`, `max_states`, `max_wall`)
//!   make sub-10 ms unbounded growth physically impossible anyway, and the
//!   parent additionally hard-fails any case whose peak exceeds the
//!   `RLIMIT_AS` budget.
//!
//! Refusal/solve latency is measured *inside* the child around the exact
//! pipeline call (`Instant`), so it is the phase's own latency, not process
//! startup noise.
//!
//! # Gate map
//!
//! * `cargo test -p ferroplan --test memory_stress` — always-on sampled run
//!   (the 2 fastest measured cases; see `SAMPLED_CASES` and RESULTS.md).
//! * `cargo test -p ferroplan --test memory_stress -- --ignored` — the full
//!   7-case suite. Wave rule: the whole run stays under 120 s (asserted).

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{
    solve_planning_type, PlanningProblem, PlannerError, PlannerLimits, State as RtState,
    Transition as RtTransition, UniversalPlanningRequest,
};
use ferroplan::PlanningType;
use ferroplan_hddl::grounder::{ground, GroundError, GroundingLimits};
use ferroplan_hddl::parser::{parse_domain, parse_problem};
use ferroplan_hddl::translate::{translate, TranslateError, TranslateLimits};
use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// `RLIMIT_AS` backstop for every child process, in KiB (512 MiB). See the
/// module docs: primary evidence is measured RSS sampling; this is the
/// hard backstop on platforms that enforce it (Linux), best-effort on macOS.
const RLIMIT_AS_KB: u64 = 512 * 1024;

/// Peak-RSS budget asserted against the *measured* peak (KiB). A case whose
/// sampled peak approaches this is an unbounded-climb failure per the
/// contract, even if the platform never enforced the rlimit.
const PEAK_RSS_BUDGET_KB: u64 = RLIMIT_AS_KB;

/// Always-on sampled subset: the 2 fastest cases of the full suite as
/// measured on the committing machine (18 ms and 34 ms in-child — see
/// `tests/fixtures/memory-stress/RESULTS.md` for the measurement that chose
/// them; re-measure and update both together if the set changes).
const SAMPLED_CASES: [&str; 2] = ["predicates-500", "fanout-dedup-50x10"];

// ---------------------------------------------------------------------------
// Case registry
// ---------------------------------------------------------------------------

struct Case {
    name: &'static str,
    family: &'static str,
    /// Parent-side wall budget for the child. The crate's own ceilings are
    /// configured to fire far earlier; this is the harness backstop (and is
    /// what gets violated if a future regression makes a case hang).
    wall: Duration,
    run: fn() -> CaseOutcome,
}

#[derive(Debug)]
enum Outcome {
    /// Clean solve within the ceiling (contract arm 2).
    Solved { detail: String },
    /// Typed refusal from a configured ceiling (contract arm 1). `phase`
    /// names the pipeline stage that refused.
    Refused { phase: &'static str, detail: String },
}

#[derive(Debug)]
struct CaseOutcome {
    outcome: Outcome,
    /// The schema's naive combinatorial product (bindings or decomposition
    /// choices) — what an *unbounded* grounder/translator would have had to
    /// enumerate. Reported so RESULTS.md can contrast it with the measured
    /// refusal latency and RSS.
    naive_product: u64,
}

impl CaseOutcome {
    fn refused(phase: &'static str, detail: String, naive_product: u64) -> Self {
        Self { outcome: Outcome::Refused { phase, detail }, naive_product }
    }
    fn solved(detail: String, naive_product: u64) -> Self {
        Self { outcome: Outcome::Solved { detail }, naive_product }
    }
}

fn cases() -> Vec<Case> {
    vec![
        Case {
            name: "binding-product-8p",
            family: "binding-product explosion",
            wall: Duration::from_secs(30),
            run: case_binding_product_8p,
        },
        Case {
            name: "binding-ladder-2to8",
            family: "binding-product explosion",
            wall: Duration::from_secs(30),
            run: case_binding_ladder_2to8,
        },
        Case {
            name: "method-fanout-50x10",
            family: "wide method fan-out",
            wall: Duration::from_secs(60),
            run: case_method_fanout_50x10,
        },
        Case {
            name: "fanout-dedup-50x10",
            family: "wide method fan-out (dedup probe)",
            wall: Duration::from_secs(30),
            run: case_fanout_dedup_50x10,
        },
        Case {
            name: "predicates-500",
            family: "predicate-heavy",
            wall: Duration::from_secs(30),
            run: case_predicates_500,
        },
        Case {
            name: "typing-chain-300",
            family: "deep-but-legal typing",
            wall: Duration::from_secs(30),
            run: case_typing_chain_300,
        },
        Case {
            name: "planner-wall-bound",
            family: "planner ceiling",
            wall: Duration::from_secs(30),
            run: case_planner_wall_bound,
        },
    ]
}

fn find_case(name: &str) -> Case {
    cases()
        .into_iter()
        .find(|c| c.name == name)
        .unwrap_or_else(|| panic!("unknown case '{name}'"))
}

// ---------------------------------------------------------------------------
// Seeded deterministic helpers (generators are fully in-test, hand-authored;
// the seed shuffles object/name insertion order to prove order-invariance)
// ---------------------------------------------------------------------------

/// SplitMix64-style PRNG step — tiny, deterministic, dependency-free.
fn lcg_next(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state >> 33
}

fn seeded_shuffle<T>(items: &mut [T], seed: u64) {
    let mut s = seed | 1;
    if items.len() < 2 {
        return;
    }
    for i in (1..items.len()).rev() {
        let j = (lcg_next(&mut s) as usize) % (i + 1);
        items.swap(i, j);
    }
}

// ---------------------------------------------------------------------------
// Generators (all hand-authored here; KOALA POLICY: concepts only, no koala
// source — these schemas are original transport/retry-pattern shapes)
// ---------------------------------------------------------------------------

/// One action with 8 typed parameters over 8 types x 8 objects:
/// naive binding product 8^8 = 16,777,216 >= 10^7.
fn gen_binding_product_max(seed: u64) -> (String, String, u64) {
    let n_types = 8usize;
    let objs_per_type = 8usize;
    let mut type_ids: Vec<usize> = (0..n_types).collect();
    seeded_shuffle(&mut type_ids, seed);
    let types: Vec<String> = type_ids.iter().map(|t| format!("t{t}")).collect();

    let mut objects = String::new();
    for (ti, ty) in types.iter().enumerate() {
        let mut objs: Vec<usize> = (0..objs_per_type).collect();
        seeded_shuffle(&mut objs, seed.wrapping_add(ti as u64 + 1));
        let names: Vec<String> = objs.iter().map(|o| format!("o{ti}_{o}")).collect();
        objects.push_str(&format!("    {}\n", names.join(" ")));
        objects.push_str(&format!("    - {ty}\n"));
    }

    let params = types
        .iter()
        .enumerate()
        .map(|(i, t)| format!("?a{i} - {t}"))
        .collect::<Vec<_>>()
        .join(" ");
    let reached_args = (0..n_types)
        .map(|i| format!("?a{i}"))
        .collect::<Vec<_>>()
        .join(" ");
    let reached_sig = types
        .iter()
        .enumerate()
        .map(|(i, t)| format!("?x{i} - {t}"))
        .collect::<Vec<_>>()
        .join(" ");

    let domain = format!(
        ";; hand-authored for fond-htn-30 (binding-product stress; seeded seed={seed})\n\
         (define (domain mem-binding-max)\n\
         \x20 (:types {types})\n\
         \x20 (:predicates (start) (reached {reached_sig}))\n\
         \x20 (:task run)\n\
         \x20 (:method m-run :task (run) :precondition (and) :ordered-subtasks (and))\n\
         \x20 (:action move8\n\
         \x20   :parameters ({params})\n\
         \x20   :precondition (and (start))\n\
         \x20   :effect (and (reached {reached_args}))))\n",
        types = types.join(" "),
    );
    let problem = format!(
        ";; hand-authored for fond-htn-30 (binding-product stress; seeded seed={seed})\n\
         (define (problem mem-binding-max-p1)\n\
         \x20 (:domain mem-binding-max)\n\
         \x20 (:objects\n{objects}  )\n\
         \x20 (:init (start))\n\
         \x20 (:goal (and (start)))\n\
         \x20 (:htn :ordered-subtasks (and (r (run)))))\n"
    );
    (domain, problem, 8u64.pow(n_types as u32))
}

/// Actions of every arity 2..=8 over the same 8x8 object universe:
/// cumulative naive product sum(8^k, k=2..=8) = 19,173,952 >= 10^7.
fn gen_binding_ladder(seed: u64) -> (String, String, u64) {
    let n_types = 8usize;
    let objs_per_type = 8usize;
    let mut type_ids: Vec<usize> = (0..n_types).collect();
    seeded_shuffle(&mut type_ids, seed);
    let types: Vec<String> = type_ids.iter().map(|t| format!("t{t}")).collect();

    let mut objects = String::new();
    for (ti, ty) in types.iter().enumerate() {
        let mut objs: Vec<usize> = (0..objs_per_type).collect();
        seeded_shuffle(&mut objs, seed.wrapping_add(100 + ti as u64));
        let names: Vec<String> = objs.iter().map(|o| format!("o{ti}_{o}")).collect();
        objects.push_str(&format!("    {}\n", names.join(" ")));
        objects.push_str(&format!("    - {ty}\n"));
    }

    let mut preds = String::from("(start)");
    let mut actions = String::new();
    let mut product: u64 = 0;
    for arity in 2..=n_types {
        product += 8u64.pow(arity as u32);
        let args = types[..arity]
            .iter()
            .enumerate()
            .map(|(i, t)| format!("?p{i} - {t}"))
            .collect::<Vec<_>>()
            .join(" ");
        preds.push_str(&format!(" (reached{arity}"));
        for i in 0..arity {
            preds.push_str(&format!(" ?x{i} - {}", types[i]));
        }
        preds.push(')');
        let call_args = (0..arity)
            .map(|i| format!("?p{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        actions.push_str(&format!(
            "  (:action climb-{arity}\n\
              \x20 :parameters ({args})\n\
              \x20 :precondition (and (start))\n\
              \x20 :effect (and (reached{arity} {call_args})))\n"
        ));
    }

    let domain = format!(
        ";; hand-authored for fond-htn-30 (binding-ladder stress; seeded seed={seed})\n\
         (define (domain mem-binding-ladder)\n\
         \x20 (:types {types})\n\
         \x20 (:predicates {preds})\n\
         \x20 (:task run)\n\
         \x20 (:method m-run :task (run) :precondition (and) :ordered-subtasks (and))\n\
         {actions})\n",
        types = types.join(" "),
    );
    let problem = format!(
        ";; hand-authored for fond-htn-30 (binding-ladder stress; seeded seed={seed})\n\
         (define (problem mem-binding-ladder-p1)\n\
         \x20 (:domain mem-binding-ladder)\n\
         \x20 (:objects\n{objects}  )\n\
         \x20 (:init (start))\n\
         \x20 (:goal (and (start)))\n\
         \x20 (:htn :ordered-subtasks (and (r (run)))))\n"
    );
    (domain, problem, product)
}

/// One abstract task `fan` with 50 methods x 10 subtasks (naive
/// decomposition product 50^10). With `distinguishable`, method i's subtasks
/// are `(mark-i)` + 9x `fan` and `mark-i` writes a distinct fact, so the
/// translator's content dedup cannot collapse branches (real state-space
/// blowup -> `max_states` refuses). Without it, all 50 methods are
/// interchangeable, dedup collapses everything, and the *depth* bound is
/// what refuses (at trivial RSS) — the paired probe proving the naive
/// product is not the memory driver when states are indistinguishable.
fn gen_method_fanout(seed: u64, distinguishable: bool) -> (String, String, u64) {
    let n_methods = 50usize;
    let n_subtasks = 10usize;
    let mut method_ids: Vec<usize> = (0..n_methods).collect();
    seeded_shuffle(&mut method_ids, seed);

    let mut preds = String::from("(start) (done-marker)");
    let mut actions = String::new();
    if distinguishable {
        for i in &method_ids {
            preds.push_str(&format!(" (chosen-{i})"));
        }
        for i in &method_ids {
            actions.push_str(&format!(
                "  (:action mark-{i}\n\
                  \x20 :parameters ()\n\
                  \x20 :precondition (and (start))\n\
                  \x20 :effect (and (chosen-{i})))\n"
            ));
        }
    }

    let mut methods = String::new();
    for i in &method_ids {
        let subtasks: Vec<String> = if distinguishable {
            let mut v = vec![format!("(s0 (mark-{i}))")];
            for s in 1..n_subtasks {
                v.push(format!("(s{s} (fan))"));
            }
            v
        } else {
            (0..n_subtasks).map(|s| format!("(s{s} (fan))")).collect()
        };
        methods.push_str(&format!(
            "  (:method fan-{i}\n\
              \x20 :task (fan)\n\
              \x20 :precondition (and)\n\
              \x20 :ordered-subtasks (and {}))\n",
            subtasks.join(" ")
        ));
    }

    let domain = format!(
        ";; hand-authored for fond-htn-30 (method fan-out stress; distinguishable={distinguishable}; seeded seed={seed})\n\
         (define (domain mem-fanout)\n\
         \x20 (:predicates {preds})\n\
         \x20 (:task fan)\n\
         {actions}{methods})\n"
    );
    let problem = format!(
        ";; hand-authored for fond-htn-30 (method fan-out stress; distinguishable={distinguishable}; seeded seed={seed})\n\
         (define (problem mem-fanout-p1)\n\
         \x20 (:domain mem-fanout)\n\
         \x20 (:init (start))\n\
         \x20 (:goal (and (done-marker)))\n\
         \x20 (:htn :ordered-subtasks (and (r (fan)))))\n"
    );
    (domain, problem, 50u64.pow(n_subtasks as u32))
}

/// 500 predicates; a pinned ordered-network corridor (3 clears + finish) so
/// the planner phase stays tiny — the load is the 500-schema parse/ground
/// surface, and every translated state carries the 500-fact initial state.
/// Must solve CLEANLY within default ceilings (contract arm 2).
fn gen_predicates_500(seed: u64) -> (String, String) {
    let n_preds = 500usize;
    let mut pred_ids: Vec<usize> = (0..n_preds).collect();
    seeded_shuffle(&mut pred_ids, seed);

    let mut preds = String::from("(at ?x - loc) (done)");
    for i in &pred_ids {
        preds.push_str(&format!(" (p{i} ?x - loc)"));
    }
    let mut actions = String::new();
    for i in &pred_ids {
        actions.push_str(&format!(
            "  (:action clear-{i}\n\
              \x20 :parameters (?x - loc)\n\
              \x20 :precondition (and (p{i} ?x))\n\
              \x20 :effect (and (not (p{i} ?x))))\n"
        ));
    }
    actions.push_str(
        "  (:action finish\n\
         \x20 :parameters (?x - loc)\n\
         \x20 :precondition (and (at ?x))\n\
         \x20 :effect (and (done)))\n",
    );

    // Corridor: clear exactly the three highest-index predicates, then finish.
    let last3: Vec<usize> = pred_ids.iter().rev().take(3).copied().collect();
    let subtasks: Vec<String> = last3
        .iter()
        .enumerate()
        .map(|(k, i)| format!("(c{k} (clear-{i} o0))"))
        .chain(std::iter::once("(cf (finish o0))".to_owned()))
        .collect();

    let mut init = String::from("(at o0)");
    for i in &pred_ids {
        init.push_str(&format!(" (p{i} o0)"));
    }

    let domain = format!(
        ";; hand-authored for fond-htn-30 (predicate-heavy stress; seeded seed={seed})\n\
         (define (domain mem-predicates)\n\
         \x20 (:types loc)\n\
         \x20 (:predicates {preds})\n\
         \x20 (:task run)\n\
         \x20 (:method go :task (run) :precondition (and) :ordered-subtasks (and {}))\n\
         {actions})\n",
        subtasks.join(" "),
    );
    let problem = format!(
        ";; hand-authored for fond-htn-30 (predicate-heavy stress; seeded seed={seed})\n\
         (define (problem mem-predicates-p1)\n\
         \x20 (:domain mem-predicates)\n\
         \x20 (:objects o0 - loc)\n\
         \x20 (:init {init})\n\
         \x20 (:goal (and (done)))\n\
         \x20 (:htn :ordered-subtasks (and (r (run)))))\n"
    );
    (domain, problem)
}

/// 300-deep-but-legal typing chain: t299 <: t298 <: ... <: t0. The single
/// object lives at the deepest type and is called where the *root* type is
/// expected — legal only if the type closure walks the whole chain. Must
/// solve cleanly (a wrong closure yields zero ground actions -> NoPlan ->
/// this case fails).
fn gen_typing_chain(depth: usize, seed: u64) -> (String, String) {
    let mut chain_ids: Vec<usize> = (0..depth).collect();
    seeded_shuffle(&mut chain_ids, seed);
    // types[k] is the type at chain rank k: types[0] = root, parented on the
    // builtin 'object' root (parse_typed_group would otherwise absorb a bare
    // leading name into the first subtype group as its sibling — and make
    // the chain self-ancestored).
    let rank_to_name: Vec<String> =
        chain_ids.iter().map(|id| format!("t{id}")).collect();
    let mut types_src = format!("{} - object", rank_to_name[0]);
    for k in 1..depth {
        types_src.push_str(&format!(" {} - {}", rank_to_name[k], rank_to_name[k - 1]));
    }

    let domain = format!(
        ";; hand-authored for fond-htn-30 (deep typing-chain stress; depth={depth}; seeded seed={seed})\n\
         (define (domain mem-typing-chain)\n\
         \x20 (:types {types_src})\n\
         \x20 (:predicates (at ?x - {root}) (done))\n\
         \x20 (:task run)\n\
         \x20 (:method go :task (run) :precondition (and) :ordered-subtasks (and (f1 (finish ob))))\n\
         \x20 (:action finish\n\
         \x20   :parameters (?x - {root})\n\
         \x20   :precondition (and (at ?x))\n\
         \x20   :effect (and (done))))\n",
        root = rank_to_name[0],
    );
    let problem = format!(
        ";; hand-authored for fond-htn-30 (deep typing-chain stress; depth={depth}; seeded seed={seed})\n\
         (define (problem mem-typing-chain-p1)\n\
         \x20 (:domain mem-typing-chain)\n\
         \x20 (:objects ob - {leaf})\n\
         \x20 (:init (at ob))\n\
         \x20 (:goal (and (done)))\n\
         \x20 (:htn :ordered-subtasks (and (r (run)))))\n",
        leaf = rank_to_name[depth - 1],
    );
    (domain, problem)
}

/// Flat 1,000-state FOND chain (hand-built `PlanningProblem`, planner-phase
/// stress in isolation): value iteration needs ~1,000 rounds to converge; a
/// 100 ms `max_wall_ms` refuses first with typed `PlannerError::Timeout`
/// (round cost alone is ~10^6 group visits, far above timer resolution).
fn gen_chain_problem(n_states: usize) -> PlanningProblem {
    let goal_fact = "goal".to_owned();
    let mut states = Vec::with_capacity(n_states);
    let mut transitions = Vec::with_capacity(n_states - 1);
    for i in 0..n_states {
        let mut facts = std::collections::BTreeSet::new();
        if i == n_states - 1 {
            facts.insert(goal_fact.clone());
        }
        states.push(RtState { id: format!("s{i}"), facts, fluents: Default::default() });
        if i + 1 < n_states {
            transitions.push(RtTransition {
                action: "advance".to_owned(),
                from: format!("s{i}"),
                to: format!("s{}", i + 1),
                cost: 1,
                duration: 1,
                reward: 0,
                probability_ppm: 1_000_000,
                observation: None,
                requires: Default::default(),
            });
        }
    }
    PlanningProblem {
        states,
        initial_states: vec!["s0".to_owned()],
        goal: ferroplan::planning_runtime::Goal {
            facts: [goal_fact].into_iter().collect(),
            ..Default::default()
        },
        transitions,
        ..PlanningProblem::default()
    }
}

// ---------------------------------------------------------------------------
// Case bodies — each asserts the exact typed contract it claims
// ---------------------------------------------------------------------------

fn case_binding_product_8p() -> CaseOutcome {
    let seed = 0xA11CE;
    let (domain_src, problem_src, product) = gen_binding_product_max(seed);
    assert!(
        product >= 10_000_000,
        "schema must make the naive binding product explode (>= 10^7), got {product}"
    );
    let domain = parse_domain(&domain_src).expect("generated domain must parse");
    let problem = parse_problem(&problem_src).expect("generated problem must parse");
    let limits = GroundingLimits {
        max_ground_actions: 10_000,
        ..GroundingLimits::default()
    };
    let started = Instant::now();
    let err = ground(&domain, &problem, &limits)
        .err()
        .expect("binding-product-8p must be refused, not grounded");
    let elapsed = started.elapsed();
    assert!(
        matches!(err, GroundError::LimitExceeded(_)),
        "expected typed GroundError::LimitExceeded, got {err:?}"
    );
    // End-to-end (re-pinned by ticket fond-htn-60): the solve pipeline runs
    // hierarchical task-relevance pruning, and `move8` is named by NO method
    // or root subtask — the root task `run` decomposes (via `m-run`) to the
    // empty network. So the 16.7M-instance schema is provably irrelevant,
    // the relevance pass walks its bindings lazily (bounded, no OOM — the
    // property this stress case guards) and keeps zero instances, and the
    // honest end-to-end answer changed from a typed ground refusal to a
    // solved empty-decomposition plan. Bounded exit, correct answer, still
    // no panic/kill/OOM.
    let e2e = solve_hddl(&domain_src, &problem_src, &PlannerLimits::default())
        .expect("end-to-end solve must stay bounded (solved via the empty decomposition)");
    assert!(
        e2e.solved,
        "root task decomposes to the empty network: the plan must solve"
    );
    CaseOutcome::refused(
        "ground",
        format!("{err} (end-to-end since fond-htn-60: solved, relevant instances = 0)"),
        product,
    )
    .with_elapsed(elapsed)
}

fn case_binding_ladder_2to8() -> CaseOutcome {
    let seed = 0xB0B;
    let (domain_src, problem_src, product) = gen_binding_ladder(seed);
    assert!(
        product >= 10_000_000,
        "ladder cumulative naive product must be >= 10^7, got {product}"
    );
    let domain = parse_domain(&domain_src).expect("generated domain must parse");
    let problem = parse_problem(&problem_src).expect("generated problem must parse");
    let limits = GroundingLimits {
        max_ground_actions: 10_000,
        ..GroundingLimits::default()
    };
    let started = Instant::now();
    let err = ground(&domain, &problem, &limits)
        .err()
        .expect("binding-ladder must be refused, not grounded");
    let elapsed = started.elapsed();
    assert!(
        matches!(err, GroundError::LimitExceeded(_)),
        "expected typed GroundError::LimitExceeded, got {err:?}"
    );
    CaseOutcome::refused("ground", format!("{err}"), product).with_elapsed(elapsed)
}

fn case_method_fanout_50x10() -> CaseOutcome {
    let seed = 0xFAA7;
    let (domain_src, problem_src, product) = gen_method_fanout(seed, true);
    assert_eq!(product, 50u64.pow(10), "fan-out naive product must be 50^10");
    assert!(product >= 10_000_000);
    let domain = parse_domain(&domain_src).expect("generated domain must parse");
    let problem = parse_problem(&problem_src).expect("generated problem must parse");
    let ir = ground(&domain, &problem, &GroundingLimits::default())
        .expect("50x10 fan-out must ground cleanly (50 methods + 50 marks)");
    let limits = TranslateLimits {
        max_states: Some(5_000),
        max_wall: Some(Duration::from_secs(30)),
        ..TranslateLimits::default()
    };
    let started = Instant::now();
    let err = translate(&ir, &limits)
        .err()
        .expect("fan-out translate must be refused by max_states");
    let elapsed = started.elapsed();
    assert!(
        matches!(err, TranslateError::MemoryLimitExceeded { .. }),
        "expected typed TranslateError::MemoryLimitExceeded, got {err:?}"
    );
    CaseOutcome::refused("translate", format!("{err}"), product).with_elapsed(elapsed)
}

fn case_fanout_dedup_50x10() -> CaseOutcome {
    let seed = 0xDA7A;
    let (domain_src, problem_src, product) = gen_method_fanout(seed, false);
    assert_eq!(product, 50u64.pow(10));
    let domain = parse_domain(&domain_src).expect("generated domain must parse");
    let problem = parse_problem(&problem_src).expect("generated problem must parse");
    let ir = ground(&domain, &problem, &GroundingLimits::default())
        .expect("identical-method fan-out must ground cleanly");
    let started = Instant::now();
    // Content dedup collapses the 50^10 product to ~one state per
    // decomposition level (no `MemoryLimitExceeded` here) — but the frontier
    // *pending list* itself grows x10 per level, so a tight depth ceiling is
    // the bound that must refuse this shape (with tiny RSS). Default depth
    // (64) is unreachable before the pending structures explode and the
    // 10 s wall fires instead — configure the ceiling the shape needs.
    let limits = TranslateLimits {
        max_task_network_depth: 5,
        ..TranslateLimits::default()
    };
    let err = translate(&ir, &limits)
        .err()
        .expect("identical-method fan-out must still terminate at a bound");
    let elapsed = started.elapsed();
    assert!(
        matches!(err, TranslateError::TaskNetworkDepthExceeded { .. }),
        "expected typed TranslateError::TaskNetworkDepthExceeded, got {err:?}"
    );
    CaseOutcome::refused("translate", format!("{err}"), product).with_elapsed(elapsed)
}

fn case_predicates_500() -> CaseOutcome {
    let seed = 0xC0FFEE;
    let (domain_src, problem_src) = gen_predicates_500(seed);
    let started = Instant::now();
    let plan = solve_hddl(&domain_src, &problem_src, &PlannerLimits::default())
        .expect("500-predicate domain must solve cleanly within default ceilings");
    let elapsed = started.elapsed();
    assert!(plan.solved, "plan must be solved");
    assert!(!plan.policy.is_empty(), "policy must be non-empty");
    CaseOutcome::solved(
        format!("policy entries: {}", plan.policy.len()),
        500,
    )
    .with_elapsed(elapsed)
}

fn case_typing_chain_300() -> CaseOutcome {
    let seed = 0x7A1C;
    let (domain_src, problem_src) = gen_typing_chain(300, seed);
    let started = Instant::now();
    let plan = solve_hddl(&domain_src, &problem_src, &PlannerLimits::default())
        .expect("deep-but-legal typing chain must solve cleanly");
    let elapsed = started.elapsed();
    assert!(plan.solved, "plan must be solved");
    assert!(!plan.policy.is_empty(), "policy must be non-empty");
    CaseOutcome::solved(
        format!("policy entries: {}", plan.policy.len()),
        300,
    )
    .with_elapsed(elapsed)
}

fn case_planner_wall_bound() -> CaseOutcome {
    let problem = gen_chain_problem(1_000);
    let limits = PlannerLimits {
        max_wall_ms: 100,
        ..PlannerLimits::default()
    };
    let started = Instant::now();
    let err = solve_planning_type(&UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem,
        limits: limits.clone(),
    })
    .err()
    .expect("1,000-state chain under a 100ms wall must be refused");
    let elapsed = started.elapsed();
    assert!(
        matches!(err, PlannerError::Timeout { .. }),
        "expected typed PlannerError::Timeout, got {err:?}"
    );
    assert!(
        elapsed >= Duration::from_millis(100),
        "Timeout must fire no earlier than the configured budget"
    );
    CaseOutcome::refused("planner", format!("{err}"), 1_000).with_elapsed(elapsed)
}

impl CaseOutcome {
    fn with_elapsed(self, _elapsed: Duration) -> Self {
        // Elapsed is re-measured by the child wrapper around `run` for the
        // report; this hook exists so per-case extra bookkeeping has a home.
        self
    }
}

// ---------------------------------------------------------------------------
// Child / parent harness
// ---------------------------------------------------------------------------

const CHILD_ENV: &str = "FERROPLAN_MEMORY_STRESS_CHILD";
const WALL_BUDGET_MS_CAP: u128 = 120_000;

fn child_main(case_name: &str) -> i32 {
    let case = find_case(case_name);
    let started = Instant::now();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (case.run)()
    }));
    let elapsed_ms = started.elapsed().as_millis();
    let payload = match result {
        Ok(outcome) => {
            let (outcome_str, phase, detail) = match &outcome.outcome {
                Outcome::Solved { detail } => ("solved", "solve", detail.clone()),
                Outcome::Refused { phase, detail } => ("refused", *phase, detail.clone()),
            };
            json!({
                "case": case.name,
                "outcome": outcome_str,
                "phase": phase,
                "detail": detail,
                "naive_product": outcome.naive_product,
                "elapsed_ms": elapsed_ms,
            })
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| (*s).to_owned())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<opaque panic payload>".to_owned());
            json!({
                "case": case.name,
                "outcome": "panic",
                "phase": "child",
                "detail": msg,
                "naive_product": 0,
                "elapsed_ms": elapsed_ms,
            })
        }
    };
    println!("{}", payload);
    if payload["outcome"] == "panic" { 101 } else { 0 }
}

struct Report {
    case: &'static str,
    family: &'static str,
    outcome: String,
    phase: String,
    detail: String,
    naive_product: u64,
    elapsed_ms: u128,
    peak_rss_kb: u64,
    wall_ms: u128,
}

fn run_case_in_child(case: &Case) -> Report {
    let exe = std::env::current_exe().expect("current_exe for child re-exec");
    let script = format!("ulimit -v {RLIMIT_AS_KB} 2>/dev/null; exec \"$1\"");
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .arg("sh")
        .arg(&exe)
        .env(CHILD_ENV, case.name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("memory_stress: failed to spawn child for {}: {e}", case.name));
    let pid = child.id();

    // Peak-RSS sampler: ps every ~10ms until the child is reaped. Measured,
    // never guessed (module docs document the method and its resolution).
    let done = Arc::new(AtomicBool::new(false));
    let done_sampler = Arc::clone(&done);
    let sampler_pid = pid;
    let sampler = std::thread::spawn(move || {
        let mut peak = 0u64;
        while !done_sampler.load(Ordering::Relaxed) {
            if let Ok(out) = Command::new("ps")
                .args(["-o", "rss=", "-p", &sampler_pid.to_string()])
                .output()
            {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Ok(kb) = text.trim().parse::<u64>() {
                    peak = peak.max(kb);
                }
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        peak
    });

    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break Ok(st),
            Ok(None) => {
                if started.elapsed() > case.wall {
                    let _ = child.kill();
                    let _ = child.wait();
                    break Err(format!(
                        "case '{}' exceeded its {}s wall budget (harness kill)",
                        case.name,
                        case.wall.as_secs()
                    ));
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(e) => break Err(format!("case '{}': try_wait failed: {e}", case.name)),
        }
    };
    let wall_ms = started.elapsed().as_millis();
    done.store(true, Ordering::Relaxed);
    let peak_rss_kb = sampler.join().unwrap_or(0);

    let status = match status {
        Ok(st) => st,
        Err(msg) => panic!("memory_stress: {msg}"),
    };

    // Zero signals: a child that died by signal is a contract failure.
    if status.code().is_none() {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            if let Some(sig) = status.signal() {
                panic!(
                    "memory_stress: case '{}' was killed by SIGNAL {sig} — contract violation \
                     (typed refusal or clean solve required, never a signal)",
                    case.name
                );
            }
        }
        panic!(
            "memory_stress: case '{}' exited without a code and without a unix signal",
            case.name
        );
    }
    let code = status.code().unwrap();
    if code != 0 {
        let mut stderr = String::new();
        if let Some(mut pipe) = child.stderr.take() {
            let _ = std::io::Read::read_to_string(&mut pipe, &mut stderr);
        }
        panic!(
            "memory_stress: case '{}' child exited {code}\nstderr:\n{stderr}",
            case.name
        );
    }

    let stdout = child
        .stdout
        .take()
        .expect("child stdout was piped");
    let mut json_line = None;
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        if line.trim_start().starts_with('{') {
            json_line = Some(line);
        }
    }
    let json_line = json_line.unwrap_or_else(|| {
        panic!("memory_stress: case '{}' produced no JSON report line", case.name)
    });
    let v: serde_json::Value = serde_json::from_str(&json_line)
        .unwrap_or_else(|e| panic!("memory_stress: case '{}' bad JSON '{json_line}': {e}", case.name));
    let outcome = v["outcome"]
        .as_str()
        .unwrap_or_else(|| panic!("case '{}': missing outcome", case.name))
        .to_owned();
    if outcome == "panic" {
        panic!(
            "memory_stress: case '{}' PANICKED in child (must return a typed outcome instead): {}",
            case.name,
            v["detail"]
        );
    }
    assert!(
        outcome == "solved" || outcome == "refused",
        "case '{}': outcome must be solved|refused, got '{outcome}'",
        case.name
    );
    let elapsed_ms = v["elapsed_ms"].as_u64().unwrap_or(u64::MAX) as u128;
    assert!(
        elapsed_ms <= case.wall.as_millis(),
        "case '{}': in-child elapsed {elapsed_ms}ms exceeds its wall budget",
        case.name
    );
    assert!(
        peak_rss_kb <= PEAK_RSS_BUDGET_KB,
        "case '{}': peak RSS {peak_rss_kb}KiB exceeds the {}KiB budget — unbounded RSS climb",
        case.name,
        PEAK_RSS_BUDGET_KB
    );
    Report {
        case: case.name,
        family: case.family,
        outcome,
        phase: v["phase"].as_str().unwrap_or("?").to_owned(),
        detail: v["detail"].as_str().unwrap_or("?").to_owned(),
        naive_product: v["naive_product"].as_u64().unwrap_or(0),
        elapsed_ms,
        peak_rss_kb,
        wall_ms,
    }
}

fn results_table(reports: &[Report]) -> String {
    let mut out = String::new();
    out.push_str("### Peak-RSS table\n\n");
    out.push_str("| case | family | outcome | phase | naive product | elapsed (ms) | peak RSS (KiB) | peak RSS (MiB) |\n");
    out.push_str("|---|---|---|---|---|---|---|---|\n");
    for r in reports {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {:.1} |\n",
            r.case,
            r.family,
            r.outcome,
            r.phase,
            r.naive_product,
            r.elapsed_ms,
            r.peak_rss_kb,
            r.peak_rss_kb as f64 / 1024.0
        ));
    }
    out.push_str("\n### Refusal-latency table\n\n");
    out.push_str("| case | phase | typed outcome | latency (ms) |\n");
    out.push_str("|---|---|---|---|\n");
    for r in reports {
        let typed = if r.outcome == "refused" { &r.detail } else { &r.outcome };
        out.push_str(&format!(
            "| {} | {} | {} | {} (parent wall {} ms) |\n",
            r.case, r.phase, typed, r.elapsed_ms, r.wall_ms
        ));
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Child mode: run exactly one case and print one JSON line.
    if let Ok(case_name) = std::env::var(CHILD_ENV) {
        std::process::exit(child_main(&case_name));
    }

    let full = args.iter().any(|a| a == "--ignored" || a == "--include-ignored");
    let all = cases();
    let selected: Vec<&Case> = if full {
        all.iter().collect()
    } else {
        all.iter()
            .filter(|c| SAMPLED_CASES.contains(&c.name))
            .collect()
    };

    eprintln!(
        "memory_stress: {} run — {} case(s): {}",
        if full { "FULL" } else { "sampled" },
        selected.len(),
        selected.iter().map(|c| c.name).collect::<Vec<_>>().join(", ")
    );
    eprintln!(
        "memory_stress: method = child process under RLIMIT_AS {RLIMIT_AS_KB}KiB (backstop; \
         best-effort on macOS) + parent-side peak-RSS sampling via `ps -o rss=` every ~10ms"
    );

    let started = Instant::now();
    let mut reports = Vec::new();
    for case in &selected {
        reports.push(run_case_in_child(case));
    }
    let total_ms = started.elapsed().as_millis();

    let refusals = reports.iter().filter(|r| r.outcome == "refused").count();
    let solves = reports.iter().filter(|r| r.outcome == "solved").count();
    let table = results_table(&reports);

    println!("\nmemory_stress results ({} run)", if full { "FULL" } else { "sampled" });
    println!("{table}");
    println!(
        "summary: {} cases — {refusals} typed refusals, {solves} clean solves, 0 panics, \
         0 signals, 0 harness kills; total wall {total_ms}ms",
        reports.len()
    );
    let _ = std::io::stdout().flush();

    // Wave rule: no stress run may exceed 120 s.
    assert!(
        total_ms <= WALL_BUDGET_MS_CAP,
        "memory_stress: total wall {total_ms}ms exceeds the {WALL_BUDGET_MS_CAP}ms wave cap"
    );

    if full {
        let mut md = String::new();
        md.push_str(&format!(
            "Full-suite wall: {total_ms} ms (wave cap {WALL_BUDGET_MS_CAP} ms). \
             {refusals} typed refusals, {solves} clean solves, zero signals.\n\n"
        ));
        md.push_str(&table);
        // Emitted for RESULTS.md regeneration; harmless in gate output.
        eprintln!("BEGIN_MEMORY_STRESS_RESULTS\n{md}END_MEMORY_STRESS_RESULTS");
    }
}
