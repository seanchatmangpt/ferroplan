//! FIELD LOG: data-parallel weighted best-first sweep.
//!
//! Every round, the swarm surfaces one batch of the lowest-cost contacts,
//! cracks them open, spawns every legal successor, scrubs the duplicates
//! against what's already been walked, then runs the FF heuristic across
//! the whole batch AT ONCE — every worker screaming in parallel, the one
//! real cost center in the whole operation. `par_map` never breaks
//! sequence and control never leaves the one thread, so the recovered
//! plan is bit-identical no matter how many hands touch it. Only the
//! clock on the wall moves.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

use crate::hash::FxHashSet;
use crate::heuristic::{relaxed_costed, relaxed_helpful, relaxed_to, Scratch};
use crate::packed::{PackedTask, State, StateKey};
use crate::par;
use crate::types::NumPre;

/// Frontier batch size — LOCKED, not scaled by hands on deck. Expansion
/// order, the recovered plan, the body count of evaluated states — all
/// stay identical no matter how many workers show up. More hands only
/// split the h-eval load inside one batch.
const BATCH: usize = 256;
/// The kill switch. States evaluated past this line, the sweep calls it —
/// deterministic, no negotiation.
pub const DEFAULT_MAX_EVAL: usize = 5_000_000;
/// Fixed-point scale for the fractional heuristic weights — keeps the
/// priority key an integer so the heap order, and the plan riding on it,
/// never drifts.
const WEIGHT_SCALE: f64 = 256.0;

/// Deterministic retained-memory target for one `search_from` pass (0.8
/// Phase 3, docs/roadmap-0.8.md): the append-only `nodes` store and the
/// `visited` keys both grow one entry per inserted successor, each carrying
/// the full state bitset — on monitor-widened tasks a single unbounded pass
/// OOMs a 15 GB box while `max_eval` (which counts only POPPED nodes) never
/// fires. The insertion cap derives from this byte target over a MODEL of
/// per-insertion cost (never RSS, never wall clock — the count is serial and
/// the model uses only static task dimensions, so the cap is identical on
/// any machine at any thread count). A capped pass returns its anytime
/// incumbent (or `Unsolvable{capped:true}`), which every caller's
/// ladder/budget machinery already treats as inconclusive — `proven` stays
/// honest. The default is far above every green fixture's retained size
/// (largest measured: ~5.4 GB total on storage qualpref p08);
/// `FF_SEARCH_NODE_CAP` overrides the node count directly (`0` disables).
pub(crate) const NODE_CAP_TARGET_BYTES: usize = 8 << 30;

/// The per-body byte model behind [`NODE_CAP_TARGET_BYTES`]: one stored
/// `State` (bits plus fluent vectors) in `nodes`, plus its hash-index
/// dedup marker (0.20 Phase 4 — the visited set quit cloning the bitset
/// a second time).
pub(crate) fn node_cap_for(task: &PackedTask) -> usize {
    node_cap_for_bytes(task, NODE_CAP_TARGET_BYTES)
}

/// [`node_cap_for`] against an explicit byte target (the budgeted-think
/// surface); `FF_SEARCH_NODE_CAP` still overrides the count directly.
pub(crate) fn node_cap_for_bytes(task: &PackedTask, bytes: usize) -> usize {
    if let Ok(v) = std::env::var("FF_SEARCH_NODE_CAP") {
        if let Ok(n) = v.trim().parse::<usize>() {
            return if n == 0 { usize::MAX } else { n };
        }
    }
    let per_node = 2 * task.words * 8 + task.fv0.len() * 8 + task.fdef0.len() + 96;
    bytes / per_node.max(1)
}

/// The dials on the weighted-best-first rig, wired through the library
/// `Options`. `w_g`/`w_h` ride as pre-scaled integers (`weight *
/// WEIGHT_SCALE`) — the default `1·g + 5·h` gait holds exact while the
/// fractional weights still turn.
///
/// Field calibration, for anyone adding a new term to the mix: one
/// h-unit runs 1280 (5·256), one g-step runs 256, one unsatisfied
/// preference runs `weight·100` (`SatGuidance::penalty`), one metric-cost
/// unit runs `w_c·256`. `w_c` defaults to 0.0 — term absent, key
/// bit-identical to the old rig — and folds the successor's accrued
/// metric cost `fv[cost_fluent]` into the ordering. The preference-metric
/// branch-and-bound loops lean on it so a numeric metric (rovers'
/// traverse costs) and the forgo-vs-satisfy calculus can steer the open
/// list instead of only cutting at the bound.
#[derive(Clone, Copy, Debug)]
pub struct SearchCfg {
    pub w_g: i64,
    pub w_h: i64,
    pub max_eval: usize,
    pub w_c: f64,
    /// Swap h for the COST-augmented relaxed plan ([`relaxed_costed`]:
    /// chosen-op cost plus length) toward the named fluent, instead of raw
    /// length. The `:action-costs` sweep runs on this so the guidance
    /// hunts cheap kills. `None` (default) keeps the length-h gait
    /// bit-for-bit unchanged.
    pub h_cost: Option<usize>,
    /// In-sweep tightening for the bounded metric runs: an accepting
    /// state gets logged as the incumbent and the bound TIGHTENS in
    /// place, no return, no restart. The sweep keeps draining — no
    /// prefix re-tread — until the eval ceiling or the open list runs
    /// dry, then hands back the best body found. Off (`false`) is the
    /// old first-kill behavior, bit-for-bit; only the preference
    /// branch-and-bound loops arm it (`FF_PREF_GREEDY=1` reverts to
    /// first-kill there). All tightening happens in the serial
    /// acceptance lane, so determinism and thread-count independence
    /// both hold.
    pub anytime: bool,
    /// The plan-LENGTH tripwire: successors at or past this depth never
    /// get inserted — a goal struck at depth g is a plan of length g, so
    /// nothing at or past the bound can outrun the incumbent that set it.
    /// `usize::MAX` (the default everywhere) means the tripwire never
    /// fires — old behavior exactly. The iterated-weight length sweep
    /// (`costs::improve_length`) leans on this; pruned states are never
    /// visited-marked, so a shorter route through a different parent
    /// stays open.
    pub g_bound: usize,
    /// Length-anytime WITHIN one run (0.10 Phase 3): first kill logs the
    /// incumbent, tightens a live g-bound to its length, and keeps
    /// draining the SAME open list — no restart, no prefix re-tread —
    /// hunting a strictly shorter plan until the drain ceiling. Eval
    /// count doubles at most (ceiling = evals-at-first-kill × 2, still
    /// under `max_eval`). Opt-in via `FF_LEN_ANYTIME=1`, measured and
    /// off by default: at the 60 s scoreboard budget the doubled drain
    /// cost 9 instances of coverage across floor-tile/visit-all/sokoban
    /// (sokoban −7) against 4 shorter sokoban plans (−234 steps) and zero
    /// gains on floor-tile/visit-all — same verdict as 0.9's
    /// improve_length restarts. Mutually exclusive with metric `anytime`.
    pub len_anytime: bool,
    /// Landmark-count ordering term (0.11 Phase 3, pre-scaled like
    /// `w_g`): folds `w_lm × unaccepted-landmark-count` into the
    /// best-first key. 0 (default) is silent — key bit-identical. The
    /// bounded experiment: `FF_CLM=<weight>` arms it, but only on the
    /// ladder's best-first fallback.
    pub w_lm: i64,
    /// Resource-trip ordering term (0.14 ext Phase 11, pre-scaled like
    /// `w_g`): folds `w_res × ⌈unmet linked goals / pool capacity⌉` into
    /// the best-first key — a semantic-landmark rung the delete
    /// relaxation is blind to (counter levels pile up under relaxation
    /// and it never sees the ceiling). 0 (default) is silent — key
    /// bit-identical. Opt-in: `FF_RESLM=<weight>` on the ladder's
    /// best-first fallback; the [`crate::resource::TripBound`] payload
    /// rides the `RESLM` cell `resolve::solve` sets while it's still
    /// holding the mutex groups.
    pub w_res: i64,
    /// Per-run retained-memory target in raw bytes (0.11 Phase 4, the
    /// budgeted-think hook): overrides the default 8 GiB ceiling inside
    /// the `node_cap_for` byte model. `None` keeps the default.
    /// Deterministic — static task dimensions only — because a think
    /// budget has to bound memory without smuggling in wall-clock
    /// nondeterminism. `FF_SEARCH_NODE_CAP` still overrides when set.
    pub node_bytes_target: Option<usize>,
}

impl Default for SearchCfg {
    fn default() -> Self {
        SearchCfg::from_weights(1.0, 5.0, None)
    }
}

impl SearchCfg {
    /// Assembled from human-facing f64 weights. `weight_g = 1.0, weight_h =
    /// 5.0` reproduces the old `1·g + 5·h` gait bit-for-bit.
    ///
    /// Every input gets scrubbed first — a malformed weight must never
    /// collapse or overflow the integer heap key. A non-finite or
    /// negative weight falls back to that term's default, everything
    /// gets clamped to a sane ceiling, and if both round to zero the
    /// defaults come back online (an all-zero key would degrade to raw
    /// insertion order — no ordering at all).
    pub fn from_weights(weight_g: f64, weight_h: f64, max_eval: Option<usize>) -> Self {
        let san = |w: f64, default: f64| {
            if w.is_finite() && w >= 0.0 {
                w.min(1e9)
            } else {
                default
            }
        };
        let mut w_g = (san(weight_g, 1.0) * WEIGHT_SCALE).round() as i64;
        let mut w_h = (san(weight_h, 5.0) * WEIGHT_SCALE).round() as i64;
        if w_g == 0 && w_h == 0 {
            w_g = WEIGHT_SCALE as i64;
            w_h = (5.0 * WEIGHT_SCALE) as i64;
        }
        SearchCfg {
            w_g,
            w_h,
            max_eval: max_eval.unwrap_or(DEFAULT_MAX_EVAL),
            w_c: 0.0,
            h_cost: None,
            anytime: false,
            g_bound: usize::MAX,
            len_anytime: false,
            w_lm: 0,
            w_res: 0,
            node_bytes_target: None,
        }
    }

    /// Retunes h to the cost-augmented relaxed plan toward `cost_fluent`
    /// (see the `h_cost` field notes).
    pub fn with_cost_h(mut self, cost_fluent: usize) -> Self {
        self.h_cost = Some(cost_fluent);
        self
    }

    /// Dials in a metric-cost ordering weight (see the struct notes).
    /// Non-finite or negative weights scrub to 0.0 — term goes silent —
    /// same discipline as `from_weights`.
    pub fn with_cost_weight(mut self, w_c: f64) -> Self {
        self.w_c = if w_c.is_finite() && w_c > 0.0 {
            w_c.min(1e9)
        } else {
            0.0
        };
        self
    }
}

pub enum PlanResult {
    Plan {
        ops: Vec<usize>,
        advance: Vec<i32>,
        evaluated: usize,
        max_g: usize,
    },
    Unsolvable {
        evaluated: usize,
        capped: bool, // true if the MAX_EVAL safety cap was hit (not proven unsolvable)
    },
}

struct Node {
    state: State,
    father: usize,
    op: usize,
    g: usize,
    /// Landmarks struck along the route so far (0.11 Phase 3, `w_lm`
    /// only — stays empty, no allocation, when the term is dark).
    lm_acc: Vec<u64>,
}

/// (Three copies of the same small tools, deliberately — the lama.rs
/// shape. search/lama/temporal each keep their own: every rig owns its
/// node layout and its hot loop, no shared dependency to drag along.)
fn clm_accept_into(accepted: &mut [u64], lms: &[u32], state: &State) {
    for (i, &f) in lms.iter().enumerate() {
        if accepted[i >> 6] & (1 << (i & 63)) == 0 && crate::bitset::test(&state.bits, f as usize) {
            accepted[i >> 6] |= 1 << (i & 63);
        }
    }
}

fn clm_unaccepted(accepted: &[u64], n: usize) -> i64 {
    n as i64 - accepted.iter().map(|w| w.count_ones() as i64).sum::<i64>()
}

/// A preference's phi, grounded in DNF: holds in a state iff ANY one
/// disjunct's positive facts all check out and its numeric comparisons
/// all clear — negative literals already arrive as compiled complements,
/// so positives alone tell the whole story. Built off the `P3COLLECT-i`
/// ops' preconditions, one disjunct per op. An empty disjunct list means
/// phi is dead on arrival — never holds, not once.
pub struct PrefPhi {
    pub disjuncts: Vec<(Vec<u32>, Vec<NumPre>)>,
}

impl PrefPhi {
    #[inline]
    pub fn holds(&self, s: &State) -> bool {
        self.disjuncts.iter().any(|(pos, num)| {
            pos.iter()
                .all(|&f| crate::bitset::test(&s.bits, f as usize))
                && num
                    .iter()
                    .all(|np| crate::types::eval_numpre(np, &s.fv, &s.fdef).unwrap_or(false))
        })
    }
}

/// The exact toll of a state's unpaid preferences — the summed weight of
/// every instance whose phi doesn't hold, precisely what the phase tail
/// ([`crate::pddl3::PhaseTail`]) will bleed in forgo actions if the plan
/// stops right here (`P3END` only flips phase facts — no phi changes).
/// Hand this to [`search_from`] and the goal test switches to
/// metric-bounded acceptance: a popped state clears iff the real goal
/// holds AND `cost-so-far + closure < bound`. The search then walks REAL
/// states only — the compiled bookkeeping goals never so much as enter
/// the room.
pub struct ClosureCost {
    /// `(forgo weight, phi)` per preference instance with positive weight.
    pub prefs: Vec<(f64, PrefPhi)>,
}

impl ClosureCost {
    pub fn cost(&self, s: &State) -> f64 {
        self.prefs
            .iter()
            .filter(|(_, phi)| !phi.holds(s))
            .map(|(w, _)| w)
            .sum()
    }
}

/// Metric guidance: tilts the open list toward states paying off more
/// preferences. Each entry rides as `(phi, heap penalty while that
/// preference stays unsatisfied)` — phi in full DNF ([`PrefPhi`]), so
/// `imply`/`exists` preferences steer correctly instead of vanishing.
/// Evaluated on the concrete successor state, so it sees the real payoff
/// — unlike the delete-relaxed heuristic, blinded by the free
/// Keyder-Geffner forgo action. Only the metric branch-and-bound passes
/// this in; it reorders nodes, never decides which ones are legal.
/// Payload for the `w_res` trip-bound term (0.14 ext Phase 11). Set once
/// per process by `resolve::solve` — the caller still holding the mutex
/// groups — when `FF_RESLM` is armed; read by `search_from` only when
/// `cfg.w_res > 0`. An experiment hatch, same lifecycle as the env vars
/// gating it.
pub(crate) static RESLM: std::sync::OnceLock<Option<crate::resource::TripBound>> =
    std::sync::OnceLock::new();

pub struct SatGuidance {
    pub prefs: Vec<(PrefPhi, i64)>,
    /// Renewable resources whose live occupancy takes a penalty on the
    /// concrete state — the exact thing delete-relaxation hides from
    /// itself. Empty unless a counter resource was flagged; see
    /// [`crate::resource`].
    pub res: Vec<crate::resource::ResourceVar>,
    /// Per-`occupancy²` weight on the resource term. 0 kills the term
    /// dead — heap key falls back bit-identical to forgone-only.
    pub res_weight: i64,
    /// Only occupancy ABOVE this line takes the hit — penalizes
    /// `(occ-thresh)²`. 0 taxes all occupancy; a value near capacity
    /// taxes only the dead-end zone (every stack committed) without
    /// choking normal pipelining.
    pub res_thresh: i64,
    /// ESPC make-deadline watch: `(trigger fact, deliverable fact,
    /// value)` triples. A **locked loss** — trigger lit, deliverable
    /// dark — means a once-only conditional payoff (openstacks'
    /// `make-product`, which fires only while `(not (made p))`) already
    /// fired WITHOUT delivering. `value` of metric is gone for good. The
    /// delete-relaxed RPG can't see this — it can re-add the deliverable
    /// as if nothing happened — so reading it off the CONCRETE state
    /// steers the search to satisfy the enabling condition (start the
    /// order) BEFORE the trigger goes off. Empty, or weight 0, means
    /// inert — key bit-identical to today.
    pub deadline: Vec<(u32, u32, i64)>,
    /// λ multiplier on the deadline term (0 kills it, keeps the key an
    /// integer).
    pub deadline_weight: i64,
}

impl SatGuidance {
    /// Total toll of preferences still unpaid in `s`.
    fn forgone(&self, s: &State) -> i64 {
        let mut pen = 0;
        for (phi, p) in &self.prefs {
            if !phi.holds(s) {
                pen += *p;
            }
        }
        pen
    }

    /// Combined heap-ordering toll: unpaid preferences plus a CONVEX
    /// renewable-resource occupancy term (`w · occupancy²`, summed over
    /// every flagged resource). Both read off the concrete state, so the
    /// search sees the resource the delete-relaxed heuristic can't.
    /// Ordering only, never legality — completeness stays intact. The
    /// convex shape punishes high simultaneous occupancy at the peak,
    /// nudging toward "release before you grab more".
    fn penalty(&self, s: &State) -> i64 {
        let mut pen = self.forgone(s);
        if self.res_weight != 0 {
            for r in &self.res {
                let over = (r.occupancy(&s.bits) as i64 - self.res_thresh).max(0);
                pen += self.res_weight * over * over;
            }
        }
        if self.deadline_weight != 0 {
            for &(trigger, deliverable, val) in &self.deadline {
                if crate::bitset::test(&s.bits, trigger as usize)
                    && !crate::bitset::test(&s.bits, deliverable as usize)
                {
                    pen += self.deadline_weight * val;
                }
            }
        }
        pen
    }
}

/// Runs toward an ARBITRARY (sub)goal from an arbitrary start state, over
/// a grounded task shared with the rest of the operation — the reusable
/// subplanner entry point for SGPlan-style partition-and-resolve.
/// `search` is the whole-task convenience wrapper riding on top.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn search_from(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: Option<usize>,
    cost_bound: f64,
    threads: usize,
    cfg: SearchCfg,
    forbidden: &[bool],
    sat: Option<&SatGuidance>,
    closure: Option<&ClosureCost>,
) -> PlanResult {
    let batch = BATCH;
    let node_cap = match cfg.node_bytes_target {
        Some(b) => node_cap_for_bytes(task, b),
        None => node_cap_for(task),
    };
    // Phase-time attribution, printed only under FF_RES_DEBUG at the cap
    // return (measurement only — never affects behavior).
    let dbg = std::env::var("FF_RES_DEBUG").is_ok();
    let t_all = crate::clock::Clock::now();
    let (mut t_h, mut t_exp, mut t_ins) = (0u128, 0u128, 0u128);

    let init = start.clone();
    // early dead-end check: if the initial state is a relaxed dead end, unsolvable
    if relaxed_to(
        task,
        &mut Scratch::new(task),
        &init.bits,
        &init.fv,
        &init.fdef,
        goal_pos,
        goal_num,
    )
    .is_none()
    {
        return PlanResult::Unsolvable {
            evaluated: 1,
            capped: false,
        };
    }

    // Classical landmark-count term (0.11 Phase 3, `w_lm` only): landmarks
    // for THIS (start, goal) pair, accepted-bitsets per node.
    let clms: Vec<u32> = if cfg.w_lm > 0 {
        crate::landmarks::landmarks_for(task, start, goal_pos)
    } else {
        Vec::new()
    };
    let reslm: Option<&crate::resource::TripBound> = if cfg.w_res > 0 {
        RESLM.get().and_then(|o| o.as_ref())
    } else {
        None
    };
    let clm_words = clms.len().div_ceil(64);
    let mut root_clm = vec![0u64; clm_words];
    if !clms.is_empty() {
        clm_accept_into(&mut root_clm, &clms, &init);
    }
    let mut nodes: Vec<Node> = vec![Node {
        state: init.clone(),
        father: usize::MAX,
        op: usize::MAX,
        g: 0,
        lm_acc: root_clm,
    }];
    // Deferred evaluation: a node's priority is set from its PARENT's h at
    // insertion; its own h is computed only when it is popped. Many inserted
    // nodes are never popped, so far fewer heuristic evaluations are done.
    // The visited key excludes irrelevant fluents (termination); under
    // branch-and-bound it also appends the cost fluent so equal-fact/different-cost
    // states stay distinct (see PackedTask::state_key_with_cost).
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0))); // init popped first
    let mut visited: FxHashSet<StateKey> = FxHashSet::default();
    visited.insert(task.state_key_with_cost(&init, cost_fluent));

    let mut evaluated = 0usize;
    let mut best = i32::MAX;
    let mut advance: Vec<i32> = Vec::new();
    let mut max_g = 0usize;
    // Anytime in-sweep tightening (`cfg.anytime`, metric B&B loops only): the
    // bound tightens in place on every acceptance and the sweep keeps going —
    // `best_acc` holds the incumbent's node index (nodes is append-only, so
    // reconstruction stays valid at return time).
    let mut cost_bound = cost_bound;
    let mut best_acc: Option<usize> = None;
    // Length-anytime (cfg.len_anytime): incumbent + live length bound + drain
    // ceiling (see the SearchCfg field docs).
    let mut len_bound: usize = usize::MAX;
    let mut len_acc: Option<usize> = None;
    let mut eval_ceiling: usize = cfg.max_eval;

    while !heap.is_empty() {
        // pop a batch of lowest-priority nodes
        let mut popped: Vec<usize> = Vec::with_capacity(batch);
        for _ in 0..batch {
            match heap.pop() {
                Some(Reverse((_, ni))) => popped.push(ni),
                None => break,
            }
        }

        // goal check (cheap, before any heuristic work). With `closure` set,
        // acceptance is METRIC-BOUNDED: the real goal must hold AND the exact
        // preference-closure completion must beat the incumbent bound — the
        // phase tail appended by the caller pays exactly `closure.cost`, so
        // this test accepts precisely the states that improve the metric.
        for &ni in &popped {
            max_g = max_g.max(nodes[ni].g);
            if !task.goal_met_with(&nodes[ni].state, goal_pos, goal_num) {
                continue;
            }
            if cfg.len_anytime {
                let g = nodes[ni].g;
                if g < len_bound {
                    len_bound = g;
                    len_acc = Some(ni);
                    // The drain may spend as much again as the first incumbent
                    // took; a later, shorter incumbent does not extend it.
                    if eval_ceiling == cfg.max_eval {
                        eval_ceiling = evaluated
                            .saturating_mul(2)
                            .max(evaluated + 10_000)
                            .min(cfg.max_eval);
                    }
                }
                continue;
            }
            if cfg.anytime {
                // In-sweep tightening: acceptance = effective plan cost
                // (cost-so-far + exact closure, or plain cost-so-far without a
                // closure) strictly beats the CURRENT bound. Accepting states
                // stay in the batch — a zero-cost extension can still satisfy
                // more preferences and improve again.
                let s = &nodes[ni].state;
                let g = cost_fluent
                    .map(|cf| if s.fdef[cf] { s.fv[cf] } else { 0.0 })
                    .unwrap_or(0.0);
                let eff = g + closure.map_or(0.0, |cl| cl.cost(s));
                if eff < cost_bound {
                    best_acc = Some(ni);
                    cost_bound = eff;
                    if eff <= 0.0 {
                        // Nothing can beat zero: the incumbent is optimal.
                        return PlanResult::Plan {
                            ops: reconstruct(&nodes, ni),
                            advance,
                            evaluated,
                            max_g,
                        };
                    }
                }
                continue;
            }
            if closure.map_or(true, |cl| {
                let s = &nodes[ni].state;
                let g = cost_fluent
                    .map(|cf| if s.fdef[cf] { s.fv[cf] } else { 0.0 })
                    .unwrap_or(0.0);
                g + cl.cost(s) < cost_bound
            }) {
                return PlanResult::Plan {
                    ops: reconstruct(&nodes, ni),
                    advance,
                    evaluated,
                    max_g,
                };
            }
        }

        // Length-anytime: a popped node at depth g spawns goals at >= g+1, so
        // g + 1 >= len_bound can never improve — drop before paying its h.
        if cfg.len_anytime && len_bound < usize::MAX {
            popped.retain(|&ni| nodes[ni].g + 1 < len_bound);
            if popped.is_empty() {
                continue;
            }
        }
        // Anytime: drop popped nodes the tightened bound has made dead — cost
        // is monotone and the closure is non-negative, so cost-so-far >= bound
        // can never reach an accepting state. Saves their h evaluations.
        if cfg.anytime {
            if let Some(cf) = cost_fluent {
                let bound_now = cost_bound;
                popped.retain(|&ni| {
                    let s = &nodes[ni].state;
                    !(s.fdef[cf] && s.fv[cf] >= bound_now)
                });
                if popped.is_empty() {
                    continue;
                }
            }
        }

        // PARALLEL: evaluate h for the popped batch (the only evaluations),
        // each worker reusing one Scratch across its chunk.
        let t_phase = crate::clock::Clock::now();
        let hs: Vec<Option<i32>> = par::par_map_with(
            &popped,
            threads,
            || Scratch::new(task),
            |sc, &ni| {
                let s = &nodes[ni].state;
                match cfg.h_cost {
                    Some(hcf) => {
                        relaxed_costed(task, sc, &s.bits, &s.fv, &s.fdef, goal_pos, goal_num, hcf)
                    }
                    None => relaxed_to(task, sc, &s.bits, &s.fv, &s.fdef, goal_pos, goal_num),
                }
            },
        );
        t_h += t_phase.elapsed_us();
        evaluated += popped.len();
        // The node cap (0.8 Phase 3) trips at the same batch boundary as the
        // eval cap: `nodes.len()` counts INSERTED successors — the quantity
        // that actually holds the memory — and is maintained serially, so the
        // check is thread-count independent. Overshoot is bounded by one
        // batch's insertions (the check precedes this batch's expansion).
        if evaluated > cfg.max_eval || evaluated > eval_ceiling || nodes.len() > node_cap {
            // Anytime: a capped sweep still hands back its incumbent — the
            // caller tightens to its cost and (with budget) sweeps again.
            if dbg {
                use std::sync::atomic::Ordering::Relaxed;
                eprintln!(
                    "[h] reset {}ms, build {}ms, extract {}ms (cumulative worker-thread time)",
                    crate::heuristic::T_RESET.load(Relaxed) / 1000,
                    crate::heuristic::T_BUILD.load(Relaxed) / 1000,
                    crate::heuristic::T_EXTRACT.load(Relaxed) / 1000,
                );
                eprintln!(
                    "[search] capped at {evaluated} evals: h {}ms, expand {}ms, insert {}ms, total {}ms",
                    t_h / 1000,
                    t_exp / 1000,
                    t_ins / 1000,
                    t_all.elapsed_ms()
                );
            }
            if let Some(ni) = best_acc.or(len_acc) {
                return PlanResult::Plan {
                    ops: reconstruct(&nodes, ni),
                    advance,
                    evaluated,
                    max_g,
                };
            }
            return PlanResult::Unsolvable {
                evaluated,
                capped: true,
            };
        }
        for h in hs.iter().flatten() {
            if *h < best {
                best = *h;
                advance.push(*h);
            }
        }

        // PARALLEL: expand non-dead-end popped nodes; successors carry the
        // parent's h as their (deferred) priority key.
        let live: Vec<(usize, i32)> = popped
            .iter()
            .zip(hs.iter())
            .filter_map(|(&ni, h)| h.map(|h| (ni, h)))
            .collect();
        let t_phase = crate::clock::Clock::now();
        let cand_chunks: Vec<Vec<(usize, usize, State, StateKey, i32)>> =
            par::par_map(&live, threads, |&(ni, ph)| {
                let st = &nodes[ni].state;
                let mut v = Vec::new();
                for oi in 0..task.n_ops {
                    if forbidden.get(oi).copied().unwrap_or(false) {
                        continue;
                    }
                    if task.op_applicable(oi, st) {
                        let ns = task.apply(oi, st);
                        if let Some(cf) = cost_fluent {
                            if ns.fdef[cf] && ns.fv[cf] >= cost_bound {
                                continue; // cost already >= bound: cannot beat incumbent
                            }
                        }
                        let k = task.state_key_with_cost(&ns, cost_fluent);
                        v.push((ni, oi, ns, k, ph));
                    }
                }
                v
            });

        t_exp += t_phase.elapsed_us();

        // SERIAL: dedup + insert (deterministic order, independent of threads).
        let t_phase = crate::clock::Clock::now();
        for chunk in cand_chunks {
            for (pi, oi, s, k, ph) in chunk {
                let g = nodes[pi].g + 1;
                if g >= cfg.g_bound || g >= len_bound {
                    continue; // cannot beat the length incumbent (see SearchCfg)
                }
                if visited.insert(k) {
                    // metric guidance: forgone-preference + renewable-resource
                    // occupancy penalty on the concrete successor (steers toward
                    // genuinely satisfying states that stay within resource pools).
                    let sat_pen = sat.map(|sg| sg.penalty(&s)).unwrap_or(0);
                    // metric-cost ordering (w_c, default 0.0 = exact zero term):
                    // single deterministic rounding, 1/256 cost resolution.
                    let cost_term = if cfg.w_c != 0.0 {
                        let c = cost_fluent
                            .map(|cf| if s.fdef[cf] { s.fv[cf] } else { 0.0 })
                            .unwrap_or(0.0);
                        (cfg.w_c * c * WEIGHT_SCALE).round() as i64
                    } else {
                        0
                    };
                    let res_term = reslm.map_or(0, |tb| cfg.w_res * tb.trips(&s.bits));
                    let lm_term = if clms.is_empty() {
                        0
                    } else {
                        let mut acc = nodes[pi].lm_acc.clone();
                        clm_accept_into(&mut acc, &clms, &s);
                        let un = clm_unaccepted(&acc, clms.len());
                        let idx = nodes.len();
                        nodes.push(Node {
                            state: s,
                            father: pi,
                            op: oi,
                            g,
                            lm_acc: acc,
                        });
                        heap.push(Reverse((
                            cfg.w_g * g as i64
                                + cfg.w_h * ph as i64
                                + cfg.w_lm * un
                                + res_term
                                + sat_pen
                                + cost_term,
                            idx,
                        )));
                        continue;
                    };
                    let idx = nodes.len();
                    nodes.push(Node {
                        state: s,
                        father: pi,
                        op: oi,
                        g,
                        lm_acc: Vec::new(),
                    });
                    heap.push(Reverse((
                        cfg.w_g * g as i64
                            + cfg.w_h * ph as i64
                            + lm_term
                            + res_term
                            + sat_pen
                            + cost_term,
                        idx,
                    )));
                }
            }
        }
        t_ins += t_phase.elapsed_us();
    }

    // Open list exhausted. Anytime: the incumbent is optimal under the original
    // bound (the caller's confirming re-sweep proves it via None + un-capped).
    if let Some(ni) = best_acc.or(len_acc) {
        return PlanResult::Plan {
            ops: reconstruct(&nodes, ni),
            advance,
            evaluated,
            max_g,
        };
    }
    PlanResult::Unsolvable {
        evaluated,
        capped: false,
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

/// Sweeps the whole task — start is the initial state, goal is the task
/// goal — with tunable weighted-best-first dials.
pub fn search(task: &PackedTask, threads: usize, cfg: SearchCfg) -> PlanResult {
    search_from(
        task,
        &task.initial(),
        &task.goal_pos,
        &task.goal_num,
        None,
        f64::INFINITY,
        threads,
        cfg,
        &[],
        None,
        None,
    )
}

/// The [`plan`] after-action report: the op sequence if the target went
/// down, states burned reaching it, and whether EHC gave up the chase and
/// best-first had to close it out.
pub struct PlanOutcome {
    pub ops: Option<Vec<usize>>,
    pub evaluated: usize,
    pub ehc_fell_back: bool,
}

/// Runs the whole task. With `ehc_first` armed, tries enforced
/// hill-climbing first — fast on most jobs — and falls back to weighted
/// best-first the moment it stalls; otherwise goes straight to
/// best-first. EHC plans are valid but not length-optimal — matches the
/// FF/Metric-FF default and is the main lever on speed.
pub fn plan(task: &PackedTask, threads: usize, cfg: SearchCfg, ehc_first: bool) -> PlanOutcome {
    plan_avoiding(task, threads, cfg, ehc_first, &[])
}

/// [`plan`], but never touches any op `oi` where `forbidden[oi]` reads
/// true. The metric optimizer's force-collect tightening leans on this —
/// forbid the forgo actions so their preferences are actually forced to
/// pay out.
pub fn plan_avoiding(
    task: &PackedTask,
    threads: usize,
    cfg: SearchCfg,
    ehc_first: bool,
    forbidden: &[bool],
) -> PlanOutcome {
    // Probe hatch (A/B eyes for the novelty rung): skip straight to it.
    if std::env::var("FF_NOVELTY_ONLY").is_ok() {
        if let Some((ops, evaluated)) =
            crate::novelty::search(task, threads, cfg.max_eval, forbidden)
        {
            return PlanOutcome {
                ops: Some(ops),
                evaluated,
                ehc_fell_back: true,
            };
        }
    }
    if ehc_first {
        if let Some((ops, evaluated)) = ehc(task, forbidden, cfg.max_eval) {
            return PlanOutcome {
                ops: Some(ops),
                evaluated,
                ehc_fell_back: false,
            };
        }
        // LAMA rung (0.9 Phase 3): EHC gave up, i.e. the relaxed plan has
        // plateaued — exactly where landmark counting + preferred-operator
        // boosting keep a gradient. Bounded, so the complete weighted
        // fallback below still gets its shot; never entered under an
        // explicit --search bfs. `FF_NO_LAMA=1` restores the two-rung ladder.
        if std::env::var("FF_NO_LAMA").is_err() {
            const LAMA_CAP: usize = 400_000;
            if let Some((ops, evaluated)) =
                crate::lama::search(task, threads, LAMA_CAP.min(cfg.max_eval), forbidden)
            {
                return PlanOutcome {
                    ops: Some(ops),
                    evaluated,
                    ehc_fell_back: true,
                };
            }
        }
        // Novelty rung (0.17 Phase 3), OPT-IN (`FF_NOVELTY=1`): width-1
        // novelty-first exploration for where the relaxed gradient is flat
        // or wrong. The referee A/B flipped it off-by-default: as a third
        // bounded rung it BURNS WALL TIME ahead of the complete fallback,
        // and under wall-clock budgets that tax cost 51 instances across
        // the classical boards against 7 gained (+3 on 2018-sat and +3 on
        // prop-2006 are real and stay reachable via the flag) — the same
        // referee arithmetic that made gen-skip opt-in in 0.15. The LAMA
        // rung survives the same structure because its win rate carries
        // its tax; novelty's does not, on today's corpora.
        if std::env::var("FF_NOVELTY").is_ok() {
            const NOVELTY_CAP: usize = 400_000;
            if let Some((ops, evaluated)) =
                crate::novelty::search(task, threads, NOVELTY_CAP.min(cfg.max_eval), forbidden)
            {
                return PlanOutcome {
                    ops: Some(ops),
                    evaluated,
                    ehc_fell_back: true,
                };
            }
        }
    }
    // Length-anytime on the PLAIN length path only (no metric machinery in
    // play). Opt-in — see the SearchCfg field docs for the measured verdict.
    let mut cfg = cfg;
    if cfg.h_cost.is_none() && !cfg.anytime && std::env::var("FF_LEN_ANYTIME").is_ok() {
        cfg.len_anytime = true;
    }
    // Classical landmark-count ordering (0.11 Phase 3), opt-in experiment:
    // FF_CLM=<weight> adds w_lm × unaccepted-landmarks to the best-first
    // fallback's key (EHC and the LAMA rung are untouched).
    if cfg.h_cost.is_none() && !cfg.anytime {
        if let Ok(v) = std::env::var("FF_CLM") {
            let w = v.trim().parse::<f64>().unwrap_or(3.0);
            if w.is_finite() && w > 0.0 {
                cfg.w_lm = (w.min(1e9) * WEIGHT_SCALE).round() as i64;
            }
        }
        // Resource-trip term (0.14 ext Phase 11), same scoping.
        if let Ok(v) = std::env::var("FF_RESLM") {
            let w = v.trim().parse::<f64>().unwrap_or(3.0);
            if w.is_finite() && w > 0.0 {
                cfg.w_res = (w.min(1e9) * WEIGHT_SCALE).round() as i64;
            }
        }
    }
    let (ops, evaluated) = match search_from(
        task,
        &task.initial(),
        &task.goal_pos,
        &task.goal_num,
        None,
        f64::INFINITY,
        threads,
        cfg,
        forbidden,
        None,
        None,
    ) {
        PlanResult::Plan { ops, evaluated, .. } => (Some(ops), evaluated),
        PlanResult::Unsolvable { evaluated, .. } => (None, evaluated),
    };
    PlanOutcome {
        ops,
        evaluated,
        ehc_fell_back: ehc_first,
    }
}

/// Enforced hill-climbing, running toward the task goal. From wherever it
/// stands, it fires a breadth-first lookahead restricted to HELPFUL
/// actions until a strictly lower-h state surfaces, jumps there, and
/// repeats. Returns the plan and the body count of states burned, or
/// `None` if it stalls or hits a dead end — the caller falls back to
/// best-first, which always finishes the job. Single-threaded,
/// deterministic.
fn ehc(task: &PackedTask, forbidden: &[bool], max_eval: usize) -> Option<(Vec<usize>, usize)> {
    let init = task.initial();
    let mut sc = Scratch::new(task);
    let (mut cur_h, _) = relaxed_helpful(
        task,
        &mut sc,
        &init.bits,
        &init.fv,
        &init.fdef,
        &task.goal_pos,
        &task.goal_num,
    )?;
    let mut evaluated = 1usize;
    if task.goal_met(&init) {
        return Some((Vec::new(), evaluated));
    }
    // Total work budget: if EHC hasn't solved it within this many evaluations it
    // is likely stuck, so bail and leave the time budget to the complete
    // best-first fallback (which often solves these much faster from scratch).
    // Scaled by op count: EHC's cumulative evals grow ~quadratically in problem
    // size, so a fixed 30k cap made large-but-easy instances (e.g. gripper) bail
    // into the unpruned best-first and explode (2.16M evals). Scaling with n_ops
    // lets EHC's near-greedy arm finish those, while the 30k floor keeps small/
    // medium domains bit-identical and the finite cap still hands genuine
    // plateaus off to the complete fallback.
    // The caller's eval budget bounds EHC too (0.11 Phase 4: a think budget
    // bounds EVERYTHING — a 1-eval think must not solve via EHC's internal
    // op-scaled cap).
    let total_cap = (200 * task.n_ops).max(30_000).min(max_eval);
    let mut current = init;
    let mut plan: Vec<usize> = Vec::new();
    loop {
        match bfs_improve(task, &mut sc, &current, cur_h, &mut evaluated, forbidden) {
            Some((ops, next, next_h)) => {
                plan.extend(ops);
                current = next;
                cur_h = next_h;
                if task.goal_met(&current) {
                    return Some((plan, evaluated));
                }
                if evaluated > total_cap {
                    return None; // taking too long — hand off to best-first
                }
            }
            None => return None, // stuck — let best-first take over
        }
    }
}

/// Breadth-first search from `start`, expanding each node with ITS helpful
/// actions, until a state with `h < h_start` is found. Returns (path, state, h).
fn bfs_improve(
    task: &PackedTask,
    sc: &mut Scratch,
    start: &State,
    h_start: i32,
    evaluated: &mut usize,
    forbidden: &[bool],
) -> Option<(Vec<usize>, State, i32)> {
    // Fail FAST: if a helpful-restricted lookahead can't improve h within this
    // many expansions it is almost certainly on a plateau EHC won't escape, so
    // bail and let the complete best-first fallback use the time budget. Kept
    // small because per-evaluation cost is high on big numeric tasks — a large
    // cap made EHC burn ~20s before falling back.
    const BFS_CAP: usize = 5_000;
    struct N {
        state: State,
        father: usize,
        op: usize,
    }
    let (_, root_helpful) = relaxed_helpful(
        task,
        sc,
        &start.bits,
        &start.fv,
        &start.fdef,
        &task.goal_pos,
        &task.goal_num,
    )?;
    let mut nodes = vec![N {
        state: start.clone(),
        father: usize::MAX,
        op: usize::MAX,
    }];
    let mut visited: FxHashSet<StateKey> = FxHashSet::default();
    visited.insert(task.state_key(start));
    let mut queue: VecDeque<(usize, Vec<u32>)> = VecDeque::new();
    queue.push_back((0, root_helpful));
    let mut expanded = 0usize;

    while let Some((ni, helpful)) = queue.pop_front() {
        for &oi in &helpful {
            let oi = oi as usize;
            if forbidden.get(oi).copied().unwrap_or(false) {
                continue;
            }
            if !task.op_applicable(oi, &nodes[ni].state) {
                continue;
            }
            let ns = task.apply(oi, &nodes[ni].state);
            if !visited.insert(task.state_key(&ns)) {
                continue;
            }
            *evaluated += 1;
            let (h_ns, helpful_ns) = match relaxed_helpful(
                task,
                sc,
                &ns.bits,
                &ns.fv,
                &ns.fdef,
                &task.goal_pos,
                &task.goal_num,
            ) {
                Some(x) => x,
                None => continue, // dead-end successor
            };
            let idx = nodes.len();
            nodes.push(N {
                state: ns.clone(),
                father: ni,
                op: oi,
            });
            if h_ns < h_start {
                let mut ops = Vec::new();
                let mut c = idx;
                while nodes[c].father != usize::MAX {
                    ops.push(nodes[c].op);
                    c = nodes[c].father;
                }
                ops.reverse();
                return Some((ops, ns, h_ns));
            }
            expanded += 1;
            if expanded > BFS_CAP {
                return None;
            }
            queue.push_back((idx, helpful_ns));
        }
    }
    None
}

/// Subplanner contract: hands back the op sequence achieving
/// `(goal_pos, goal_num)` from `start`, or `None` if the job is dead.
/// This is what `sgp` calls per partition.
pub fn solve_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    threads: usize,
    cfg: SearchCfg,
) -> Option<Vec<usize>> {
    solve_subgoal_avoiding(task, start, goal_pos, goal_num, &[], threads, cfg)
}

/// `solve_subgoal`, but never touching any op `oi` where `forbidden[oi]`
/// reads true — the resolver's sibling-protection lever, forbidding ops
/// that would erase a sibling's already-won facts. An empty mask
/// forbids nothing at all.
#[allow(clippy::too_many_arguments)]
pub fn solve_subgoal_avoiding(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
) -> Option<Vec<usize>> {
    match search_from(
        task,
        start,
        goal_pos,
        goal_num,
        None,
        f64::INFINITY,
        threads,
        cfg,
        forbidden,
        None,
        None,
    ) {
        PlanResult::Plan { ops, .. } => Some(ops),
        PlanResult::Unsolvable { .. } => None,
    }
}

/// Subplanner running under a monotone COST ceiling on `cost_fluent`:
/// hands back a plan reaching the goal at final cost < `bound`, or
/// `None` if nothing clears it. The anytime branch-and-bound metric
/// optimizer (sgp) calls this with a bound that keeps tightening.
#[allow(clippy::too_many_arguments)]
pub fn solve_subgoal_bounded(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
    bound: f64,
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) -> (Option<Vec<usize>>, usize, bool) {
    match search_from(
        task,
        start,
        goal_pos,
        goal_num,
        Some(cost_fluent),
        bound,
        threads,
        cfg,
        &[],
        sat,
        None,
    ) {
        PlanResult::Plan { ops, evaluated, .. } => (Some(ops), evaluated, false),
        PlanResult::Unsolvable { evaluated, capped } => (None, evaluated, capped),
    }
}

/// The exact-closure metric subplanner (`crate::pddl3::metric_optimize`'s
/// default route): walks REAL states only — `forbidden` masks off every
/// synthetic `P3END`/collect/forgo op — and takes a state iff the real
/// goal holds AND `cost-so-far + closure(state) < bound` (the caller
/// bolts on the phase tail afterward, which pays exactly `closure`).
/// Returns the plan minus the tail, plus the evaluated-state count so
/// the caller can run a DETERMINISTIC budget across tightening rounds,
/// and the capped flag — the bound is proven unbeatable only when
/// exhaustion happens uncapped.
#[allow(clippy::too_many_arguments)]
pub fn solve_closure_bounded(
    task: &PackedTask,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
    bound: f64,
    closure: &ClosureCost,
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) -> (Option<Vec<usize>>, usize, bool) {
    match search_from(
        task,
        &task.initial(),
        goal_pos,
        goal_num,
        Some(cost_fluent),
        bound,
        threads,
        cfg,
        forbidden,
        sat,
        Some(closure),
    ) {
        PlanResult::Plan { ops, evaluated, .. } => (Some(ops), evaluated, false),
        PlanResult::Unsolvable { evaluated, capped } => (None, evaluated, capped),
    }
}

/// [`solve_subgoal_avoiding`] plus [`SatGuidance`]: the partitioned-ESPC
/// per-stage subplanner (`crate::espc`). Combines a forbidden-op mask
/// (sibling protection) with the λ-weighted penalty guidance —
/// `search_from` carries both, but no other wrapper exposes the
/// combination. No cost bound here: on the openstacks shape the metric
/// only accrues in the post-composition collect/forgo tail, so a
/// per-stage bound could never prune anything — bounds stay global,
/// composed-plan cost against the incumbent.
#[allow(clippy::too_many_arguments)]
pub fn solve_subgoal_guided(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) -> (Option<Vec<usize>>, usize) {
    match search_from(
        task,
        start,
        goal_pos,
        goal_num,
        None,
        f64::INFINITY,
        threads,
        cfg,
        forbidden,
        sat,
        None,
    ) {
        PlanResult::Plan { ops, evaluated, .. } => (Some(ops), evaluated),
        PlanResult::Unsolvable { evaluated, .. } => (None, evaluated),
    }
}
