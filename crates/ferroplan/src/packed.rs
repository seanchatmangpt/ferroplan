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

/// Build the successor generator's index from the positive preconditions:
/// each op anchored at its rarest precondition fact (ties to the smallest
/// fact id), ops without one in the always-list.
pub fn build_succ(pre_pos: &Csr<u32>, n_facts: usize, n_ops: usize) -> (Csr<u32>, Vec<u32>) {
    let mut uses = vec![0u32; n_facts];
    for oi in 0..n_ops {
        for &f in pre_pos.slice(oi) {
            uses[f as usize] += 1;
        }
    }
    let mut anchored: Vec<Vec<u32>> = vec![Vec::new(); n_facts];
    let mut always = Vec::new();
    for oi in 0..n_ops {
        let pre = pre_pos.slice(oi);
        match pre.iter().copied().min_by_key(|&f| (uses[f as usize], f)) {
            Some(f) => anchored[f as usize].push(oi as u32),
            None => always.push(oi as u32),
        }
    }
    let mut b = CsrBuilder::new();
    for row in anchored {
        b.push_row(row);
    }
    (b.finish(), always)
}

/// The grounded planning task in data-oriented form.
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
    /// THE SUCCESSOR GENERATOR (0.27, the per-evaluation-cost lane). Every
    /// op is anchored at ONE of its positive preconditions -- the fact that
    /// anchors the fewest ops, so the candidate lists stay short -- and
    /// `succ_by_fact.slice(f)` is the ops anchored at `f`. An op with no
    /// positive precondition lives in `succ_always`. Applicability is then
    /// a walk over the state's TRUE facts rather than over every grounded
    /// op: the expansion loops used to test all `n_ops` per node, which on
    /// a 61k-op grounding is the wall (driver-log-2014 at 1k evals/s).
    /// Exact, and order-preserving: `applicable_ops` returns the same ops
    /// the linear scan did, in the same order, so search is byte-identical.
    pub succ_by_fact: Csr<u32>,
    pub succ_always: Arc<[u32]>,
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

    /// Arm the numeric-precondition charge (0.21 Phase 3) in relaxed-plan
    /// extraction. True on the classical/numeric grounding entries; on the
    /// temporal snap/session entries (stratified/fixpoint), whose compiled
    /// tasks always carry `pre_num`, true only under `FF_NUMPRE_TEMPORAL`
    /// (0.26 F3, opt-in) — the pre-damping charge re-routed the village
    /// workshop economy (27-step carve plan → 47-step chisel-sale plan),
    /// and the temporal boards are other phases' referee surface, so
    /// unset they keep 0.20's h byte-identical. `FF_NO_NUMPRE` is the deep
    /// restore either way (heuristic.rs gate).
    pub charge_pre_num: bool,

    /// The h-surgery probe (0.21 Phase 8, opt-in `FF_H_ENDGATE=1`): op id ->
    /// paired END op id for snap-START ops, `u32::MAX` otherwise. Populated
    /// ONLY by the temporal think paths (from `Kind::Start { end_op }`, after
    /// `build_kind`) and only under the flag; `None` everywhere else, so the
    /// classical heuristic provably never enters the end-gate discount.
    pub pair_end: Option<Vec<u32>>,

    /// TRPG-lite tables (0.23 Phase 4 probe 2, opt-in `FF_TRPG=1`): the
    /// time-stamped relaxation's per-task constants — END fire anchors,
    /// TIL floors, and the over-all-invariant windows the END payout is
    /// gated on. Populated ONLY by the temporal solve path (from
    /// `build_kind`'s classification + the `InvMap` + the TIL agenda) and
    /// only under the flag; `None` everywhere else, so the classical
    /// heuristic provably never enters the timed build (the `pair_end`
    /// rule). Arc'd: the table is search-lifetime read-only.
    pub trpg: Option<Arc<crate::heuristic::TrpgInfo>>,

    pub fact_names: Arc<[String]>,
    /// fluent id -> display string `(NAME ARGS)` for metric/cost-fluent
    /// lookup.
    pub fluent_names: Arc<[String]>,
    /// DEFINED-STATIC fluents the 0.21 fluent compaction dropped from
    /// `fv0`/`fdef0` (display -> init value), retained task-side for
    /// NAME-resolved readers: temporal duration grounding/eval reads
    /// statics from here when [`Self::fluent_id`] misses. Empty when the
    /// compaction is off (`FF_NO_FLUENT_COMPACT=1`) or on the session/
    /// validator grounding entries, which keep full tables.
    pub static_fluents: Arc<[(String, f64)]>,

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
    /// The ops applicable in `s`, ascending by op index -- exactly what
    /// `(0..n_ops).filter(|oi| op_applicable(oi, s))` yields, found through
    /// the anchor index instead of a full scan. `out` is cleared first.
    pub fn applicable_ops(&self, s: &State, out: &mut Vec<u32>) {
        out.clear();
        out.extend_from_slice(&self.succ_always);
        for (wi, &w) in s.bits.iter().enumerate() {
            let mut bits = w;
            while bits != 0 {
                let b = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let f = wi * 64 + b;
                if f < self.n_facts {
                    out.extend_from_slice(self.succ_by_fact.slice(f));
                }
            }
        }
        // Each op is anchored once, so there are no duplicates; the sort
        // restores the op-index order the linear scan produced.
        out.sort_unstable();
        out.retain(|&oi| self.op_applicable(oi as usize, s));
    }

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

    /// Does conditional effect `ce` fire in source state `s`? Shared with
    /// the temporal monitor context's pending-violation check (0.23
    /// Phase 2), which asks it about the SHARED monitor transitions.
    #[inline]
    pub(crate) fn cond_holds(&self, ce: &CondEff, s: &State) -> bool {
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
        // The common op has no conditional and no numeric effects: dels then
        // adds, and none of the four temporaries the general path builds
        // (0.27, the per-evaluation-cost lane -- this runs once per
        // successor, and `insert` was a third of a best-first wall).
        if self.n_cond_effs(oi) == 0 && self.num_eff.slice(oi).is_empty() {
            for &f in self.del.slice(oi) {
                bitset::clear(&mut ns.bits, f as usize);
            }
            for &f in self.add.slice(oi) {
                bitset::set(&mut ns.bits, f as usize);
            }
            return ns;
        }
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

    /// Value of a DEFINED-STATIC fluent the 0.21 compaction dropped from
    /// `fv0` — the name-resolved fallback behind [`Self::fluent_id`].
    /// `None` = not a dropped static (either live in `fv0`, or undefined —
    /// undefined statics never enter this table, so a miss on both sources
    /// reads exactly like an undefined fluent).
    pub fn static_fluent(&self, disp: &str) -> Option<f64> {
        self.static_fluents
            .iter()
            .find(|(n, _)| n == disp)
            .map(|&(_, v)| v)
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

    /// Streaming hash of EXACTLY the content [`Self::state_key_with_cost`]
    /// would clone: bits words, relevant-fluent quantized vals, then the
    /// cost val when present (0.20 Phase 4, retained-state compression).
    /// With [`Self::state_key_eq`] it lets a visited structure hold one
    /// u64 and a node index per state instead of a second copy of the
    /// bitset — the dedup verdicts are identical (equality stays exact;
    /// the hash only routes to candidates), so search behavior is
    /// byte-identical.
    pub fn state_key_hash(&self, s: &State, cost_fluent: Option<usize>) -> u64 {
        use std::hash::Hasher;
        let mut h = crate::hash::FxHasher::default();
        for &w in &s.bits {
            h.write_u64(w);
        }
        for &i in self.rel_fluents.iter() {
            h.write_i64(Self::quantized(s, i as usize));
        }
        if let Some(cf) = cost_fluent {
            h.write_i64(Self::quantized(s, cf));
        }
        h.finish()
    }

    /// Exact equality of the [`Self::state_key_with_cost`] content between
    /// two states — the collision check behind [`Self::state_key_hash`].
    pub fn state_key_eq(&self, a: &State, b: &State, cost_fluent: Option<usize>) -> bool {
        a.bits == b.bits
            && self
                .rel_fluents
                .iter()
                .all(|&i| Self::quantized(a, i as usize) == Self::quantized(b, i as usize))
            && cost_fluent.map_or(true, |cf| Self::quantized(a, cf) == Self::quantized(b, cf))
    }

    // pub(crate): the FF_NUMNOV novelty envelope quantizes fluents with
    // exactly the state-key contract (one 1e-6 quantizer everywhere).
    #[inline]
    pub(crate) fn quantized(s: &State, i: usize) -> i64 {
        if s.fdef[i] {
            (s.fv[i] * 1e6).round() as i64
        } else {
            0
        }
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

#[cfg(test)]
mod succ_tests {
    use super::*;

    fn task(dom: &str, prb: &str) -> PackedTask {
        let d = crate::parser::parse_domain(dom).unwrap();
        let p = crate::parser::parse_problem(prb).unwrap();
        crate::ground::ground_task(&d, &p, 1).unwrap()
    }

    fn linear(task: &PackedTask, s: &State) -> Vec<u32> {
        (0..task.n_ops)
            .filter(|&oi| task.op_applicable(oi, s))
            .map(|oi| oi as u32)
            .collect()
    }

    /// The generator returns exactly the linear scan's ops, in its order,
    /// on the initial state and on every state one step out -- including
    /// an op with no positive precondition, which lives in the always-list.
    #[test]
    fn the_generator_agrees_with_the_linear_scan_everywhere_it_is_asked() {
        let t = task(
            "(define (domain g) (:requirements :typing)
               (:types loc obj)
               (:predicates (at ?o - obj ?l - loc) (free ?l - loc) (tick))
               (:action move :parameters (?o - obj ?a ?b - loc)
                 :precondition (and (at ?o ?a) (free ?b))
                 :effect (and (not (at ?o ?a)) (at ?o ?b) (free ?a) (not (free ?b))))
               (:action wait :parameters () :precondition () :effect (tick))
               (:action unwait :parameters () :precondition (tick) :effect (not (tick))))",
            "(define (problem g1) (:domain g)
               (:objects a b c - loc x y - obj)
               (:init (at x a) (at y b) (free c))
               (:goal (and (at x c) (tick))))",
        );
        assert!(t.n_ops >= 5, "{} ops", t.n_ops);
        assert!(
            !t.succ_always.is_empty(),
            "wait has no positive precondition"
        );
        let init = t.initial();
        let mut out = Vec::new();
        t.applicable_ops(&init, &mut out);
        assert_eq!(out, linear(&t, &init));
        assert!(!out.is_empty());
        for &oi in &out.clone() {
            let ns = t.apply(oi as usize, &init);
            t.applicable_ops(&ns, &mut out);
            assert_eq!(out, linear(&t, &ns), "after op {oi}");
            for &oj in &out.clone() {
                let ns2 = t.apply(oj as usize, &ns);
                t.applicable_ops(&ns2, &mut out);
                assert_eq!(out, linear(&t, &ns2), "after ops {oi},{oj}");
            }
        }
    }

    /// Every op is anchored exactly once, at a fact among its preconditions.
    #[test]
    fn every_op_is_anchored_once() {
        let t = task(
            "(define (domain a) (:predicates (p) (q) (r))
               (:action x :precondition (and (p) (q)) :effect (r))
               (:action y :precondition (q) :effect (p))
               (:action z :precondition () :effect (q)))",
            "(define (problem a1) (:domain a) (:init (p)) (:goal (r)))",
        );
        let mut seen = vec![0u32; t.n_ops];
        for f in 0..t.n_facts {
            for &oi in t.succ_by_fact.slice(f) {
                assert!(t.pre_pos.slice(oi as usize).contains(&(f as u32)));
                seen[oi as usize] += 1;
            }
        }
        for &oi in t.succ_always.iter() {
            assert!(t.pre_pos.slice(oi as usize).is_empty());
            seen[oi as usize] += 1;
        }
        assert!(seen.iter().all(|&n| n == 1), "{seen:?}");
    }
}
