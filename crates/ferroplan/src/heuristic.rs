//! Recon over the packed task, no allocator on the hot circuit: every working
//! buffer lives in a reusable `Scratch` a worker claims once and wipes clean
//! between passes — never rebuilt from scratch, never handed back to the heap.
//! That kills the per-state allocation churn — and the cross-thread contention
//! it was feeding the global allocator — the choke point capping both raw
//! speed and how far this scales across workers.
//!
//! Same tradecraft as the (oracle-verified) metricff heuristic: strip the
//! deletes, build the graph, walk it in two passes with monotone numeric
//! interval bounds, take the earliest achiever, count the numeric repeats.
//! The best-first engine only wants `h` — the helpful-action set stays dark
//! unless someone asks for it.

use crate::bitset;
use crate::packed::PackedTask;
use crate::types::{eval_numpre, AssignOp, CompOp, NExpr, NumEff, NumPre};

const LAYER_CAP: u32 = 2000;
const INF: u32 = u32::MAX;

/// Instrumentation only, no load-bearing role — tallies for FF_RES_DEBUG,
/// dumped when the search hits its cap. Kept atomic; `h` runs hot across
/// worker threads and these counters take hits from all of them.
pub static T_RESET: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static T_BUILD: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static T_EXTRACT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Standing kit, issued once per worker, reused every run of `relaxed`.
pub struct Scratch {
    reached: Vec<bool>,
    fact_layer: Vec<u32>,
    op_layer: Vec<u32>,
    /// Timestamp tag for `op_layer` / `selected` / `need_fact` membership — a
    /// cell reads "live this pass" only when its stamp matches `gen`. Lets
    /// `reset()` advance the clock in one tick instead of scrubbing every
    /// array by hand each pass.
    gen: u32,
    op_stamp: Vec<u32>,
    applicable: Vec<u32>,
    lb: Vec<f64>,
    ub: Vec<f64>,
    selected: Vec<u32>,
    need_fact: Vec<u32>,
    queue: Vec<u32>,
    /// Ops already fired that carry ≥1 numeric effect worth tracking —
    /// re-widened every layer.
    num_applied: Vec<u32>,
    /// Ops already fired that carry ≥1 conditional effect — re-checked every
    /// layer; the conditional payload triggers once its trigger condition
    /// goes relaxed-reached.
    cond_ops: Vec<u32>,
    /// The helpful-action manifest: relaxed-plan ops already live in the
    /// current state (op_layer 0). Filled during extraction, read by EHC.
    helpful: Vec<u32>,
}

impl Scratch {
    pub fn new(task: &PackedTask) -> Self {
        let nfl = task.fv0.len();
        Scratch {
            reached: vec![false; task.n_facts],
            fact_layer: vec![INF; task.n_facts],
            op_layer: vec![INF; task.n_ops],
            gen: 0,
            op_stamp: vec![0; task.n_ops],
            applicable: Vec::with_capacity(task.n_ops),
            lb: vec![0.0; nfl],
            ub: vec![0.0; nfl],
            selected: vec![0; task.n_ops],
            need_fact: vec![0; task.n_facts],
            queue: Vec::with_capacity(task.n_facts),
            num_applied: Vec::with_capacity(task.n_ops),
            cond_ops: Vec::new(),
            helpful: Vec::new(),
        }
    }

    fn reset(&mut self, task: &PackedTask, bits: &[u64], fv: &[f64]) {
        for f in 0..task.n_facts {
            self.reached[f] = bitset::test(bits, f);
        }
        self.fact_layer.iter_mut().enumerate().for_each(|(f, l)| {
            *l = if self.reached[f] { 0 } else { INF };
        });
        // Bump the generation instead of clearing op_layer/selected/need_fact —
        // their stale values are unobservable because every read is stamp-gated
        // (== gen). This removes 2*n_ops + n_facts dense writes per evaluation.
        self.gen = self.gen.wrapping_add(1);
        if self.gen == 0 {
            // wrapped after ~4e9 evals: hard-clear once so stamps can't collide.
            self.op_stamp.fill(0);
            self.selected.fill(0);
            self.need_fact.fill(0);
            self.gen = 1;
        }
        self.lb.copy_from_slice(fv);
        self.ub.copy_from_slice(fv);
        self.queue.clear();
        self.num_applied.clear();
        self.cond_ops.clear();
        self.helpful.clear();
    }
}

/// Push the monotone bounds outward from op `oi`'s numeric effects, RELEVANT
/// fluents only — a fluent no precondition or goal ever reads can't move the
/// heuristic's needle, so it's left alone. Skipping it costs nothing on
/// accuracy and shuts down unbounded drift nobody's watching. Reports back
/// whether any watched bound actually shifted.
fn widen(
    neffs: &[NumEff],
    relevant: &[bool],
    lb: &mut [f64],
    ub: &mut [f64],
    def: &[bool],
) -> bool {
    let mut changed = false;
    for ne in neffs {
        let t = ne.target as usize;
        if !relevant[t] {
            continue;
        }
        if let Some((vl, vu)) = eval_iv(&ne.value, lb, ub, def) {
            let before = (lb[t], ub[t]);
            match ne.op {
                AssignOp::Increase => {
                    ub[t] += vu.max(0.0);
                    lb[t] += vl.min(0.0);
                }
                AssignOp::Decrease => {
                    lb[t] -= vu.max(0.0);
                    ub[t] -= vl.min(0.0);
                }
                AssignOp::Assign => {
                    lb[t] = lb[t].min(vl);
                    ub[t] = ub[t].max(vu);
                }
                AssignOp::ScaleUp => ub[t] *= vu.max(1.0),
                AssignOp::ScaleDown => lb[t] /= vu.max(1.0),
            }
            if (lb[t], ub[t]) != before {
                changed = true;
            }
        }
    }
    changed
}

fn op_has_relevant_neff(task: &PackedTask, oi: usize) -> bool {
    task.num_eff
        .slice(oi)
        .iter()
        .any(|ne| task.relevant_fluent[ne.target as usize])
}

fn eval_iv(e: &NExpr, lb: &[f64], ub: &[f64], def: &[bool]) -> Option<(f64, f64)> {
    Some(match e {
        NExpr::Num(n) => (*n, *n),
        NExpr::Fluent(i) => {
            let i = *i as usize;
            if !def[i] {
                return None;
            }
            (lb[i], ub[i])
        }
        NExpr::Neg(a) => {
            let (l, u) = eval_iv(a, lb, ub, def)?;
            (-u, -l)
        }
        NExpr::Add(a, b) => {
            let (al, au) = eval_iv(a, lb, ub, def)?;
            let (bl, bu) = eval_iv(b, lb, ub, def)?;
            (al + bl, au + bu)
        }
        NExpr::Sub(a, b) => {
            let (al, au) = eval_iv(a, lb, ub, def)?;
            let (bl, bu) = eval_iv(b, lb, ub, def)?;
            (al - bu, au - bl)
        }
        NExpr::Mul(a, b) => {
            let (al, au) = eval_iv(a, lb, ub, def)?;
            let (bl, bu) = eval_iv(b, lb, ub, def)?;
            let c = [al * bl, al * bu, au * bl, au * bu];
            (
                c.iter().cloned().fold(f64::INFINITY, f64::min),
                c.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            )
        }
        NExpr::Div(a, b) => {
            let (al, au) = eval_iv(a, lb, ub, def)?;
            let (bl, bu) = eval_iv(b, lb, ub, def)?;
            if bl <= 0.0 && bu >= 0.0 {
                (f64::NEG_INFINITY, f64::INFINITY)
            } else {
                let c = [al / bl, al / bu, au / bl, au / bu];
                (
                    c.iter().cloned().fold(f64::INFINITY, f64::min),
                    c.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                )
            }
        }
    })
}

fn num_sat(np: &NumPre, lb: &[f64], ub: &[f64], def: &[bool]) -> bool {
    let l = match eval_iv(&np.lhs, lb, ub, def) {
        Some(x) => x,
        None => return false,
    };
    let r = match eval_iv(&np.rhs, lb, ub, def) {
        Some(x) => x,
        None => return false,
    };
    match np.op {
        CompOp::Lt => l.0 < r.1,
        CompOp::Le => l.0 <= r.1,
        CompOp::Gt => l.1 > r.0,
        CompOp::Ge => l.1 >= r.0,
        CompOp::Eq => l.0 <= r.1 && r.0 <= l.1,
    }
}

fn goal_done(
    goal_pos: &[u32],
    goal_num: &[NumPre],
    reached: &[bool],
    lb: &[f64],
    ub: &[f64],
    def: &[bool],
) -> bool {
    goal_pos.iter().all(|&f| reached[f as usize])
        && goal_num.iter().all(|np| num_sat(np, lb, ub, def))
}

/// Build the stripped-down planning graph into `sc` — op_layer, reached
/// facts, bounds. With `to_fixpoint` it ignores the goal entirely and keeps
/// pushing until the graph stops moving, so every reachable op ends up with
/// a layer stamped on it. Otherwise it cuts the run the instant the goal
/// goes relaxed-reached. `sc` has to be wiped before this runs.
fn build_rpg(
    task: &PackedTask,
    sc: &mut Scratch,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    def: &[bool],
    to_fixpoint: bool,
) {
    // ---- build the relaxed planning graph (two-phase, incremental) ----
    // Only UNAPPLIED ops are re-scanned each layer; applied ops never lose
    // applicability (delete-relaxed), so they are skipped — except those with
    // relevant numeric effects, which are re-widened each layer from
    // `num_applied` so monotone fluents (e.g. consumed-resources) can grow to
    // reach numeric goals. (A counter-based build — watch lists decrementing
    // per reached fact — was implemented and measured EQUIVALENT on
    // transport11: identical 20,126 evals, h wall 12.85 s vs 12.83 s. The
    // per-eval cost is the relaxation FLOOR — nearly every op fires in every
    // build — so the scan is not the term; recorded 2026-07-19, not shipped.)
    let mut layer: u32 = 0;
    loop {
        if !to_fixpoint && goal_done(goal_pos, goal_num, &sc.reached, &sc.lb, &sc.ub, def) {
            break;
        }
        let mut changed = false;

        // (a) re-widen bounds from previously-applied relevant-numeric ops
        for idx in 0..sc.num_applied.len() {
            let oi = sc.num_applied[idx] as usize;
            if widen(
                task.num_eff.slice(oi),
                &task.relevant_fluent,
                &mut sc.lb,
                &mut sc.ub,
                def,
            ) {
                changed = true;
            }
        }

        // (a2) conditional effects of applied ops: fire those whose condition is
        // now relaxed-reached (positive facts reached + numeric satisfied;
        // negative conditions are dropped by the delete-relaxation).
        for idx in 0..sc.cond_ops.len() {
            let oi = sc.cond_ops[idx] as usize;
            for ce in task.cond_effs(oi) {
                let pos_ok = ce.cond_pos.iter().all(|&c| sc.reached[c as usize]);
                let num_ok = ce
                    .cond_num
                    .iter()
                    .all(|np| num_sat(np, &sc.lb, &sc.ub, def));
                if pos_ok && num_ok {
                    for &f in &ce.add {
                        let f = f as usize;
                        if !sc.reached[f] {
                            sc.reached[f] = true;
                            sc.fact_layer[f] = layer + 1;
                            changed = true;
                        }
                    }
                    if !ce.num.is_empty()
                        && widen(&ce.num, &task.relevant_fluent, &mut sc.lb, &mut sc.ub, def)
                    {
                        changed = true;
                    }
                }
            }
        }

        // (b) scan only unapplied ops for new applicability
        sc.applicable.clear();
        for oi in 0..task.n_ops {
            if sc.op_stamp[oi] == sc.gen {
                continue; // already applied this evaluation
            }
            let ok = task
                .pre_pos
                .slice(oi)
                .iter()
                .all(|&f| sc.reached[f as usize])
                && task
                    .pre_num
                    .slice(oi)
                    .iter()
                    .all(|np| num_sat(np, &sc.lb, &sc.ub, def));
            if ok {
                sc.op_stamp[oi] = sc.gen;
                sc.op_layer[oi] = layer;
                sc.applicable.push(oi as u32);
                changed = true;
            }
        }

        // (c) apply newly-applicable ops: reach their adds, widen + register
        for k in 0..sc.applicable.len() {
            let oi = sc.applicable[k] as usize;
            for &f in task.add.slice(oi) {
                let f = f as usize;
                if !sc.reached[f] {
                    sc.reached[f] = true;
                    sc.fact_layer[f] = layer + 1;
                    changed = true;
                }
            }
            if op_has_relevant_neff(task, oi) {
                if widen(
                    task.num_eff.slice(oi),
                    &task.relevant_fluent,
                    &mut sc.lb,
                    &mut sc.ub,
                    def,
                ) {
                    changed = true;
                }
                sc.num_applied.push(oi as u32);
            }
            if task.n_cond_effs(oi) > 0 {
                sc.cond_ops.push(oi as u32);
            }
        }

        layer += 1;
        if !changed || layer > LAYER_CAP {
            break;
        }
    }
}

/// Goal-blind sweep, run to a FIXPOINT: hands back `(fact_layer, op_layer)`
/// with `u32::MAX` marking anything never reached. One graph build, that's
/// all — landmark extraction ([`crate::landmarks`]) needs every reachable
/// op's layer on record, not just the shallow slice that touches a goal.
pub fn reachability_layers(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) -> (Vec<u32>, Vec<u32>) {
    sc.reset(task, bits, fv);
    build_rpg(task, sc, &[], &[], def, true);
    let op_layer: Vec<u32> = (0..task.n_ops)
        .map(|oi| {
            if sc.op_stamp[oi] == sc.gen {
                sc.op_layer[oi]
            } else {
                u32::MAX
            }
        })
        .collect();
    (sc.fact_layer.clone(), op_layer)
}

/// Range-check toward an ARBITRARY (sub)goal, `sc` reused across the call.
/// `None` reads as dead end. This is the subplanner's compass — SGPlan-style
/// partitioning steers by it, one subproblem goal at a time.
pub fn relaxed_to(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
) -> Option<i32> {
    use std::sync::atomic::Ordering::Relaxed;
    let t0 = crate::clock::Clock::now();
    sc.reset(task, bits, fv);
    T_RESET.fetch_add(t0.elapsed_us() as u64, Relaxed);

    let t0 = crate::clock::Clock::now();
    build_rpg(task, sc, goal_pos, goal_num, def, false);
    T_BUILD.fetch_add(t0.elapsed_us() as u64, Relaxed);

    if !goal_done(goal_pos, goal_num, &sc.reached, &sc.lb, &sc.ub, def) {
        return None;
    }
    let t0 = crate::clock::Clock::now();
    let r = relaxed_extract(task, sc, bits, fv, goal_pos, goal_num, def);
    T_EXTRACT.fetch_add(t0.elapsed_us() as u64, Relaxed);
    r
}

#[allow(clippy::too_many_arguments)]
fn relaxed_extract(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    goal_pos: &[u32],
    goal_num: &[NumPre],
    def: &[bool],
) -> Option<i32> {
    // ---- relaxed-plan extraction (count actions) ----
    let mut count: i32 = 0;
    let mut head = 0usize;
    for &g in goal_pos {
        let f = g as usize;
        if sc.need_fact[f] != sc.gen {
            sc.need_fact[f] = sc.gen;
            sc.queue.push(g);
        }
    }

    while head < sc.queue.len() {
        let f = sc.queue[head] as usize;
        head += 1;
        if bitset::test(bits, f) {
            continue;
        }
        if let Some(oi) = achiever(task, &sc.op_layer, &sc.op_stamp, sc.gen, &sc.fact_layer, f) {
            select(task, sc, oi, 1, &mut count);
            queue_cond_for(task, sc, oi, f);
        }
    }

    for np in goal_num {
        if eval_numpre(np, fv, def).unwrap_or(false) {
            continue;
        }
        if let Some((oi, reps)) = numeric_achiever(task, np, fv, def, &sc.op_stamp, sc.gen) {
            select(task, sc, oi, reps, &mut count);
            while head < sc.queue.len() {
                let f = sc.queue[head] as usize;
                head += 1;
                if bitset::test(bits, f) {
                    continue;
                }
                if let Some(o2) =
                    achiever(task, &sc.op_layer, &sc.op_stamp, sc.gen, &sc.fact_layer, f)
                {
                    select(task, sc, o2, 1, &mut count);
                    queue_cond_for(task, sc, o2, f);
                }
            }
        }
    }

    Some(count)
}

/// Shortcut: point the range-check at the task's own goal, no detours.
pub fn relaxed(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) -> Option<i32> {
    relaxed_to(task, sc, bits, fv, def, &task.goal_pos, &task.goal_num)
}

/// Range-check plus the helpful-action manifest — relaxed-plan ops already
/// live in this state. Enforced hill-climbing reads it to narrow which
/// moves it bothers expanding. `None` means dead end at this range. Op ids
/// come back in fixed order — the order the relaxed plan picked them.
pub fn relaxed_helpful(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
) -> Option<(i32, Vec<u32>)> {
    let h = relaxed_to(task, sc, bits, fv, def, goal_pos, goal_num)?;
    // really applicable in THIS state (op_layer 0 is only relaxed-applicable —
    // numeric interval bounds are optimistic, so re-check exactly).
    let applicable = |oi: usize| {
        task.pre_pos
            .slice(oi)
            .iter()
            .all(|&f| bitset::test(bits, f as usize))
            && task
                .pre_num
                .slice(oi)
                .iter()
                .all(|np| eval_numpre(np, fv, def) == Some(true))
    };
    // helpful = the relaxed plan's applicable-now ops. Filter for REAL
    // applicability: on numeric domains a selected op can be relaxed-applicable
    // (op_layer 0) yet not actually applicable.
    let mut helpful: Vec<u32> = sc
        .helpful
        .iter()
        .copied()
        .filter(|&oi| applicable(oi as usize))
        .collect();
    // If that leaves nothing (typical when the relaxed plan is gated by numeric
    // preconditions), fall back to numeric subgoals: applicable ops whose numeric
    // effects touch a fluent an unsatisfied numeric precondition of a relaxed-plan
    // op reads.
    if helpful.is_empty() && h > 0 {
        let mut wanted = vec![false; fv.len()];
        let mut any = false;
        let mut tmp = Vec::new();
        for oi in 0..task.n_ops {
            if sc.selected[oi] != sc.gen {
                continue;
            }
            for np in task.pre_num.slice(oi) {
                if eval_numpre(np, fv, def) == Some(true) {
                    continue;
                }
                tmp.clear();
                np.lhs.collect_fluents(&mut tmp);
                np.rhs.collect_fluents(&mut tmp);
                for &fl in &tmp {
                    wanted[fl as usize] = true;
                    any = true;
                }
            }
        }
        if any {
            for oi in 0..task.n_ops {
                if applicable(oi)
                    && task
                        .num_eff
                        .slice(oi)
                        .iter()
                        .any(|ne| wanted[ne.target as usize])
                {
                    helpful.push(oi as u32);
                }
            }
        }
        // last resort: every really-applicable op, so EHC can act rather than
        // instantly failing (still h-guided, just unpruned for this state).
        if helpful.is_empty() {
            for oi in 0..task.n_ops {
                if applicable(oi) {
                    helpful.push(oi as u32);
                }
            }
        }
    }
    Some((h, helpful))
}

/// Fallback net when the primary pick comes up empty (0.11 Phase 2): grab
/// every truly-applicable op whose ADD lands a fact some relaxed-plan op is
/// still owed — a positive precondition not yet true in this state. Only
/// good for the instant right after a `relaxed_to`/`relaxed_helpful` call on
/// the SAME state — it reads `sc.selected` at the current clock tick. The
/// temporal path leans on this when the strict helpful set turns up nothing:
/// its relaxed plans often route through END ops the agenda fires rather
/// than chooses, and the Start-only filter was starving the pruned pass into
/// full scans (storage/model-train: stored helpful averaged 0.0).
pub fn helpful_needed_adders(
    task: &PackedTask,
    sc: &Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) -> Vec<u32> {
    let mut needed = vec![false; task.n_facts];
    let mut any = false;
    for oi in 0..task.n_ops {
        if sc.selected[oi] != sc.gen {
            continue;
        }
        for &f in task.pre_pos.slice(oi) {
            if !bitset::test(bits, f as usize) {
                needed[f as usize] = true;
                any = true;
            }
        }
    }
    if !any {
        return Vec::new();
    }
    let applicable = |oi: usize| {
        task.pre_pos
            .slice(oi)
            .iter()
            .all(|&f| bitset::test(bits, f as usize))
            && task
                .pre_num
                .slice(oi)
                .iter()
                .all(|np| eval_numpre(np, fv, def) == Some(true))
    };
    (0..task.n_ops)
        .filter(|&oi| applicable(oi) && task.add.slice(oi).iter().any(|&f| needed[f as usize]))
        .map(|oi| oi as u32)
        .collect()
}

/// Completion COST of a subgoal, priced from this state: run the extraction
/// toward `goal_pos`/`goal_num`, then tally the SELECTED ops' `increase`
/// hits on `cost_fluent`, each priced against this state's readings — exact
/// when the increase amounts read only static fluents (the IPC numeric-metric
/// shape, rovers' traverse costs and the like). Ops get counted once, set
/// semantics, so a plan forced to repeat an op reads UNDER its real price —
/// the safe lean for the forgo-aware seed, an underestimate never prices a
/// cheap preference out of consideration. `None` reads as dead end even with
/// the deletes stripped out.
#[allow(clippy::too_many_arguments)]
pub fn relaxed_plan_cost(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
) -> Option<f64> {
    relaxed_to(task, sc, bits, fv, def, goal_pos, goal_num)?;
    Some(selected_increase_sum(task, sc, fv, def, cost_fluent))
}

/// Tally of `increase cost_fluent` amounts across the ops SELECTED by the
/// last extraction sitting in `sc` — good until the next reset scrubs it —
/// each priced against this state's readings. Ops count once, set semantics,
/// so a plan forced to repeat an op reads under its real price.
fn selected_increase_sum(
    task: &PackedTask,
    sc: &Scratch,
    fv: &[f64],
    def: &[bool],
    cost_fluent: usize,
) -> f64 {
    let mut cost = 0.0;
    for oi in 0..task.n_ops {
        if sc.selected[oi] != sc.gen {
            continue;
        }
        for ne in task.num_eff.slice(oi) {
            if ne.target as usize == cost_fluent && ne.op == AssignOp::Increase {
                if let Some(v) = ne.value.eval(fv, def) {
                    cost += v.max(0.0);
                }
            }
        }
    }
    cost
}

/// Cost-augmented reading: relaxed-plan LENGTH plus the tallied `increase
/// cost_fluent` across selected ops — "cost + 1 per action," so a search
/// steered by it favors cheap achievers while zero-cost stretches still
/// carry a distance signal (a pure-cost h goes flat the moment remaining
/// moves are free). Units run cost+steps; callers weight it like any h.
/// `None` reads as dead end.
#[allow(clippy::too_many_arguments)]
pub fn relaxed_costed(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
) -> Option<i32> {
    let count = relaxed_to(task, sc, bits, fv, def, goal_pos, goal_num)?;
    let cost = selected_increase_sum(task, sc, fv, def, cost_fluent);
    Some(count.saturating_add(cost.min(1e9).round() as i32))
}

/// Earliest-layer op that lands fact `f` — the doctrine is earliest achiever
/// wins. Pulls from the precomputed add-by-fact index, no full-op scan.
fn achiever(
    task: &PackedTask,
    op_layer: &[u32],
    op_stamp: &[u32],
    gen: u32,
    fact_layer: &[u32],
    f: usize,
) -> Option<usize> {
    let fl = fact_layer[f];
    if fl == INF || fl == 0 {
        return None;
    }
    let mut best = None;
    let mut best_layer = INF;
    for &oi in task.add_by_fact.slice(f) {
        let oi = oi as usize;
        if op_stamp[oi] == gen && op_layer[oi] < fl && op_layer[oi] < best_layer {
            best_layer = op_layer[oi];
            best = Some(oi);
        }
    }
    best
}

/// When fact `f` only lands through op `oi`'s CONDITIONAL effect — no clean
/// unconditional add — flag that effect's trigger facts as extra subgoals so
/// the relaxed plan doesn't skip the cost of earning the condition.
fn queue_cond_for(task: &PackedTask, sc: &mut Scratch, oi: usize, f: usize) {
    if task.add.slice(oi).iter().any(|&x| x as usize == f) {
        return; // unconditional add — nothing extra
    }
    let mut best_layer = INF;
    let mut best: Option<&crate::packed::CondEff> = None;
    for ce in task.cond_effs(oi) {
        if ce.add.iter().any(|&x| x as usize == f) {
            let cl = ce
                .cond_pos
                .iter()
                .map(|&c| sc.fact_layer[c as usize])
                .max()
                .unwrap_or(0);
            if cl != INF && cl < best_layer {
                best_layer = cl;
                best = Some(ce);
            }
        }
    }
    if let Some(ce) = best {
        for &cf in &ce.cond_pos {
            let c = cf as usize;
            if sc.need_fact[c] != sc.gen {
                sc.need_fact[c] = sc.gen;
                sc.queue.push(cf);
            }
        }
    }
}

/// Draft op `oi` (×`reps`) into the relaxed plan and flag its preconditions.
fn select(task: &PackedTask, sc: &mut Scratch, oi: usize, reps: i32, count: &mut i32) {
    if sc.selected[oi] == sc.gen {
        return;
    }
    sc.selected[oi] = sc.gen;
    // a selected op applicable in the current state (layer 0) is a helpful action.
    if sc.op_stamp[oi] == sc.gen && sc.op_layer[oi] == 0 {
        sc.helpful.push(oi as u32);
    }
    *count += reps.max(1);
    for &pf in task.pre_pos.slice(oi) {
        let f = pf as usize;
        if sc.need_fact[f] != sc.gen {
            sc.need_fact[f] = sc.gen;
            sc.queue.push(pf);
        }
    }
}

fn numeric_achiever(
    task: &PackedTask,
    np: &NumPre,
    fv: &[f64],
    def: &[bool],
    op_stamp: &[u32],
    gen: u32,
) -> Option<(usize, i32)> {
    let target = match &np.lhs {
        NExpr::Fluent(i) => *i,
        _ => return None,
    };
    let want = match &np.rhs {
        NExpr::Num(n) => *n,
        _ => return None,
    };
    let cur = if def[target as usize] {
        fv[target as usize]
    } else {
        0.0
    };
    let need_raise = cur < want;
    let mut best: Option<(usize, i32)> = None;
    // only ops with a numeric effect on `target` can help (op-id order preserved,
    // so the min-reps tie-break is identical to the former full scan)
    for &oi in task.neff_by_fluent.slice(target as usize) {
        let oi = oi as usize;
        if op_stamp[oi] != gen {
            continue;
        }
        for ne in task.num_eff.slice(oi) {
            if ne.target != target {
                continue;
            }
            let delta = match ne.value.eval(fv, def) {
                Some(v) => v,
                None => continue,
            };
            let helps = match ne.op {
                AssignOp::Increase => need_raise && delta > 0.0,
                AssignOp::Decrease => !need_raise && delta > 0.0,
                _ => false,
            };
            if helps {
                let reps = (((want - cur).abs() / delta.abs().max(1e-9)).ceil() as i32).max(1);
                if best.map(|(_, r)| reps < r).unwrap_or(true) {
                    best = Some((oi, reps));
                }
            }
        }
    }
    best
}
