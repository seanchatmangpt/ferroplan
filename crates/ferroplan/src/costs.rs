//! IPC6 `:action-costs`, run through the classical path (0.9 roadmap Phase 2).
//!
//! IPC-2008 put a price on every move: a numeric fluent — conventionally
//! `total-cost` — set at `:init`, ticked up by constant amounts in action
//! effects, and driven down by `(:metric minimize (total-cost))`. Ferroplan's
//! numeric machinery already *tracks* that fluent through grounding and
//! `apply`; this module is what makes the classical (non-preference) path
//! *reason* about it:
//!
//! 1. [`metric_fluent`] reads the metric off the wire — `minimize` of a
//!    single ground fluent, the only shape supported — and names it for
//!    [`PackedTask::fluent_id`].
//! 2. [`plan_cost`] replays the plan to pull the metric's real final tally
//!    (the exact number the external validator will also land on).
//! 3. [`improve`] runs the anytime sweep: a bounded branch-and-bound
//!    best-first pass under the first plan's cost, ordered by accumulated
//!    spend (`w_c`) and guided by the cost-augmented relaxed plan
//!    ([`crate::heuristic::relaxed_costed`]), holding the cheapest incumbent
//!    as it tightens. The first plan itself rides untouched EHC /
//!    weighted-best-first machinery — fast, bit-identical to the pre-cost
//!    days — so cost support never drags the classical baseline down; only
//!    the polish pass is new ground.
//!
//! The sweep's budget stays on a short leash (`sweep_budget`): a polish pass
//! must never dwarf the solve that bought the plan in the first place.
//! `FF_COST_SWEEP_EVALS` overrides it — `0` kills the sweep outright.

use crate::packed::PackedTask;
use crate::search::{search_from, PlanResult, SearchCfg};
use crate::types::{Expr, MetricDir, Problem, Term};

/// Floor on the sweep's budget: even a plan found nearly for free — EHC in
/// a few dozen evals — earns a real polish pass, not a token gesture.
const SWEEP_FLOOR: usize = 30_000;
/// The sweep spends at most this multiple of what the first plan cost to
/// find — the polish stays proportionate. A hard instance that already
/// needed the big best-first fallback doesn't get to double its bill
/// polishing on top of that.
const SWEEP_MULT: usize = 2;

/// Read the wire for the one supported IPC6 cost-metric shape on a
/// classical problem: `(:metric minimize <ground fluent>)`. Returns the
/// fluent's display string (e.g. `"(TOTAL-COST)"`) for
/// [`PackedTask::fluent_id`]. Anything else — maximize, compound
/// expressions, lifted terms — comes back `None`: those metrics are never
/// silently optimized. Callers report a plan with no metric claim attached,
/// and the PDDL3 path keeps sole custody of `is-violated` metrics.
pub fn metric_fluent(problem: &Problem) -> Option<String> {
    let (dir, e) = problem.metric.as_ref()?;
    if !matches!(dir, MetricDir::Minimize) {
        return None;
    }
    if let Expr::Fluent(name, args) = e {
        let mut parts = vec![name.to_uppercase()];
        for t in args {
            match t {
                Term::Const(c) => parts.push(c.to_uppercase()),
                Term::Var(_) => return None,
            }
        }
        Some(if parts.len() == 1 {
            format!("({})", parts[0])
        } else {
            format!("({} {})", parts[0], parts[1..].join(" "))
        })
    } else {
        None
    }
}

/// The metric's true value after running `ops` from the initial state — an
/// exact replay through [`PackedTask::apply`], so conditional cost effects
/// and non-constant increases all get their due. `None` if the fluent never
/// resolves — no `:init` assignment, and nothing wrote it along the way.
pub fn plan_cost(task: &PackedTask, cf: usize, ops: &[usize]) -> Option<f64> {
    let mut s = task.initial();
    for &oi in ops {
        s = task.apply(oi, &s);
    }
    if s.fdef[cf] {
        Some(s.fv[cf])
    } else {
        None
    }
}

/// What the anytime sweep dragged back.
pub struct CostOutcome {
    /// The best plan on hand — the input plan itself, if the sweep found
    /// nothing to beat it.
    pub ops: Vec<usize>,
    /// Its true cost, replayed, never estimated.
    pub cost: f64,
    /// The sweep struck something strictly cheaper.
    pub improved: bool,
    /// The sweep burned through the whole bounded space, no caps hit —
    /// nothing cheaper than `cost` exists out there. Proven, not just
    /// best-found.
    pub proven: bool,
    /// States the sweep evaluated, for the budget ledger.
    pub evaluated: usize,
}

/// The sweep's eval budget: proportionate to the solve (`SWEEP_MULT` times
/// its evals, floored at `SWEEP_FLOOR`), never spilling past what's left of
/// the caller's overall allowance. `FF_COST_SWEEP_EVALS` overrides it — `0`
/// disables the sweep entirely.
fn sweep_budget(spent: usize, cfg_max: usize) -> usize {
    if let Ok(v) = std::env::var("FF_COST_SWEEP_EVALS") {
        if let Ok(n) = v.trim().parse::<usize>() {
            return n;
        }
    }
    cfg_max
        .saturating_sub(spent)
        .min((SWEEP_MULT * spent).max(SWEEP_FLOOR))
}

/// The anytime cost sweep: bounded branch-and-bound best-first under
/// `first_cost`, ordered by accumulated metric spend, guided by the
/// cost-augmented relaxed plan. Deterministic top to bottom — every knob is
/// integer, the sweep itself is thread-count independent. Hands back the
/// best plan it found — never worse than what walked in.
pub fn improve(
    task: &PackedTask,
    cf: usize,
    ops: Vec<usize>,
    first_cost: f64,
    threads: usize,
    base: SearchCfg,
    spent: usize,
) -> CostOutcome {
    if first_cost <= 0.0 {
        // Nothing can beat a free plan.
        return CostOutcome {
            ops,
            cost: first_cost,
            improved: false,
            proven: true,
            evaluated: 0,
        };
    }
    let budget = sweep_budget(spent, base.max_eval);
    if budget == 0 {
        return CostOutcome {
            ops,
            cost: first_cost,
            improved: false,
            proven: false,
            evaluated: 0,
        };
    }
    let cfg = SearchCfg {
        max_eval: budget,
        anytime: true,
        ..base.with_cost_weight(1.0).with_cost_h(cf)
    };
    match search_from(
        task,
        &task.initial(),
        &task.goal_pos,
        &task.goal_num,
        Some(cf),
        first_cost,
        threads,
        cfg,
        &[],
        None,
        None,
    ) {
        PlanResult::Plan {
            ops: better,
            evaluated,
            ..
        } => {
            // Replay for the REAL cost — the sweep's bound math is trusted,
            // but the reported number must be the executable plan's value.
            match plan_cost(task, cf, &better) {
                Some(c) if c < first_cost => CostOutcome {
                    ops: better,
                    cost: c,
                    improved: true,
                    proven: false,
                    evaluated,
                },
                _ => CostOutcome {
                    ops,
                    cost: first_cost,
                    improved: false,
                    proven: false,
                    evaluated,
                },
            }
        }
        PlanResult::Unsolvable { evaluated, capped } => CostOutcome {
            ops,
            cost: first_cost,
            // Un-capped exhaustion under the bound = no cheaper plan exists.
            proven: !capped,
            improved: false,
            evaluated,
        },
    }
}

/// Shared text-path hook for `run_planner` / `run_ff`: read the cost
/// metric, replay the plan's spend, run the sweep — unless `optimize` says
/// stand down — and swap `ops` for whatever's cheaper. Hands back the final
/// cost and a short field note, or `None` when the problem carries no
/// supported cost metric at all (text output then stays byte-identical).
pub fn optimize_text(
    problem: &Problem,
    task: &PackedTask,
    optimize: bool,
    threads: usize,
    cfg: SearchCfg,
    ops: &mut Vec<usize>,
) -> Option<(f64, &'static str)> {
    let disp = metric_fluent(problem)?;
    let cf = task.fluent_id(&disp)?;
    let c0 = plan_cost(task, cf, ops)?;
    if !optimize {
        return Some((c0, " (not optimized: --satisfice)"));
    }
    let r = improve(task, cf, std::mem::take(ops), c0, threads, cfg, 0);
    *ops = r.ops;
    let note = if r.proven {
        " (proven optimal)"
    } else if r.improved {
        " (anytime-improved)"
    } else {
        ""
    };
    Some((r.cost, note))
}

/// The iterated-weight anytime pass for unit-cost quality (the 0.9 Phase 3
/// remainder): once the first plan lands on a metric-free problem, re-run
/// the whole weighted best-first at decreasing heuristic weights — w_h = 3,
/// 2, 1, greedy sliding toward optimal, the LAMA recipe — each rung
/// bounded to an equal slice of the sweep budget, keeping the shortest plan
/// standing. No new engine underneath: each rung is a plain `search_from`
/// run, so determinism carries through and the result is never worse than
/// what walked in. Opt-in only (`FF_LEN_SWEEP_EVALS=<evals>`; unset or `0`
/// means off, byte-identical first-found behavior) — a measured negative
/// default: under the cost sweep's proportionate budgets these restarts
/// move nothing on the visitall targets (2x-solve budgets, zero gain), and
/// the gain that does exist elsewhere is bought far above the polish
/// doctrine's price (2M evals — ~28x the p01 solve — buys 226 -> 222, about
/// 1.8%). The restart SHAPE is the ceiling here, not the machinery — a
/// length-anytime that tightens inside one search (the cost sweep's own
/// shape), or landmark-guided restarts, are the next recorded ideas.
pub fn improve_length(
    task: &PackedTask,
    ops: Vec<usize>,
    threads: usize,
    base: SearchCfg,
    spent: usize,
) -> (Vec<usize>, usize, bool) {
    let _ = spent; // proportionate default measured toothless; opt-in only
    let budget = std::env::var("FF_LEN_SWEEP_EVALS")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(0)
        .min(base.max_eval);
    const RUNGS: [f64; 3] = [3.0, 2.0, 1.0];
    let per = budget / RUNGS.len();
    if per == 0 || ops.is_empty() {
        return (ops, 0, false);
    }
    let mut best = ops;
    let mut evals = 0usize;
    let mut improved = false;
    for wh in RUNGS {
        // The incumbent's length prunes every rung: nothing at or past
        // best.len() is inserted, which is what gives the low-weight rungs
        // a tractable space (LAMA's restart-with-bound recipe).
        let cfg = SearchCfg {
            g_bound: best.len(),
            ..SearchCfg::from_weights(1.0, wh, Some(per))
        };
        match search_from(
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
        ) {
            PlanResult::Plan {
                ops: cand,
                evaluated,
                ..
            } => {
                evals += evaluated;
                if cand.len() < best.len() {
                    best = cand;
                    improved = true;
                }
            }
            PlanResult::Unsolvable { evaluated, .. } => evals += evaluated,
        }
    }
    (best, evals, improved)
}
