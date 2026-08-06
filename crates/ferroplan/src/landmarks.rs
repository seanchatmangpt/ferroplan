//! Fact landmarks, traced backward from the goal, first-achiever by first-achiever
//! (0.9 roadmap Phase 3).
//!
//! A landmark is a fact no plan can dodge — every route through the job makes it
//! true somewhere along the way. This module runs the classic delete-relaxation
//! trace, LAMA's own backbone, the Hoffmann/Porteous/Sebastia play: build the
//! relaxed planning graph out from the initial state to fixpoint, then walk
//! backward from the goal. For a landmark `f` that isn't already true, every plan
//! has to pass through one of its FIRST achievers — the ops that add `f` from a
//! layer strictly earlier than `f`'s own — so any fact sitting in every first
//! achiever's precondition list is a landmark too, no exceptions. Sound (never
//! calls a bluff on a non-landmark), incomplete (some hide from it), cheap to run:
//! O(landmarks × achiever preconditions) after one graph build, O(n_facts) memory
//! — no bloated per-fact table dragging behind it.
//!
//! Feeds the LAMA-style rung ([`crate::lama`]) a path-dependent landmark count:
//! landmarks not yet banked on the current path measure what's still owed, a
//! signal the FF heuristic goes blind on exactly where it stalls out — the long
//! goal-interaction chains (parking, floortile, barman).

use crate::heuristic::{reachability_layers, Scratch};
use crate::packed::PackedTask;

/// The goal's landmark set, sorted and deduped, fact ids only. Carries the goal
/// facts themselves — the easy landmarks — minus anything already true in
/// `:init`; crediting those would be paying out for standing still. No noise, same
/// answer every time.
pub fn goal_landmarks(task: &PackedTask) -> Vec<u32> {
    let init = task.initial();
    landmarks_for(task, &init, &task.goal_pos)
}

/// [`goal_landmarks`], cut loose from a fixed starting point — takes any start
/// state and any goal-fact subset, the per-SUBGOAL cut the partition cascade's
/// LAMA rung needs (`resolve::solve` chases subgoals through states that keep
/// shifting under it). Same backward walk, same soundness case; landmarks already
/// live at `start` get dropped, same reason as always — no credit for standing
/// still.
pub fn landmarks_for(
    task: &PackedTask,
    start: &crate::packed::State,
    goal_pos: &[u32],
) -> Vec<u32> {
    // Relaxed reachability layers from the start state, goal-blind (to fixpoint).
    let mut sc = Scratch::new(task);
    let (fact_layer, op_layer) =
        reachability_layers(task, &mut sc, &start.bits, &start.fv, &start.fdef);

    let mut is_lm = vec![false; task.n_facts];
    let mut queue: Vec<u32> = Vec::new();
    for &g in goal_pos {
        // Unreachable goals mean the task is unsolvable; landmark counting is
        // moot but must not crash — skip them.
        if fact_layer[g as usize] != u32::MAX && !is_lm[g as usize] {
            is_lm[g as usize] = true;
            queue.push(g);
        }
    }

    let mut head = 0;
    while head < queue.len() {
        let f = queue[head] as usize;
        head += 1;
        let fl = fact_layer[f];
        if fl == 0 {
            continue; // true in init: no achiever needed
        }
        // First achievers: ops adding f from a strictly earlier layer.
        let mut common: Option<Vec<u32>> = None;
        for &oi in task.add_by_fact.slice(f) {
            let oi = oi as usize;
            if op_layer[oi] >= fl {
                continue;
            }
            let pre: Vec<u32> = task.pre_pos.slice(oi).to_vec();
            common = Some(match common {
                None => pre,
                Some(prev) => prev.into_iter().filter(|p| pre.contains(p)).collect(),
            });
            if common.as_ref().is_some_and(|c| c.is_empty()) {
                break;
            }
        }
        for p in common.unwrap_or_default() {
            if !is_lm[p as usize] {
                is_lm[p as usize] = true;
                queue.push(p);
            }
        }
    }

    // Landmarks already true at the start are pre-accepted — dropping them
    // here keeps the count meaning "necessary facts not yet made true".
    let mut out: Vec<u32> = (0..task.n_facts as u32)
        .filter(|&f| is_lm[f as usize] && !crate::bitset::test(&start.bits, f as usize))
        .collect();
    out.sort_unstable();
    out
}
