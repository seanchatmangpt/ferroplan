//! FOND performance tripwires (ticket `fond-htn-26`, wave v26.9.17).
//!
//! The `benches/fond.rs` criterion suite REPORTS walls; this file makes the
//! generated configurations FAIL CI when they regress. Bounds are
//! profile-aware because this test runs under plain `cargo test`
//! (unoptimized dev profile, measured ~5-6x slower than the bench profile on
//! the recording machine — see `benches/BENCH-FOND.md` for both):
//!
//! - release (`cargo test --release`): the ticket's shape — 200-state strong
//!   fixpoint < 50 ms;
//! - dev: generous upper bounds (>= 5x the measured dev wall) so the same
//!   assertions stay CI-stable while still catching order-of-magnitude
//!   regressions (lost early-exits, super-linear round counts).
//!
//! Generator provenance: hand-authored for fond-htn-26, deterministic
//! splitmix64 (SEED recorded below); the generators are hand-shared with
//! `benches/fond.rs` because test targets cannot import bench targets — if
//! you change a generator here, mirror it there or the baselines in
//! `benches/BENCH-FOND.md` stop describing this tripwire's instances.

use ferroplan::{
    solve_hddl, solve_planning_type, PlannerLimits, PlanningProblem, PlanningType, UniversalGoal,
    UniversalPlanningRequest, UniversalState, UniversalTransition,
};
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

/// Fixed seed — identical to `benches/fond.rs`, so the tripwire replays the
/// exact instances the baselines were recorded against.
const SEED: u64 = 0x5EED_2026_0917;

fn limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 10_000,
        ..PlannerLimits::default()
    }
}

/// The ticket's example bound is release-shaped; dev gets a >= 5x-measured
/// generous counterpart so one assertion serves both profiles.
fn bound(release_ms: u64) -> Duration {
    if cfg!(debug_assertions) {
        Duration::from_millis(release_ms * 24)
    } else {
        Duration::from_millis(release_ms)
    }
}

// ----------------------------------------------------------- builders ----
// (hand-shared with benches/fond.rs — see the provenance note above)

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

struct SplitMix64(u64);

impl SplitMix64 {
    fn below(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        (z ^ (z >> 31)) % bound
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

fn mixed_ladder(n: usize) -> UniversalPlanningRequest {
    let decoys = n / 5;
    let budget = n - 1 - decoys;
    let traps = budget / 6;
    let rungs = budget - 2 * traps;

    let mut transitions = Vec::new();
    let mut states = Vec::with_capacity(n);

    states.push(state("g", &["goal"]));
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
    for k in 0..traps {
        let id = format!("p{k}");
        let dead = format!("d{k}");
        let rung = format!("m{}", k % rungs);
        transitions.push(edge("wait", &id, &id, 1_000_000));
        transitions.push(edge("venture", &id, &dead, 500_000));
        transitions.push(edge("venture", &id, &rung, 500_000));
        states.push(state(&id, &[]));
        states.push(state(&dead, &[]));
    }
    for j in 0..decoys {
        let id = format!("x{j}");
        transitions.push(edge("spin", &id, &id, 1_000_000));
        states.push(state(&id, &[]));
    }

    fond_problem(states, "m0", transitions)
}

// ---------------------------------------------------------- tripwires ----

/// Strong fixpoint (chain + lattice) at every ticket size must stay under its
/// bound: 50 ms release-shaped at 200 states (the ticket's example), scaled
/// generously for the dev profile. A regression here means the least-fixpoint
/// loop lost its early-exit or admits in super-linear rounds.
#[test]
fn strong_fixpoint_chain_and_lattice_within_threshold() {
    for (n, release_ms) in [(10_usize, 2_u64), (50, 8), (200, 50)] {
        let started = Instant::now();
        let plan = solve_planning_type(&chain(n)).expect("chain must solve");
        let wall = started.elapsed();
        assert!(plan.solved);
        assert_eq!(
            plan.policy.len(),
            n - 1,
            "chain policy covers every non-goal state"
        );
        assert!(
            wall < bound(release_ms),
            "{n}-state strong fixpoint (chain) regressed: {wall:?} >= {:?}",
            bound(release_ms)
        );
    }
    for (layers, width, release_ms) in [(5_usize, 2_usize, 2_u64), (10, 5, 8), (20, 10, 50)] {
        let started = Instant::now();
        let plan = solve_planning_type(&lattice(layers, width)).expect("lattice must solve");
        let wall = started.elapsed();
        assert!(plan.solved);
        assert!(
            wall < bound(release_ms),
            "{}x{} strong fixpoint (lattice) regressed: {wall:?} >= {:?}",
            layers,
            width,
            bound(release_ms)
        );
    }
}

/// Mixed strong-cyclic ladder at every ticket size must stay under its bound
/// (50 ms release-shaped at 200 states), must route through the strong-cyclic
/// fallback (the strong fixpoint NoPlans on the retry rungs), and Phase 3
/// must prune every trap from the returned policy (traps survive Phase 2 on
/// their pure self-loop; only the Phase-3 committable sweep removes them).
#[test]
fn strong_cyclic_mixed_ladder_within_threshold() {
    for (n, release_ms) in [(10_usize, 2_u64), (50, 8), (200, 50)] {
        let started = Instant::now();
        let plan = solve_planning_type(&mixed_ladder(n)).expect("mixed ladder must solve");
        let wall = started.elapsed();
        assert!(plan.solved);
        assert!(
            plan.notes.iter().any(|n| n.contains("strong-cyclic")),
            "{n}-state retry ladder must route through the strong-cyclic fallback, notes: {:?}",
            plan.notes
        );
        let pruned = plan
            .policy
            .iter()
            .filter(|e| e.state.starts_with('p'))
            .count();
        assert_eq!(
            pruned, 0,
            "Phase 3 must prune every trap (pure-self-loop survivor) from the policy"
        );
        assert!(
            wall < bound(release_ms),
            "{n}-state strong-cyclic mixed ladder regressed: {wall:?} >= {:?}",
            bound(release_ms)
        );
    }
}

/// The 7 hand-authored micro domains must each solve (or typed-NoPlan for the
/// deliberately unsolvable one) end-to-end well under 50 ms even unoptimized.
#[test]
fn solve_hddl_micro_fixtures_within_threshold() {
    const CASES: &[(&str, bool)] = &[
        ("drop-retry", true),
        ("tray-dirty", true),
        ("sense-then-branch", true),
        ("grow-loop", true),
        ("supervisor-fail", true),
        ("plain-chain", true),
        ("both-branches-deadend", false),
    ];
    for (dir, solvable) in CASES {
        let domain = std::fs::read_to_string(format!(
            "{}/tests/fixtures/fond-htn-micro/{dir}/domain.hddl",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("fixture domain reads");
        let problem = std::fs::read_to_string(format!(
            "{}/tests/fixtures/fond-htn-micro/{dir}/problem.hddl",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("fixture problem reads");
        let started = Instant::now();
        let result = solve_hddl(&domain, &problem, &limits());
        let wall = started.elapsed();
        assert!(
            wall < bound(50),
            "{dir}: solve_hddl regressed: {wall:?} >= {:?}",
            bound(50)
        );
        assert_eq!(
            result.is_ok(),
            *solvable,
            "{dir}: solvability flipped (regression or fixture drift)"
        );
    }
}
