//! Four agents, one budget, one target. The sequential portfolio scheduler
//! (ferroplan-roadmap.md Phase 6) — the house's answer to betting everything
//! on a single runner.
//!
//! Instead of one configuration burning the whole allotment alone, a crew of
//! complementary classical agents work the same problem under one shared,
//! deterministic eval budget. Members rotate through rounds on doubling
//! slices — a fast solve gets found cheap, early; a stubborn one earns depth
//! in the rounds that follow. The budget is counted in evaluated states, never
//! the clock, charged against each member's real spend — so the whole run
//! plays out the same on any machine, any thread count. The house doesn't
//! negotiate on that.
//!
//! The crew, run order fixed:
//!   1. `ladder`  — EHC into a bounded LAMA rung into weighted best-first.
//!      The default runner; closes most jobs before round one ends.
//!   2. `lama`    — landmark and preferred-operator tracking alone, running
//!      loose inside its slice. Cracks the plateau domains — barman, parking,
//!      floortile — where the ladder stalls.
//!   3. `bfs-w3`  — weighted best-first at w_h = 3, no EHC detour. Moves in
//!      where helpful-action pruning leads the others astray.
//!   4. `bfs-w1`  — near-uniform best-first. Trades speed for depth-quality.
//!
//! First plan through the wire wins — tagged with whichever member found it.
//! The portfolio doesn't chase a global-best metric; that job stays
//! downstream, in the cost/length sweeps that run on the winning plan same as
//! they would on any single config. If a member's search is complete and
//! comes back empty-handed — no cap, no ceiling, just proven dead — the whole
//! job is called unsolvable on the spot.

use crate::packed::PackedTask;
use crate::search::{plan, search_from, PlanResult, SearchCfg};

/// Opening slice, every member's first shot; doubles each round after.
const SLICE0: usize = 50_000;

pub struct Outcome {
    pub ops: Option<Vec<usize>>,
    pub evaluated: usize,
    /// Which member cracked it — logged for the report.
    pub winner: Option<&'static str>,
}

pub fn solve(task: &PackedTask, threads: usize, cfg: SearchCfg) -> Outcome {
    let names: [&'static str; 4] = ["ladder", "lama", "bfs-w3", "bfs-w1"];
    let mut alive = [true; 4];
    let mut pool = cfg.max_eval;
    let mut evaluated = 0usize;
    let mut round = 0u32;

    // Budget-aware phase A (the settled Phase 6 verdict): the DEFAULT member
    // runs to its NATURAL END on the FULL pool before diversification spends
    // anything. The doubling schedule preempted the ladder and net-LOST 11
    // corpus instances (sokoban −7, visit-all −4 — domains where the ladder
    // needs the whole budget; diversification won only +2). Ladder-first
    // makes portfolio coverage ≥ default BY CONSTRUCTION: the ladder sees
    // exactly the default's budget, and the others run only on what it left
    // behind (an early internal wall — node cap, LAMA cap, dead end).
    // `FF_PORTFOLIO_SLICED=1` restores the pure doubling schedule.
    if std::env::var("FF_PORTFOLIO_SLICED").is_err() {
        let (ops, used, _) = run_member(task, 0, threads, cfg, pool);
        evaluated += used;
        pool = pool.saturating_sub(used.max(1));
        if let Some(ops) = ops {
            return Outcome {
                ops: Some(ops),
                evaluated,
                winner: Some(names[0]),
            };
        }
        alive[0] = false;
    }

    while pool > 0 && alive.iter().any(|&a| a) {
        let slice = (SLICE0 << round).min(pool);
        for (m, &name) in names.iter().enumerate() {
            if !alive[m] || pool == 0 {
                continue;
            }
            let budget = slice.min(pool);
            let (ops, used, proven_unsolvable) = run_member(task, m, threads, cfg, budget);
            evaluated += used;
            pool = pool.saturating_sub(used.max(1));
            if let Some(ops) = ops {
                return Outcome {
                    ops: Some(ops),
                    evaluated,
                    winner: Some(name),
                };
            }
            if proven_unsolvable {
                // A COMPLETE member exhausted the space under no bound:
                // the task is unsolvable, no schedule can change that.
                return Outcome {
                    ops: None,
                    evaluated,
                    winner: None,
                };
            }
            // A member that spent less than its slice without a plan or a
            // proof hit an internal wall (node cap, dead end): re-running
            // it bigger cannot help less than a fresh slice can — keep it
            // alive only if it actually consumed the slice (more budget
            // could genuinely reach further).
            if used < budget {
                alive[m] = false;
            }
        }
        round += 1;
    }
    Outcome {
        ops: None,
        evaluated,
        winner: None,
    }
}

/// One runner, one bounded shift on the clock. Comes back with a plan, a
/// body count of evals spent, and a flag if the search proved the ground
/// dead rather than just running out of budget.
fn run_member(
    task: &PackedTask,
    member: usize,
    threads: usize,
    cfg: SearchCfg,
    budget: usize,
) -> (Option<Vec<usize>>, usize, bool) {
    match member {
        0 => {
            let o = plan(
                task,
                threads,
                SearchCfg {
                    max_eval: budget,
                    ..cfg
                },
                true,
            );
            // plan() folds EHC + LAMA + best-first; its None is budget-capped
            // in practice — never claim a proof through the wrapper.
            (o.ops, o.evaluated.max(1), false)
        }
        1 => match crate::lama::search(task, threads, budget, &[]) {
            Some((ops, ev)) => (Some(ops), ev.max(1), false),
            None => (None, budget, false), // lama doesn't report evals on failure
        },
        _ => {
            let wh = if member == 2 { 3.0 } else { 1.0 };
            match search_from(
                task,
                &task.initial(),
                &task.goal_pos,
                &task.goal_num,
                None,
                f64::INFINITY,
                threads,
                SearchCfg::from_weights(1.0, wh, Some(budget)),
                &[],
                None,
                None,
            ) {
                PlanResult::Plan { ops, evaluated, .. } => (Some(ops), evaluated.max(1), false),
                PlanResult::Unsolvable { evaluated, capped } => (None, evaluated.max(1), !capped),
            }
        }
    }
}
