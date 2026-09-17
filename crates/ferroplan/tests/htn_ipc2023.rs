//! IPC-2023 deterministic HTN pipeline suite — `solve_hddl` end-to-end
//! (ticket `fond-htn-13-test-htn-ipc2023`, wave `v26.9.17-fond-htn-hardening`).
//!
//! Provenance: the fixtures under `tests/fixtures/htn-ipc2023/` are verbatim
//! copies of the 13 curated deterministic instances from
//! `/tmp/fond-corpus/htn-ipc2023/` (IPC 2023 hierarchical-track competition
//! data — canonical inputs, not koala code; see that corpus's README for the
//! selection criteria and compat-watch list). Copying competition data into
//! repo fixtures is explicitly admitted by the wave context.
//!
//! Contract under test (one `#[test]` per instance, each with a 30 s wall
//! budget): `solve_hddl` must either return a **solved plan whose policy is
//! closed** (checked in-test against the independently re-derived ground IR:
//! every non-goal state reachable from an initial state under the policy has
//! a policy entry whose outcomes exactly match the IR transition relation;
//! any reported `steps` replay as real IR edges) or fail with an **honest
//! typed error** (`NoPlan`, a limit/timeout, or a recorded parse/ground/
//! translate pipeline gap) — never panic, hang past the budget, or return
//! garbage (an `Ok` plan that is not solved, an unclosed policy, or a
//! `WorkerPanicked`).
//!
//! Per-instance results are printed as machine-readable `IPCRESULT|...`
//! lines (capture with `cargo test ... -- --nocapture`); they are the source
//! for the committed `tests/fixtures/htn-ipc2023/RESULTS.md`.
//!
//! Known-gap policy: parser gaps are *recorded verbatim* (and mirrored into
//! RESULTS.md), never fixed on this branch.

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits, PolicyEntry, UniversalPlan};
use ferroplan_hddl::translate::PlanningProblem as IrProblem;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

/// Per-instance wall budget (`solve_hddl`'s watchdog is keyed off
/// `PlannerLimits::max_wall_ms`).
const BUDGET_MS: u64 = 30_000;
/// Above this, RESULTS.md flags the instance SLOW.
const SLOW_MS: u128 = 10_000;

type Instance = (&'static str, &'static str, &'static str);

/// The 13 curated instances: (test suffix, domain source, problem source).
/// Problem-file extensions are verbatim from the corpus — note Lamps ships
/// its problem as `pfile01.pddl`, a corpus-wide quirk (compat-watch #6).
const INSTANCES: &[Instance] = &[
    (
        "blocksworld_gtohp",
        include_str!("fixtures/htn-ipc2023/Blocksworld-GTOHP/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Blocksworld-GTOHP/p01.hddl"),
    ),
    (
        "blocksworld_hpddl",
        include_str!("fixtures/htn-ipc2023/Blocksworld-HPDDL/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Blocksworld-HPDDL/pfile_005.hddl"),
    ),
    (
        "depots",
        include_str!("fixtures/htn-ipc2023/Depots/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Depots/p01.hddl"),
    ),
    (
        "factories_simple",
        include_str!("fixtures/htn-ipc2023/Factories-simple/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Factories-simple/pfile01.hddl"),
    ),
    (
        "lamps",
        include_str!("fixtures/htn-ipc2023/Lamps/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Lamps/pfile01.pddl"),
    ),
    (
        "multiarm_blocksworld",
        include_str!("fixtures/htn-ipc2023/Multiarm-Blocksworld/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Multiarm-Blocksworld/pfile_01_005.hddl"),
    ),
    (
        "pcp_1",
        include_str!("fixtures/htn-ipc2023/PCP_1/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/PCP_1/p-pcp01.hddl"),
    ),
    (
        "po_satellite",
        include_str!("fixtures/htn-ipc2023/PO_Satellite/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/PO_Satellite/1obs-1sat-1mod.hddl"),
    ),
    (
        "po_transport",
        include_str!("fixtures/htn-ipc2023/PO_Transport/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/PO_Transport/pfile01.hddl"),
    ),
    (
        "robot",
        include_str!("fixtures/htn-ipc2023/Robot/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Robot/pfile_01_001.hddl"),
    ),
    (
        "satellite_gtohp",
        include_str!("fixtures/htn-ipc2023/Satellite-GTOHP/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Satellite-GTOHP/p01.hddl"),
    ),
    (
        "towers",
        include_str!("fixtures/htn-ipc2023/Towers/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Towers/pfile_01.hddl"),
    ),
    (
        "transport",
        include_str!("fixtures/htn-ipc2023/Transport/domain.hddl"),
        include_str!("fixtures/htn-ipc2023/Transport/pfile01.hddl"),
    ),
];

fn limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: BUDGET_MS,
        ..PlannerLimits::default()
    }
}

/// How the pipeline answered. `Gap` covers honest typed refusals that are
/// recorded verbatim (parser gaps are explicitly out of scope for this
/// branch); contract violations never become an `Outcome` — they panic.
#[derive(Debug)]
enum Outcome {
    NoPlan,
    Limit {
        kind: String,
    },
    Gap {
        phase: &'static str,
    },
}

fn classify(err: &HddlError) -> Outcome {
    match err {
        HddlError::Planner(PlannerError::NoPlan) => Outcome::NoPlan,
        HddlError::Planner(PlannerError::Timeout { .. }) => Outcome::Limit {
            kind: "solver-wall".to_owned(),
        },
        HddlError::Planner(PlannerError::ResourceBound { resource, .. }) => Outcome::Limit {
            kind: format!("resource:{resource}"),
        },
        HddlError::Timeout { .. } => Outcome::Limit {
            kind: "pipeline-watchdog".to_owned(),
        },
        HddlError::Parse(_) => Outcome::Gap { phase: "parse" },
        HddlError::Ground(msg) => {
            if msg.contains("wall-clock limit exceeded") {
                Outcome::Limit {
                    kind: "ground-wall".to_owned(),
                }
            } else {
                Outcome::Gap { phase: "ground" }
            }
        }
        HddlError::Translate(msg) => {
            if msg.contains("wall-clock limit exceeded") {
                Outcome::Limit {
                    kind: "translate-wall".to_owned(),
                }
            } else {
                Outcome::Gap { phase: "translate" }
            }
        },
        // Honest typed planner-side refusal (NoMethod / HierarchyCycle / ...):
        // not "NoPlan" strictly, but an honest typed answer, not garbage —
        // recorded verbatim like the other gaps.
        HddlError::Planner(_) => Outcome::Gap {
            phase: "plan",
        },
        HddlError::WorkerPanicked(msg) => panic!(
            "CONTRACT VIOLATION: solve_hddl worker thread panicked: {msg}"
        ),
    }
}

/// Re-derive the ground IR independently of `solve_hddl`'s internals (same
/// public `ferroplan_hddl` pipeline) so the policy can be checked against
/// the actual transition relation rather than taken on faith.
fn build_ir(domain_src: &str, problem_src: &str) -> IrProblem {
    let domain = ferroplan_hddl::parser::parse_domain(domain_src).expect("IR: domain parses");
    let problem = ferroplan_hddl::parser::parse_problem(problem_src).expect("IR: problem parses");
    let grounded = ferroplan_hddl::grounder::ground(&domain, &problem, &Default::default())
        .expect("IR: grounds");
    ferroplan_hddl::translate::translate(&grounded, &Default::default()).expect("IR: translates")
}

fn assert_closed_policy(plan: &UniversalPlan, ir: &IrProblem) {
    let states: BTreeMap<&str, &ferroplan_hddl::translate::State> = ir
        .states
        .iter()
        .map(|s| (s.id.as_str(), s))
        .collect();
    let goal_facts = &ir.goal.facts;
    let is_goal =
        |id: &str| goal_facts.iter().all(|f| states[id].facts.contains(f));

    // (from, action) -> set of outcome states, straight from the IR.
    let mut edges: BTreeMap<(&str, &str), BTreeSet<&str>> = BTreeMap::new();
    for t in &ir.transitions {
        edges
            .entry((t.from.as_str(), t.action.as_str()))
            .or_default()
            .insert(t.to.as_str());
    }

    let policy: BTreeMap<&str, &PolicyEntry> = plan
        .policy
        .iter()
        .map(|e| (e.state.as_str(), e))
        .collect();

    // Closure: from every initial state, walk the policy. Every reached
    // non-goal state must have an entry; every entry's outcomes must be
    // exactly the IR outcomes of its chosen action (no invented or elided
    // outcomes); the walk then continues through all of them.
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut stack: Vec<&str> = ir.initial_states.iter().map(String::as_str).collect();
    while let Some(state) = stack.pop() {
        if !seen.insert(state) || is_goal(state) {
            continue;
        }
        let entry = policy.get(state).unwrap_or_else(|| {
            panic!(
                "POLICY NOT CLOSED: non-goal state {state:?} is reachable under the \
                 policy but has no policy entry (plan notes: {:?})",
                plan.notes
            )
        });
        let ir_outcomes = edges
            .get(&(state, entry.action.as_str()))
            .unwrap_or_else(|| {
                panic!(
                    "POLICY NOT CLOSED: state {state:?} chooses action {:?} which has \
                     no outgoing transition in the ground IR",
                    entry.action
                )
            });
        let claimed: BTreeSet<&str> =
            entry.outcomes.iter().map(|o| o.state.as_str()).collect();
        assert_eq!(
            claimed, *ir_outcomes,
            "POLICY OUTCOMES MISMATCH at state {state:?} action {:?}: plan claims \
             {claimed:?}, ground IR says {ir_outcomes:?}",
            entry.action
        );
        for next in ir_outcomes {
            if !seen.contains(next) {
                stack.push(next);
            }
        }
    }

    // Replay: any reported step must be a real IR edge, and consecutive
    // fully-labelled steps must chain.
    let ir_edges: BTreeSet<(&str, &str, &str)> = ir
        .transitions
        .iter()
        .map(|t| (t.from.as_str(), t.action.as_str(), t.to.as_str()))
        .collect();
    let mut prev_to: Option<&str> = None;
    for step in &plan.steps {
        if let (Some(from), Some(to)) = (&step.from, &step.to) {
            assert!(
                ir_edges.contains(&(from.as_str(), step.action.as_str(), to.as_str())),
                "REPLAY MISMATCH: step {:?} ({from:?} -> {to:?}) is not an edge of the \
                 ground IR",
                step.action
            );
            if let Some(prev) = prev_to {
                assert_eq!(
                    prev,
                    from.as_str(),
                    "REPLAY MISMATCH: step chain breaks at {from:?} (previous step \
                     ends at {prev:?})"
                );
            }
            prev_to = Some(to.as_str());
        }
    }
}

fn run_instance(name: &str, domain_src: &str, problem_src: &str) {
    let start = Instant::now();
    let result = solve_hddl(domain_src, problem_src, &limits());
    let wall_ms = start.elapsed().as_millis();

    match result {
        Ok(plan) => {
            assert!(
                plan.solved,
                "CONTRACT VIOLATION: solve_hddl returned Ok with solved=false \
                 (policy entries: {}, steps: {}) — garbage, not an honest answer",
                plan.policy.len(),
                plan.steps.len()
            );
            assert_closed_policy(&plan, &build_ir(domain_src, problem_src));
            let slow = if wall_ms > SLOW_MS { "SLOW" } else { "ok" };
            println!(
                "IPCRESULT|{name}|SOLVED|{wall_ms}|{}|{}|{slow}",
                plan.policy.len(),
                plan.steps.len()
            );
        }
        Err(err) => {
            let outcome = classify(&err);
            let slow = if wall_ms > SLOW_MS { "SLOW" } else { "ok" };
            let label = match &outcome {
                Outcome::NoPlan => "NOPLAN".to_owned(),
                Outcome::Limit { kind } => format!("LIMIT:{kind}"),
                Outcome::Gap { phase } => format!("GAP:{phase}"),
            };
            // Verbatim first-failure text for RESULTS.md (every non-solved
            // outcome carries one; newlines collapsed for one-line parse).
            let verbatim = err.to_string().replace(['\n', '\r'], "; ");
            println!("IPCRESULT|{name}|{label}|{wall_ms}|0|0|{slow}");
            println!("IPCERR|{name}|{label}|{verbatim}");
        }
    }
}

macro_rules! ipc_test {
    ($fn_name:ident, $key:literal) => {
        #[test]
        fn $fn_name() {
            let (name, domain, problem) = INSTANCES
                .iter()
                .find(|(n, _, _)| *n == $key)
                .expect("instance table entry");
            run_instance(name, domain, problem);
        }
    };
}

ipc_test!(ipc2023_blocksworld_gtohp, "blocksworld_gtohp");
ipc_test!(ipc2023_blocksworld_hpddl, "blocksworld_hpddl");
ipc_test!(ipc2023_depots, "depots");
ipc_test!(ipc2023_factories_simple, "factories_simple");
ipc_test!(ipc2023_lamps, "lamps");
ipc_test!(ipc2023_multiarm_blocksworld, "multiarm_blocksworld");
ipc_test!(ipc2023_pcp_1, "pcp_1");
ipc_test!(ipc2023_po_satellite, "po_satellite");
ipc_test!(ipc2023_po_transport, "po_transport");
ipc_test!(ipc2023_robot, "robot");
ipc_test!(ipc2023_satellite_gtohp, "satellite_gtohp");
ipc_test!(ipc2023_towers, "towers");
ipc_test!(ipc2023_transport, "transport");

// PCP_1 note: this is the deliberate unbounded-decomposition case
// (compat-watch #8: zero objects, recursive method cycles — Post-Correspondence
// encoding). The assertion for it is the *bounded exit itself*: a typed limit
// error (watchdog or solver wall) or `NoPlan` is acceptable; a hang (test never
// returning) or panic is not. `run_instance` already enforces exactly that
// contract for every instance, PCP_1 included.
