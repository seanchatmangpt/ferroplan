//! Phase B, wire-side: the partition-and-resolve decomposer for the durative/numeric
//! path. Same signal as the SGPlan partition loop ([`crate::resolve`]), rerouted
//! through the temporal grid, behind the `FF_TDECOMP` breaker.
//!
//! Cut the world goal into contracts. Run each as its own TEMPORAL subproblem off the
//! running composed state (`temporal::solve_from`). Splice the timed subplans in
//! strict SEQUENCE — each contract dispatched past the previous makespan plus an ε
//! seam — and when two contracts collide, MERGE the groups down toward a monolithic
//! `temporal::solve`. Net effect: the decomposer is solvable EXACTLY when the
//! monolith is — no dead channel. Every subplan already carries its own ε-separation;
//! `validate`/`treplay` order same-epoch happenings on an ε-grid-rounded time key
//! (ends clear before starts), so the spliced timeline holds without re-separation.
//! Sequential composition doesn't lose completeness here — the RPG domain runs single
//! agent, no `over all` invariant spans contracts — anything needing cross-contract
//! concurrency drops straight through to the monolith.

use crate::ground::{ground_stratified, Outcome};
use crate::hash::FxHashSet;
use crate::packed::PackedTask;
use crate::partition::{interaction_partition, merge_at, merge_with_neighbor, Subgoal};
use crate::temporal::{
    self, build_kind, solve_from, treplay, validate, Kind, TimedPlan, TimedStep,
};
use crate::types::{Domain, Problem};

/// The ε gap kept between mutex happenings — PDDL2.1 spec, matches `temporal::EPS`.
/// Small enough to be noise, big enough to keep two events from landing on the same
/// tick.
const EPS: f64 = 0.001;

/// One cleared contract inside a [`Decomp`]: the sub-goal it settled (rendered for the
/// log), the timed sub-plan that ran it, and where that sub-plan sits on the stitched
/// global timeline (`offset`).
pub(crate) struct ContractRec {
    pub goal: String,
    pub plan: TimedPlan,
    pub offset: f64,
}

/// The full dispatch record for a decomposed temporal goal: the ordered contracts, the
/// stitched whole-goal plan, and whether the run had to fall back to a monolithic
/// solve (un-splittable goal, or a split that failed validation).
pub(crate) struct Decomp {
    pub contracts: Vec<ContractRec>,
    pub plan: TimedPlan,
    pub monolithic: bool,
}

/// Render a contract's sub-goal — positive facts plus numeric thresholds — into a
/// readable log line, e.g. `(order o1), (order o2)` or `coin >= 15`.
fn render_subgoal(task: &PackedTask, g: &crate::partition::Subgoal) -> String {
    let mut parts: Vec<String> = g
        .pos
        .iter()
        .map(|&f| task.fact_names[f as usize].clone())
        .collect();
    for np in &g.num {
        parts.push(render_numpre(task, np));
    }
    if parts.is_empty() {
        "(empty)".to_string()
    } else {
        parts.join(", ")
    }
}

/// Render a numeric goal for the dispatch log. The canonical recipe-gate shape
/// `(fluent op number)` prints clean as `fluent op number`; anything stranger falls
/// back to a compact debug form.
fn render_numpre(task: &PackedTask, np: &crate::types::NumPre) -> String {
    use crate::types::{CompOp, NExpr};
    let op = match np.op {
        CompOp::Lt => "<",
        CompOp::Le => "<=",
        CompOp::Eq => "=",
        CompOp::Ge => ">=",
        CompOp::Gt => ">",
    };
    match (&np.lhs, &np.rhs) {
        (NExpr::Fluent(t), NExpr::Num(v)) => {
            let name = task
                .fluent_names
                .get(*t as usize)
                .cloned()
                .unwrap_or_else(|| format!("fluent#{t}"));
            format!("{name} {op} {v}")
        }
        _ => format!("{:?} {op} {:?}", np.lhs, np.rhs),
    }
}

/// Blackout fallback: wrap a whole-goal solve as a single-contract [`Decomp`] — the
/// partition failed to cut, or the cut plan didn't validate. `plan` is the raw
/// whole-goal plan straight off `temporal::solve`.
fn monolithic_decomp(goal: String, plan: TimedPlan) -> Decomp {
    Decomp {
        contracts: vec![ContractRec {
            goal,
            offset: 0.0,
            plan: plan.clone(),
        }],
        plan,
        monolithic: true,
    }
}

pub fn solve(domain: &Domain, problem: &Problem, threads: usize) -> Option<TimedPlan> {
    decompose(domain, problem, threads).map(|d| d.plan)
}

/// The escalation ladder's decomposer rung, called from [`temporal::solve`]: run the
/// decompose pass but cut the monolithic fallback wire — by the time the ladder
/// reaches here it has already burned the monolithic search at both the ambient and
/// `Full` tiers. Re-running it (single-group collapse, or a failed composition
/// validate) would spend the same node budget a third time chasing a guaranteed
/// `None`.
pub(crate) fn solve_after_ladder(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    tier: crate::features::DemandMode,
) -> Option<TimedPlan> {
    // `tier` comes from the ladder itself (its rung-0 tier), not a fresh global
    // read — so the "already exhausted" premise can't drift if a concurrent
    // caller flips the process-global override mid-solve.
    decompose_inner(domain, problem, threads, tier, false).map(|d| d.plan)
}

/// Cut `problem`'s temporal goal into ordered contracts, run and splice them, and
/// hand back the full dispatch record as a [`Decomp`] — or `None` if even the
/// monolithic fallback goes dark. `solve` is this signal, stripped of the contract
/// breakdown.
pub(crate) fn decompose(domain: &Domain, problem: &Problem, threads: usize) -> Option<Decomp> {
    let tier = crate::features::demand_mode();
    decompose_inner(domain, problem, threads, tier, true)
}

/// `monolithic_fallback`: whether a single-group collapse or a failed composition
/// validate drops back to the whole-goal search (direct `FF_TDECOMP` / `decompose`
/// calls — the completeness guarantee holds) or goes straight to `None` (the ladder
/// path, where the monolithic search already ran both tiers and came back empty).
fn decompose_inner(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    tier: crate::features::DemandMode,
    monolithic_fallback: bool,
) -> Option<Decomp> {
    let c = temporal::compile(domain, problem);
    let task = match ground_stratified(&c.domain, &c.problem, threads) {
        Outcome::Task(t) => t,
        Outcome::GoalTrue => {
            let empty = TimedPlan {
                steps: Vec::new(),
                makespan: 0.0,
            };
            return Some(monolithic_decomp("(goal already satisfied)".into(), empty));
        }
        _ => return None,
    };
    // A statically unproducible whole goal can't be solved by any contract split —
    // fail here instead of grinding the O(n²) merge cascade to prove it.
    if temporal::statically_unsolvable(&task, &task.initial(), &task.goal_pos, &task.goal_num) {
        return None;
    }
    let (kind, dur_exprs, inv) = build_kind(&task, &c);
    // Rendered whole-goal description for the monolithic-fallback contract.
    let whole_goal = render_subgoal(
        &task,
        &crate::partition::Subgoal {
            pos: task.goal_pos.clone(),
            num: task.goal_num.clone(),
        },
    );
    let mutex = crate::invariants::synthesize(domain, &task);
    let mut groups = partition_temporal(&task, &kind, &mutex);
    let init = task.initial();
    let dbg = std::env::var("FF_RES_DEBUG").is_ok();
    if dbg {
        eprintln!("[TDECOMP] {} initial contracts", groups.len());
    }

    loop {
        // A single group IS the whole goal solved from init: fall back to the full
        // `temporal::solve` (monolithic + escalation ladder — so a direct
        // `FF_TDECOMP` / `decompose` call still gets the Full-tier rescue). This
        // CANNOT recurse unboundedly: the ladder's decomposer rung re-enters here
        // with `monolithic_fallback == false`, which returns `None` instead — the
        // flag, not the callee, is what breaks the cycle.
        if groups.len() == 1 {
            if !monolithic_fallback {
                return None;
            }
            return temporal::solve(domain, problem, threads)
                .map(|p| monolithic_decomp(whole_goal.clone(), p));
        }

        // Sequential validated composition over the evolving (state, offset).
        let mut state = init.clone();
        let mut offset = 0.0_f64;
        let mut composed: Vec<TimedStep> = Vec::new();
        let mut done = vec![false; groups.len()];
        let mut conflict: Option<(usize, Option<usize>)> = None;
        // Per-pass record of solved contracts (goal + sub-plan + offset) for the
        // inspectable decomposition; only the final successful pass is returned.
        let mut record: Vec<ContractRec> = Vec::new();

        for i in 0..groups.len() {
            if groups[i].is_empty() || task.goal_met_with(&state, &groups[i].pos, &groups[i].num) {
                done[i] = true;
                continue;
            }
            // Protect already-achieved siblings: forbid ops that would delete one of
            // their goal FACTS (the ∞-penalty sibling protection from resolve.rs).
            // (Numeric underflow can't be a static op mask — it is caught instead by
            // the post-replay goal re-check below, which then triggers a merge.)
            let protected: FxHashSet<u32> = (0..i)
                .filter(|&j| done[j])
                .flat_map(|j| groups[j].pos.iter().copied())
                .collect();
            let forbidden: Vec<bool> = if protected.is_empty() {
                Vec::new()
            } else {
                (0..task.n_ops)
                    .map(|oi| task.del.slice(oi).iter().any(|f| protected.contains(f)))
                    .collect()
            };

            // Solve the contract from the CURRENT state under sibling protection;
            // if infeasible, relax the protection (a real break is caught below).
            let mut unlimited = usize::MAX;
            let plan_i = match solve_from(
                &task,
                &kind,
                &dur_exprs,
                &inv,
                &state,
                &groups[i].pos,
                &groups[i].num,
                &forbidden,
                &[], // the decomposer doesn't handle timed initial literals
                threads,
                tier,
                &mut unlimited,
                crate::search::NODE_CAP_TARGET_BYTES,
            )
            .or_else(|| {
                if forbidden.is_empty() {
                    None
                } else {
                    solve_from(
                        &task,
                        &kind,
                        &dur_exprs,
                        &inv,
                        &state,
                        &groups[i].pos,
                        &groups[i].num,
                        &[],
                        &[],
                        threads,
                        tier,
                        &mut { usize::MAX },
                        crate::search::NODE_CAP_TARGET_BYTES,
                    )
                }
            }) {
                Some(p) => p,
                None => {
                    if dbg {
                        eprintln!("[TDECOMP] contract {i} UNSOLVABLE from current state");
                    }
                    conflict = Some((i, None));
                    break;
                }
            };

            // Advance the shared state by replaying the contract's plan; None means
            // it no longer applies from `state` (shared-resource shortfall).
            let ns = match treplay(&task, &state, &plan_i) {
                Some(s) => s,
                None => {
                    conflict = Some((i, None));
                    break;
                }
            };
            // Reject if this contract broke an already-achieved sibling (predicate OR
            // numeric — the goal_met_with re-check is the numeric-conflict detector).
            if let Some(j) = (0..i)
                .find(|&j| done[j] && !task.goal_met_with(&ns, &groups[j].pos, &groups[j].num))
            {
                conflict = Some((i, Some(j)));
                break;
            }

            // Splice the contract's timed steps past the running offset (ε seam).
            for st in &plan_i.steps {
                composed.push(TimedStep {
                    time: st.time + offset,
                    action: st.action.clone(),
                    duration: st.duration,
                });
            }
            record.push(ContractRec {
                goal: render_subgoal(&task, &groups[i]),
                offset,
                plan: plan_i.clone(),
            });
            offset += plan_i.makespan + EPS;
            state = ns;
            done[i] = true;
        }

        if conflict.is_none() && task.goal_met_with(&state, &task.goal_pos, &task.goal_num) {
            // Each contract's subplan is already ε-separated internally and the seam
            // adds EPS, so the concatenation is valid as-is — a GLOBAL re-separation
            // would re-tighten across seams and can break applicability. Hard-validate
            // the result; on the off chance it doesn't hold, fall back to the
            // monolithic search rather than ever returning an invalid plan.
            let plan = TimedPlan {
                steps: composed,
                makespan: offset,
            };
            if validate(domain, problem, &plan).is_ok() {
                return Some(Decomp {
                    contracts: record,
                    plan,
                    monolithic: false,
                });
            }
            // Full `temporal::solve` (ladder included) for direct callers; on the
            // ladder path skip outright (already exhausted). Recursion is broken
            // by the `monolithic_fallback` flag — the ladder's rung re-enters with
            // it false and gets `None`, never a cycle.
            if !monolithic_fallback {
                return None;
            }
            return temporal::solve(domain, problem, threads)
                .map(|p| monolithic_decomp(whole_goal.clone(), p));
        }

        // Resolve: coalesce the actual conflicting pair, else the stuck group with a
        // neighbor (verbatim from resolve.rs). Each merge strictly shrinks groups.len()
        // toward the monolithic fallback above, so the loop terminates.
        let last = groups.len() - 1;
        if dbg {
            eprintln!(
                "[TDECOMP] conflict={:?} -> merge (groups {} -> {})",
                conflict,
                groups.len(),
                groups.len() - 1
            );
        }
        match conflict {
            Some((i, Some(j))) => merge_at(&mut groups, i, j),
            Some((i, None)) => merge_with_neighbor(&mut groups, i),
            None => merge_with_neighbor(&mut groups, last),
        };
    }
}

/// Cut the world goal into contracts. v1: positive goal facts grouped by the
/// goal-interaction graph over synthesized mutex vars, numeric goals riding solo
/// (straight [`interaction_partition`]); then ORDER the contracts so a goal that is
/// itself a recipe input to another goal fires LAST — consumer dispatched before the
/// consumed goal. Skip this and the consumer drains the producer-goal dry, and the
/// resolver merges them back into one hard conjunction. Residual conflicts still get
/// coarsened by merge, same as ever.
fn partition_temporal(task: &PackedTask, kind: &[Kind], mutex: &[Vec<u32>]) -> Vec<Subgoal> {
    let mut groups = regress_predicate_preconds(task, kind);
    let base = interaction_partition(task, mutex);
    // Append the goal contracts after the regressed precondition contracts (deduping
    // a regressed fact that is itself a goal), so a precond like `built-house` is
    // produced before its consumer `built-square`.
    let have: FxHashSet<u32> = groups.iter().flat_map(|g| g.pos.iter().copied()).collect();
    for g in base {
        if g.pos.len() == 1 && have.contains(&g.pos[0]) {
            continue;
        }
        groups.push(g);
    }
    order_contracts(task, kind, groups)
}

/// Stage 3: for each PREDICATE goal fact, trace its achiever's PERSISTENT predicate
/// preconditions back (`forall`-expanded at grounding) into contracts of their own —
/// e.g. `built-square` needs `(forall (?s) (built-house ?s))`, but if no contract
/// builds the houses, the square branch is the dead channel that fails monolithic.
/// Emit `{built-house s0}`, `{built-house s1}`. Filtered down to facts whose achiever
/// carries a numeric precondition (real structural builds) — trivially-achieved
/// travel/location facts don't make the cut.
fn regress_predicate_preconds(task: &PackedTask, kind: &[Kind]) -> Vec<Subgoal> {
    let goal: FxHashSet<u32> = task.goal_pos.iter().copied().collect();
    // Bridge a fact `f`'s achiever (an END snap) to its matching START (via the
    // RUNNING token, like extract_landmarks) and run `body` on each such START.
    let on_start_achiever = |f: u32, body: &mut dyn FnMut(usize)| {
        for &end in task.add_by_fact.slice(f as usize) {
            for &pf in task.pre_pos.slice(end as usize) {
                for &s in task.add_by_fact.slice(pf as usize) {
                    if matches!(kind[s as usize], Kind::Start { .. }) {
                        body(s as usize);
                    }
                }
            }
        }
    };
    // A fact is a worthwhile sub-contract iff building it COSTS resources (its START
    // has a numeric `>=` precond or a consume effect) — structural builds like
    // `built-house`, not trivially-achieved travel/location facts.
    let expensive = |f: u32| -> bool {
        let mut hit = false;
        on_start_achiever(f, &mut |s| {
            if !task.pre_num.slice(s).is_empty()
                || task
                    .num_eff
                    .slice(s)
                    .iter()
                    .any(|ne| matches!(ne.op, crate::types::AssignOp::Decrease))
            {
                hit = true;
            }
        });
        hit
    };
    let mut out: Vec<u32> = Vec::new();
    let mut seen: FxHashSet<u32> = FxHashSet::default();
    for &gf in &task.goal_pos {
        for &oi in task.add_by_fact.slice(gf as usize) {
            // bridge the goal-fact achiever END -> its START, take that START's
            // PREDICATE preconds (the `forall`-expanded sub-structures).
            for &f in task.pre_pos.slice(oi as usize) {
                for &start in task.add_by_fact.slice(f as usize) {
                    if !matches!(kind[start as usize], Kind::Start { .. }) {
                        continue;
                    }
                    for &pf in task.pre_pos.slice(start as usize) {
                        if pf != gf && !goal.contains(&pf) && expensive(pf) && seen.insert(pf) {
                            out.push(pf);
                        }
                    }
                }
            }
        }
    }
    out.into_iter()
        .map(|f| Subgoal {
            pos: vec![f],
            num: Vec::new(),
        })
        .collect()
}

/// The goal fluents a contract calls on directly — the numeric goal LHS fluents, the
/// wire's demand signature.
fn group_resources(g: &Subgoal) -> Vec<u32> {
    g.num
        .iter()
        .filter_map(|np| match &np.lhs {
            crate::types::NExpr::Fluent(t) => Some(*t),
            _ => None,
        })
        .collect()
}

/// Stable topological dispatch order: edge a→b (a before b) when running contract
/// `a` CONSUMES a resource that is contract `b`'s own goal — so `b`, the consumed
/// goal, fires after its consumers and its final ledger value survives the run.
/// Cycles (mutual consumption, a closed loop with no clean exit) fall back to
/// original order and get handled downstream by merge.
fn order_contracts(task: &PackedTask, kind: &[Kind], groups: Vec<Subgoal>) -> Vec<Subgoal> {
    let n = groups.len();
    if n < 2 {
        return groups;
    }
    let res: Vec<Vec<u32>> = groups.iter().map(group_resources).collect();
    let chains: Vec<FxHashSet<u32>> = groups
        .iter()
        .map(|g| {
            temporal::demand_resources(task, kind, &g.num)
                .into_iter()
                .collect()
        })
        .collect();
    let mut after: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];
    for a in 0..n {
        for (b, rb) in res.iter().enumerate() {
            if a != b && rb.iter().any(|r| chains[a].contains(r)) {
                after[a].push(b); // a consumes b's goal resource -> a before b
                indeg[b] += 1;
            }
        }
    }
    let mut used = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        // smallest-index ready node (stable); on a cycle, smallest unused.
        let i = (0..n)
            .find(|&i| !used[i] && indeg[i] == 0)
            .unwrap_or_else(|| (0..n).find(|&i| !used[i]).unwrap());
        used[i] = true;
        order.push(i);
        for &b in &after[i] {
            indeg[b] = indeg[b].saturating_sub(1);
        }
    }
    order.into_iter().map(|i| groups[i].clone()).collect()
}
