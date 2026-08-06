//! LAMA rung, dropped into the 0.9 roadmap, Phase 3. Greedy best-first,
//! running two signals at once — the FF relaxed-plan heuristic and a
//! path-dependent **landmark count** ([`crate::landmarks`]) — and
//! **preferred-operator** boosting off a dual open list: a successor
//! reached through a parent's helpful action gets dropped into a second,
//! favored heap. Standard LAMA doctrine, run cold.
//!
//! Field note on why this rung exists at all: EHC and plain weighted
//! best-first — the FF lineage — flatline exactly where the relaxed plan
//! plateaus. Long goal-interaction chains eat them alive (parking,
//! floortile, barman, tidybot). Landmarks still unclaimed on the current
//! path hold a progress gradient across those dead zones; helpful-action
//! boosting keeps the branching factor tight to the relaxed plan's. This
//! rung stays BOUNDED — wakes after EHC taps out, sleeps before the
//! complete weighted fallback takes over. Net effect: coverage only, never
//! a regression. Kill it with `FF_NO_LAMA=1`; explicit `--search bfs`
//! never sees it in the first place.
//!
//! Determinism is non-negotiable: fixed batch sizes off each heap,
//! order-preserving parallel h evaluation, serial insertion. Same plan
//! every time, any thread count — same contract `search_from` runs on.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::hash::FxHashSet;
use crate::heuristic::{relaxed_helpful, Scratch};
use crate::packed::{PackedTask, State, StateKey};
use crate::par;

/// Batch pulled each round: boosted count off the preferred heap, plain count off the normal one.
const PREF_BATCH: usize = 192;
const NORM_BATCH: usize = 64;
/// Weighting split between FF-h and landmark count in the greedy priority key.
const W_FF: i64 = 2;
const W_LM: i64 = 4;

/// One expansion candidate: (parent idx, op, successor state, visited key,
/// parent's FF h, reached via a helpful op).
type Cand = (usize, usize, State, StateKey, i32, bool);

struct Node {
    state: State,
    father: usize,
    op: usize,
    /// Landmarks banked on the path to this node — bitset over the
    /// landmark LIST index, not fact ids.
    accepted: Vec<u64>,
}

fn accept_into(accepted: &mut [u64], lms: &[u32], state: &State) {
    for (i, &f) in lms.iter().enumerate() {
        if accepted[i >> 6] & (1 << (i & 63)) == 0 && crate::bitset::test(&state.bits, f as usize) {
            accepted[i >> 6] |= 1 << (i & 63);
        }
    }
}

fn unaccepted(accepted: &[u64], n: usize) -> i64 {
    n as i64 - accepted.iter().map(|w| w.count_ones() as i64).sum::<i64>()
}

/// Bounded landmark/preferred greedy run at the task goal. Comes back with
/// the plan ops and the eval count, or nothing — dead end, budget cap, or
/// node cap, take your pick.
pub fn search(
    task: &PackedTask,
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
) -> Option<(Vec<usize>, usize)> {
    let init = task.initial();
    // Length-anytime on the whole-task rung only (subgoal probes return on
    // first goal — a cascade merge wants speed, not polish). Opt-in; see
    // SearchCfg::len_anytime for the measured default-off verdict.
    let len_anytime = std::env::var("FF_LEN_ANYTIME").is_ok();
    search_subgoal(
        task,
        &init,
        &task.goal_pos,
        &task.goal_num,
        threads,
        max_eval,
        forbidden,
        len_anytime,
    )
}

/// [`search`], generalized over a start state and a subgoal — the shape the
/// partition cascade (`resolve::solve`) needs on the ground. Landmarks get
/// recomputed fresh for this exact (start, subgoal) pair, so the count
/// stays an honest remaining-necessary-work signal for the piece in hand.
#[allow(clippy::too_many_arguments)]
pub fn search_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[crate::types::NumPre],
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    len_anytime: bool,
) -> Option<(Vec<usize>, usize)> {
    let lms = crate::landmarks::landmarks_for(task, start, goal_pos);
    let lm_words = lms.len().div_ceil(64);
    let node_cap = crate::search::node_cap_for(task);

    let init = start.clone();
    let mut accepted0 = vec![0u64; lm_words];
    accept_into(&mut accepted0, &lms, &init);
    let mut nodes = vec![Node {
        state: init.clone(),
        father: usize::MAX,
        op: usize::MAX,
        accepted: accepted0,
    }];
    if task.goal_met_with(&init, goal_pos, goal_num) {
        return Some((Vec::new(), 0));
    }

    let mut pref_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut norm_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    norm_heap.push(Reverse((0, 0)));
    let mut visited: FxHashSet<StateKey> = FxHashSet::default();
    visited.insert(task.state_key(&init));
    // A node can sit in BOTH heaps (preferred successors do, so the normal
    // queue's completeness is untouched); expand it only once.
    let mut expanded = vec![false; 1];
    let mut evaluated = 0usize;
    // Length-anytime incumbent (see search.rs SearchCfg::len_anytime): keep
    // draining the same dual open lists for a strictly shorter plan until the
    // drain ceiling (evals-at-first-incumbent × 2).
    let mut best_plan: Option<Vec<usize>> = None;
    let mut best_len = usize::MAX;
    let mut eval_ceiling = max_eval;

    loop {
        // Deterministic mixed batch: boosted share from the preferred heap,
        // the rest from the normal one.
        let mut popped: Vec<usize> = Vec::with_capacity(PREF_BATCH + NORM_BATCH);
        for _ in 0..PREF_BATCH {
            match pref_heap.pop() {
                Some(Reverse((_, ni))) if !expanded[ni] => {
                    expanded[ni] = true;
                    popped.push(ni);
                }
                Some(_) => continue,
                None => break,
            }
        }
        for _ in 0..NORM_BATCH {
            match norm_heap.pop() {
                Some(Reverse((_, ni))) if !expanded[ni] => {
                    expanded[ni] = true;
                    popped.push(ni);
                }
                Some(_) => continue,
                None => break,
            }
        }
        if popped.is_empty() {
            // both open lists exhausted (with an incumbent: it is final)
            return best_plan.map(|p| (p, evaluated));
        }

        for &ni in &popped {
            if task.goal_met_with(&nodes[ni].state, goal_pos, goal_num) {
                if !len_anytime {
                    return Some((reconstruct(&nodes, ni), evaluated));
                }
                let plan = reconstruct(&nodes, ni);
                if plan.len() < best_len {
                    best_len = plan.len();
                    best_plan = Some(plan);
                    if eval_ceiling == max_eval {
                        eval_ceiling = evaluated
                            .saturating_mul(2)
                            .max(evaluated + 10_000)
                            .min(max_eval);
                    }
                }
            }
        }

        // PARALLEL: FF h + helpful set per popped node (the only evaluations).
        let hs: Vec<Option<(i32, Vec<u32>)>> = par::par_map_with(
            &popped,
            threads,
            || Scratch::new(task),
            |sc, &ni| {
                let s = &nodes[ni].state;
                relaxed_helpful(task, sc, &s.bits, &s.fv, &s.fdef, goal_pos, goal_num)
            },
        );
        evaluated += popped.len();
        if evaluated > max_eval || evaluated > eval_ceiling || nodes.len() > node_cap {
            // budget spent: the incumbent (if any), else hand off to the fallback
            return best_plan.map(|p| (p, evaluated));
        }

        // PARALLEL: expand live nodes; preferred = successor via a helpful op.
        let chunks: Vec<Vec<Cand>> = {
            let live: Vec<(usize, i32, &Vec<u32>)> = popped
                .iter()
                .zip(hs.iter())
                .filter_map(|(&ni, h)| h.as_ref().map(|(h, help)| (ni, *h, help)))
                .collect();
            par::par_map(&live, threads, |&(ni, ph, helpful)| {
                let st = &nodes[ni].state;
                let mut v = Vec::new();
                for oi in 0..task.n_ops {
                    if forbidden.get(oi).copied().unwrap_or(false) {
                        continue;
                    }
                    if task.op_applicable(oi, st) {
                        let ns = task.apply(oi, st);
                        let k = task.state_key(&ns);
                        let pref = helpful.contains(&(oi as u32));
                        v.push((ni, oi, ns, k, ph, pref));
                    }
                }
                v
            })
        };

        // SERIAL: dedup + insert (deterministic).
        for chunk in chunks {
            for (pi, oi, s, k, ph, pref) in chunk {
                if visited.insert(k) {
                    let mut accepted = nodes[pi].accepted.clone();
                    accept_into(&mut accepted, &lms, &s);
                    // Landmark count is EXACT for the successor (cheap bit
                    // math); the FF term is deferred from the parent.
                    let h_lm = unaccepted(&accepted, lms.len());
                    let key = W_FF * ph as i64 + W_LM * h_lm;
                    let idx = nodes.len();
                    nodes.push(Node {
                        state: s,
                        father: pi,
                        op: oi,
                        accepted,
                    });
                    expanded.push(false);
                    norm_heap.push(Reverse((key, idx)));
                    if pref {
                        pref_heap.push(Reverse((key, idx)));
                    }
                }
            }
        }
    }
}

fn reconstruct(nodes: &[Node], mut ni: usize) -> Vec<usize> {
    let mut ops = Vec::new();
    while nodes[ni].father != usize::MAX {
        ops.push(nodes[ni].op);
        ni = nodes[ni].father;
    }
    ops.reverse();
    ops
}
