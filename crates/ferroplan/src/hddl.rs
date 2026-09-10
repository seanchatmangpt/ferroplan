//! HDDL entry point: parse -> ground -> translate (all via `ferroplan_hddl`)
//! -> adapt into [`crate::planning_runtime::PlanningProblem`] -> the
//! existing FOND solver (`solve_planning_type` with
//! [`PlanningType::Fond`], which dispatches to the existing `fond_policy`).
//!
//! This is the "explicit hddl-entry function" alternative rather than a new
//! `PlanningType::Hddl` variant: `PlanningType` already has a closed,
//! 18-member `ALL` array and a `required_capabilities`/`rail` match arm per
//! variant in `planning_types.rs` that every admitted paradigm must answer
//! for — HDDL is a *front-end* (a text syntax that compiles down to the
//! existing flat FOND model), not a new solving paradigm, so it does not
//! need its own entry in that constitution. The result of solving an HDDL
//! problem is still reported as `PlanningType::Fond` in the returned
//! `UniversalPlan`, honestly reflecting which solver actually ran.
//!
//! Dependency direction: `ferroplan` depends on `ferroplan-hddl` (declared in
//! `Cargo.toml`), never the reverse — `ferroplan-hddl` defines its own
//! ground-IR types in `translate.rs` rather than importing
//! `planning_runtime`'s, specifically so this edge does not create a cycle.
//! The `adapt_problem` function below is the one place that maps one shape
//! onto the other.

use crate::planning_runtime::{
    solve_planning_type, Goal, Method, PlannerError, PlannerLimits, PlanningProblem, State, Task,
    Transition, UniversalPlan, UniversalPlanningRequest,
};
use crate::planning_types::PlanningType;
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HddlError {
    Parse(String),
    Ground(String),
    Translate(String),
    Planner(PlannerError),
    /// `limits.max_wall_ms` elapsed before `solve_hddl`'s parse -> ground ->
    /// translate -> solve pipeline finished — see `solve_hddl`'s doc comment
    /// for how this differs from (and is a superset guard over)
    /// `Planner(PlannerError::Timeout)`, `ferroplan_hddl::GroundError::Timeout`,
    /// and `ferroplan_hddl::TranslateError::Timeout`: those three are checked
    /// *inside* their respective phases and so cannot cover the parser (which
    /// has no internal iteration/wall-clock concept at all — see the
    /// timeout/thread-safety audit this responds to), while this variant
    /// bounds the caller's wait on the *entire* call regardless of which
    /// phase is slow.
    Timeout {
        elapsed_ms: u128,
        limit_ms: u128,
    },
    /// `solve_hddl`'s spawned worker thread (see that function's doc
    /// comment) panicked before it could send a result back -- the channel
    /// disconnected with nothing received. Distinct from every other
    /// variant here in that it reflects a bug in this pipeline itself (an
    /// unhandled panic), never a property of the input HDDL text.
    WorkerPanicked(String),
}

impl fmt::Display for HddlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(msg) => write!(f, "HDDL parse error: {msg}"),
            Self::Ground(msg) => write!(f, "HDDL grounding error: {msg}"),
            Self::Translate(msg) => write!(f, "HDDL translation error: {msg}"),
            Self::Planner(e) => write!(f, "planner error: {e}"),
            Self::Timeout {
                elapsed_ms,
                limit_ms,
            } => write!(
                f,
                "solve_hddl wall-clock limit exceeded: {elapsed_ms}ms elapsed, limit {limit_ms}ms"
            ),
            Self::WorkerPanicked(msg) => write!(f, "solve_hddl worker thread panicked: {msg}"),
        }
    }
}
impl std::error::Error for HddlError {}

impl From<PlannerError> for HddlError {
    fn from(e: PlannerError) -> Self {
        Self::Planner(e)
    }
}

/// Field-for-field adapter from `ferroplan_hddl::translate::PlanningProblem`
/// (that crate's own, dependency-free ground-IR shape) to this crate's
/// `planning_runtime::PlanningProblem`. The two shapes are intentionally
/// parallel, so this is a straight structural mapping, not a reinterpretation.
fn adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) -> PlanningProblem {
    PlanningProblem {
        states: p
            .states
            .into_iter()
            .map(|s| State {
                id: s.id,
                facts: s.facts,
                fluents: Default::default(),
            })
            .collect(),
        initial_states: p.initial_states,
        goal: Goal {
            facts: p.goal.facts,
            numeric_min: Default::default(),
            numeric_max: Default::default(),
        },
        unsafe_states: Default::default(),
        soft_goal_facts: Default::default(),
        transitions: p
            .transitions
            .into_iter()
            .map(|t| {
                // Decomposition transitions ("htn:decompose:...") are pure
                // search bookkeeping (method choice), not real-world action
                // execution -- give them zero cost/duration so Classical/
                // CostOptimal/Temporal metrics, if ever run over an
                // HDDL-sourced problem, don't inflate plan length/cost with
                // bookkeeping steps. `fond_policy` (the only planning type
                // `solve_hddl` actually dispatches to) ignores cost/duration
                // entirely, so this changes nothing for the path exercised
                // below -- it's forward-looking correctness only.
                let is_bookkeeping = t.action.starts_with("htn:decompose:");
                Transition {
                    action: t.action,
                    from: t.from,
                    to: t.to,
                    cost: if is_bookkeeping { 0 } else { 1 },
                    duration: if is_bookkeeping { 0 } else { 1 },
                    reward: 0,
                    probability_ppm: t.probability_ppm,
                    observation: None,
                    requires: Default::default(),
                }
            })
            .collect(),
        tasks: p
            .tasks
            .into_iter()
            .map(|t| Task {
                id: t.id,
                primitive_action: t.primitive_action,
                requires: Default::default(),
            })
            .collect(),
        root_tasks: p.root_tasks,
        methods: p
            .methods
            .into_iter()
            .map(|m| Method {
                id: m.id,
                task: m.task,
                subtasks: m.subtasks,
            })
            .collect(),
        workflow_edges: Default::default(),
        queues: Default::default(),
        agents: Default::default(),
        tools: Default::default(),
        rdf: Default::default(),
    }
}

/// Parse, ground, and translate an HDDL domain+problem pair, then run it
/// through the existing FOND solver. This is the real, sequential pipeline
/// with no wall-clock guard of its own -- `ferroplan_hddl::grounder::ground`
/// and `ferroplan_hddl::translate::translate` each already enforce their own
/// `GroundingLimits::max_wall`/`TranslateLimits::max_wall` internally (see
/// those crates' modules), and `solve_planning_type` enforces
/// `limits.max_wall_ms` inside `fond_policy`/`fond_policy_strong_cyclic` --
/// but `ferroplan_hddl::parser::parse_domain`/`parse_problem` have no
/// iteration or wall-clock concept at all. `solve_hddl` (below) wraps this
/// function in a watchdog so the parser (or any future phase that similarly
/// lacks an internal check) is covered too.
fn solve_hddl_inner(
    domain_src: &str,
    problem_src: &str,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, HddlError> {
    let domain = ferroplan_hddl::parser::parse_domain(domain_src)
        .map_err(|e| HddlError::Parse(e.to_string()))?;
    let problem = ferroplan_hddl::parser::parse_problem(problem_src)
        .map_err(|e| HddlError::Parse(e.to_string()))?;
    let ir = ferroplan_hddl::grounder::ground(&domain, &problem, &Default::default())
        .map_err(|e| HddlError::Ground(e.to_string()))?;
    let translated = ferroplan_hddl::translate::translate(
        &ir,
        &ferroplan_hddl::translate::TranslateLimits::default(),
    )
    .map_err(|e| HddlError::Translate(e.to_string()))?;
    let planning_problem = adapt_problem(translated);
    let request = UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: planning_problem,
        limits: limits.clone(),
    };
    Ok(solve_planning_type(&request)?)
}

/// Parse, ground, and translate an HDDL domain+problem pair, then run it
/// through the existing FOND solver -- `solve_hddl_inner`, run under a
/// wall-clock watchdog keyed off `limits.max_wall_ms` (`0` means unbounded,
/// matching `PlannerLimits::max_wall_ms`'s own convention).
///
/// This crate has no cross-platform way to forcibly cancel a running OS
/// thread (Rust deliberately provides none -- there is no safe
/// `pthread_cancel` equivalent), so this is a *watchdog*, not a preemptive
/// kill: `solve_hddl_inner` runs on a spawned thread, and this function
/// returns `Err(HddlError::Timeout)` to the *caller* the moment
/// `limits.max_wall_ms` elapses, whether or not the spawned thread has
/// finished. That is the property a BEAM/wasmex caller actually needs (its
/// own call never blocks past the budget) even though the orphaned worker
/// thread is not synchronously reclaimed. The worker thread is not left
/// truly unbounded either, though: `ground`/`translate` each carry their own
/// internal wall-clock check (`GroundingLimits::max_wall`/
/// `TranslateLimits::max_wall`, both defaulted from the same 10s order of
/// magnitude) and `fond_policy`/`fond_policy_strong_cyclic` check
/// `limits.max_wall_ms` directly, so every phase past the parser also exits
/// on its own within roughly one more `max_wall_ms`-scaled budget even if
/// this watchdog has already returned. The one phase with no such internal
/// check is the parser itself (`ferroplan_hddl::parser`, which -- see the
/// timeout/thread-safety audit this responds to -- has no
/// iteration/wall-clock concept), so an adversarial input that makes
/// parsing alone hang is the one case where the orphaned thread's own
/// runtime is bounded only by whatever makes the parser itself eventually
/// terminate (or not) -- this watchdog still guarantees the *caller* never
/// waits past `max_wall_ms` for that case, which is the property asked for.
pub fn solve_hddl(
    domain_src: &str,
    problem_src: &str,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, HddlError> {
    if limits.max_wall_ms == 0 {
        return solve_hddl_inner(domain_src, problem_src, limits);
    }
    let budget = Duration::from_millis(limits.max_wall_ms);
    let domain_owned = domain_src.to_owned();
    let problem_owned = problem_src.to_owned();
    let limits_owned = limits.clone();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = solve_hddl_inner(&domain_owned, &problem_owned, &limits_owned);
        // The receiver may already have timed out and been dropped; a failed
        // send here just means nobody is listening any more, which is fine.
        let _ = tx.send(result);
    });
    let start = Instant::now();
    match rx.recv_timeout(budget) {
        Ok(result) => result,
        Err(mpsc::RecvTimeoutError::Timeout) => Err(HddlError::Timeout {
            elapsed_ms: start.elapsed().as_millis(),
            limit_ms: u128::from(limits.max_wall_ms),
        }),
        // The sender was dropped without sending -- only possible if the
        // spawned thread itself panicked before reaching `tx.send`.
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(HddlError::WorkerPanicked(
            "channel disconnected before producing a result".to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_C_DOMAIN: &str = include_str!("../../ferroplan-hddl/fixtures/c/domain.hddl");
    const FIXTURE_C_PROBLEM: &str = include_str!("../../ferroplan-hddl/fixtures/c/problem.hddl");

    /// Fixture C's root "reach" task genuinely has NO valid strong FOND
    /// policy once `ferroplan_hddl::translate` is decomposition-aware
    /// (see that crate's `translate.rs` module docs). Its domain declares
    /// two methods for "reach": "m-direct" (one subtask, "cross-bridge")
    /// and "m-two-step" ("cross-bridge" then "walk"). Committing to either
    /// method happens *before* the oneof `cross-bridge` outcome is known,
    /// and neither method's remaining task network covers BOTH outcomes:
    /// "m-direct"'s task network is already exhausted after either outcome
    /// (so the "blocked" outcome, landing at l3 instead of l2, is a stuck
    /// non-goal terminal), and "m-two-step"'s "walk" step only applies from
    /// l3 (so its "success" outcome, landing directly at l2, is a stuck
    /// terminal too, since "walk(l3,l2)" is inapplicable from l2 and the
    /// task network still expects it). So no method choice has all its
    /// outcomes reaching a completed, goal-satisfying task network --
    /// `fond_policy` correctly reports `NoPlan`.
    ///
    /// Before `translate.rs` became decomposition-aware, this test asserted
    /// `plan.solved` and found a policy choosing "cross-bridge" directly
    /// from the initial state -- that was only possible because the old,
    /// decomposition-blind BFS let "walk" fire from ANY state satisfying its
    /// precondition, regardless of which method (if either) was ever
    /// "chosen". That was exactly the class of unsound shortcut this fix
    /// eliminates (see `ferroplan_hddl::translate`'s own
    /// `refuses_a_shortcut_action_unreachable_via_any_decomposition` test),
    /// so the corrected, honest result here is `NoPlan`, not a spurious
    /// success.
    #[test]
    fn reach_htn_with_non_covering_methods_has_no_valid_fond_policy() {
        let err = solve_hddl(
            FIXTURE_C_DOMAIN,
            FIXTURE_C_PROBLEM,
            &PlannerLimits::default(),
        )
        .expect_err("fixture C's HTN has no policy that covers both oneof outcomes");
        assert!(
            matches!(err, HddlError::Planner(PlannerError::NoPlan)),
            "expected Planner(NoPlan), got {err:?}"
        );
    }

    #[test]
    fn rejects_malformed_hddl_with_a_parse_error() {
        let err = solve_hddl(
            "(define (domain broken",
            FIXTURE_C_PROBLEM,
            &PlannerLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(err, HddlError::Parse(_)));
    }

    const FIXTURE_A_DOMAIN: &str = include_str!("../../ferroplan-hddl/fixtures/a/domain.hddl");
    const FIXTURE_A_PROBLEM: &str = include_str!("../../ferroplan-hddl/fixtures/a/problem.hddl");

    /// Thread-safety audit finding #2 (see `~/.claude/rules/tools.md`-style
    /// concurrency checks this responds to): confirm `solve_hddl` produces
    /// correct, independent results when called concurrently from real OS
    /// threads (`std::thread`, not `async`/a single-threaded executor,
    /// so this genuinely exercises parallel execution) — not merely that it
    /// doesn't panic. Two fixtures with opposite, unambiguous outcomes
    /// (fixture A: a fully deterministic deliver domain with a real strong
    /// FOND policy; fixture C: an HTN whose two methods provably cover
    /// neither `oneof` outcome, so `fond_policy`/`fond_policy_strong_cyclic`
    /// both correctly report `NoPlan` — see
    /// `reach_htn_with_non_covering_methods_has_no_valid_fond_policy`'s own
    /// doc comment for the proof) are interleaved across many concurrent
    /// calls; every thread must observe exactly the outcome its own input
    /// implies. If `ferroplan`/`ferroplan_hddl` held any shared mutable
    /// state across calls (a `static`, a thread-unsafe cache, ...), the
    /// two fixtures' concurrent calls would be the kind of interleaving
    /// likely to make that visible as a wrong-fixture result on at least
    /// one thread — this test does not merely assert "no crash", it asserts
    /// each of 40 concurrent calls got the answer *only its own* input
    /// determines. Run this test both under default (parallel) `cargo test`
    /// and under `--test-threads=1` — passing under both is required: a
    /// data race inside `solve_hddl`'s own concurrency (this test spawns its
    /// own real OS threads regardless of the harness's outer test
    /// parallelism) would still be caught with `--test-threads=1`, since
    /// that flag only serializes *across* `#[test]` functions, not the
    /// `std::thread::spawn` calls this one test makes internally.
    #[test]
    fn solve_hddl_produces_correct_independent_results_under_concurrent_calls() {
        use std::sync::Arc;
        use std::thread;

        let domain_a = Arc::new(FIXTURE_A_DOMAIN.to_owned());
        let problem_a = Arc::new(FIXTURE_A_PROBLEM.to_owned());
        let domain_c = Arc::new(FIXTURE_C_DOMAIN.to_owned());
        let problem_c = Arc::new(FIXTURE_C_PROBLEM.to_owned());

        let handles: Vec<_> = (0..40)
            .map(|i| {
                let (domain, problem, expect_solved) = if i % 2 == 0 {
                    (Arc::clone(&domain_a), Arc::clone(&problem_a), true)
                } else {
                    (Arc::clone(&domain_c), Arc::clone(&problem_c), false)
                };
                thread::spawn(move || {
                    let result = solve_hddl(&domain, &problem, &PlannerLimits::default());
                    (i, expect_solved, result)
                })
            })
            .collect();

        let mut solved_count = 0;
        let mut no_plan_count = 0;
        for handle in handles {
            let (i, expect_solved, result) = handle.join().expect("worker thread panicked");
            if expect_solved {
                let plan = result.unwrap_or_else(|e| {
                    panic!("thread {i} (fixture A, expected a solved policy): {e:?}")
                });
                assert!(
                    plan.solved,
                    "thread {i} (fixture A): plan not marked solved"
                );
                assert!(
                    !plan.policy.is_empty(),
                    "thread {i} (fixture A): solved plan carries no policy entries"
                );
                solved_count += 1;
            } else {
                let err = match result {
                    Err(e) => e,
                    Ok(plan) => panic!(
                        "thread {i} (fixture C): expected NoPlan, got a solved plan \
                         ({} policy entries) -- cross-thread state contamination \
                         from fixture A's run?",
                        plan.policy.len()
                    ),
                };
                assert!(
                    matches!(err, HddlError::Planner(PlannerError::NoPlan)),
                    "thread {i} (fixture C): expected Planner(NoPlan), got {err:?}"
                );
                no_plan_count += 1;
            }
        }
        assert_eq!(solved_count, 20, "expected 20 fixture-A threads to solve");
        assert_eq!(
            no_plan_count, 20,
            "expected 20 fixture-C threads to get NoPlan"
        );
    }
}
