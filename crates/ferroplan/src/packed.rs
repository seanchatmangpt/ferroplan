//! The grounded task, stripped for the run: Structure-of-Arrays / CSR, bitset
//! state. No struct-per-op padding, no pointer chasing.
//!
//! Operators live column-wise in CSR arrays (`flat` + `off`), not a `Vec` of
//! structs — so the hot loops (applicability, successor gen, heuristic
//! relaxation) burn straight through contiguous memory and split clean
//! across threads over one shared, immutable task.

use crate::bitset;
use crate::types::{eval_numpre, AssignOp, NumEff, NumPre};
use std::sync::Arc;

/// A trip-wire: ADL conditional effect `(when condition effect)`. Check the
/// source state — if `cond_pos` are lit, `cond_neg` are dark, `cond_num`
/// clears — the charge goes off: `add`/`del`/`num` fire, alongside the
/// unconditional effects, all read off the SAME source-state snapshot.
#[derive(Clone, Debug, Default)]
pub struct CondEff {
    pub cond_pos: Vec<u32>,
    pub cond_neg: Vec<u32>,
    pub cond_num: Vec<NumPre>,
    pub add: Vec<u32>,
    pub del: Vec<u32>,
    pub num: Vec<NumEff>,
}

/// Compressed-sparse-row rig: item `i` claims `flat[off[i]..off[i+1]]`.
/// `Arc`-backed since 0.13 — a [`PackedTask`] clone shares the payload, no
/// re-copy. N sessions riding one world pay for one grounding, not N
/// (`Session::fork`).
#[derive(Debug)]
pub struct Csr<T> {
    pub flat: Arc<[T]>,
    pub off: Arc<[u32]>,
}

// Manual impl: an Arc bump needs no `T: Clone`, and the derive would demand it.
impl<T> Clone for Csr<T> {
    fn clone(&self) -> Self {
        Csr {
            flat: Arc::clone(&self.flat),
            off: Arc::clone(&self.off),
        }
    }
}

impl<T> Csr<T> {
    pub fn slice(&self, i: usize) -> &[T] {
        &self.flat[self.off[i] as usize..self.off[i + 1] as usize]
    }
}

/// Row-by-row assembler: bolt on one row, offsets keep pace.
pub struct CsrBuilder<T> {
    pub flat: Vec<T>,
    pub off: Vec<u32>,
}
impl<T> Default for CsrBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> CsrBuilder<T> {
    pub fn new() -> Self {
        CsrBuilder {
            flat: Vec::new(),
            off: vec![0],
        }
    }
    pub fn push_row(&mut self, items: impl IntoIterator<Item = T>) {
        self.flat.extend(items);
        self.off.push(self.flat.len() as u32);
    }
    pub fn finish(self) -> Csr<T> {
        Csr {
            flat: self.flat.into(),
            off: self.off.into(),
        }
    }
}

/// The task, grounded and data-oriented. This is the world after recon.
///
/// `Clone` runs CHEAP by design (0.13 Phase 2): the grounded payload —
/// operator CSR columns, names, achiever indexes, the monitor block — sits
/// behind `Arc`, shared across every clone. Only the thin per-clone slice
/// (live facts/fluents, goal, fluent relevance) actually gets copied.
/// `Session::fork` spins up a whole population of minds over ONE world this
/// way — no re-grounding tax per instance.
#[derive(Clone)]
pub struct PackedTask {
    pub n_facts: usize,
    pub words: usize,
    pub n_ops: usize,

    /// Per-op call sign for the plan readout, e.g. `WALK A0 P0 P1`.
    pub op_display: Arc<[String]>,

    pub pre_pos: Csr<u32>,
    pub add: Csr<u32>,
    pub del: Csr<u32>,
    pub pre_num: Csr<NumPre>,
    pub num_eff: Csr<NumEff>,
    /// Per-op ADL conditional effects — dark, empty rows for the plain
    /// STRIPS/numeric jobs.
    pub cond: Csr<CondEff>,
    /// The SHARED monitor block (0.8 Phase 2, docs/roadmap-0.8.md):
    /// trajectory-monitor transitions grounded ONCE, then patched in — after
    /// the op's own `cond` row, same 0.7 tail order — for every op flagged
    /// live in [`Self::monitored`]. Dark on every constraint-free task. Read
    /// per-op conditional effects through [`Self::cond_effs`] only — never
    /// crack `cond` open alone.
    pub shared_cond: Arc<[CondEff]>,
    /// Per-op tripwire: does this one carry [`Self::shared_cond`]? Live for
    /// ops grounded off actions with the monitor block; dark for the
    /// synthetic bookkeeping ops (P3*, TRAJ-END, REACH-GOAL).
    pub monitored: Arc<[bool]>,

    /// fact id -> ops that light it up (achiever lookup — skips the O(n_ops)
    /// crawl).
    pub add_by_fact: Csr<u32>,
    /// fluent id -> ops carrying a numeric hit on it (numeric-achiever
    /// lookup).
    pub neff_by_fluent: Csr<u32>,
    /// fluent id -> flagged live by some numeric precondition or goal
    /// (widening filter).
    pub relevant_fluent: Vec<bool>,
    /// the live fluent ids, sorted — the compact `state_key` value vector.
    pub rel_fluents: Vec<u32>,

    pub init_bits: Vec<u64>,
    pub fv0: Vec<f64>,
    pub fdef0: Vec<bool>,

    pub goal_pos: Vec<u32>,
    pub goal_num: Vec<NumPre>,

    pub fact_names: Arc<[String]>,
    /// fluent id -> display string `(NAME ARGS)` for metric/cost-fluent
    /// lookup.
    pub fluent_names: Arc<[String]>,

    // timing-footer stats
    pub n_easy: usize,
    pub n_hard: usize,
    pub n_reach_facts: usize,
    pub n_reach_actions: usize,
    pub n_relevant_fluents: usize,
}

// PackedTask is read-only during search, so sharing &PackedTask across threads
// is sound. (All fields are Send + Sync.)

impl PackedTask {
    #[inline]
    pub fn op_applicable(&self, oi: usize, s: &State) -> bool {
        self.pre_pos
            .slice(oi)
            .iter()
            .all(|&f| bitset::test(&s.bits, f as usize))
            && self
                .pre_num
                .slice(oi)
                .iter()
                .all(|np| eval_numpre(np, &s.fv, &s.fdef).unwrap_or(false))
    }

    /// Every conditional effect op `oi` runs: its own `cond` row first, then
    /// — for monitored ops — the shared monitor block. 0.7 tail order held,
    /// so achiever/bucket/apply orders stay identical.
    #[inline]
    pub fn cond_effs(&self, oi: usize) -> impl Iterator<Item = &CondEff> + Clone {
        let shared: &[CondEff] = if self.monitored[oi] {
            &self.shared_cond
        } else {
            &[]
        };
        self.cond.slice(oi).iter().chain(shared.iter())
    }

    /// Body count: conditional effects op `oi` runs (own + shared).
    #[inline]
    pub fn n_cond_effs(&self, oi: usize) -> usize {
        self.cond.slice(oi).len()
            + if self.monitored[oi] {
                self.shared_cond.len()
            } else {
                0
            }
    }

    /// Does the tripwire `ce` snap in source state `s`?
    #[inline]
    fn cond_holds(&self, ce: &CondEff, s: &State) -> bool {
        ce.cond_pos
            .iter()
            .all(|&f| bitset::test(&s.bits, f as usize))
            && ce
                .cond_neg
                .iter()
                .all(|&f| !bitset::test(&s.bits, f as usize))
            && ce
                .cond_num
                .iter()
                .all(|np| eval_numpre(np, &s.fv, &s.fdef).unwrap_or(false))
    }

    /// Run op `oi` against `s`, hand back the successor state. No safety
    /// check — assumes applicable, you called `op_applicable` already.
    /// Every effect, unconditional and any tripwire that snapped, reads off
    /// the SAME source-state snapshot and lands at once: dels first, adds
    /// after (add wins on conflict); numeric deltas summed from source.
    pub fn apply(&self, oi: usize, s: &State) -> State {
        let mut ns = s.clone();
        let conds: Vec<&CondEff> = self.cond_effs(oi).collect();
        let firing: Vec<bool> = conds.iter().map(|ce| self.cond_holds(ce, s)).collect();

        // numeric deltas (from source): unconditional + firing conditional
        let mut deltas: Vec<(usize, AssignOp, f64)> = self
            .num_eff
            .slice(oi)
            .iter()
            .map(|ne| {
                (
                    ne.target as usize,
                    ne.op,
                    ne.value.eval(&s.fv, &s.fdef).unwrap_or(0.0),
                )
            })
            .collect();
        for (ce, &fire) in conds.iter().zip(&firing) {
            if fire {
                for ne in &ce.num {
                    deltas.push((
                        ne.target as usize,
                        ne.op,
                        ne.value.eval(&s.fv, &s.fdef).unwrap_or(0.0),
                    ));
                }
            }
        }

        // logical: all dels first, then all adds
        for &f in self.del.slice(oi) {
            bitset::clear(&mut ns.bits, f as usize);
        }
        for (ce, &fire) in conds.iter().zip(&firing) {
            if fire {
                for &f in &ce.del {
                    bitset::clear(&mut ns.bits, f as usize);
                }
            }
        }
        for &f in self.add.slice(oi) {
            bitset::set(&mut ns.bits, f as usize);
        }
        for (ce, &fire) in conds.iter().zip(&firing) {
            if fire {
                for &f in &ce.add {
                    bitset::set(&mut ns.bits, f as usize);
                }
            }
        }

        for (t, aop, v) in deltas {
            match aop {
                AssignOp::Assign => {
                    ns.fv[t] = v;
                    ns.fdef[t] = true;
                }
                AssignOp::Increase => ns.fv[t] += v,
                AssignOp::Decrease => ns.fv[t] -= v,
                AssignOp::ScaleUp => ns.fv[t] *= v,
                AssignOp::ScaleDown => ns.fv[t] /= v,
            }
        }
        ns
    }

    /// Trace a fluent id from a display string, e.g. `(TOTAL-COST)`.
    pub fn fluent_id(&self, disp: &str) -> Option<usize> {
        self.fluent_names.iter().position(|s| s == disp)
    }

    /// Look up a fact id by display string, e.g. `(AT A0 P1)`.
    pub fn fact_id(&self, disp: &str) -> Option<usize> {
        self.fact_names.iter().position(|s| s == disp)
    }

    pub fn initial(&self) -> State {
        State {
            bits: self.init_bits.clone(),
            fv: self.fv0.clone(),
            fdef: self.fdef0.clone(),
        }
    }

    pub fn goal_met(&self, s: &State) -> bool {
        self.goal_met_with(s, &self.goal_pos, &self.goal_num)
    }

    /// The visited-set fingerprint: facts, plus only the fluent values that
    /// matter. A fluent goes dark iff it never surfaces in a
    /// precondition/goal AND, transitively, never feeds the RHS of an
    /// effect writing a live fluent — a pure write-only accumulator
    /// (walkedTime/drivenTime/fuelUsed, ticking away unread). Such a fluent
    /// can't move applicability or the goal, ever — two states differing
    /// only there are the same mark and must collapse to one; skip that and
    /// an unbounded counter spins out infinite "distinct" states, the
    /// search never closes on an unsolvable board. `relevant_fluent` is the
    /// transitive closure built in ground.rs — that's what keeps this
    /// sound.
    pub fn state_key(&self, s: &State) -> StateKey {
        // Compact: only the RELEVANT fluents (usually few) go in the key, in a
        // fixed order. Irrelevant/undefined ones never distinguish states, so
        // omitting them is exact and shrinks the cloned+hashed key dramatically
        // (pure-STRIPS keys carry no vals at all).
        let vals: Vec<i64> = self
            .rel_fluents
            .iter()
            .map(|&i| {
                let i = i as usize;
                if s.fdef[i] {
                    (s.fv[i] * 1e6).round() as i64
                } else {
                    0
                }
            })
            .collect();
        StateKey {
            bits: s.bits.clone(),
            vals,
        }
    }

    /// Visited-key variant for the branch-and-bound bounded search: the
    /// compact key with the cost fluent's value tacked on, so two states
    /// with matching facts but different cost stay distinct (the cost
    /// fluent is read by no precond/goal, so `rel_fluents` never sees it).
    /// One code path serves both init and successors.
    pub fn state_key_with_cost(&self, s: &State, cost_fluent: Option<usize>) -> StateKey {
        let mut k = self.state_key(s);
        if let Some(cf) = cost_fluent {
            k.vals.push(if s.fdef[cf] {
                (s.fv[cf] * 1e6).round() as i64
            } else {
                0
            });
        }
        k
    }

    /// Goal test against an arbitrary (sub)goal — used by the subplanner API.
    pub fn goal_met_with(&self, s: &State, goal_pos: &[u32], goal_num: &[NumPre]) -> bool {
        goal_pos.iter().all(|&f| bitset::test(&s.bits, f as usize))
            && goal_num
                .iter()
                .all(|np| eval_numpre(np, &s.fv, &s.fdef).unwrap_or(false))
    }
}

/// A snapshot of the world mid-run: fact bitset + dense fluent values.
#[derive(Clone)]
pub struct State {
    pub bits: Vec<u64>,
    pub fv: Vec<f64>,
    pub fdef: Vec<bool>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StateKey {
    pub bits: Vec<u64>,
    pub vals: Vec<i64>,
}
