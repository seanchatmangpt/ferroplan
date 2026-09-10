//! Diagnostic harness (not part of the test suite): parse -> ground ->
//! translate the real IPC2020 blocksworld fixture (`fixtures/f`) and print
//! real numbers -- ground action/method counts, translated state/transition
//! counts (split into decomposition vs. execution moves), elapsed wall time
//! for grounding and for translating, and whether a real goal-satisfying
//! state was found in the translated state graph. `TranslateLimits::max_wall`
//! is set from the `FIXTURE_F_WALL_SECS` env var (default 30) so a run that
//! doesn't finish in time reports `TranslateError::Timeout` instead of
//! hanging, rather than requiring an external process-level timeout.
//!
//! Run: `cargo run --release --example fixture_f_stats`

use ferroplan_hddl::grounder::{self, GroundingLimits};
use ferroplan_hddl::parser;
use ferroplan_hddl::translate::{self, TranslateLimits};
use std::time::{Duration, Instant};

fn main() {
    let domain_src = include_str!("../fixtures/f/domain.hddl");
    let problem_src = include_str!("../fixtures/f/problem.hddl");

    let domain = parser::parse_domain(domain_src).expect("fixture F domain parses");
    let problem = parser::parse_problem(problem_src).expect("fixture F problem parses");
    println!(
        "parsed: {} methods, {} actions, {} tasks, {} objects",
        domain.methods.len(),
        domain.actions.len(),
        domain.tasks.len(),
        problem.objects.len()
    );

    let ground_limits = GroundingLimits::default();
    let t0 = Instant::now();
    let ir = grounder::ground(&domain, &problem, &ground_limits).expect("fixture F grounds");
    let ground_elapsed = t0.elapsed();
    println!(
        "grounded in {ground_elapsed:?}: {} ground actions, {} ground methods",
        ir.actions.len(),
        ir.methods.len()
    );

    let wall_secs: u64 = std::env::var("FIXTURE_F_WALL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let translate_limits = TranslateLimits {
        max_wall: Some(Duration::from_secs(wall_secs)),
        ..TranslateLimits::default()
    };
    println!("translating with max_wall = {wall_secs}s ...");
    let t1 = Instant::now();
    let result = translate::translate(&ir, &translate_limits);
    let translate_elapsed = t1.elapsed();

    match result {
        Ok(plan) => {
            let decompose = plan
                .transitions
                .iter()
                .filter(|t| t.action.starts_with("htn:decompose:"))
                .count();
            let exec = plan
                .transitions
                .iter()
                .filter(|t| t.action.starts_with("htn:exec:"))
                .count();
            let goal_reachable = plan
                .states
                .iter()
                .any(|s| plan.goal.facts.is_subset(&s.facts));
            println!(
                "translated in {translate_elapsed:?}: {} states, {} transitions \
                 ({decompose} decompose, {exec} exec)",
                plan.states.len(),
                plan.transitions.len()
            );
            println!("goal_reachable = {goal_reachable}");
        }
        Err(e) => {
            println!("translate FAILED after {translate_elapsed:?}: {e}");
        }
    }
}
