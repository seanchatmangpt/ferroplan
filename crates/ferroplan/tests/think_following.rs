//! `Session::think_following` (0.29): the follow-biased, budget-stamped
//! think the WASI `session_think{prefer_follow}` op routes through, plus the
//! `replan_budgeted` -> `think` parity scan that gated switching the WASI
//! `session_think` op onto `Session::think`.
//!
//! Chicago style throughout: real parsed domains, real grounding, real
//! search; every assertion is on returned state (plans, verdicts, eval
//! counts). No doubles.

use ferroplan::api::Plan;
use ferroplan::{Options, Session, ThinkBudget, ThinkVerdict};

/// A four-room corridor: a -> b -> c -> d. The only plan is three `go`s.
const CORRIDOR_DOMAIN: &str = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))";

const CORRIDOR_PROBLEM: &str = "(define (problem corridor)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c d))
  (:goal (at d)))";

/// Same corridor with the d-room disconnected: provably unreachable.
const DEAD_END_PROBLEM: &str = "(define (problem dead-end)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c))
  (:goal (at d)))";

/// A symmetric gripper: interchangeable balls, the classic orbit shape
/// (object symmetry is exactly what `think`'s orbit canonicalization adds
/// over `replan_budgeted`).
const GRIPPER_DOMAIN: &str = "(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move
    :parameters (?from - room ?to - room)
    :precondition (at-robby ?from)
    :effect (and (at-robby ?to) (not (at-robby ?from))))
  (:action pick
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))";

const GRIPPER_PROBLEM: &str = "(define (problem gripper-4)
  (:domain gripper)
  (:objects rooma roomb - room b1 b2 b3 b4 - ball left right - gripper)
  (:init (at-robby rooma) (free left) (free right)
         (at b1 rooma) (at b2 rooma) (at b3 rooma) (at b4 rooma))
  (:goal (and (at b1 roomb) (at b2 roomb) (at b3 roomb) (at b4 roomb))))";

/// The WASI adapter's own session options (`op_session_new`): single
/// thread, production eval cap.
fn wasi_opts() -> Options {
    Options {
        threads: 1,
        max_evaluated: Some(ferroplan::ProductionLimits::default().max_evaluated),
        ..Default::default()
    }
}

fn session(domain: &str, problem: &str) -> Session {
    Session::new(domain, problem, &wasi_opts()).expect("fixture grounds")
}

fn budget(evals: usize) -> ThinkBudget {
    ThinkBudget {
        max_evaluated: Some(evals),
        wall_ms: None,
        memory_mb: Some(64),
    }
}

fn step_names(plan: &Plan) -> Vec<String> {
    plan.steps
        .iter()
        .map(|s| format!("{} {}", s.action, s.args.join(" ")).to_ascii_lowercase())
        .collect()
}

fn solved_plan(s: &Session) -> Plan {
    let think = s.think(&budget(100_000));
    assert_eq!(think.verdict, ThinkVerdict::Solved, "{:?}", think.solution);
    think.solution.plan.expect("solved think carries a plan")
}

/// Unchanged world, cursor at 0: the prior plan replays whole — zero
/// search, identical steps — while the unbiased think pays real evals.
#[test]
fn follow_replays_an_intact_plan_with_zero_search() {
    let s = session(CORRIDOR_DOMAIN, CORRIDOR_PROBLEM);
    let plan = solved_plan(&s);
    assert_eq!(plan.steps.len(), 3);

    let followed = s.think_following(&plan, 0, &budget(100_000));
    assert_eq!(followed.verdict, ThinkVerdict::Solved);
    assert!(!followed.capped);
    assert_eq!(followed.spent_evals, 0, "pure replay spends no search");
    assert_eq!(
        step_names(followed.solution.plan.as_ref().unwrap()),
        step_names(&plan)
    );

    let unbiased = s.think(&budget(100_000));
    assert!(
        unbiased.spent_evals > followed.spent_evals,
        "unbiased think {} evals must exceed follow's {}",
        unbiased.spent_evals,
        followed.spent_evals
    );
}

/// Mid-flight: the host executed step 0 and reported it (world now at b);
/// following from step 1 keeps the remaining suffix verbatim, again with
/// fewer evals than an unbiased rethink.
#[test]
fn follow_from_the_cursor_keeps_the_remaining_suffix() {
    let mut s = session(CORRIDOR_DOMAIN, CORRIDOR_PROBLEM);
    let plan = solved_plan(&s);
    s.set_fact("(at a)", false).unwrap();
    s.set_fact("(at b)", true).unwrap();

    let followed = s.think_following(&plan, 1, &budget(100_000));
    assert_eq!(followed.verdict, ThinkVerdict::Solved);
    let expected_suffix: Vec<String> = step_names(&plan)[1..].to_vec();
    assert_eq!(
        step_names(followed.solution.plan.as_ref().unwrap()),
        expected_suffix
    );
    let unbiased = s.think(&budget(100_000));
    assert!(followed.spent_evals < unbiased.spent_evals);
}

/// A prior whose first remaining step no longer applies replays nothing;
/// only the tail is searched, and it still solves.
#[test]
fn a_broken_prefix_searches_only_the_tail() {
    let mut s = session(CORRIDOR_DOMAIN, CORRIDOR_PROBLEM);
    let plan = solved_plan(&s);
    // Teleport to c: steps 0 and 1 no longer apply from here.
    s.set_fact("(at a)", false).unwrap();
    s.set_fact("(at c)", true).unwrap();
    let followed = s.think_following(&plan, 0, &budget(100_000));
    assert_eq!(followed.verdict, ThinkVerdict::Solved);
    let got = step_names(followed.solution.plan.as_ref().unwrap());
    assert_eq!(got, vec!["go c d".to_string()]);
}

/// A provably unreachable goal: the fallback think's complete search is
/// the only source of `Exhausted`.
#[test]
fn unreachable_goal_reports_exhausted_not_capped() {
    let s = session(CORRIDOR_DOMAIN, DEAD_END_PROBLEM);
    let prior = solved_plan(&session(CORRIDOR_DOMAIN, CORRIDOR_PROBLEM));
    let t = s.think_following(&prior, 0, &budget(100_000));
    assert_eq!(t.verdict, ThinkVerdict::Exhausted);
    assert!(!t.capped);
    assert!(!t.solution.solved);
    let plain = s.think(&budget(100_000));
    assert_eq!(plain.verdict, ThinkVerdict::Exhausted);
}

/// One eval of budget on a plan that needs search: capped, never
/// "unreachable".
#[test]
fn a_one_eval_budget_reports_capped() {
    let s = session(GRIPPER_DOMAIN, GRIPPER_PROBLEM);
    let empty = Plan {
        steps: Vec::new(),
        length: 0,
        metric: None,
        makespan: None,
    };
    let t = s.think_following(&empty, 0, &budget(1));
    assert_eq!(t.verdict, ThinkVerdict::Capped, "{:?}", t.solution.notes);
    assert!(t.capped);
    assert!(!t.solution.solved);
}

/// With no eval cap anywhere, `think_following` is exactly `think`.
#[test]
fn no_eval_cap_is_exactly_think() {
    let s = Session::new(
        CORRIDOR_DOMAIN,
        CORRIDOR_PROBLEM,
        &Options {
            threads: 1,
            ..Default::default()
        },
    )
    .unwrap();
    let prior = Plan {
        steps: Vec::new(),
        length: 0,
        metric: None,
        makespan: None,
    };
    let unbounded = ThinkBudget::default();
    let a = s.think_following(&prior, 0, &unbounded);
    let b = s.think(&unbounded);
    assert_eq!(a.verdict, b.verdict);
    assert_eq!(a.spent_evals, b.spent_evals);
    assert_eq!(
        step_names(a.solution.plan.as_ref().unwrap()),
        step_names(b.solution.plan.as_ref().unwrap())
    );
}

/// The parity scan that gated moving WASI `session_think` from
/// `replan_budgeted` to `think`: on every fixture here (including the
/// orbit-shaped gripper and the repo's own example domains), both paths
/// must agree on solvedness, plan length and the exact step sequence
/// (measured 2026-09-25: identical on every fixture, gripper included with
/// orbit canonicalization armed). A divergence fails here: that is the
/// gate on the switch. Each comparison is printed so the scan stays a
/// witnessed artifact.
#[test]
fn replan_budgeted_and_think_agree_on_the_fixture_corpus() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples");
    let mut cases: Vec<(String, String, String)> = vec![
        (
            "corridor".into(),
            CORRIDOR_DOMAIN.into(),
            CORRIDOR_PROBLEM.into(),
        ),
        (
            "gripper".into(),
            GRIPPER_DOMAIN.into(),
            GRIPPER_PROBLEM.into(),
        ),
    ];
    for (dir, problem) in [
        ("cabin", "crew-pair.pddl"),
        ("jobshop", "p1.pddl"),
        ("reachability", "problem.pddl"),
        ("rpg", "build-1worker.pddl"),
        ("village", "graph.pddl"),
        ("villagers", "errand.pddl"),
    ] {
        let d = std::fs::read_to_string(format!("{root}/{dir}/domain.pddl"));
        let p = std::fs::read_to_string(format!("{root}/{dir}/{problem}"));
        if let (Ok(d), Ok(p)) = (d, p) {
            cases.push((dir.to_string(), d, p));
        }
    }
    assert!(cases.len() >= 4, "fixture corpus shrank: {}", cases.len());
    let mut compared = 0;
    for (name, d, p) in &cases {
        let Ok(s) = Session::new(d, p, &wasi_opts()) else {
            continue;
        };
        let legacy = s.replan_budgeted(100_000, Some(64));
        let think = s.think(&budget(100_000));
        assert_eq!(
            legacy.solved, think.solution.solved,
            "{name}: solvedness diverged"
        );
        if let (Some(a), Some(b)) = (&legacy.plan, &think.solution.plan) {
            let orbit = think
                .solution
                .notes
                .iter()
                .any(|n| n.contains("orbit canonicalization"));
            let same = step_names(a) == step_names(b);
            eprintln!(
                "parity {name}: legacy_len={} think_len={} same_steps={same} orbit={orbit}",
                a.steps.len(),
                b.steps.len()
            );
            assert_eq!(a.steps.len(), b.steps.len(), "{name}: plan length diverged");
            assert!(same, "{name}: steps diverged (orbit armed: {orbit})");
        }
        compared += 1;
    }
    assert!(compared >= 4, "only {compared} fixtures ground");
}
