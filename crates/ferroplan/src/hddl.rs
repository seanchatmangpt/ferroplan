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
//!
//! Eve integration: [`solve_hddl_from_eve`] is the consumer of Eve's
//! `DecomposeHddl` lifecycle stage — it takes the typed
//! [`crate::eve::EveHandoff`] and runs its `hddl` decomposition request
//! through [`solve_hddl`]. Previously that stage was emitted by
//! `crate::eve` with nothing downstream consuming it; this bridge is the
//! missing consumer.

use crate::eve::EveHandoff;
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
    /// The HDDL problem text already declares its own `(:htn ...)` root task
    /// network and it does not match the single root task named by
    /// [`EveHandoff::hddl`] (consumed by [`solve_hddl_from_eve`]). Eve's
    /// handoff is the *exact* hierarchical decomposition request, so a
    /// conflicting in-file network means the Genesis world is internally
    /// inconsistent; the bridge refuses loudly rather than silently picking
    /// a winner. `root_task` is Eve's task call; `problem_root_network`
    /// lists the task calls declared by the problem's `:htn` section, each
    /// rendered `(name arg...)`.
    RootTaskMismatch {
        root_task: String,
        problem_root_network: Vec<String>,
    },
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
            Self::RootTaskMismatch {
                root_task,
                problem_root_network,
            } => write!(
                f,
                "Eve root task {root_task} conflicts with the problem's own \
                 :htn network {problem_root_network:?}"
            ),
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

/// Consumer of Eve's `DecomposeHddl` lifecycle stage: take the typed
/// [`EveHandoff`] emitted by [`crate::eve::Eve::enter`] and run its
/// `handoff.hddl` decomposition request (`domain` + `problem` + `root_task`)
/// through [`solve_hddl`].
///
/// # Root-task adaptation
///
/// `solve_hddl` takes raw HDDL text, so the root task must be expressed
/// *inside* the problem text — as the problem's `(:htn ...)` section, whose
/// network the grounder grounds into the root task addresses the translator
/// BFS decomposes from (see `ferroplan_hddl::grounder::ground_root_network`).
/// Eve, however, carries the root task as a standalone task-call string
/// (e.g. `"(deploy-service production)"`), and Eve-authored problem text may
/// legitimately omit the `:htn` section entirely. The bridge therefore
/// adapts as follows:
///
/// 1. The problem text is parsed first (a parse failure here is returned as
///    [`HddlError::Parse`] — the same error [`solve_hddl`]'s parse phase
///    would produce, just earlier, because the decision below needs the
///    parsed shape).
/// 2. If the problem declares **no** root network (empty `:htn`), the Eve
///    root task is spliced in as a trailing
///    `(:htn :ordered-subtasks ROOT_TASK)` section — inserted *before the
///    problem's closing parenthesis* by a comment-aware depth scan
///    ([`insert_htn_section`]), never naive end-of-text appending (a
///    trailing `;` comment would otherwise end up inside the new section).
/// 3. If the problem **does** declare a root network, it must be exactly the
///    single Eve root task; anything else is a
///    [`HddlError::RootTaskMismatch`] refusal. Eve's handoff is the exact
///    decomposition request, but silently overriding the in-file network (or
///    silently keeping it) would bury a Genesis-world inconsistency, so the
///    mismatch is refused instead of resolved.
///
/// The Eve root task itself is parsed with the *real* problem grammar (via a
/// synthetic probe problem, [`eve_root_task_network`]) rather than a
/// hand-rolled mini-parser, so whatever the pipeline ultimately accepts, the
/// bridge accepts and compares exactly the same shapes.
///
/// # Regimes
///
/// Both [`PlanningRegime::Deterministic`] and
/// [`PlanningRegime::Probabilistic`] handoffs run the *same* HDDL half:
/// parse -> ground -> translate -> FOND solve. The PPDDL half of a
/// Probabilistic handoff (`handoff.ppddl`) is **explicitly out of scope**
/// here — governing uncertainty is the `GovernUncertaintyPpddl` stage, owned
/// by the PPDDL pipeline (`crate::ppddl`), not by the HDDL decomposer; this
/// bridge deliberately leaves it untouched rather than half-running it.
///
/// # Errors
///
/// Every [`HddlError`] variant `solve_hddl` can produce, plus
/// [`HddlError::RootTaskMismatch`] per rule 3 above. The wall-clock watchdog
/// semantics of [`solve_hddl`] (including `limits.max_wall_ms == 0` meaning
/// unbounded) carry over unchanged, since this function delegates to it.
pub fn solve_hddl_from_eve(
    handoff: &EveHandoff,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, HddlError> {
    let problem_src = problem_with_eve_root_task(&handoff.hddl.problem, &handoff.hddl.root_task)?;
    solve_hddl(&handoff.hddl.domain, &problem_src, limits)
}

/// Resolve the problem text handed to [`solve_hddl`] so its root network is
/// exactly Eve's single root task — see [`solve_hddl_from_eve`]'s
/// "# Root-task adaptation" for the three rules this implements.
fn problem_with_eve_root_task(problem_src: &str, root_task: &str) -> Result<String, HddlError> {
    let problem = ferroplan_hddl::parser::parse_problem(problem_src)
        .map_err(|e| HddlError::Parse(e.to_string()))?;

    let eve_network = eve_root_task_network(root_task)?;
    if problem.htn.subtasks.is_empty() {
        return Ok(insert_htn_section(problem_src, root_task));
    }

    let declared = problem
        .htn
        .subtasks
        .iter()
        .map(|st| task_call_key(&st.task.name, &st.task.args))
        .collect::<Vec<_>>();
    if declared == eve_network {
        // The problem already says exactly what Eve says; use its text
        // verbatim so the request that enters the pipeline is byte-for-byte
        // the one authored in the Genesis world.
        return Ok(problem_src.to_owned());
    }
    Err(HddlError::RootTaskMismatch {
        root_task: root_task.to_owned(),
        problem_root_network: declared,
    })
}

/// Parse Eve's standalone root-task string with the *real* problem grammar
/// and return its root network as rendered task calls `(name arg...)`. The
/// probe problem needs no matching domain: `parse_problem` deliberately does
/// not cross-check `:domain` references (validation is the grounder's job,
/// never reached for the probe), so a placeholder name is honest here.
fn eve_root_task_network(root_task: &str) -> Result<Vec<String>, HddlError> {
    let probe = format!(
        "(define (problem eve-root-task-probe) (:domain eve-root-task-probe) \
         (:htn :ordered-subtasks {}))",
        root_task.trim()
    );
    let problem = ferroplan_hddl::parser::parse_problem(&probe)
        .map_err(|e| HddlError::Parse(format!("Eve root task `{root_task}`: {e}")))?;
    Ok(problem
        .htn
        .subtasks
        .iter()
        .map(|st| task_call_key(&st.task.name, &st.task.args))
        .collect())
}

/// Render a task call as `(name arg...)` (bare `(name)` when argument-free)
/// — the canonical form used both to compare the problem's declared root
/// network against Eve's root task and to report a mismatch. Terms render
/// as their own token (`?v` variables, bare constants), which is exactly the
/// spelling the HDDL tokenizer produced them from.
fn task_call_key(name: &str, args: &[ferroplan_hddl::ast::Term]) -> String {
    if args.is_empty() {
        return format!("({name})");
    }
    let rendered = args
        .iter()
        .map(|arg| match arg {
            ferroplan_hddl::ast::Term::Var(v) => format!("?{v}"),
            ferroplan_hddl::ast::Term::Const(c) => c.clone(),
        })
        .collect::<Vec<_>>()
        .join(" ");
    format!("({name} {rendered})")
}

/// Splice `(:htn :ordered-subtasks ROOT_TASK)` into the problem text as its
/// final section, inserted immediately before the `)` that closes the
/// top-level `(define (problem ...) ...)` form.
///
/// The insertion point is found with a character scan that tracks paren
/// depth and skips `;` line comments (the tokenizer in
/// `ferroplan_hddl::parser` strips those, so a commented-out `)` or a
/// comment containing parens must not derail the depth count). If the scan
/// never finds a balanced top-level form, the text is handed through
/// *unchanged*: there is no safe splice point, and the parser invoked by
/// [`solve_hddl`] then produces the honest [`HddlError::Parse`] for whatever
/// is malformed about the input, rather than this helper inventing a
/// different failure shape.
fn insert_htn_section(problem_src: &str, root_task: &str) -> String {
    let mut depth: usize = 0;
    let mut in_comment = false;
    for (idx, ch) in problem_src.char_indices() {
        if in_comment {
            if ch == '\n' {
                in_comment = false;
            }
            continue;
        }
        match ch {
            ';' => in_comment = true,
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let mut out = String::with_capacity(
                        problem_src.len() + root_task.len() + " (:htn :ordered-subtasks )".len(),
                    );
                    out.push_str(&problem_src[..idx]);
                    out.push_str("(:htn :ordered-subtasks ");
                    out.push_str(root_task.trim());
                    out.push(')');
                    out.push_str(&problem_src[idx..]);
                    return out;
                }
            }
            _ => {}
        }
    }
    problem_src.to_owned()
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

    // ---- solve_hddl_from_eve (Eve `DecomposeHddl` consumer) ----
    //
    // Micro world, exactly the ticket's shape: 1 compound task -> 1 method
    // -> 1 primitive. Hand-authored here (provenance: this ticket); it is a
    // minimal pipeline exercise, not copied from any external corpus.

    const EVE_MICRO_DOMAIN: &str = r#"(define (domain eve-micro)
  (:predicates (done))
  (:task do-thing)
  (:action act
    :parameters ()
    :precondition (and)
    :effect (and (done)))
  (:method m-do-thing
    :task (do-thing)
    :ordered-subtasks (and (t1 (act)))))"#;

    const EVE_MICRO_PROBLEM: &str = r#"(define (problem eve-micro-p1)
  (:domain eve-micro)
  (:init)
  (:goal (and (done))))"#;

    /// Build a minimal but fully valid Eve request whose HDDL surface is the
    /// micro world above (construction mirrors
    /// `tests/eve_genesis.rs::request`), with the problem text and root task
    /// overridable per-test.
    fn eve_micro_request(
        problem: String,
        root_task: String,
        ppddl: bool,
    ) -> crate::eve::EveRequest {
        crate::eve::EveRequest {
            purpose: crate::eve::HumanPurpose {
                statement: "Do the one thing".to_string(),
                desired_consequence: "done holds".to_string(),
                actor: None,
                activators: vec![],
            },
            genesis: crate::eve::GenesisWorld {
                ontology_rdf: "@prefix fp: <urn:ferroplan:> .".to_string(),
                construct_query: "CONSTRUCT { ?s ?p ?o } WHERE { ?s ?p ?o }".to_string(),
                hddl: crate::eve::HddlSurface {
                    domain: EVE_MICRO_DOMAIN.to_string(),
                    problem,
                    root_task,
                },
                ppddl: ppddl.then(|| crate::eve::PpddlSurface {
                    domain: "(define (domain eve-micro-uncertain))".to_string(),
                    problem: "(define (problem eve-micro-uncertain-p1))".to_string(),
                }),
            },
            manufacture: crate::eve::ManufactureTarget {
                name: "do-thing.part".to_string(),
                template: "ggen://truex/part".to_string(),
                artifact_kind: ".part.wasm".to_string(),
                output: "target/parts/do-thing.part.wasm".to_string(),
            },
            capability: crate::eve::CapabilityTarget {
                capability: "do-thing".to_string(),
                route: "powl://do-thing/v1".to_string(),
                authority_scopes: vec![],
            },
        }
    }

    /// Rule 3 of `solve_hddl_from_eve`'s root-task adaptation: a problem
    /// text that declares its own `(:htn ...)` network *different* from
    /// Eve's root task is a Genesis-world inconsistency and must be refused
    /// with the typed `RootTaskMismatch` variant (naming both sides), never
    /// silently overridden in either direction.
    #[test]
    fn eve_bridge_refuses_a_problem_network_conflicting_with_the_eve_root_task() {
        let request = eve_micro_request(
            r#"(define (problem eve-micro-p1)
  (:domain eve-micro)
  (:htn :ordered-subtasks (some-other-task))
  (:init)
  (:goal (and (done))))"#
                .to_string(),
            "(do-thing)".to_string(),
            false,
        );
        let handoff = crate::eve::Eve::enter(request).expect("valid Eve request");
        let err = solve_hddl_from_eve(&handoff, &PlannerLimits::default()).unwrap_err();
        match err {
            HddlError::RootTaskMismatch {
                root_task,
                problem_root_network,
            } => {
                assert_eq!(root_task, "(do-thing)");
                assert_eq!(problem_root_network, vec!["(some-other-task)"]);
            }
            other => panic!("expected RootTaskMismatch, got {other:?}"),
        }
    }

    /// Rule 3's complement: a problem whose own network *is* the Eve root
    /// task passes through byte-for-byte and solves — the same end-to-end
    /// result as the spliced case, proving the pass-through branch is the
    /// equal-network case, not just "any existing network wins".
    #[test]
    fn eve_bridge_accepts_a_problem_whose_network_matches_the_eve_root_task() {
        let request = eve_micro_request(
            r#"(define (problem eve-micro-p1)
  (:domain eve-micro)
  (:htn :ordered-subtasks (do-thing))
  (:init)
  (:goal (and (done))))"#
                .to_string(),
            "(do-thing)".to_string(),
            false,
        );
        let handoff = crate::eve::Eve::enter(request).expect("valid Eve request");
        let plan = solve_hddl_from_eve(&handoff, &PlannerLimits::default())
            .expect("matching root networks must solve");
        assert!(plan.solved);
        assert!(!plan.policy.is_empty());
    }

    /// Rule 2's mechanics: the splice inserts the `(:htn ...)` section
    /// *before the problem's closing parenthesis*, surviving a trailing
    /// `;` comment that a naive end-of-text append would swallow into the
    /// new section (the comment runs to end-of-line, past the inserted
    /// text's start).
    #[test]
    fn eve_bridge_splices_the_root_task_before_the_closing_paren_even_with_a_trailing_comment() {
        let problem = r#"(define (problem eve-micro-p1)
  (:domain eve-micro)
  (:init)
  (:goal (and (done))))
; trailing comment after the problem form)"#;
        let spliced = insert_htn_section(problem, "(do-thing)");
        let parsed =
            ferroplan_hddl::parser::parse_problem(&spliced).expect("spliced text must still parse");
        assert_eq!(parsed.htn.subtasks.len(), 1);
        assert_eq!(parsed.htn.subtasks[0].task.name, "do-thing");
    }

    /// An Eve root task that is not a well-formed task call is refused with
    /// the parse error the real grammar produces (via the probe problem),
    /// never a panic and never a silent empty network.
    #[test]
    fn eve_bridge_reports_a_parse_error_for_a_malformed_eve_root_task() {
        let request = eve_micro_request(
            EVE_MICRO_PROBLEM.to_string(),
            "do-thing".to_string(), // no parens: not a task call
            false,
        );
        let handoff = crate::eve::Eve::enter(request).expect("valid Eve request");
        let err = solve_hddl_from_eve(&handoff, &PlannerLimits::default()).unwrap_err();
        assert!(
            matches!(err, HddlError::Parse(ref msg) if msg.contains("do-thing")),
            "expected Parse error naming the bad root task, got {err:?}"
        );
    }
}
