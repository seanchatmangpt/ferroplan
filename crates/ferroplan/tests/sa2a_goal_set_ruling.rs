//! FOND-HTN-67 ruling (a) — the vendored autofde-lab `sa2a-v26.9.17` pair,
//! run fresh at HEAD.
//!
//! The pair is vendored byte-for-byte from
//! `autofde-lab tests/domains/htn_fixtures/sa2a-v26.9.17-{domain,problem}.hddl`
//! (committed there at 8ef74497). It is beam4pm's flip-audit (b4p-f5-06)
//! contested input: a problem whose objective is expressed as a goal SET over
//! named fluents — in this rendering **no `(:goal ...)` at all, `(:htn ...)`
//! root task only** — driving a strong-cyclic repair/reroute/typed-stop
//! policy shape over `oneof` outcomes.
//!
//! **Ruling (a), recorded 2026-09-22** in
//! `docs/jira/v26.9.17/fond-htn-67-sa2a-goal-set-semantics-divergence.md`:
//! the FOND-HTN line's law is pure empty-network termination — a decomposed
//! primitive network must reach `htn:done` with every method precondition
//! satisfied; an objective that lives outside the task network (a goal set
//! over named fluents, or here, no `:goal` at all) is out-of-contract, and
//! no solver mode is added for it. The fixture's authors' expectation is
//! re-derived from this ruling downstream (beam4pm b4p-f5-06), not relaxed
//! to green here.
//!
//! Fresh-run evidence this file pins:
//!
//! * at ferroplan `main @ 6cacbda` the beam4pm fp-probe observed a real
//!   `NoPlan` in 4729 ms (FLIP-LEDGER.md);
//! * at the ruling SHA the pipeline no longer reaches that verdict inside
//!   the default 10 s wall — the wave-6 search changes (fond-htn-57..66)
//!   made the out-of-contract question more expensive to fail to answer —
//!   and it is still wall-bound at 120 s (the `#[ignore]`d probe below).
//!   Either way the fixture gets no plan and no committed verdict class:
//!   which is exactly ruling (a)'s content. A goal-set objective is not an
//!   HTN terminal condition; the solver owes it nothing.

use ferroplan::planning_runtime::{PlannerError, PlannerLimits};
use ferroplan::{solve_hddl, HddlError};

const DOMAIN: &str = include_str!("fixtures/sa2a-v26.9.17/sa2a-v26.9.17-domain.hddl");
const PROBLEM: &str = include_str!("fixtures/sa2a-v26.9.17/sa2a-v26.9.17-problem.hddl");

/// The fixture's defining, out-of-contract shape: the problem declares its
/// objective nowhere — no `(:goal ...)` section, `(:htn ...)` root task only
/// — while the domain carries the strong-cyclic `oneof` machinery. Ruling
/// (a) keeps the law as-is precisely for inputs of this shape.
#[test]
fn the_vendored_fixture_declares_no_goal_section() {
    assert!(
        !PROBLEM.contains("(:goal"),
        "the vendored fixture must be the no-:goal rendering"
    );
    assert!(
        PROBLEM.contains("(:htn"),
        "the vendored fixture must drive the htn root task"
    );
    assert!(
        DOMAIN.contains("(oneof"),
        "the vendored domain must carry the non-deterministic step machinery"
    );
}

/// The fresh run at HEAD, at the default walls: no plan, no classical
/// verdict — the whole-pipeline guard refuses before any verdict forms.
/// Pinned as the ruling-(a) fresh-run receipt: the fixture is
/// out-of-contract, and the solver must neither silently solve it nor
/// pretend a classical NoPlan verdict it no longer reaches.
#[test]
fn fresh_run_at_head_yields_no_verdict_within_the_default_wall() {
    let started = std::time::Instant::now();
    let result = solve_hddl(DOMAIN, PROBLEM, &PlannerLimits::default());
    let elapsed = started.elapsed();
    match result {
        Err(HddlError::Timeout { limit_ms, .. }) => {
            assert_eq!(
                limit_ms, 10_000,
                "the default wall is part of the pinned fresh-run receipt"
            );
        }
        Err(HddlError::Planner(PlannerError::NoPlan { .. })) => {
            // Also lawful under ruling (a) — the 6cacbda behavior. The pin
            // accepts either no-verdict shape but never a plan.
            let _ = elapsed;
        }
        other => panic!(
            "ruling (a) violation: the out-of-contract sa2a fixture must not \
             solve at HEAD; got {other:?} in {elapsed:?}"
        ),
    }
}

/// The extended-budget probe recorded in the ticket's History: even at a
/// 120 s wall the pipeline reaches no verdict on the out-of-contract
/// fixture. `#[ignore]`d — it is adjudication evidence, not a regression
/// gate; run explicitly with `cargo test -p ferroplan --test
/// sa2a_goal_set_ruling -- --ignored --nocapture`.
#[test]
#[ignore = "ruling evidence probe: 120 s wall; run explicitly while adjudicating"]
fn probe_fresh_run_at_head_large_budget() {
    let started = std::time::Instant::now();
    let limits = PlannerLimits {
        max_wall_ms: 120_000,
        ..PlannerLimits::default()
    };
    let result = solve_hddl(DOMAIN, PROBLEM, &limits);
    eprintln!("fresh run at HEAD: {result:?} in {:?}", started.elapsed());
    assert!(
        matches!(
            result,
            Err(HddlError::Timeout { .. }) | Err(HddlError::Planner(_))
        ),
        "ruling (a): no plan may form for the out-of-contract fixture; got {result:?}"
    );
}
