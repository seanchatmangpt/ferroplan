//! Scaling ladder (ticket `fond-htn-28`, wave-4 v26.9.17): find the capacity
//! knee per pipeline stage (parse / ground / translate / solve) for the
//! FOND-HTN front-end.
//!
//! Provenance (KOALA POLICY): both domain families are HAND-AUTHORED
//! parametric generators, inline in this file — no koala files are vendored,
//! koala is an external test oracle only. The families scale up the wave's
//! own micro-domain patterns (`tests/fixtures/fond-htn-micro/`):
//!
//! | family | pattern | shape | determinism |
//! |---|---|---|---|
//! | `chain-world` | blocksworld-style, total order | n blocks; one `build` method with n−1 totally-ordered `place` subtasks (pickup+stack each); classic pickup/stack physics | fully deterministic — the linear baseline |
//! | `transport-drop` | Transport (drop-retry) | n packages, m=8 locations; per-package `deliver` abstract task with a `oneof` drop that FAILS half the time and a retry-until-done method; explicit drive/load/drop with a seeded per-package destination | genuinely FOND — solvable only by the strong-cyclic fallback |
//!
//! Ladder: n ∈ {4, 8, 16, 32, 64, 128}. Each rung is driven stage-by-stage
//! through the exact public pipeline `solve_hddl` drives internally
//! (`parser::parse_domain/parse_problem` → `grounder::ground` →
//! `translate::translate` → `solve_planning_type(Fond)` — see
//! `ferroplan::hddl::solve_hddl_inner`), timing each stage separately. Every
//! stage is wall-bounded at 60 s:
//!
//! - parse: no internal wall check (see `solve_hddl`'s doc comment), so the
//!   test applies the same watchdog-thread pattern `solve_hddl` itself uses;
//! - ground/translate: `GroundingLimits::max_wall` / `TranslateLimits::max_wall`
//!   set to 60 s (count caps raised to 10M/2M so the WALL is the binding
//!   constraint, not the defaults' 10 s / 10k / 200k);
//! - solve: `PlannerLimits::max_wall_ms = 60_000`.
//!
//! The ladder stops at the first non-Solved rung per family and names the
//! stage that knelt. No cherry-picking: the full sweep (including refusals
//! and every skipped rung) is printed and written to RESULTS.md.
//!
//! Gates:
//! - CI heartbeat (always-on, sampled rung n=8):
//!   `cargo test -p ferroplan --test scaling_ladder` — exit 0.
//! - Long-run full ladder: `cargo test -p ferroplan --test scaling_ladder
//!   -- --ignored --nocapture` — exit 0; when `SCALING_LADDER_RESULTS=<path>`
//!   is set the machine-written RESULTS body (command, seed, limits, machine
//!   note, full sweep table, doubling factors) is written to that path for
//!   committing under `tests/fixtures/scaling-ladder/`.
//!
//! Reproducibility: the generators are deterministic given
//! `SEED = 20260917` (destination assignment uses a 64-bit LCG); state and
//! transition counts are exact functions of (family, n); only the wall
//! columns vary run-to-run. Run the long-run test under
//! `/usr/bin/time -l` and pass the max-RSS through
//! `SCALING_LADDER_RSS_KB` so the committed RESULTS records it measured,
//! not guessed (wave-4 rules).

use ferroplan::planning_runtime::{
    solve_planning_type, Goal, Method, PlannerLimits, PlanningProblem, State, Task, Transition,
    UniversalPlanningRequest,
};
use ferroplan::planning_types::PlanningType;
use ferroplan_hddl::{ast, grounder, parser, translate};
use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Seed for the transport-drop destination LCG (recorded in RESULTS).
const SEED: u64 = 2026_0917;
/// Fixed location count for the transport-drop family (n varies, m does not).
const TRANSPORT_LOCS: usize = 8;
/// The ladder rungs.
const LADDER: [usize; 6] = [4, 8, 16, 32, 64, 128];
/// Per-stage wall bound (ticket: wall-bounded at 60 s per stage).
const STAGE_WALL: Duration = Duration::from_secs(60);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    ChainWorld,
    TransportDrop,
}

impl Family {
    fn name(self) -> &'static str {
        match self {
            Family::ChainWorld => "chain-world",
            Family::TransportDrop => "transport-drop",
        }
    }
}

/// Walls per stage, milliseconds. A stage that was never reached records 0
/// (the `outcome` names where the rung stopped).
#[derive(Clone, Copy, Debug, Default)]
struct StageWalls {
    parse_ms: u128,
    ground_ms: u128,
    translate_ms: u128,
    solve_ms: u128,
}

/// Rung outcome. `Refused` = a stage hit its 60 s wall / count bound (the
/// stage that knelt); `NoPlan` = solve completed and proved the task
/// unsolvable by the FOND decision procedure (solve knelt, honestly);
/// `Solved` = policy found.
#[derive(Clone, Debug)]
enum Outcome {
    Solved { policy_entries: usize },
    NoPlan,
    Refused { stage: &'static str, reason: String },
}

impl Outcome {
    fn label(&self) -> String {
        match self {
            Outcome::Solved { policy_entries } => format!("solved(policy={policy_entries})"),
            Outcome::NoPlan => "nosolution(solve)".to_owned(),
            Outcome::Refused { stage, reason } => format!("refused({stage}): {reason}"),
        }
    }
}

#[derive(Clone, Debug)]
struct Rung {
    family: Family,
    n: usize,
    m: usize,
    ground_actions: usize,
    ground_methods: usize,
    states: usize,
    transitions: usize,
    walls: StageWalls,
    outcome: Outcome,
}

impl Rung {
    fn solved(&self) -> bool {
        matches!(self.outcome, Outcome::Solved { .. })
    }
}

// ---------------------------------------------------------------------------
// Limits: 60 s per stage; count caps raised so the WALL is what binds.
// ---------------------------------------------------------------------------

fn ground_limits() -> grounder::GroundingLimits {
    grounder::GroundingLimits {
        max_ground_actions: 10_000_000,
        max_ground_methods: 10_000_000,
        // Both pruning pre-passes deliberately OFF: this ladder measures the
        // *full combinatorial* grounding (its rung sizes are ground-instance
        // counts), not solve_hddl's pipeline — which since ticket fond-htn-60
        // runs `prune_irrelevant = true` behind the same caps.
        prune_unreachable: false,
        prune_irrelevant: false,
        max_wall: Some(STAGE_WALL),
    }
}

fn translate_limits() -> translate::TranslateLimits {
    translate::TranslateLimits {
        max_task_network_depth: 4096,
        max_wall: Some(STAGE_WALL),
        max_states: Some(2_000_000),
    }
}

fn solve_limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 60_000,
        max_states: 2_000_000,
        max_iterations: 100_000,
        max_depth: 100_000,
    }
}

// ---------------------------------------------------------------------------
// Seeded parametric generators (hand-authored; provenance in every header).
// ---------------------------------------------------------------------------

/// Total-order blocksworld-style family: build a tower `b1←b2←…←bn`
/// (bottom `b1` up? no — physical order: stack `b_{n-1}` ONTO `b_n` first,
/// then `b_{n-2}` onto `b_{n-1}`, …, `b1` onto `b2`; every stack's target is
/// clear when its turn comes). Goal: `(on b1 b2) (on b2 b3) … (on b_{n-1} b_n)`.
fn chain_world(n: usize) -> (String, String) {
    assert!(n >= 2, "chain-world needs at least 2 blocks");
    let mut domain = String::new();
    domain.push_str(
        ";; hand-authored, blocksworld-pattern total-order family (ticket fond-htn-28)\n",
    );
    domain.push_str(";; generated parametrically by tests/scaling_ladder.rs::chain_world; no koala files vendored\n");
    domain.push_str(
        "(define (domain chain-world)\n\
         \x20 (:types block)\n\
         \x20 (:predicates (on ?a - block ?b - block) (clear ?b - block)\n\
         \x20   (holding ?b - block) (on-table ?b - block))\n\
         \x20 (:action pickup\n\
         \x20   :parameters (?x - block)\n\
         \x20   :precondition (and (on-table ?x) (clear ?x))\n\
         \x20   :effect (and (holding ?x) (not (on-table ?x)) (not (clear ?x))))\n\
         \x20 (:action stack\n\
         \x20   :parameters (?x - block ?y - block)\n\
         \x20   :precondition (and (holding ?x) (clear ?y))\n\
         \x20   :effect (and (on ?x ?y) (clear ?x) (not (holding ?x)) (not (clear ?y))))\n\
         \x20 (:task place :parameters (?x - block ?y - block))\n\
         \x20 (:task build :parameters ())\n\
         \x20 (:method place\n\
         \x20   :parameters (?x - block ?y - block)\n\
         \x20   :task (place ?x ?y)\n\
         \x20   :precondition (and)\n\
         \x20   :ordered-subtasks (and (u1 (pickup ?x)) (u2 (stack ?x ?y))))\n",
    );
    // build: place b_{n-1} onto b_n FIRST … b1 onto b2 LAST (total order).
    let places: Vec<String> = (1..n)
        .rev()
        .map(|i| format!("(p{i} (place b{i} b{}))", i + 1))
        .collect();
    domain.push_str(&format!(
        "  (:method build\n\
         \x20   :parameters ()\n\
         \x20   :task (build)\n\
         \x20   :precondition (and)\n\
         \x20   :ordered-subtasks (and {})))\n",
        places.join(" ")
    ));

    let objects: Vec<String> = (1..=n).map(|i| format!("b{i}")).collect();
    let init: Vec<String> = (1..=n)
        .flat_map(|i| [format!("(on-table b{i})"), format!("(clear b{i})")])
        .collect();
    let goal: Vec<String> = (1..n).map(|i| format!("(on b{i} b{})", i + 1)).collect();
    let problem = format!(
        ";; hand-authored, blocksworld-pattern total-order family (ticket fond-htn-28)\n\
         ;; generated parametrically by tests/scaling_ladder.rs::chain_world; no koala files vendored\n\
         (define (problem chain-world-n{n})\n\
         \x20 (:domain chain-world)\n\
         \x20 (:objects {} - block)\n\
         \x20 (:init {})\n\
         \x20 (:goal (and {}))\n\
         \x20 (:htn :ordered-subtasks (and (r1 (build)))))\n",
        objects.join(" "),
        init.join(" "),
        goal.join(" ")
    );
    (domain, problem)
}

/// Transport-style family with a genuinely failing drop `oneof` (the
/// `drop-retry` micro pattern scaled up): n packages, m locations, seeded
/// destinations. Each package's `deliver` task dispatches
/// (drive→load→drop); the drop `oneof` succeeds (delivered) or FAILS
/// (empty branch), and a retry method loops until success — solvable only
/// by the strong-cyclic fixpoint. Truck drives between consecutive
/// destinations (explicit per-package dispatch methods carry the ground
/// route constants).
fn transport_drop(n: usize, m: usize, seed: u64) -> (String, String) {
    assert!(n >= 1, "transport-drop needs at least 1 package");
    assert!(m >= 2, "transport-drop needs at least 2 locations");
    // Seeded LCG (PCG-style 64-bit LCG; high bits used). Destinations avoid
    // a self-drive (drive d→d would assert and retract `(at d)` in one
    // branch); the first destination also differs from the depot l0.
    let lcg = |x: u64| {
        x.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407)
    };
    let mut x = seed;
    let mut dests: Vec<usize> = Vec::with_capacity(n);
    let mut prev = 0usize; // depot l0
    for _ in 0..n {
        loop {
            x = lcg(x);
            let d = ((x >> 33) as usize) % m;
            if d != prev {
                dests.push(d);
                prev = d;
                break;
            }
        }
    }

    let loc = |i: usize| format!("l{i}");

    let mut domain = String::new();
    domain
        .push_str(";; hand-authored, transport-pattern failing-drop family (ticket fond-htn-28)\n");
    domain.push_str(";; generated parametrically by tests/scaling_ladder.rs::transport_drop; no koala files vendored\n");
    domain.push_str(
        "(define (domain transport-drop)\n\
         \x20 (:types package loc)\n\
         \x20 (:predicates (at ?l - loc) (ready ?p - package) (holding ?p - package)\n\
         \x20   (delivered ?p - package) (dest ?p - package ?l - loc))\n\
         \x20 (:action drive\n\
         \x20   :parameters (?a - loc ?b - loc)\n\
         \x20   :precondition (at ?a)\n\
         \x20   :effect (and (at ?b) (not (at ?a))))\n\
         \x20 (:action load\n\
         \x20   :parameters (?p - package ?l - loc)\n\
         \x20   :precondition (at ?l)\n\
         \x20   :effect (and (holding ?p) (not (ready ?p))))\n\
         \x20 (:action drop\n\
         \x20   :parameters (?p - package ?l - loc)\n\
         \x20   :precondition (and (holding ?p) (at ?l))\n\
         \x20   :effect (oneof\n\
         \x20     (and (delivered ?p) (not (holding ?p)))\n\
         \x20     (and)))\n\
         \x20 (:task deliver :parameters (?p - package ?d - loc))\n\
         \x20 (:method deliver-done\n\
         \x20   :parameters (?p - package ?d - loc)\n\
         \x20   :task (deliver ?p ?d)\n\
         \x20   :precondition (delivered ?p)\n\
         \x20   :ordered-subtasks (and))\n\
         \x20 (:method deliver-retry\n\
         \x20   :parameters (?p - package ?d - loc)\n\
         \x20   :task (deliver ?p ?d)\n\
         \x20   :precondition (holding ?p)\n\
         \x20   :ordered-subtasks (and (t1 (drop ?p ?d)) (t2 (deliver ?p ?d))))\n",
    );
    // Per-package dispatch methods with ground route constants: truck moves
    // from the previous destination (or the depot l0) to this package's
    // destination, loads, then HANDS BACK to the (deliver ?p ?d) task — the
    // retry method owns the drop, so a failed drop leaves a live [deliver]
    // frontier (a one-shot drop here would discharge the network on failure
    // and leave a dead sink: package still held, `htn:done` falsely set,
    // strong-cyclic correctly NoPlan).
    let mut route_from = 0usize;
    for (i, &d) in dests.iter().enumerate() {
        let pi = format!("p{}", i + 1);
        let dl = loc(d);
        domain.push_str(&format!(
            "  (:method dispatch-{pi}\n\
             \x20   :parameters ()\n\
             \x20   :task (deliver {pi} {dl})\n\
             \x20   :precondition (ready {pi})\n\
             \x20   :ordered-subtasks (and (t1 (drive {} {dl})) (t2 (load {pi} {dl})) (t3 (deliver {pi} {dl}))))\n",
            loc(route_from)
        ));
        route_from = d;
    }
    domain.push_str(")\n");

    let packages: Vec<String> = (1..=n).map(|i| format!("p{i}")).collect();
    let locs: Vec<String> = (0..m).map(loc).collect();
    let init: Vec<String> = std::iter::once("(at l0)".to_owned())
        .chain(
            packages
                .iter()
                .enumerate()
                .map(|(i, p)| format!("(ready {p}) (dest {p} {})", loc(dests[i]))),
        )
        .collect();
    let goal: Vec<String> = packages
        .iter()
        .map(|p| format!("(delivered {p})"))
        .collect();
    let root: Vec<String> = packages
        .iter()
        .enumerate()
        .map(|(i, p)| format!("(r{} (deliver {p} {}))", i + 1, loc(dests[i])))
        .collect();
    let problem = format!(
        ";; hand-authored, transport-pattern failing-drop family (ticket fond-htn-28)\n\
         ;; generated parametrically by tests/scaling_ladder.rs::transport_drop; no koala files vendored\n\
         ;; seed={seed} dests={:?}\n\
         (define (problem transport-drop-n{n}-m{m})\n\
         \x20 (:domain transport-drop)\n\
         \x20 (:objects {} - package {} - loc)\n\
         \x20 (:init {})\n\
         \x20 (:goal (and {}))\n\
         \x20 (:htn :ordered-subtasks (and {})))\n",
        dests,
        packages.join(" "),
        locs.join(" "),
        init.join(" "),
        goal.join(" "),
        root.join(" ")
    );
    (domain, problem)
}

// ---------------------------------------------------------------------------
// Stage-by-stage rung driver (the exact pipeline solve_hddl_inner drives).
// ---------------------------------------------------------------------------

/// Faithful replica of `ferroplan::hddl::adapt_problem` (private there):
/// flat translate output → runtime `PlanningProblem`, bookkeeping
/// decomposition transitions at zero cost/duration, everything else at 1.
fn adapt_problem(p: translate::PlanningProblem) -> PlanningProblem {
    PlanningProblem {
        states: p
            .states
            .into_iter()
            .map(|s| State {
                id: s.id,
                facts: s.facts,
                ..Default::default()
            })
            .collect(),
        initial_states: p.initial_states,
        goal: Goal {
            facts: p.goal.facts,
            ..Default::default()
        },
        transitions: p
            .transitions
            .into_iter()
            .map(|t| {
                let is_bookkeeping = t.action.starts_with("htn:decompose:");
                Transition {
                    action: t.action,
                    from: t.from,
                    to: t.to,
                    cost: if is_bookkeeping { 0 } else { 1 },
                    duration: if is_bookkeeping { 0 } else { 1 },
                    reward: 0,
                    probability_ppm: t.probability_ppm,
                    observation: None,
                    requires: BTreeSet::new(),
                }
            })
            .collect(),
        tasks: p
            .tasks
            .into_iter()
            .map(|t| Task {
                id: t.id,
                primitive_action: t.primitive_action,
                ..Default::default()
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
        ..Default::default()
    }
}

fn generate(family: Family, n: usize) -> (String, String) {
    match family {
        Family::ChainWorld => chain_world(n),
        Family::TransportDrop => transport_drop(n, TRANSPORT_LOCS, SEED),
    }
}

/// Drives one rung stage-by-stage with a 60 s bound per stage. The parse
/// stage has no internal wall check, so — exactly like `solve_hddl` — it
/// runs under a watchdog thread here.
fn run_rung(family: Family, n: usize) -> Rung {
    let (domain_src, problem_src) = generate(family, n);
    let mut rung = Rung {
        family,
        n,
        m: if family == Family::TransportDrop {
            TRANSPORT_LOCS
        } else {
            0
        },
        ground_actions: 0,
        ground_methods: 0,
        states: 0,
        transitions: 0,
        walls: StageWalls::default(),
        outcome: Outcome::Refused {
            stage: "parse",
            reason: "unreached".to_owned(),
        },
    };

    // Stage 1: parse (watchdog-bounded; the parser has no internal wall).
    let t = Instant::now();
    let parsed = {
        let (d, p) = (domain_src.clone(), problem_src.clone());
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let r: Result<(ast::Domain, ast::Problem), String> = (|| {
                let d = parser::parse_domain(&d).map_err(|e| e.to_string())?;
                let p = parser::parse_problem(&p).map_err(|e| e.to_string())?;
                Ok((d, p))
            })();
            let _ = tx.send(r);
        });
        match rx.recv_timeout(STAGE_WALL) {
            Ok(r) => r,
            Err(_) => {
                rung.walls.parse_ms = STAGE_WALL.as_millis();
                rung.outcome = Outcome::Refused {
                    stage: "parse",
                    reason: format!("watchdog {STAGE_WALL:?} elapsed"),
                };
                return rung;
            }
        }
    };
    rung.walls.parse_ms = t.elapsed().as_millis();
    let (domain_ast, problem_ast) = match parsed {
        Ok(x) => x,
        Err(e) => {
            rung.outcome = Outcome::Refused {
                stage: "parse",
                reason: e,
            };
            return rung;
        }
    };

    // Stage 2: ground (internal 60 s wall + raised count caps).
    let t = Instant::now();
    let ir = match grounder::ground(&domain_ast, &problem_ast, &ground_limits()) {
        Ok(ir) => ir,
        Err(e) => {
            rung.walls.ground_ms = t.elapsed().as_millis();
            rung.outcome = Outcome::Refused {
                stage: "ground",
                reason: e.to_string(),
            };
            return rung;
        }
    };
    rung.walls.ground_ms = t.elapsed().as_millis();
    rung.ground_actions = ir.actions.len();
    rung.ground_methods = ir.methods.len();

    // Stage 3: translate (internal 60 s wall + 2M state guard).
    let t = Instant::now();
    let flat = match translate::translate(&ir, &translate_limits()) {
        Ok(f) => f,
        Err(e) => {
            rung.walls.translate_ms = t.elapsed().as_millis();
            rung.outcome = Outcome::Refused {
                stage: "translate",
                reason: e.to_string(),
            };
            return rung;
        }
    };
    rung.walls.translate_ms = t.elapsed().as_millis();
    rung.states = flat.states.len();
    rung.transitions = flat.transitions.len();

    // Stage 4: solve (Fond: strong fixpoint first, strong-cyclic fallback —
    // the exact dispatch solve_planning_type performs), 60 s wall.
    let request = UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: adapt_problem(flat),
        limits: solve_limits(),
    };
    let t = Instant::now();
    rung.outcome = match solve_planning_type(&request) {
        Ok(plan) if plan.solved => Outcome::Solved {
            policy_entries: plan.policy.len(),
        },
        Ok(_) => Outcome::NoPlan,
        Err(e) => Outcome::Refused {
            stage: "solve",
            reason: e.to_string(),
        },
    };
    rung.walls.solve_ms = t.elapsed().as_millis();
    rung
}

fn rung_row(r: &Rung) -> String {
    format!(
        "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
        r.family.name(),
        r.n,
        r.m,
        r.ground_actions,
        r.ground_methods,
        r.states,
        r.transitions,
        r.walls.parse_ms,
        r.walls.ground_ms,
        r.walls.translate_ms,
        r.walls.solve_ms,
        r.outcome.label()
    )
}

const SWEEP_HEADER: &str = "| family | n | m | ground_actions | ground_methods | states | transitions | parse_ms | ground_ms | translate_ms | solve_ms | outcome |\n|---|---|---|---|---|---|---|---|---|---|---|---|";

// ---------------------------------------------------------------------------
// Machine note + RESULTS writer (long-run test only).
// ---------------------------------------------------------------------------

fn machine_note() -> String {
    Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown-machine".to_owned())
}

fn utc_now() -> String {
    Command::new("date")
        .arg("-u")
        .arg("+%Y-%m-%dT%H:%M:%SZ")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "unknown-time".to_owned())
}

/// Doubling factors between the last two Solved rungs of a family
/// (wall ratios are indicative, count ratios are exact functions of n).
fn doubling_factors(rungs: &[&Rung]) -> Option<String> {
    let last = rungs.last()?;
    let prev = rungs.len().checked_sub(2).and_then(|i| rungs.get(i))?;
    let f = |a: u128, b: u128| {
        if b == 0 {
            "n/a".to_owned()
        } else {
            format!("{:.2}", a as f64 / b as f64)
        }
    };
    let cells = [
        last.family.name().to_owned(),
        format!("{}→{}", prev.n, last.n),
        f(last.walls.parse_ms, prev.walls.parse_ms),
        f(last.walls.ground_ms, prev.walls.ground_ms),
        f(last.walls.translate_ms, prev.walls.translate_ms),
        f(last.walls.solve_ms, prev.walls.solve_ms),
        f(last.ground_actions as u128, prev.ground_actions as u128),
        f(last.ground_methods as u128, prev.ground_methods as u128),
        f(last.states as u128, prev.states as u128),
        f(last.transitions as u128, prev.transitions as u128),
        prev.states.to_string(),
        last.states.to_string(),
    ];
    Some(format!("| {} |", cells.join(" | ")))
}

fn write_results(all: &[Rung], stops: &[(Family, String)]) {
    let Some(path) = std::env::var("SCALING_LADDER_RESULTS").ok() else {
        return;
    };
    let mut md = String::new();
    md.push_str("# Scaling ladder results (ticket fond-htn-28) — MACHINE-WRITTEN\n\n");
    md.push_str("Generated by `crates/ferroplan/tests/scaling_ladder.rs` (hand-authored parametric generators; no koala files vendored — koala is an external test oracle only).\n\n");
    md.push_str("## Reproduction\n\n```text\n");
    md.push_str(
        "command:  cargo test -p ferroplan --test scaling_ladder -- --ignored --nocapture\n",
    );
    md.push_str("           (with SCALING_LADDER_RESULTS set to this file's path)\n");
    md.push_str(&format!("seed:      {SEED} (transport-drop destination LCG; state/transition counts are exact functions of family+n)\n"));
    md.push_str(&format!("machine:   {}\n", machine_note()));
    md.push_str(&format!("ended:     {}\n", utc_now()));
    if let Ok(rss) = std::env::var("SCALING_LADDER_RSS_KB") {
        md.push_str(&format!("max_rss:   {rss} KB (measured via /usr/bin/time -l over the whole cargo test process)\n"));
    } else {
        md.push_str("max_rss:   not recorded this run\n");
    }
    md.push_str("```\n\n");
    md.push_str("## Stage limits (the bounds the ladder ran under)\n\n```text\n");
    md.push_str("parse:     watchdog thread, 60 s (parser has no internal wall check — solve_hddl's own pattern)\n");
    md.push_str("ground:    max_wall = 60 s, max_ground_actions = 10,000,000, max_ground_methods = 10,000,000, prune_unreachable = false, prune_irrelevant = false\n");
    md.push_str(
        "translate: max_wall = 60 s, max_states = 2,000,000, max_task_network_depth = 4096\n",
    );
    md.push_str("solve:     PlannerLimits { max_wall_ms = 60000, max_states = 2,000,000, max_iterations = 100,000, max_depth = 100,000 }, PlanningType::Fond (strong fixpoint, strong-cyclic fallback)\n");
    md.push_str("```\n\n");
    md.push_str("## Full sweep (no cherry-picking: every rung, including refusals)\n\n");
    md.push_str(SWEEP_HEADER);
    md.push('\n');
    for r in all {
        md.push_str(&rung_row(r));
        md.push('\n');
    }
    md.push('\n');
    md.push_str("## Where each family's ladder stopped\n\n");
    for (family, why) in stops {
        md.push_str(&format!("- `{}`: {why}\n", family.name()));
    }
    md.push('\n');
    md.push_str(
        "## Doubling factors (last two SOLVED rungs per family; linear doubling = 2.00)\n\n",
    );
    md.push_str("| family | rung | parse | ground | translate | solve | g_acts | g_methods | states | transitions | states@prev | states@last |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    let mut by_family: BTreeMap<&str, Vec<&Rung>> = Default::default();
    for r in all.iter().filter(|r| r.solved()) {
        by_family.entry(r.family.name()).or_default().push(r);
    }
    for (_, rungs) in by_family {
        if let Some(row) = doubling_factors(&rungs) {
            md.push_str(&row);
            md.push('\n');
        }
    }
    let _ = std::fs::write(&path, md);
}

// ---------------------------------------------------------------------------
// Gates.
// ---------------------------------------------------------------------------

/// CI heartbeat (always-on, ticket-specified sampled rung n=8): both
/// families must solve at n=8 with every stage inside its 60 s bound.
#[test]
fn ci_heartbeat_n8_sampled_rung() {
    for family in [Family::ChainWorld, Family::TransportDrop] {
        let r = run_rung(family, 8);
        println!("heartbeat {}", rung_row(&r));
        assert!(
            r.walls.parse_ms < 60_000
                && r.walls.ground_ms < 60_000
                && r.walls.translate_ms < 60_000
                && r.walls.solve_ms < 60_000,
            "{} n=8 blew a stage budget: {:?}",
            family.name(),
            r.walls
        );
        assert!(
            r.solved(),
            "{} n=8 heartbeat must solve, got {}",
            family.name(),
            r.outcome.label()
        );
        assert!(
            r.states > 0,
            "{} n=8 must translate to a non-empty state space",
            family.name()
        );
    }
}

/// Generators are deterministic: the same (family, n, seed) yields the same
/// problem text twice, and the transport destinations respect the
/// no-self-drive constraint and differ from the depot route start.
#[test]
fn generators_are_deterministic_and_wellformed() {
    for n in [4, 8, 16] {
        let (d1, p1) = chain_world(n);
        let (d2, p2) = chain_world(n);
        assert_eq!(&d1, &d2);
        assert_eq!(&p1, &p2);
        let (d1, p1) = transport_drop(n, TRANSPORT_LOCS, SEED);
        let (d2, p2) = transport_drop(n, TRANSPORT_LOCS, SEED);
        assert_eq!(&d1, &d2);
        assert_eq!(&p1, &p2);
        // Parse + ground sanity at the generator level (cheap rungs only).
        let dom = parser::parse_domain(&d1).expect("chain/transport domain parses");
        let prob = parser::parse_problem(&p1).expect("chain/transport problem parses");
        let ir = grounder::ground(&dom, &prob, &ground_limits()).expect("grounds");
        assert!(!ir.actions.is_empty() && !ir.methods.is_empty());
    }
}

/// Long-run full ladder (`-- --ignored`): n ∈ {4,8,16,32,64,128} for both
/// families, 60 s per stage, stop at the first non-Solved rung per family
/// and name the stage that knelt. Writes RESULTS when
/// `SCALING_LADDER_RESULTS` is set.
#[test]
#[ignore = "long-run scaling ladder (up to 6 rungs x 2 families x 60 s/stage); run with -- --ignored"]
fn full_scaling_ladder() {
    let mut all: Vec<Rung> = Vec::new();
    let mut stops: Vec<(Family, String)> = Vec::new();
    for family in [Family::ChainWorld, Family::TransportDrop] {
        let mut solved_rungs = 0usize;
        for &n in LADDER.iter() {
            let r = run_rung(family, n);
            println!("ladder {}", rung_row(&r));
            let solved = r.solved();
            let outcome = r.outcome.label();
            all.push(r);
            if solved {
                solved_rungs += 1;
            } else {
                stops.push((family, format!("stopped at n={n}: {outcome} (stage knelt)")));
                break;
            }
        }
        if solved_rungs == LADDER.len() {
            stops.push((
                family,
                "completed all rungs — no refusal on this ladder".to_owned(),
            ));
        }
        assert!(
            solved_rungs >= 1,
            "{} refused at the very first rung: the generator or pipeline is broken, not merely scaled",
            family.name()
        );
    }
    write_results(&all, &stops);
}
