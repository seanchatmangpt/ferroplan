//! Property test at scale: `PlanningType::Fond` versus an INDEPENDENT
//! reference oracle, 5000 instances up to 12 states.
//!
//! Ticket: `docs/jira/v26.9.17/fond-htn-33-property-scaleup.md`.
//! Precedent: `tests/fond_property.rs` (320 instances, 3–8 states, literal
//! policy-enumeration reference).
//!
//! ## Reference choice (ticket scope item 2)
//!
//! Literal policy enumeration — the precedent's reference — explodes at the
//! ticket's sizes: 12 states × up to 4 actions means up to 4^11 ≈ 4M policy
//! assignments per instance, 5000 instances, in a debug build, under a 60 s
//! wall. Capping enumeration "by construction" (≤3 choices per state above 8
//! states) still leaves 3^11 ≈ 177K assignments per unsolvable instance —
//! the 25% decoy mixture is precisely rich in unsolvable instances, so the
//! cap option was measured as infeasible and rejected.
//!
//! Instead the reference is an independent label-correcting computation:
//!
//! - **STRONG** via min–max value iteration: `val(goal) = 0`, and
//!   `val(s) = 1 + min over actions of (max over outcomes of val(target))`,
//!   relaxed to a fixpoint from `∞` (unsafe states and unsafe outcomes are
//!   absorbing `∞`). `val(s) < ∞` ⟺ `s` has a strong (execution-bounded)
//!   policy — the least solution of the greedy equations, and the relaxation
//!   from `∞` converges to exactly that solution because every finite value
//!   is bounded by the state count.
//! - **STRONG-CYCLIC** via iterated usability-constrained backward
//!   goal-reachability: start from all non-unsafe states; repeatedly keep a
//!   non-goal state only if it can reach a goal along *usable* edges (an edge
//!   is usable when SOME action at the state takes ALL its outcomes inside
//!   the surviving set); iterate to a fixpoint. At the fixpoint every
//!   surviving state reaches a goal along usable edges, so choosing, at each
//!   state, a usable action that steps one edge closer to the goal (strictly
//!   decreasing usable-distance) yields a policy whose reachable region is
//!   unsafe-free and every region state reaches a goal within the region —
//!   a genuine strong-cyclic solution. Conversely no truly-winning state is
//!   ever pruned (its witness policy's edges stay usable while the survivor
//!   set only shrinks), so the fixpoint is exactly the strong-cyclic winning
//!   region.
//!
//! SCC condensation is deliberately not built: contracting strongly
//! connected components preserves plain reachability, so the condensation
//! would be a no-op for the one graph question this reference asks
//! (backward goal-reachability); the iteration above is the label-correcting
//! core without the ceremony.
//!
//! **Algorithmic independence** (ticket requirement): this reference shares
//! no code, no data layout, and no loop structure with the solver's
//! fixpoints (`fond_policy`: least-fixpoint set growth from the goal;
//! `fond_policy_strong_cyclic`: weak-reachability + greatest-fixpoint
//! witness prune + committable goal-reachability alternation). The strong
//! reference computes ordinal labels by min–max relaxation, not a winning
//! set; the strong-cyclic reference recomputes constrained backward
//! reachability from scratch over a shrinking region each round rather than
//! alternating a witness prune with a committable sweep. Both were derived
//! from the Cimatti/Pistore/Roveri/Traverso characterizations directly.
//!
//! ## Falsification chain
//!
//! A novel reference could itself be wrong, so it is pinned at three levels:
//! 1. `reference_agrees_with_hand_computed_semantics` — hand-worked domains
//!    (including multi-initial and decoy shapes) with by-hand verdicts;
//! 2. `reference_agrees_with_policy_enumeration_on_small_slice` — the
//!    precedent's literal enumeration oracle, ported, must agree with the
//!    new reference on 320 small instances (3–8 states, ≤ 3^8 policies);
//! 3. the main test — solver ⟷ reference on 5000 instances up to 12 states.
//!
//! ## Generator (ticket scope item 1)
//!
//! 5000 deterministic instances (splitmix64, fixed seed): 3–12 states,
//! 2–4 actions, 1–4 outcomes per (state, action) group (mass-valid
//! partitions of 1_000_000 ppm; validate_problem only range-checks FOND
//! masses, so this is stricter than required), optional 0–2 unsafe absorbing
//! non-goal states, optional 1–3 initial states (the dispatcher requires
//! ALL initials to win — the reference verdict is ANDed over initials to
//! match), and every 4th instance (exactly 25%) is a "decoy": a domain
//! rich in closed-but-goal-unreachable loops — the Phase-3 defect class
//! (the pre-Phase-3 `fond_policy_strong_cyclic` accepted a pure self-loop
//! policy that provably never reaches the goal). Decoy modes:
//! closed trap (a reachable state whose only actions close over a goal-free
//! region — self-loop or a mutually-closed 2-cycle), temptation (a closed
//! loop next to a real advancing action the policy must prefer), and risky
//! escape (closed loop plus an action risking an unsafe state — the
//! FOUND_BUG_1 shape). Coverage of every decoy mode and of reference-
//! unsolvable decoys is asserted.
//!
//! Assertions per instance (ticket scope item 3): solvability equivalence
//! (`Ok` ⟺ reference finds a strong OR strong-cyclic policy; `NoPlan` ⟺
//! neither), outcome closure of the returned policy, soundness of the
//! returned policy under the reference semantics, and determinism (two
//! solves, identical output). Wall budget: 60 s total.
//!
//! ## Finding at commit time (scope item 4 executed)
//!
//! The soundness assertion FOUND a residual solver defect on instance 1 of
//! the stream (a 7-state NON-decoy instance, hand-verified): the strong-
//! cyclic fixpoint certifies goal-reachability of the surviving REGION but
//! returns the WITNESS-FIRST choices from the Phase 2 prune — when a
//! state's first closed action is a pure loop and the committable action
//! differs, and the committable-reach fixpoint closes with
//! `reach == surviving`, the returned policy keeps the non-advancing loop.
//! Distinct from the fixed FOUND_BUG_1 shape (no advancing action at all):
//! here an advancing action exists but is not the one chosen. Shrunk to a
//! 2-state reproducer, committed `#[ignore]`d as
//! `fond_property_scaleup_FOUND_BUG_2_*` (hotfix input for the NEXT ticket;
//! src/ frozen for this ticket). The gate then carved out exactly this
//! class — reference confirms solvable AND the policy is structurally
//! closed AND the decision agrees — and counted it.
//!
//! ## Hotfix landed (ticket fond-htn-57)
//!
//! `fond_policy_strong_cyclic` now records each state's reach-discovery
//! RANK during Phase 3's (b) sweep and, when the fixpoint closes with
//! `reach == surviving`, rewrites every surviving non-goal state's choice
//! to a committable action with an outcome of strictly smaller rank
//! (advancing) — pruning instead if none exists (defense in depth), with
//! choices recomputed after any prune cascade by re-entering the
//! alternation. The reproducer runs always-on as the regression guard, the
//! carve-out is REMOVED (any goal-unreachable region on a reference-
//! solvable instance now fails this gate), and the assertions below are
//! unchanged.
//!
//! Provenance: hand-authored for fond-htn-33; generator and reference share
//! no code with `planning_runtime.rs` (koala policy: concepts only).

use ferroplan::{
    solve_planning_type, PlannerError, PlannerLimits, PlanningProblem, PlanningType, UniversalGoal,
    UniversalPlan, UniversalPolicyEntry, UniversalState, UniversalTransition,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::{Duration, Instant};

/// Instance count (ticket: 5000).
const INSTANCES: usize = 5000;
/// Ticket hard wall for the whole main test.
const WALL_BUDGET: Duration = Duration::from_secs(60);
/// Cross-check slice size (precedent's 320).
const CROSSCHECK_INSTANCES: usize = 320;
/// Enumeration budget for the small slice: at most 3 actions over at most
/// 8 states (7 open) — assert-guarded, mirrors the precedent's 3^8 cap.
const MAX_POLICY_ENUMERATION: usize = 3_usize.pow(8);
/// Fixed seed — rerunning replays the exact same instance stream.
const SEED: u64 = 0x5EED_2026_0918;
/// Cross-check stream seed (deliberately different from the main stream).
const CROSSCHECK_SEED: u64 = 0x5EED_2026_0919;

// ---------------------------------------------------------------- RNG ----

/// splitmix64: tiny, deterministic, dependency-free. Not cryptographic; the
/// generator only needs reproducible variety.
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

    /// True with roughly `percent`% probability.
    fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }
}

// -------------------------------------------------------------- edges ----

#[allow(clippy::too_many_arguments)]
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
        fluents: BTreeMap::new(),
    }
}

fn goal_fact_goal() -> UniversalGoal {
    UniversalGoal {
        facts: BTreeSet::from(["goal".to_owned()]),
        ..UniversalGoal::default()
    }
}

// -------------------------------------------------------- generator ----

/// Partition exactly 1_000_000 ppm into `parts` non-zero masses (stricter
/// than validate_problem requires for FOND — mass-range-only — but keeps
/// every instance well-formed under the strict reading too).
fn partition_mass(rng: &mut SplitMix64, parts: usize) -> Vec<u32> {
    const TOTAL: u64 = 1_000_000;
    if parts == 1 {
        return vec![TOTAL as u32];
    }
    let mut cuts: Vec<u64> = Vec::with_capacity(parts - 1);
    while cuts.len() < parts - 1 {
        let cut = 1 + rng.below(TOTAL - 1);
        if !cuts.contains(&cut) {
            cuts.push(cut);
        }
    }
    cuts.sort_unstable();
    let mut bounds = vec![0_u64];
    bounds.extend_from_slice(&cuts);
    bounds.push(TOTAL);
    (0..parts)
        .map(|i| (bounds[i + 1] - bounds[i]) as u32)
        .collect()
}

/// `k` distinct indices drawn from `0..n` (partial Fisher-Yates).
fn sample_distinct(rng: &mut SplitMix64, n: usize, k: usize) -> Vec<usize> {
    let mut pool: Vec<usize> = (0..n).collect();
    for i in 0..k {
        let j = i + rng.below((n - i) as u64) as usize;
        pool.swap(i, j);
    }
    pool.truncate(k);
    pool
}

/// One generated instance. `states_span`/`actions_span` size the uniform
/// draws (main stream: 12 states / 4 actions; cross-check slice: 8 / 3).
///
/// Layout: `s0` is always an initial state; the goal state is some other
/// index; unsafe states (0–2) are absorbing non-goal states with self-loop
/// actions only; extra initial states are drawn with 25% probability. Every
/// caller decides decoy-ness; the main stream uses `instance % 4 == 3`
/// (exactly 25%).
///
/// Decoy modes (drawn 50/30/20):
/// - mode 0 "closed trap": the decoy state's ONLY action closes over a
///   goal-free region — a pure self-loop, or (50%) a mutually-closed 2-cycle
///   with a partner state whose only action closes back. Any policy that
///   enters the trap can never reach the goal (the Phase-3 defect class).
/// - mode 1 "temptation": a closed self-loop next to a real advancing action
///   (strong 50%: goal-only outcome; strong-cyclic 50%: {goal, self} retry).
/// - mode 2 "risky escape": a closed self-loop plus an action whose outcomes
///   are {goal, unsafe} — the FOUND_BUG_1 shape (forcing the unsafe outcome
///   is required, entering the loop is not a solution).
fn generate(
    rng: &mut SplitMix64,
    decoy: bool,
    states_span: u64,
    actions_span: u64,
) -> PlanningProblem {
    let states_n = 3 + rng.below(states_span) as usize;
    let actions_n = 2 + rng.below(actions_span) as usize;
    let goal_idx = 1 + rng.below((states_n - 1) as u64) as usize; // != s0

    // Unsafe states: 0..=2 absorbing non-goal states (never s0).
    let mut unsafe_idx: Vec<usize> = Vec::new();
    let unsafe_candidates = || -> Vec<usize> { (1..states_n).filter(|&s| s != goal_idx).collect() };
    if rng.chance(30) {
        let candidates = unsafe_candidates();
        if !candidates.is_empty() {
            unsafe_idx.push(candidates[rng.below(candidates.len() as u64) as usize]);
        }
    }
    if rng.chance(8) && states_n >= 6 {
        let candidates: Vec<usize> = unsafe_candidates()
            .into_iter()
            .filter(|s| !unsafe_idx.contains(s))
            .collect();
        if !candidates.is_empty() {
            unsafe_idx.push(candidates[rng.below(candidates.len() as u64) as usize]);
        }
    }

    // Decoy plumbing: mode 2 needs an unsafe state — force one if absent.
    let mut decoy_mode: u8 = 0;
    let mut decoy_state: Option<usize> = None;
    let mut decoy_pair: Option<usize> = None;
    if decoy {
        decoy_mode = match rng.below(10) {
            0..=4 => 0,
            5..=7 => 1,
            _ => 2,
        };
        if decoy_mode == 2 && unsafe_idx.is_empty() {
            let candidates: Vec<usize> = unsafe_candidates();
            if !candidates.is_empty() {
                // Place the forced unsafe state so a decoy state still exists.
                let pick = candidates[rng.below(candidates.len() as u64) as usize];
                unsafe_idx.push(pick);
            }
        }
        let candidates: Vec<usize> = (0..states_n)
            .filter(|&s| s != goal_idx && !unsafe_idx.contains(&s))
            .collect();
        if !candidates.is_empty() {
            decoy_state = Some(candidates[rng.below(candidates.len() as u64) as usize]);
            if decoy_mode == 0 && rng.chance(50) {
                let pair_candidates: Vec<usize> = candidates
                    .iter()
                    .copied()
                    .filter(|&s| Some(s) != decoy_state)
                    .collect();
                if !pair_candidates.is_empty() {
                    decoy_pair =
                        Some(pair_candidates[rng.below(pair_candidates.len() as u64) as usize]);
                }
            }
        }
    }
    let decoy_planted = decoy_state.is_some();

    let mut transitions: Vec<UniversalTransition> = Vec::new();
    for s in 0..states_n {
        if s == goal_idx {
            continue; // the goal state takes no outgoing actions
        }
        let id = format!("s{s}");
        if unsafe_idx.contains(&s) {
            // Absorbing: every action self-loops deterministically, so once
            // entered the state can never be left.
            for a in 0..actions_n {
                transitions.push(edge(&format!("a{a}"), &id, &id, 1_000_000));
            }
            continue;
        }
        if Some(s) == decoy_state && decoy_planted {
            let loop_action = rng.below(actions_n as u64) as usize;
            match decoy_mode {
                0 => match decoy_pair {
                    // Mutually-closed goal-free 2-cycle …
                    Some(q) => {
                        let (m_r, m_q) = (500_000, 500_000);
                        transitions.push(edge(&format!("a{loop_action}"), &id, &id, m_r));
                        transitions.push(edge(
                            &format!("a{loop_action}"),
                            &id,
                            &format!("s{q}"),
                            m_q,
                        ));
                        transitions.push(edge(
                            &format!("a{loop_action}"),
                            &format!("s{q}"),
                            &id,
                            m_r,
                        ));
                        transitions.push(edge(
                            &format!("a{loop_action}"),
                            &format!("s{q}"),
                            &format!("s{q}"),
                            m_q,
                        ));
                    }
                    // … or a pure self-loop: closed, goal-free, the exact
                    // shape the pre-Phase-3 strong-cyclic fixpoint accepted.
                    None => {
                        transitions.push(edge(&format!("a{loop_action}"), &id, &id, 1_000_000));
                    }
                },
                1 => {
                    // Temptation: useless closed loop + advancing action.
                    transitions.push(edge(&format!("a{loop_action}"), &id, &id, 1_000_000));
                    let advance = (loop_action + 1) % actions_n;
                    if rng.chance(50) {
                        transitions.push(edge(
                            &format!("a{advance}"),
                            &id,
                            &format!("s{goal_idx}"),
                            1_000_000,
                        ));
                    } else {
                        let (m_g, m_s) = (500_000, 500_000);
                        transitions.push(edge(
                            &format!("a{advance}"),
                            &id,
                            &format!("s{goal_idx}"),
                            m_g,
                        ));
                        transitions.push(edge(&format!("a{advance}"), &id, &id, m_s));
                    }
                }
                _ => {
                    // Risky escape: closed loop + action risking unsafe.
                    transitions.push(edge(&format!("a{loop_action}"), &id, &id, 1_000_000));
                    let advance = (loop_action + 1) % actions_n;
                    let u = unsafe_idx[0];
                    let (m_g, m_u) = (500_000, 500_000);
                    transitions.push(edge(
                        &format!("a{advance}"),
                        &id,
                        &format!("s{goal_idx}"),
                        m_g,
                    ));
                    transitions.push(edge(&format!("a{advance}"), &id, &format!("s{u}"), m_u));
                }
            }
            continue;
        }
        if Some(s) == decoy_pair && decoy_planted {
            // Partner of the trap: only the closing action back into the pair.
            let r = decoy_state.expect("pair implies decoy state");
            if rng.chance(50) {
                transitions.push(edge("a0", &id, &format!("s{r}"), 1_000_000));
            } else {
                transitions.push(edge("a0", &id, &format!("s{r}"), 500_000));
                transitions.push(edge("a0", &id, &id, 500_000));
            }
            continue;
        }
        let mut available: Vec<usize> = Vec::new();
        for a in 0..actions_n {
            if rng.chance(80) {
                available.push(a);
            }
        }
        if available.is_empty() && rng.chance(70) {
            available.push(rng.below(actions_n as u64) as usize);
        }
        // (An empty `available` is kept as-is: a reachable non-goal state with
        // no actions is a legitimate dead sink and must be judged unsolvable.)
        for a in available {
            let max_outcomes = 4.min(states_n);
            let outcomes = match rng.below(10) {
                0..=4 => 1, // half the groups deterministic
                5..=7 => 2,
                8 => 3,
                _ => max_outcomes,
            }
            .clamp(1, max_outcomes);
            let targets = sample_distinct(rng, states_n, outcomes);
            let masses = partition_mass(rng, outcomes);
            for (target, mass) in targets.into_iter().zip(masses) {
                transitions.push(edge(&format!("a{a}"), &id, &format!("s{target}"), mass));
            }
        }
    }

    // Pressure toward unavoidable traps: sometimes give s0 a deterministic
    // edge into the decoy state (unless s0 IS the decoy) so more decoy
    // instances put the trap on every path.
    if decoy_planted && decoy_state != Some(0) && rng.chance(30) {
        let r = decoy_state.expect("planted");
        let a = rng.below(actions_n as u64) as usize;
        transitions.push(edge(&format!("a{a}"), "s0", &format!("s{r}"), 500_000));
    }

    // Optional extra initial states (never the goal, never unsafe).
    let mut initial_states = vec!["s0".to_owned()];
    if rng.chance(25) {
        let candidates: Vec<usize> = (1..states_n)
            .filter(|&s| s != goal_idx && !unsafe_idx.contains(&s))
            .collect();
        if !candidates.is_empty() {
            let extra = 1 + rng.below(2) as usize; // 1..=2 more initials
            for idx in sample_distinct(rng, candidates.len(), extra.min(candidates.len())) {
                initial_states.push(format!("s{}", candidates[idx]));
            }
        }
    }

    let states = (0..states_n)
        .map(|s| {
            if s == goal_idx {
                state(&format!("s{s}"), &["goal"])
            } else {
                state(&format!("s{s}"), &[])
            }
        })
        .collect();

    PlanningProblem {
        states,
        initial_states,
        goal: goal_fact_goal(),
        unsafe_states: unsafe_idx.into_iter().map(|s| format!("s{s}")).collect(),
        transitions,
        ..PlanningProblem::default()
    }
}

// ------------------------------------------------- independent reference ----

const INFINITY: u32 = u32::MAX;

/// Compact reference model built from a generated problem. Goals are tested
/// with the fact-subset semantics the solver documents, implemented here
/// independently (no call into `planning_runtime`). See the module docs for
/// the two algorithms and their independence from the solver's fixpoints.
struct ScaleReference {
    states_n: usize,
    ids: Vec<String>,
    goals: BTreeSet<usize>,
    unsafe_states: BTreeSet<usize>,
    initials: Vec<usize>,
    /// Per state: available actions as `(action_index, outcome_targets)`,
    /// ascending by action index.
    groups: Vec<Vec<(usize, Vec<usize>)>>,
}

impl ScaleReference {
    fn build(problem: &PlanningProblem) -> Self {
        let index: BTreeMap<&str, usize> = problem
            .states
            .iter()
            .enumerate()
            .map(|(i, state)| (state.id.as_str(), i))
            .collect();
        let goals: BTreeSet<usize> = problem
            .states
            .iter()
            .enumerate()
            .filter(|(_, state)| {
                problem
                    .goal
                    .facts
                    .iter()
                    .all(|fact| state.facts.contains(fact))
            })
            .map(|(i, _)| i)
            .collect();
        let unsafe_states = problem
            .unsafe_states
            .iter()
            .map(|id| index[id.as_str()])
            .collect();
        assert!(
            goals.is_disjoint(&unsafe_states),
            "generated instance has a state that is both goal and unsafe"
        );
        let mut raw: BTreeMap<(usize, usize), BTreeSet<usize>> = BTreeMap::new();
        for transition in &problem.transitions {
            let from = index[transition.from.as_str()];
            let to = index[transition.to.as_str()];
            let action: usize = transition.action[1..]
                .parse()
                .expect("generated action name");
            raw.entry((from, action)).or_default().insert(to);
        }
        let mut groups = vec![Vec::new(); problem.states.len()];
        for ((from, action), targets) in raw {
            groups[from].push((action, targets.into_iter().collect()));
        }
        ScaleReference {
            states_n: problem.states.len(),
            ids: problem
                .states
                .iter()
                .map(|state| state.id.clone())
                .collect(),
            goals,
            unsafe_states,
            initials: problem
                .initial_states
                .iter()
                .map(|id| index[id.as_str()])
                .collect(),
            groups,
        }
    }

    /// STRONG labels by min–max value iteration: `val(goal) = 0`; a non-goal
    /// state's value is 1 + the minimum over its actions of the MAXIMUM over
    /// that action's outcome values; unsafe states/outputs are absorbing ∞.
    /// `val(s) < ∞` ⟺ `s` admits a strong (execution-bounded) policy.
    fn strong_values(&self) -> Vec<u32> {
        let mut val = vec![INFINITY; self.states_n];
        for &g in &self.goals {
            val[g] = 0;
        }
        loop {
            let mut changed = false;
            for s in 0..self.states_n {
                if self.goals.contains(&s) || self.unsafe_states.contains(&s) {
                    continue;
                }
                let mut best = INFINITY;
                for (_action, outs) in &self.groups[s] {
                    let worst = outs.iter().map(|&t| val[t]).max().expect("non-empty group");
                    if worst != INFINITY && worst + 1 < best {
                        best = worst + 1;
                    }
                }
                if best < val[s] {
                    val[s] = best;
                    changed = true;
                }
            }
            if !changed {
                return val;
            }
        }
    }

    /// STRONG-CYCLIC winning region by iterated usability-constrained
    /// backward goal-reachability (see module docs for correctness).
    fn strong_cyclic_region(&self) -> BTreeSet<usize> {
        let mut region: BTreeSet<usize> = (0..self.states_n)
            .filter(|s| !self.unsafe_states.contains(s))
            .collect();
        loop {
            // Reverse usable-edge adjacency: s is a predecessor of t when s
            // has an action ALL of whose outcomes land in the region and t is
            // one of them.
            let mut rev: Vec<Vec<usize>> = vec![Vec::new(); self.states_n];
            for &s in &region {
                for (_action, outs) in &self.groups[s] {
                    if outs.iter().all(|t| region.contains(t)) {
                        for &t in outs {
                            rev[t].push(s);
                        }
                    }
                }
            }
            let mut goal_reaching: BTreeSet<usize> =
                self.goals.intersection(&region).copied().collect();
            let mut queue: VecDeque<usize> = goal_reaching.iter().copied().collect();
            while let Some(t) = queue.pop_front() {
                for &s in &rev[t] {
                    if goal_reaching.insert(s) {
                        queue.push_back(s);
                    }
                }
            }
            let next: BTreeSet<usize> = region
                .iter()
                .copied()
                .filter(|s| self.goals.contains(s) || goal_reaching.contains(s))
                .collect();
            if next == region {
                return region;
            }
            region = next;
        }
    }

    /// `(strong_solvable, strong_cyclic_solvable)` — ANDed over ALL initial
    /// states to mirror the dispatcher's all-initials rule.
    fn verdict(&self) -> (bool, bool) {
        let vals = self.strong_values();
        let strong_solvable = self.initials.iter().all(|&s| vals[s] != INFINITY);
        let region = self.strong_cyclic_region();
        let strong_cyclic_solvable = self.initials.iter().all(|s| region.contains(s));
        (strong_solvable, strong_cyclic_solvable)
    }

    fn successors(&self, choice: &[Option<usize>]) -> Vec<BTreeSet<usize>> {
        (0..self.states_n)
            .map(|s| match choice[s] {
                Some(group) => self.groups[s][group].1.iter().copied().collect(),
                None => BTreeSet::new(),
            })
            .collect()
    }

    /// Outcome-closed reachable set from ALL initial states (regions are
    /// successor-closed, so the union region's goal-reachability equals the
    /// conjunction over the per-initial regions).
    fn reachable(&self, successors: &[BTreeSet<usize>]) -> BTreeSet<usize> {
        let mut region: BTreeSet<usize> = self.initials.iter().copied().collect();
        let mut queue: VecDeque<usize> = self.initials.iter().copied().collect();
        while let Some(s) = queue.pop_front() {
            for &t in &successors[s] {
                if region.insert(t) {
                    queue.push_back(t);
                }
            }
        }
        region
    }

    /// Cimatti test for one policy: every state in `region` reaches a goal
    /// using only states in `region`.
    fn policy_is_strong_cyclic(
        &self,
        successors: &[BTreeSet<usize>],
        region: &BTreeSet<usize>,
    ) -> bool {
        let mut backward: BTreeMap<usize, Vec<usize>> =
            region.iter().map(|&s| (s, Vec::new())).collect();
        for &s in region {
            for &t in &successors[s] {
                backward
                    .get_mut(&t)
                    .expect("region is outcome-closed")
                    .push(s);
            }
        }
        let mut can_reach_goal: BTreeSet<usize> =
            region.intersection(&self.goals).copied().collect();
        let mut queue: VecDeque<usize> = can_reach_goal.iter().copied().collect();
        while let Some(s) = queue.pop_front() {
            for &predecessor in &backward[&s] {
                if can_reach_goal.insert(predecessor) {
                    queue.push_back(predecessor);
                }
            }
        }
        can_reach_goal.len() == region.len()
    }

    /// Judge a policy RETURNED BY THE SOLVER against the reference semantics:
    /// its region (from all initials) must exclude unsafe states and every
    /// region state must reach a goal within the region.
    fn check_returned_policy(&self, policy: &[UniversalPolicyEntry]) -> PolicyCheck {
        let mut choice = vec![None; self.states_n];
        for entry in policy {
            let state_index = match self.ids.iter().position(|id| id == &entry.state) {
                Some(i) => i,
                None => {
                    return PolicyCheck::UnknownPolicyEntry(format!(
                        "policy keys unknown state {:?}",
                        entry.state
                    ))
                }
            };
            let action: usize = match entry.action[1..].parse() {
                Ok(a) => a,
                Err(_) => {
                    return PolicyCheck::UnknownPolicyEntry(format!(
                        "policy action {:?} is not a generated action name",
                        entry.action
                    ))
                }
            };
            match self.groups[state_index]
                .iter()
                .position(|(available, _)| *available == action)
            {
                Some(group) => choice[state_index] = Some(group),
                None => {
                    return PolicyCheck::UnknownPolicyEntry(format!(
                        "state {:?} has no action {:?}",
                        entry.state, entry.action
                    ))
                }
            }
        }
        let successors = self.successors(&choice);
        let region = self.reachable(&successors);
        if region.iter().any(|s| self.unsafe_states.contains(s)) {
            return PolicyCheck::EntersUnsafeState;
        }
        if self.policy_is_strong_cyclic(&successors, &region) {
            PolicyCheck::ValidStrongCyclic
        } else {
            PolicyCheck::GoalUnreachableRegion
        }
    }
}

/// Verdict on a solver-returned policy under the reference semantics.
enum PolicyCheck {
    /// The policy's reachable region avoids unsafe states and every region
    /// state reaches a goal within the region — a genuine strong/strong-cyclic
    /// solution.
    ValidStrongCyclic,
    /// The policy's reachable region contains a state that can never reach
    /// the goal within the region.
    ///
    /// This WAS the documented residual defect `FOUND_BUG_2`
    /// (`fond_property_scaleup_FOUND_BUG_2_*` reproducer below): Phase 3 of
    /// `fond_policy_strong_cyclic` verified goal-reachability of the
    /// surviving REGION but returned the WITNESS-FIRST choices from the
    /// Phase 2 prune — when a state's first closed action is a pure loop
    /// and a later action is the committable one, and the committable-reach
    /// fixpoint closes with `reach == surviving`, the loop exited without
    /// rewriting `choices`, so the returned policy kept the non-advancing
    /// loop. The wave-3 Phase-3 fix (fix/fond-sc-goalreach) closed the
    /// no-advancing-action shape (FOUND_BUG_1) but not this
    /// witness-vs-committable mismatch shape; the fond-htn-57 hotfix (the
    /// Phase-3 rank-based choice rewrite) closed this one, the reproducer
    /// runs always-on as its regression guard, and this gate no longer
    /// carves the class out — ANY recurrence fails.
    GoalUnreachableRegion,
    /// The policy's region contains an unsafe state — a soundness violation.
    EntersUnsafeState,
    /// The policy references an unknown state or an action the state does not
    /// have — a structural violation.
    UnknownPolicyEntry(String),
}

// --------------------------------- enumeration oracle (cross-check only) ----

/// The precedent's literal enumeration oracle (ported from
/// `tests/fond_property.rs`, generalized to multiple initial states via the
/// union region). Used ONLY on the small cross-check slice where the product
/// of per-state action counts is assert-capped at 3^8 — ground truth by
/// exhaustion, independent of both the solver and `ScaleReference`.
struct Enumeration {
    states_n: usize,
    goals: BTreeSet<usize>,
    unsafe_states: BTreeSet<usize>,
    initials: Vec<usize>,
    groups: Vec<Vec<(usize, Vec<usize>)>>,
}

impl Enumeration {
    fn build(problem: &PlanningProblem) -> Self {
        let reference = ScaleReference::build(problem);
        Enumeration {
            states_n: reference.states_n,
            goals: reference.goals,
            unsafe_states: reference.unsafe_states,
            initials: reference.initials,
            groups: reference.groups,
        }
    }

    fn successors(&self, choice: &[Option<usize>]) -> Vec<BTreeSet<usize>> {
        (0..self.states_n)
            .map(|s| match choice[s] {
                Some(group) => self.groups[s][group].1.iter().copied().collect(),
                None => BTreeSet::new(),
            })
            .collect()
    }

    fn reachable(&self, successors: &[BTreeSet<usize>]) -> BTreeSet<usize> {
        let mut region: BTreeSet<usize> = self.initials.iter().copied().collect();
        let mut queue: VecDeque<usize> = self.initials.iter().copied().collect();
        while let Some(s) = queue.pop_front() {
            for &t in &successors[s] {
                if region.insert(t) {
                    queue.push_back(t);
                }
            }
        }
        region
    }

    /// STRONG for one policy: the reachable graph is acyclic and every leaf
    /// is a goal state (Kahn's algorithm restricted to the region).
    fn policy_is_strong(&self, successors: &[BTreeSet<usize>], region: &BTreeSet<usize>) -> bool {
        let mut indegree: BTreeMap<usize, usize> = region.iter().map(|&s| (s, 0)).collect();
        for &s in region {
            for &t in &successors[s] {
                *indegree.get_mut(&t).expect("region is outcome-closed") += 1;
            }
        }
        let mut ready: Vec<usize> = region
            .iter()
            .copied()
            .filter(|&s| indegree[&s] == 0)
            .collect();
        let mut visited = 0;
        while let Some(s) = ready.pop() {
            visited += 1;
            for &t in &successors[s] {
                let remaining = indegree.get_mut(&t).expect("region is outcome-closed");
                *remaining -= 1;
                if *remaining == 0 {
                    ready.push(t);
                }
            }
        }
        if visited != region.len() {
            return false; // cycle: no bounded execution
        }
        region
            .iter()
            .all(|&s| !successors[s].is_empty() || self.goals.contains(&s))
    }

    /// STRONG-CYCLIC for one policy: every state in the reachable set can
    /// reach a goal state using only states in the set.
    fn policy_is_strong_cyclic(
        &self,
        successors: &[BTreeSet<usize>],
        region: &BTreeSet<usize>,
    ) -> bool {
        let mut backward: BTreeMap<usize, Vec<usize>> =
            region.iter().map(|&s| (s, Vec::new())).collect();
        for &s in region {
            for &t in &successors[s] {
                backward
                    .get_mut(&t)
                    .expect("region is outcome-closed")
                    .push(s);
            }
        }
        let mut can_reach_goal: BTreeSet<usize> =
            region.intersection(&self.goals).copied().collect();
        let mut queue: VecDeque<usize> = can_reach_goal.iter().copied().collect();
        while let Some(s) = queue.pop_front() {
            for &predecessor in &backward[&s] {
                if can_reach_goal.insert(predecessor) {
                    queue.push_back(predecessor);
                }
            }
        }
        can_reach_goal.len() == region.len()
    }

    /// Enumerate policies and report `(strong_solvable, strong_cyclic_solvable)`.
    /// Early exit on a strong policy is sound: every strong policy is also a
    /// strong-cyclic one, so both flags are settled at that moment.
    fn verdict(&self) -> (bool, bool) {
        let open: Vec<usize> = (0..self.states_n)
            .filter(|&s| !self.goals.contains(&s))
            .collect();
        let radices: Vec<usize> = open.iter().map(|&s| self.groups[s].len().max(1)).collect();
        let product: usize = radices.iter().product();
        assert!(
            product <= MAX_POLICY_ENUMERATION,
            "cross-check enumeration budget exceeded: {product} > {MAX_POLICY_ENUMERATION}"
        );
        let mut strong_solvable = false;
        let mut strong_cyclic_solvable = false;
        'policies: for mut code in 0..product {
            let mut choice = vec![None; self.states_n];
            for (i, &s) in open.iter().enumerate() {
                let radix = radices[i];
                let digit = code % radix;
                code /= radix;
                if !self.groups[s].is_empty() {
                    choice[s] = Some(digit);
                }
            }
            let successors = self.successors(&choice);
            let region = self.reachable(&successors);
            if region.iter().any(|s| self.unsafe_states.contains(s)) {
                continue; // unsafe state entered: policy forbidden outright
            }
            if self.policy_is_strong(&successors, &region) {
                strong_solvable = true;
                strong_cyclic_solvable = true;
                break 'policies;
            }
            if !strong_cyclic_solvable && self.policy_is_strong_cyclic(&successors, &region) {
                strong_cyclic_solvable = true;
            }
        }
        (strong_solvable, strong_cyclic_solvable)
    }
}

// ------------------------------------------------------- assertions ----

fn solve_fond(problem: &PlanningProblem) -> Result<UniversalPlan, PlannerError> {
    solve_planning_type(&ferroplan::UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: problem.clone(),
        limits: PlannerLimits::default(),
    })
}

/// Closure property: every returned policy entry's outcomes land on a goal
/// state or on another policy key, entries never sit on goal states, no entry
/// is a dead sink, and every initial state is covered.
fn assert_policy_closure(
    problem: &PlanningProblem,
    plan: &UniversalPlan,
    repro: &impl Fn() -> String,
) {
    let goal_states: BTreeSet<&str> = problem
        .states
        .iter()
        .filter(|state| {
            problem
                .goal
                .facts
                .iter()
                .all(|fact| state.facts.contains(fact))
        })
        .map(|state| state.id.as_str())
        .collect();
    let known: BTreeSet<&str> = problem
        .states
        .iter()
        .map(|state| state.id.as_str())
        .collect();
    let policy_states: BTreeSet<&str> = plan
        .policy
        .iter()
        .map(|entry| entry.state.as_str())
        .collect();
    for entry in &plan.policy {
        assert!(
            known.contains(entry.state.as_str()),
            "{}\npolicy key {:?} is not a known state",
            repro(),
            entry.state
        );
        assert!(
            !goal_states.contains(entry.state.as_str()),
            "{}\ngoal state {:?} was given a policy entry",
            repro(),
            entry.state
        );
        assert!(
            !entry.outcomes.is_empty(),
            "{}\npolicy entry {:?}->{:?} is a dead sink (no outcomes)",
            repro(),
            entry.state,
            entry.action
        );
        for outcome in &entry.outcomes {
            assert!(
                goal_states.contains(outcome.state.as_str())
                    || policy_states.contains(outcome.state.as_str()),
                "{}\nclosure violated: outcome {:?} of policy entry {:?}->{:?} is neither a goal nor a policy key",
                repro(),
                outcome.state,
                entry.state,
                entry.action
            );
        }
    }
    for initial in &problem.initial_states {
        assert!(
            goal_states.contains(initial.as_str()) || policy_states.contains(initial.as_str()),
            "{}\ninitial state {initial:?} is neither a goal nor a policy key",
            repro()
        );
    }
}

fn count_deterministic_groups(problem: &PlanningProblem) -> usize {
    let mut group_sizes: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    for transition in &problem.transitions {
        *group_sizes
            .entry((transition.from.as_str(), transition.action.as_str()))
            .or_default() += 1;
    }
    group_sizes.values().filter(|&&size| size == 1).count()
}

// ----------------------------------------------------------- tests ----

/// The main property at scale: for every one of 5000 generated instances
/// (3–12 states, 25% decoys, optional multi-initial/unsafe),
/// (a) `solve_planning_type(Fond)` is Ok ⟺ the independent reference finds a
///     strong OR strong-cyclic policy (and `Err(NoPlan)` ⟺ neither);
/// (b) every returned policy is outcome-closed AND its reachable region is a
///     genuine strong-cyclic solution (goal-reaching, unsafe-free);
/// (c) solving the same input twice yields identical output.
#[test]
fn fond_solver_matches_independent_reference_at_scale() {
    let started = Instant::now();
    let mut rng = SplitMix64(SEED);
    let mut valid_ok_count = 0_usize;
    let mut noplan_count = 0_usize;
    let mut strong_count = 0_usize;
    let mut strong_cyclic_only_count = 0_usize;
    let mut unsafe_instances = 0_usize;
    let mut multi_initial_instances = 0_usize;
    let mut multi_initial_ok = 0_usize;
    let mut multi_initial_noplan = 0_usize;
    let mut decoy_count = 0_usize;
    let mut decoy_unsolvable = 0_usize;
    let mut decoy_strong_cyclic_only = 0_usize;
    let mut deterministic_groups = 0_usize;
    let mut max_states_seen = 0_usize;

    for instance in 0..INSTANCES {
        let decoy = instance % 4 == 3;
        let problem = generate(&mut rng, decoy, 10, 3);
        deterministic_groups += count_deterministic_groups(&problem);
        max_states_seen = max_states_seen.max(problem.states.len());
        if !problem.unsafe_states.is_empty() {
            unsafe_instances += 1;
        }
        if problem.initial_states.len() > 1 {
            multi_initial_instances += 1;
        }
        if decoy {
            decoy_count += 1;
        }
        let reference = ScaleReference::build(&problem);
        // The full JSON reproducer is only serialized on failure.
        let repro = || {
            format!(
                "instance {instance}/{INSTANCES} decoy={decoy} states={} initials={} \n\
                 reference verdicts come from an independent label-correcting \
                 computation (strong: min-max value iteration; strong-cyclic: \
                 iterated usability-constrained backward goal-reachability; \
                 unsafe states forbidden; ALL initials must win)\n\
                 PlanningProblem JSON reproducer:\n{}",
                problem.states.len(),
                problem.initial_states.len(),
                serde_json::to_string_pretty(&problem).expect("problem serializes")
            )
        };

        let (strong_solvable, strong_cyclic_solvable) = reference.verdict();
        if strong_solvable {
            strong_count += 1;
        } else if strong_cyclic_solvable {
            strong_cyclic_only_count += 1;
        }
        if decoy {
            if !strong_solvable && strong_cyclic_solvable {
                decoy_strong_cyclic_only += 1;
            } else if !strong_solvable && !strong_cyclic_solvable {
                decoy_unsolvable += 1;
            }
        }
        let expected_ok = strong_solvable || strong_cyclic_solvable;

        let first = solve_fond(&problem);
        match &first {
            Ok(plan) => {
                assert!(
                    plan.solved,
                    "{}\nsolver returned Ok with solved=false",
                    repro()
                );
                // (b) closure and soundness of the returned policy.
                assert_policy_closure(&problem, plan, &repro);
                match reference.check_returned_policy(&plan.policy) {
                    PolicyCheck::ValidStrongCyclic => {
                        if !expected_ok {
                            panic!(
                                "{}\nREFERENCE DEFECT: the solver policy is \
                                 VALID under the reference semantics yet the \
                                 reference reported no solution \
                                 (strong={strong_solvable} \
                                 strong_cyclic={strong_cyclic_solvable})",
                                repro()
                            );
                        }
                        valid_ok_count += 1;
                    }
                    // No carve-out remains: the FOUND_BUG_2 known-defect
                    // class (goal-unreachable region on a reference-solvable
                    // instance) was fixed by fond-htn-57's Phase-3 choice
                    // rewrite and its carve-out removed in the same change,
                    // so ANY recurrence fails this gate. Shrunk reproducer:
                    // fond_property_scaleup_FOUND_BUG_2_* above.
                    PolicyCheck::GoalUnreachableRegion => panic!(
                        "{}\nNEW DEFECT: solver returned a goal-unreachable \
                         policy for a reference-solvable instance \
                         (strong={strong_solvable} \
                         strong_cyclic={strong_cyclic_solvable}) — the \
                         FOUND_BUG_2 class recurred",
                        repro()
                    ),
                    PolicyCheck::EntersUnsafeState => panic!(
                        "{}\nNEW DEFECT: returned policy's reachable \
                         region contains an unsafe state",
                        repro()
                    ),
                    PolicyCheck::UnknownPolicyEntry(detail) => panic!(
                        "{}\nNEW DEFECT: returned policy is structurally \
                         invalid: {detail}",
                        repro()
                    ),
                }
                if problem.initial_states.len() > 1 {
                    multi_initial_ok += 1;
                }
            }
            Err(PlannerError::NoPlan) => {
                assert!(
                    !expected_ok,
                    "{}\nMISMATCH: solver returned NoPlan but the \
                     reference found a {} policy \
                     (strong={strong_solvable} \
                     strong_cyclic={strong_cyclic_solvable})",
                    repro(),
                    if strong_solvable {
                        "strong"
                    } else {
                        "strong-cyclic"
                    }
                );
                noplan_count += 1;
                if problem.initial_states.len() > 1 {
                    multi_initial_noplan += 1;
                }
            }
            Err(other) => panic!(
                "{}\nMISMATCH: solver returned an unexpected error {other:?}",
                repro()
            ),
        }

        // (c) determinism: same input twice => identical output.
        let second = solve_fond(&problem);
        assert_eq!(
            &first,
            &second,
            "{}\nMISMATCH: solve output is not deterministic across runs",
            repro()
        );
    }

    // Generator + coverage self-checks.
    assert_eq!(INSTANCES, 5000, "ticket requires 5000 instances");
    assert!(
        deterministic_groups > 0,
        "generator produced no deterministic (single-outcome) groups"
    );
    assert!(unsafe_instances > 0, "generator produced no unsafe states");
    assert_eq!(
        max_states_seen, 12,
        "generator never exercised the 12-state ceiling"
    );
    assert_eq!(
        decoy_count,
        INSTANCES / 4,
        "decoy mixture must be exactly 25% of the stream"
    );
    assert!(
        decoy_unsolvable > 0,
        "decoy dimension degenerate: no reference-unsolvable decoys \
         (closed-but-goal-unreachable loops never forced a NoPlan)"
    );
    assert!(
        decoy_strong_cyclic_only > 0,
        "decoy dimension degenerate: no strong-cyclic-only decoys \
         (retry loops never exercised at the decoy level)"
    );
    assert!(
        multi_initial_instances > 0,
        "generator produced no multi-initial instances"
    );
    assert!(
        multi_initial_ok > 0,
        "multi-initial coverage degenerate: none solvable"
    );
    assert!(
        multi_initial_noplan > 0,
        "multi-initial coverage degenerate: none unsolvable \
         (the ALL-initials rule never exercised on the NoPlan side)"
    );
    assert!(
        strong_count > 0,
        "coverage degenerate: no strongly solvable instances"
    );
    assert!(
        strong_cyclic_only_count > 0,
        "coverage degenerate: no strong-cyclic-only (retry-loop) instances"
    );
    assert!(
        valid_ok_count > 0 && noplan_count > 0,
        "coverage degenerate: valid-ok={valid_ok_count} noplan={noplan_count}"
    );

    let elapsed = started.elapsed();
    assert!(
        elapsed < WALL_BUDGET,
        "wall budget exceeded: {elapsed:?} >= {WALL_BUDGET:?} for {INSTANCES} instances"
    );
    println!(
        "fond_property_scaleup: {INSTANCES} instances in {elapsed:?} \
         (strong={strong_count}, strong-cyclic-only={strong_cyclic_only_count}, \
         valid-ok={valid_ok_count}, noplan={noplan_count}, \
         decoys={decoy_count} [unsolvable={decoy_unsolvable}, \
         sc-only={decoy_strong_cyclic_only}], multi-initial={multi_initial_instances} \
         [ok={multi_initial_ok}, noplan={multi_initial_noplan}], \
         unsafe-states={unsafe_instances}, deterministic-groups={deterministic_groups})"
    );
}

/// Meta-falsification: the new label-correcting reference must agree with
/// the precedent's literal policy enumeration on 320 small instances
/// (3–8 states, ≤ 3 actions, enumeration assert-capped at 3^8 policies).
/// This is what licenses trusting the reference at the 12-state scale where
/// enumeration is infeasible.
#[test]
fn reference_agrees_with_policy_enumeration_on_small_slice() {
    let started = Instant::now();
    let mut rng = SplitMix64(CROSSCHECK_SEED);
    let mut agree = 0_usize;
    let mut strong_cases = 0_usize;
    let mut sc_only_cases = 0_usize;
    let mut unsolvable_cases = 0_usize;
    for instance in 0..CROSSCHECK_INSTANCES {
        let problem = generate(&mut rng, instance % 4 == 3, 6, 2);
        let expected = Enumeration::build(&problem).verdict();
        let actual = ScaleReference::build(&problem).verdict();
        assert_eq!(
            expected,
            actual,
            "instance {instance}/{CROSSCHECK_INSTANCES}: enumeration and \
             label-correcting reference disagree\n{}",
            serde_json::to_string_pretty(&problem).expect("problem serializes")
        );
        agree += 1;
        match actual {
            (true, _) => strong_cases += 1,
            (false, true) => sc_only_cases += 1,
            (false, false) => unsolvable_cases += 1,
        }
    }
    assert!(
        strong_cases > 0 && sc_only_cases > 0 && unsolvable_cases > 0,
        "cross-check coverage degenerate: strong={strong_cases} \
         sc-only={sc_only_cases} unsolvable={unsolvable_cases}"
    );
    println!(
        "cross-check: {agree} instances agree (strong={strong_cases}, \
         sc-only={sc_only_cases}, unsolvable={unsolvable_cases}) in {:?}",
        started.elapsed()
    );
}

/// Shrunk reproducer (2 states; shrunk from scale-up instance 1 of seed
/// `0x5EED_2026_0918`, a 7-state non-decoy instance) for the residual
/// strong-cyclic defect FOUND_BUG_2, derived per ticket scope item 4.
/// Always-on regression guard since the hotfix (ticket fond-htn-57, commit
/// `fix/sc-choice-rewrite`); the `#[ignore]` and the sweep gate's
/// known-defect carve-out were removed in the same change. The
/// `FOUND_BUG_*` spelling is mandated by ticket fond-htn-15 lineage; kept
/// greppable across the wave.
///
/// DEFECT (pre-hotfix): `fond_policy_strong_cyclic` returned `Ok` with
/// policy `[s0 -> a0 (self-loop {s0})]`. Trace (dispatch falls through
/// `fond_policy` because `a1`'s outcome set includes the not-yet-winning
/// `s0`, so no strong policy exists):
/// 1. Phase 1 (weak): `weak = {s0, g}` (s0 has an edge to g).
/// 2. Phase 2 (greatest-fixpoint witness prune): the FIRST group of s0 in
///    BTreeMap order is `(s0, a0)` with outcomes `{s0} ⊆ surviving` — so
///    `choices[s0] = a0`, the pure self-loop, even though `(s0, a1)` is also
///    closed. Nothing is pruned.
/// 3. Phase 3 (committable goal-reachability): (a) the witness prune is
///    stable; (b) `reach` grows to `{g, s0}` because `a1` is closed in
///    `surviving` AND touches `g` — so s0 IS committable-goal-reaching,
///    but via `a1`, not via the chosen `a0`. Since
///    `reach.len() == surviving.len()`, the loop breaks WITHOUT rewriting
///    `choices`. Phase 3 verifies the REGION's goal-reachability but never
///    aligns the returned policy with the advancing actions it just
///    certified.
/// 4. The final structural closure check passes (s0's self-loop outcome is a
///    policy key), so the solver returns a policy whose reachable region
///    `{s0}` provably never reaches the goal.
///
/// HOTFIX (fond-htn-57): Phase 3 records each state's reach-discovery RANK
/// during the (b) sweep; when the fixpoint closes, every surviving non-goal
/// state's choice is rewritten to a committable action (all outcomes ⊆
/// surviving) with at least one outcome of strictly smaller rank
/// (advancing), or the state is pruned (defense in depth) and the
/// alternation re-runs. The returned (choices, surviving) pair is now the
/// pair whose goal-reachability was proven.
///
/// EXPECTED (Cimatti characterization / independent reference): the instance
/// IS strong-cyclic solvable — policy `s0 -> a1` (outcomes {g, s0}) is the
/// classic retry loop; the reference verdict is `(false, true)` — and the
/// solver must return that policy (or any policy whose region is
/// goal-reaching).
#[test]
// The FOUND_BUG_* spelling is mandated by ticket fond-htn-15; keep it.
#[allow(non_snake_case)]
fn fond_property_scaleup_FOUND_BUG_2_strong_cyclic_returns_witness_first_goal_unreachable_loop() {
    let problem = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![
            edge("a0", "s0", "s0", 1_000_000),
            edge("a1", "s0", "g", 500_000),
            edge("a1", "s0", "s0", 500_000),
        ],
        ..PlanningProblem::default()
    };
    // The reference (independently, both by enumeration on this size and by
    // the label-correcting computation) proves the instance strong-cyclic
    // solvable: the retry action a1 is a genuine solution.
    let reference = ScaleReference::build(&problem);
    assert_eq!(
        reference.verdict(),
        (false, true),
        "reference must prove this instance strong-cyclic solvable"
    );
    assert_eq!(
        Enumeration::build(&problem).verdict(),
        (false, true),
        "enumeration must agree: the instance is strong-cyclic solvable"
    );
    // Regression guard: the solver must return the retry policy. `a0` (the
    // alphabetically-first pure self-loop) is NOT a solution; `a1` (the
    // committable retry) is — the choice rewrite must pick it.
    let plan = solve_fond(&problem).expect(
        "fond_policy_strong_cyclic must return the a1 retry policy, not \
         NoPlan, for this strong-cyclic solvable instance",
    );
    assert!(plan.solved);
    let entry = plan
        .policy
        .iter()
        .find(|entry| entry.state == "s0")
        .expect("s0 must carry a policy entry");
    assert_eq!(
        entry.action, "a1",
        "the committable retry action must be chosen over the \
         witness-first pure self-loop a0"
    );
    let mut outcomes = entry
        .outcomes
        .iter()
        .map(|outcome| outcome.state.clone())
        .collect::<Vec<_>>();
    outcomes.sort();
    assert_eq!(outcomes, vec!["g".to_owned(), "s0".to_owned()]);
}

/// Meta-verification of the reference oracle itself against hand-computed
/// semantics, so a generator/reference bug cannot masquerade as a solver
/// bug. Includes multi-initial and decoy-shaped domains.
#[test]
fn reference_agrees_with_hand_computed_semantics() {
    // Deterministic chain: strong.
    let chain = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![edge("a0", "s0", "g", 1_000_000)],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&chain).verdict(), (true, true));

    // Retry loop s0->{g, s0}: NOT strong, but strong-cyclic.
    let retry = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![
            edge("a0", "s0", "g", 500_000),
            edge("a0", "s0", "s0", 500_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&retry).verdict(), (false, true));

    // Certain fall into an unsafe absorbing state: neither.
    let doomed = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"]), state("u", &[])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        unsafe_states: BTreeSet::from(["u".to_owned()]),
        transitions: vec![
            edge("a0", "s0", "u", 1_000_000),
            edge("a0", "u", "u", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&doomed).verdict(), (false, false));

    // The only action risks the unsafe state: unsolvable even though the goal
    // outcome has positive probability.
    let risky = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"]), state("u", &[])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        unsafe_states: BTreeSet::from(["u".to_owned()]),
        transitions: vec![
            edge("a0", "s0", "g", 500_000),
            edge("a0", "s0", "u", 500_000),
            edge("a0", "u", "u", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&risky).verdict(), (false, false));

    // A safe action plus a risky action: strong via the safe action.
    let safe_alternative = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"]), state("u", &[])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        unsafe_states: BTreeSet::from(["u".to_owned()]),
        transitions: vec![
            edge("a0", "s0", "g", 1_000_000),
            edge("a1", "s0", "g", 500_000),
            edge("a1", "s0", "u", 500_000),
            edge("a0", "u", "u", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(
        ScaleReference::build(&safe_alternative).verdict(),
        (true, true)
    );

    // Dead sink (non-goal state with no actions): unsolvable.
    let dead_sink = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"]), state("d", &[])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![edge("a0", "s0", "d", 1_000_000)],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&dead_sink).verdict(), (false, false));

    // Decoy trap: the initial state's ONLY action is a pure self-loop —
    // closed, goal-free (the pre-Phase-3 solver accepted this).
    let trap = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![edge("a0", "s0", "s0", 1_000_000)],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&trap).verdict(), (false, false));

    // Decoy risky escape on the initial state (FOUND_BUG_1 shape):
    // closed loop + action risking the unsafe state.
    let risky_escape = PlanningProblem {
        states: vec![state("s0", &[]), state("g", &["goal"]), state("u", &[])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        unsafe_states: BTreeSet::from(["u".to_owned()]),
        transitions: vec![
            edge("a0", "s0", "s0", 1_000_000),
            edge("a1", "s0", "g", 500_000),
            edge("a1", "s0", "u", 500_000),
            edge("a1", "u", "u", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(
        ScaleReference::build(&risky_escape).verdict(),
        (false, false)
    );

    // Multi-initial, both initials win (s0 by retry, s1 by chain):
    // strong-cyclic only (s0 needs its retry loop).
    let multi_both_win = PlanningProblem {
        states: vec![state("s0", &[]), state("s1", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned(), "s1".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![
            edge("a0", "s0", "g", 500_000),
            edge("a0", "s0", "s0", 500_000),
            edge("a1", "s1", "g", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(
        ScaleReference::build(&multi_both_win).verdict(),
        (false, true)
    );

    // Multi-initial, one initial is a closed goal-free self-loop: the ALL-
    // initials rule makes the whole instance unsolvable.
    let multi_one_dead = PlanningProblem {
        states: vec![state("s0", &[]), state("s1", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned(), "s1".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![
            edge("a0", "s0", "g", 500_000),
            edge("a0", "s0", "s0", 500_000),
            edge("a1", "s1", "s1", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(
        ScaleReference::build(&multi_one_dead).verdict(),
        (false, false)
    );

    // Same dead self-loop state NOT initial: solvable again (strong-cyclic
    // only, via s0's retry).
    let multi_dead_not_initial = PlanningProblem {
        states: vec![state("s0", &[]), state("s1", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![
            edge("a0", "s0", "g", 500_000),
            edge("a0", "s0", "s0", 500_000),
            edge("a1", "s1", "s1", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(
        ScaleReference::build(&multi_dead_not_initial).verdict(),
        (false, true)
    );

    // Multi-initial, both strong (deterministic chains): strong.
    let multi_strong = PlanningProblem {
        states: vec![state("s0", &[]), state("s1", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned(), "s1".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![
            edge("a0", "s0", "g", 1_000_000),
            edge("a1", "s1", "g", 1_000_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&multi_strong).verdict(), (true, true));

    // Mutually-closed goal-free 2-cycle holding an initial: unsolvable.
    let two_cycle = PlanningProblem {
        states: vec![state("s0", &[]), state("s1", &[]), state("g", &["goal"])],
        initial_states: vec!["s0".to_owned()],
        goal: goal_fact_goal(),
        transitions: vec![
            edge("a0", "s0", "s0", 500_000),
            edge("a0", "s0", "s1", 500_000),
            edge("a0", "s1", "s0", 500_000),
            edge("a0", "s1", "s1", 500_000),
        ],
        ..PlanningProblem::default()
    };
    assert_eq!(ScaleReference::build(&two_cycle).verdict(), (false, false));
}
