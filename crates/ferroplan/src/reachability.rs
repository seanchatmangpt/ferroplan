//! Inverse-reachability safe-set compilation (dissertation Ch3, Theorem 3.1:
//! `S_safe = X \ ∪_k Reach⁻ᵏ(S_prohibited)`).
//!
//! Given a transition relation (successors per state) and a set of
//! prohibited states — the planner-side analog of CASTLE's Γ_prohibited
//! gating mandate construction — compute the backward reachable set to
//! depth `k` (BFS over predecessors) and expose `is_safe(state)`: a state
//! is safe exactly when it cannot reach a prohibited state within k steps.
//!
//! Termination is bounded by `max_depth`: the computation saturates when a
//! BFS level adds nothing new, and the flag distinguishes "exhausted the
//! graph" from "stopped at the bound".

use crate::bitset::{count, set, test};
use std::collections::VecDeque;

/// Result of a bounded backward-reachability compilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackwardSafeSet {
    n_states: usize,
    /// Bitset over state ids: bit set = state is UNSAFE (can reach a
    /// prohibited state within the compiled depth).
    unsafe_words: Vec<u64>,
    /// Depth actually compiled: `max_depth` unless the frontier emptied
    /// early (full saturation of the predecessor closure).
    depth_reached: u32,
    /// True when the backward closure exhausted (no new predecessors at
    /// some level) before the depth bound.
    saturated: bool,
}

impl BackwardSafeSet {
    /// Compile from a successor-list transition relation. `successors[s]`
    /// lists the states one step from `s`; `prohibited` lists prohibited
    /// state ids (each compiled at depth 0). `max_depth` bounds the BFS.
    pub fn from_successors(
        successors: &[Vec<u32>],
        prohibited: &[u32],
        max_depth: u32,
    ) -> Self {
        let n_states = successors.len();
        let mut preds: Vec<Vec<u32>> = vec![Vec::new(); n_states];
        for (s, succs) in successors.iter().enumerate() {
            for &t in succs {
                preds[t as usize].push(s as u32);
            }
        }
        Self::from_predecessors(&preds, prohibited, max_depth)
    }

    /// Compile from an explicit predecessor relation — the direct shape of
    /// the dissertation recursion: seed the prohibited set at depth 0, then
    /// pull in predecessors level by level until `max_depth` or saturation.
    pub fn from_predecessors(
        predecessors: &[Vec<u32>],
        prohibited: &[u32],
        max_depth: u32,
    ) -> Self {
        let n_states = predecessors.len();
        let mut words = vec![0u64; n_states.div_ceil(64)];
        let mut frontier: VecDeque<u32> = VecDeque::new();
        let mut depth_reached: u32 = 0;
        let mut saturated = false;

        // Depth 0: prohibited states are themselves unsafe.
        for &p in prohibited {
            if (p as usize) < n_states {
                if !test(&words, p as usize) {
                    set(&mut words, p as usize);
                    frontier.push_back(p);
                }
                // NOTE(mutant-anchor): the predecessor pull-in below is the
                // load-bearing step of Theorem 3.1. Dropping it (or one edge
                // of it) shrinks ∪_k Reach⁻ᵏ(S_prohibited) and wrongly
                // declares a state that reaches prohibition "safe" — the
                // linear-chain test kills exactly this mutant.
            }
        }

        // BFS: each level extends the unsafe set by one backward step.
        while depth_reached < max_depth && !frontier.is_empty() {
            let mut next: VecDeque<u32> = VecDeque::new();
            for &p in &frontier {
                for &q in &predecessors[p as usize] {
                    if (q as usize) < n_states && !test(&words, q as usize) {
                        set(&mut words, q as usize);
                        next.push_back(q);
                }
                }
            }
            if next.is_empty() {
                saturated = true;
                break;
            }
            frontier = next;
            depth_reached += 1;
        }
        if frontier.is_empty() {
            saturated = true;
        }

        BackwardSafeSet {
            n_states,
            unsafe_words: words,
            depth_reached,
            saturated,
        }
    }

    /// Theorem 3.1 predicate: `is_safe(s) = s ∉ ∪_k Reach⁻ᵏ(S_prohibited)`.
    /// An out-of-range id is safe (not in the compiled universe).
    #[inline]
    pub fn is_safe(&self, state: u32) -> bool {
        (state as usize >= self.n_states) || !crate::bitset::test(&self.unsafe_words, state as usize)
    }

    /// Number of unsafe (excluded) states in the compiled set.
    pub fn unsafe_count(&self) -> usize {
        crate::bitset::count(&self.unsafe_words)
    }

    /// Depth actually compiled.
    pub fn depth_reached(&self) -> u32 {
        self.depth_reached
    }

    /// True when the backward closure exhausted before the depth bound.
    pub fn saturated(&self) -> bool {
        self.saturated
    }
}

/// Anti-vacuity mutant anchor: drop the predecessor pull-in entirely and
/// `linear_chain_backward_reach` fails (A and B come out "safe"). See the
/// NOTE(mutant-anchor) in [`BackwardSafeSet::from_predecessors`].
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_chain_backward_reach() {
        // A(0) -> B(1) -> C(2, prohibited); D(3) has no path to C.
        let succ = vec![vec![1], vec![2], vec![], vec![]];
        let ss = BackwardSafeSet::from_successors(&succ, &[2], 10);
        assert!(!ss.is_safe(2));
        assert!(!ss.is_safe(1));
        assert!(!ss.is_safe(0));
        assert!(ss.is_safe(3));
        assert!(ss.saturated());
        assert_eq!(ss.unsafe_count(), 3);
        assert_eq!(ss.depth_reached(), 2);
    }

    #[test]
    fn depth_bound_stops_before_full_closure() {
        // X -> Y -> Z_prohibited, depth 1 only: Y unsafe, X stays safe at k=1.
        let succ = vec![vec![1], vec![2], vec![]];
        let ss = BackwardSafeSet::from_successors(&succ, &[2], 1);
        assert!(!ss.is_safe(1));
        assert!(ss.is_safe(0), "depth-1 bound must leave X safe");
        assert!(!ss.saturated());
        assert_eq!(ss.depth_reached(), 1);
    }

    #[test]
    fn cycle_saturates_without_hang() {
        // A <-> B, B -> C prohibited: both unsafe, saturates.
        let succ = vec![vec![1], vec![0, 2], vec![]];
        let ss = BackwardSafeSet::from_successors(&succ, &[2], 5);
        assert!(!ss.is_safe(0));
        assert!(!ss.is_safe(1));
        assert!(ss.saturated());
    }

    #[test]
    fn disconnected_component_stays_safe() {
        // Component {0,1,2} flows into prohibited 2; component {3,4} disjoint.
        let succ = vec![vec![1], vec![2], vec![], vec![4], vec![]];
        let ss = BackwardSafeSet::from_successors(&succ, &[2], 10);
        assert!(!ss.is_safe(0));
        assert!(!ss.is_safe(1));
        assert!(ss.is_safe(3));
        assert!(ss.is_safe(4));
    }

    #[test]
    fn prohibited_seed_only_depth_zero() {
        // max_depth = 0: only the prohibited state itself is unsafe.
        let succ = vec![vec![1], vec![2], vec![]];
        let ss = BackwardSafeSet::from_successors(&succ, &[2], 0);
        assert!(!ss.is_safe(2));
        assert!(ss.is_safe(1));
        assert_eq!(ss.depth_reached(), 0);
        assert!(!ss.saturated(), "seed frontier unexpanded at bound → not saturated");
    }

    #[test]
    fn out_of_range_state_is_safe() {
        let succ = vec![vec![1], vec![2], vec![]];
        let ss = BackwardSafeSet::from_successors(&succ, &[2], 10);
        assert!(ss.is_safe(99));
    }
}
