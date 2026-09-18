//! FOND solver and `solve_hddl` micro-domain criterion benchmarks
//! (ticket `fond-htn-26`, wave v26.9.17). Idiom precedent:
//! `benches/ppddl.rs` / `benches/planning.rs`.
//!
//! Groups:
//!
//! 1. `fond_policy` — the strong (acyclic) fixpoint, driven through the
//!    public [`solve_planning_type`] dispatch on generated chains and
//!    lattices (10/50/200 states). Acyclic instances never trigger the
//!    strong-cyclic fallback, so the wall is the strong fixpoint alone.
//! 2. `fond_policy_strong_cyclic` — retry-loop ladders (10/50/200 total
//!    states, mixed): every rung advances only through a
//!    nondeterministic `{self, next}` action (so the strong fixpoint
//!    NoPlans and dispatch falls through), plus 20% self-loop-only decoy
//!    states (in every O(states x groups) sweep of Phases 1-3) and
//!    "trap" states that survive the Phase-2 witness prune on a pure
//!    self-loop but are pruned by the Phase-3 committable
//!    goal-reachability sweep — i.e. Phase 3 does real pruning work.
//!    The measured wall includes the preceding strong-fixpoint NoPlan
//!    pass, because that is the production dispatch path (`fond_policy`
//!    and `fond_policy_strong_cyclic` are private; only the dispatch is
//!    public).
//! 3. `solve_planning_type_dispatch` — `PlanningType::Fond` overhead on a
//!    minimal 2-state deterministic chain: validate + index + groups +
//!    one fixpoint round + policy build. The floor every Fond request pays.
//! 4. `solve_hddl` — the 7 hand-authored `tests/fixtures/fond-htn-micro/`
//!    domains end-to-end (parse + ground + translate + solve wall), plus
//!    per-stage walls for the stages the public `ferroplan_hddl` API
//!    exposes separately (parse / ground / translate). The solve stage has
//!    no public entry point of its own (`solve_hddl_inner` is private), so
//!    solve-only cost is observable only as `e2e` minus the front stages.
//!
//! Seeded generators are inline and fully deterministic (splitmix64,
//! no external RNG dependency, no external corpora in the bench path).
//! Provenance: hand-authored for fond-htn-26; the generator shares no code
//! with `planning_runtime.rs`. Run: `cargo bench -p ferroplan --bench fond`.

use criterion::{criterion_group, criterion_main, Criterion};
use ferroplan::{
    solve_hddl, solve_planning_type, PlannerLimits, PlanningProblem, PlanningType, UniversalGoal,
    UniversalPlanningRequest, UniversalState, UniversalTransition,
};
use ferroplan_hddl::grounder::{ground, GroundingLimits};
use ferroplan_hddl::parser::{parse_domain, parse_problem};
use ferroplan_hddl::translate::{translate, TranslateLimits};
use std::collections::BTreeSet;
use std::hint::black_box;

/// Fixed seed — the generators replay the exact same instances every run.
/// Same constant as `tests/fond_property.rs` so wave artifacts cross-reference.
const SEED: u64 = 0x5EED_2026_0917;

/// Wall budget for every solve (the solvers' own no-hang contract; none of
/// these instances comes anywhere near it).
fn limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 10_000,
        ..PlannerLimits::default()
    }
}

// ---------------------------------------------------------------- RNG ----

/// splitmix64: tiny, deterministic, dependency-free (same construction as
/// `tests/fond_property.rs`; hand-copied so the bench does not depend on
/// test code).
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Value in `0..bound` (bound >= 1).
    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

// ----------------------------------------------------------- builders ----

fn edge(action: &str, from: &str, to: &str, probability_ppm: u32) -> UniversalTransition {
    UniversalTransition {
        action: action.to_owned(),
        from: from.to_owned(),
        to: to.to_owned(),
        cost: 1,
        duration: 1,
        reward: 0,
        probability_ppm,
        observation: None,
        requires: BTreeSet::new(),
    }
}

fn state(id: &str, facts: &[&str]) -> UniversalState {
    UniversalState {
        id: id.to_owned(),
        facts: facts.iter().map(|&f| f.to_owned()).collect(),
        fluents: Default::default(),
    }
}

fn fond_problem(
    states: Vec<UniversalState>,
    initial: &str,
    transitions: Vec<UniversalTransition>,
) -> UniversalPlanningRequest {
    UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: PlanningProblem {
            states,
            initial_states: vec![initial.to_owned()],
            goal: UniversalGoal {
                facts: BTreeSet::from(["goal".to_owned()]),
                ..UniversalGoal::default()
            },
            transitions,
            ..PlanningProblem::default()
        },
        limits: limits(),
    }
}

/// Deterministic forward chain `s0 -> s1 -> ... -> s{n-1}`; the last state
/// is the goal. Worst-case shape for the strong fixpoint: the least fixpoint
/// admits exactly one rung per round, so it needs `n - 1` changing rounds —
/// the maximum round count for an acyclic instance.
fn chain(n: usize) -> UniversalPlanningRequest {
    let mut transitions = Vec::with_capacity(n - 1);
    let mut states = Vec::with_capacity(n);
    for i in 0..n - 1 {
        let (from, to) = (format!("s{i}"), format!("s{}", i + 1));
        transitions.push(edge("advance", &from, &to, 1_000_000));
    }
    for i in 0..n {
        let goal = i == n - 1;
        states.push(state(&format!("s{i}"), if goal { &["goal"] } else { &[] }));
    }
    fond_problem(states, "s0", transitions)
}

/// Layered DAG `L x W` (`n == L * W` states), two deterministic actions per
/// state (`stay` -> same column next layer, `cross` -> seeded random column
/// next layer). Still acyclic (strong-fixpoint territory, admitted one layer
/// per round) but with 2n action groups, so every group scan costs twice the
/// chain's.
fn lattice(layers: usize, width: usize) -> UniversalPlanningRequest {
    let mut rng = SplitMix64(SEED ^ (layers as u64).wrapping_mul(0x9E37_79B9));
    let mut transitions = Vec::new();
    let mut states = Vec::new();
    for l in 0..layers {
        for w in 0..width {
            let id = format!("s{l}_{w}");
            let goal = l + 1 == layers;
            states.push(state(&id, if goal { &["goal"] } else { &[] }));
            if goal {
                continue;
            }
            let next = format!("s{}_{}", l + 1, w);
            transitions.push(edge("stay", &id, &next, 1_000_000));
            let cross_w = rng.below(width as u64) as usize;
            let cross = format!("s{}_{}", l + 1, cross_w);
            transitions.push(edge("cross", &id, &cross, 1_000_000));
        }
    }
    fond_problem(states, "s0_0", transitions)
}

/// Mixed strong-cyclic ladder over EXACTLY `n` states:
///
/// - 20% **decoys**: pure self-loop states (`spin`). Never admitted into the
///   strong-cyclic region (no lucky run reaches the goal), but present in
///   every O(states x groups) sweep of Phases 1-3.
/// - **rungs**: `try` with outcomes `{self 50%, next 50%}` — a retry loop
///   the strong fixpoint cannot express (it NoPlans) but the strong-cyclic
///   Phase-2 greatest fixpoint admits.
/// - **traps**: `wait` (pure self-loop) plus `venture` with outcomes
///   `{dead sink, some rung}`. The venture outcome puts the trap in the weak
///   set and the pure `wait` self-loop keeps it alive through the Phase-2
///   witness prune (venture is never committable: its dead-sink outcome is
///   pruned) — exactly the shape only the Phase-3 committable
///   goal-reachability sweep can prune, and pruning a trap re-runs the
///   Phase-2 witness prune, so the alternation does real work.
/// - each trap also carries a terminal, non-goal **dead sink** state (never
///   in the weak set, so venture is never a closed action).
fn mixed_ladder(n: usize) -> UniversalPlanningRequest {
    let decoys = n / 5; // the ticket's 20%
    let budget = n - 1 - decoys; // states left after goal + decoys
    let traps = budget / 6; // 1 trap costs 2 states; ~1/6 of the budget
    let rungs = budget - 2 * traps;

    let mut transitions = Vec::new();
    let mut states = Vec::with_capacity(n);

    // Goal state.
    states.push(state("g", &["goal"]));
    // Rungs: m_i --try--> {m_i, m_{i+1}}; m_{rungs-1} --try--> g.
    for i in 0..rungs {
        let id = format!("m{i}");
        let next = if i + 1 == rungs {
            "g".to_owned()
        } else {
            format!("m{}", i + 1)
        };
        transitions.push(edge("try", &id, &id, 500_000));
        transitions.push(edge("try", &id, &next, 500_000));
        states.push(state(&id, &[]));
    }
    // Traps: p_k --wait--> {p_k}; p_k --venture--> {d_k, m_{k % rungs}}.
    for k in 0..traps {
        let id = format!("p{k}");
        let dead = format!("d{k}");
        let rung = format!("m{}", k % rungs);
        transitions.push(edge("wait", &id, &id, 1_000_000));
        transitions.push(edge("venture", &id, &dead, 500_000));
        transitions.push(edge("venture", &id, &rung, 500_000));
        states.push(state(&id, &[]));
        // Dead sink: terminal, non-goal, never in the weak set.
        states.push(state(&dead, &[]));
    }
    // Decoys: pure self-loops, scanned by every phase, never admitted.
    for j in 0..decoys {
        let id = format!("x{j}");
        transitions.push(edge("spin", &id, &id, 1_000_000));
        states.push(state(&id, &[]));
    }

    fond_problem(states, "m0", transitions)
}

fn solve_fond(request: &UniversalPlanningRequest) -> bool {
    solve_planning_type(black_box(request))
        .map(|plan| black_box(plan.solved))
        .expect("generated benchmark instance must solve")
}

// ------------------------------------------------------------ groups ----

/// Group (a): `fond_policy` strong fixpoint on generated chains/lattices.
fn bench_strong_fixpoint(c: &mut Criterion) {
    let mut g = c.benchmark_group("fond_policy");
    for n in [10_usize, 50, 200] {
        let chain = chain(n);
        g.bench_function(format!("strong_fixpoint_chain_{n}"), |b| {
            b.iter(|| solve_fond(&chain))
        });
    }
    for n in [10_usize, 50, 200] {
        let (layers, width) = match n {
            10 => (5, 2),
            50 => (10, 5),
            _ => (20, 10),
        };
        let lattice = lattice(layers, width);
        g.bench_function(format!("strong_fixpoint_lattice_{n}"), |b| {
            b.iter(|| solve_fond(&lattice))
        });
    }
    g.finish();
}

/// Group (b): `fond_policy_strong_cyclic` on mixed retry-loop ladders.
fn bench_strong_cyclic(c: &mut Criterion) {
    let mut g = c.benchmark_group("fond_policy_strong_cyclic");
    for n in [10_usize, 50, 200] {
        let ladder = mixed_ladder(n);
        g.bench_function(format!("strong_cyclic_mixed_ladder_{n}"), |b| {
            b.iter(|| solve_fond(&ladder))
        });
    }
    g.finish();
}

/// Group (c): `solve_planning_type(Fond)` dispatch overhead on a minimal
/// 2-state deterministic chain.
fn bench_dispatch(c: &mut Criterion) {
    let tiny = chain(2);
    c.bench_function("solve_planning_type_dispatch_tiny_chain", |b| {
        b.iter(|| solve_fond(&tiny))
    });
}

/// Group (d): `solve_hddl` on the 7 hand-authored micro domains, end-to-end
/// plus the separately-exposed parse/ground/translate stages.
fn bench_solve_hddl(c: &mut Criterion) {
    const DIRS: &[&str] = &[
        "drop-retry",
        "tray-dirty",
        "sense-then-branch",
        "grow-loop",
        "supervisor-fail",
        "plain-chain",
        "both-branches-deadend",
    ];
    let mut g = c.benchmark_group("solve_hddl");
    for dir in DIRS {
        let domain = fixture(dir, "domain.hddl");
        let problem = fixture(dir, "problem.hddl");
        let name = dir.replace('-', "_");
        g.bench_function(format!("{name}_e2e"), |b| {
            b.iter(|| {
                let result = solve_hddl(
                    black_box(&domain),
                    black_box(&problem),
                    black_box(&limits()),
                );
                black_box(result.is_ok())
            })
        });
        let parsed_domain = parse_domain(&domain).expect("fixture domain parses");
        let parsed_problem = parse_problem(&problem).expect("fixture problem parses");
        g.bench_function(format!("{name}_parse"), |b| {
            b.iter(|| {
                black_box(parse_domain(black_box(&domain)).is_ok())
                    && black_box(parse_problem(black_box(&problem)).is_ok())
            })
        });
        let grounded = ground(&parsed_domain, &parsed_problem, &GroundingLimits::default())
            .expect("fixture grounds");
        g.bench_function(format!("{name}_ground"), |b| {
            b.iter(|| {
                black_box(
                    ground(
                        black_box(&parsed_domain),
                        black_box(&parsed_problem),
                        &GroundingLimits::default(),
                    )
                    .is_ok(),
                )
            })
        });
        g.bench_function(format!("{name}_translate"), |b| {
            b.iter(|| {
                black_box(translate(black_box(&grounded), &TranslateLimits::default()).is_ok())
            })
        });
    }
    g.finish();
}

/// Path to a fixture file. The micro fixtures live INSIDE this package
/// (`crates/ferroplan/tests/fixtures/fond-htn-micro/`), so the path is
/// manifest-relative with no `..`.
fn fixture(dir: &str, file: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/fond-htn-micro/{dir}/{file}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

criterion_group!(
    benches,
    bench_strong_fixpoint,
    bench_strong_cyclic,
    bench_dispatch,
    bench_solve_hddl,
);
criterion_main!(benches);
