//! Differential test: canonical flat-FOND domains (tireworld family), each
//! hand-authored in two encodings — an explicit `PlanningProblem`
//! (`problem.json`) and an HDDL embedding (`domain.hddl` + `problem.hddl`,
//! nondeterministic effects via `oneof`, a `finish` primitive whose
//! precondition is the goal facts, and a recursive root method that leaves
//! the retry choice to the planner) — solved by BOTH engines:
//!
//! 1. ferroplan's `solve_planning_type(PlanningType::Fond)` dispatcher on the
//!    explicit encoding,
//! 2. ferroplan's `solve_hddl` (parse -> ground -> translate -> the same
//!    dispatcher) on the HDDL pair,
//! 3. the external koala oracle (`/tmp/fond-oracle/oracle-run.sh`, T01
//!    flock-serialized runner) on the HDDL pair — its observed verdicts are
//!    committed as FACTS in `fixtures/fond-flat/oracle-goldens.json` (wave
//!    KOALA POLICY: golden JSON of oracle RESULTS is committable; no koala
//!    code or files are vendored anywhere in this repo).
//!
//! All three must agree with the literature class recorded in each domain's
//! `verdict.json`:
//!
//! - `strong`: an acyclic strong policy exists (`faults-1bit`, `boolean-not`);
//! - `cyclic-only`: no strong policy exists, but a strong-cyclic one does
//!   (`tireworld`, `triangle-tireworld-3cities`, `islands-2`, `wall-2rows`,
//!   `coffee`);
//! - `unsolvable`: neither exists (`river-unsafe`).
//!
//! Solvable classes are additionally required to produce outcome-closed
//! policies: walking the policy's own outcome edges from the initial states
//! must land only on states that are goal or have a next policy entry.
//!
//! Oracle semantics finding (observed 2026-09-17, recorded in the goldens):
//! koala `flexible` (and `fixed-ld`, the wave-protocol cross-check) searches
//! finite strong plan TREES. Domains whose only solutions are cyclic
//! policies have no finite strong plan tree, so the oracle diverges: TIMEOUT
//! on a <=12-state instance is the observed, reproducible signature of
//! `cyclic-only`, while `strong` instances solve in ~0.1s and `unsolvable`
//! reports NOSOLUTION in ~0.15s. The goldens freeze these verdicts; this
//! test asserts exactly that signature against the literature class.

use ferroplan::planning_runtime::{
    solve_planning_type, Goal, PlannerError, PlannerLimits, PlanningProblem,
    UniversalPlanningRequest,
};
use ferroplan::planning_types::PlanningType;
use ferroplan::{solve_hddl, HddlError};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::PathBuf;

const DOMAINS: &[&str] = &[
    "tireworld",
    "triangle-tireworld-3cities",
    "islands-2",
    "wall-2rows",
    "faults-1bit",
    "boolean-not",
    "coffee",
    "river-unsafe",
];

/// The `notes` string `fond_policy` stamps on acyclic strong policies
/// (`planning_runtime.rs::policy_from_choices`). Matching the exact prefix
/// (not a substring) keeps the strong/cyclic distinction honest: a silent
/// change of either constant fails loudly here.
const STRONG_NOTE: &str = "strong FOND fixed point";
/// The `notes` string `fond_policy_strong_cyclic` stamps on strong-cyclic
/// policies.
const STRONG_CYCLIC_NOTE: &str = "strong-cyclic FOND fixpoint";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FondClass {
    Strong,
    CyclicOnly,
    Unsolvable,
}

fn parse_class(raw: &str) -> FondClass {
    match raw {
        "strong" => FondClass::Strong,
        "cyclic-only" => FondClass::CyclicOnly,
        "unsolvable" => FondClass::Unsolvable,
        other => panic!("unknown literature class {other:?} in a verdict.json"),
    }
}

#[derive(Deserialize)]
struct Verdict {
    #[allow(dead_code)]
    domain: String,
    literature_class: String,
    #[allow(dead_code)]
    provenance: String,
    #[allow(dead_code)]
    literature_ref: String,
    #[allow(dead_code)]
    class_rationale: String,
}

#[derive(Deserialize)]
struct Goldens {
    #[allow(dead_code)]
    description: String,
    results: BTreeMap<String, GoldenDomain>,
}

#[derive(Deserialize)]
struct GoldenDomain {
    #[allow(dead_code)]
    literature_class: String,
    flexible: GoldenRun,
    fixed_ld: Option<GoldenRun>,
}

#[derive(Deserialize)]
struct GoldenRun {
    status: String,
    #[allow(dead_code)]
    wall_s: f64,
    #[allow(dead_code)]
    artifact: String,
}

fn fixture_dir(domain: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/fond-flat")
        .join(domain)
}

fn load_verdict(domain: &str) -> Verdict {
    let raw = std::fs::read_to_string(fixture_dir(domain).join("verdict.json"))
        .unwrap_or_else(|e| panic!("{domain}: read verdict.json: {e}"));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("{domain}: parse verdict.json: {e}"))
}

fn load_goldens() -> Goldens {
    let raw = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/fond-flat/oracle-goldens.json"),
    )
    .expect("read oracle-goldens.json");
    serde_json::from_str(&raw).expect("parse oracle-goldens.json")
}

fn load_explicit_problem(domain: &str) -> PlanningProblem {
    let raw = std::fs::read_to_string(fixture_dir(domain).join("problem.json"))
        .unwrap_or_else(|e| panic!("{domain}: read problem.json: {e}"));
    let problem: PlanningProblem =
        serde_json::from_str(&raw).unwrap_or_else(|e| panic!("{domain}: parse problem.json: {e}"));
    assert!(
        (4..=12).contains(&problem.states.len()),
        "{domain}: explicit fixture must be tiny (4-12 states), got {}",
        problem.states.len()
    );
    problem
}

fn load_hddl(domain: &str) -> (String, String) {
    let domain_src = std::fs::read_to_string(fixture_dir(domain).join("domain.hddl"))
        .unwrap_or_else(|e| panic!("{domain}: read domain.hddl: {e}"));
    let problem_src = std::fs::read_to_string(fixture_dir(domain).join("problem.hddl"))
        .unwrap_or_else(|e| panic!("{domain}: read problem.hddl: {e}"));
    (domain_src, problem_src)
}

/// Classify one explicit-path run of the Fond dispatcher. The dispatcher
/// tries the acyclic strong fixpoint first and falls back to strong-cyclic
/// only when strong fails, so the stamped note IS the class evidence.
fn classify_explicit(
    domain: &str,
    problem: &PlanningProblem,
) -> (FondClass, ferroplan::planning_runtime::UniversalPlan) {
    let request = UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: problem.clone(),
        limits: PlannerLimits::default(),
    };
    match solve_planning_type(&request) {
        Ok(plan) => {
            let note = plan
                .notes
                .first()
                .unwrap_or_else(|| panic!("{domain}: explicit plan carries no solver note"));
            let class = if note.starts_with(STRONG_NOTE) {
                FondClass::Strong
            } else if note.starts_with(STRONG_CYCLIC_NOTE) {
                FondClass::CyclicOnly
            } else {
                panic!("{domain}: unrecognized solver note {note:?}")
            };
            (class, plan)
        }
        Err(PlannerError::NoPlan) => (FondClass::Unsolvable, Default::default()),
        Err(other) => panic!("{domain}: explicit Fond dispatch failed unexpectedly: {other:?}"),
    }
}

/// Classify one HDDL-path run (same dispatcher behind `solve_hddl`).
fn classify_hddl(
    domain: &str,
    domain_src: &str,
    problem_src: &str,
) -> (FondClass, ferroplan::planning_runtime::UniversalPlan) {
    match solve_hddl(domain_src, problem_src, &PlannerLimits::default()) {
        Ok(plan) => {
            let note = plan
                .notes
                .first()
                .unwrap_or_else(|| panic!("{domain}: hddl plan carries no solver note"));
            let class = if note.starts_with(STRONG_NOTE) {
                FondClass::Strong
            } else if note.starts_with(STRONG_CYCLIC_NOTE) {
                FondClass::CyclicOnly
            } else {
                panic!("{domain}: unrecognized hddl solver note {note:?}")
            };
            (class, plan)
        }
        Err(HddlError::Planner(PlannerError::NoPlan)) => {
            (FondClass::Unsolvable, Default::default())
        }
        Err(other) => panic!("{domain}: hddl pipeline failed unexpectedly: {other:?}"),
    }
}

/// Re-run the HDDL parse -> ground -> translate pipeline to obtain the flat
/// problem the HDDL path actually solves, so the outcome-closure check can
/// run over the composite state graph. Deterministic (BTreeMap interning
/// throughout `ferroplan_hddl`), so this is exactly the graph
/// `solve_hddl`'s internal pipeline produced — verified by asserting every
/// policy state exists in the mirrored problem.
fn mirrored_hddl_problem(domain_src: &str, problem_src: &str) -> PlanningProblem {
    let domain = ferroplan_hddl::parser::parse_domain(domain_src).expect("mirror: parse domain");
    let problem =
        ferroplan_hddl::parser::parse_problem(problem_src).expect("mirror: parse problem");
    let ir = ferroplan_hddl::grounder::ground(&domain, &problem, &Default::default())
        .expect("mirror: ground");
    let translated =
        ferroplan_hddl::translate::translate(&ir, &Default::default()).expect("mirror: translate");
    // Field-for-field mirror of `ferroplan::hddl::adapt_problem` for the
    // fields the closure check consumes (states/goal/transitions; the
    // adapter leaves unsafe_states empty and cost/duration are policy-
    // irrelevant).
    PlanningProblem {
        states: translated
            .states
            .iter()
            .map(|s| ferroplan::planning_runtime::State {
                id: s.id.clone(),
                facts: s.facts.clone(),
                fluents: Default::default(),
            })
            .collect(),
        initial_states: translated.initial_states.clone(),
        goal: Goal {
            facts: translated.goal.facts.clone(),
            ..Default::default()
        },
        unsafe_states: Default::default(),
        transitions: translated
            .transitions
            .iter()
            .map(|t| ferroplan::planning_runtime::Transition {
                action: t.action.clone(),
                from: t.from.clone(),
                to: t.to.clone(),
                cost: 1,
                duration: 1,
                reward: 0,
                probability_ppm: t.probability_ppm,
                observation: None,
                requires: Default::default(),
            })
            .collect(),
        ..Default::default()
    }
}

/// Outcome-closed policy check: walking ONLY the policy's own outcome edges
/// from the initial states, every reached state must either satisfy the goal
/// or carry a policy entry whose outcome list is non-empty and total. A
/// reached non-goal state without an entry is a policy hole; a goal state
/// needs no entry (the fixpoint solvers leave goal states entry-less).
fn assert_outcome_closed(
    domain: &str,
    path: &str,
    problem: &PlanningProblem,
    plan: &ferroplan::planning_runtime::UniversalPlan,
) {
    assert!(
        plan.solved,
        "{domain}/{path}: plan must be marked solved for an outcome-closure check"
    );
    assert!(
        !plan.policy.is_empty(),
        "{domain}/{path}: solvable domain must produce a non-empty policy"
    );
    let states: BTreeMap<&str, &ferroplan::planning_runtime::State> =
        problem.states.iter().map(|s| (s.id.as_str(), s)).collect();
    let policy: BTreeMap<&str, &ferroplan::planning_runtime::PolicyEntry> =
        plan.policy.iter().map(|e| (e.state.as_str(), e)).collect();
    let mut queue: VecDeque<&str> = problem.initial_states.iter().map(String::as_str).collect();
    let mut seen: BTreeSet<&str> = queue.iter().copied().collect();
    while let Some(id) = queue.pop_front() {
        let state = states.get(id).unwrap_or_else(|| {
            panic!("{domain}/{path}: policy walk reached state {id:?} absent from the problem")
        });
        if goal_holds(domain, path, &problem.goal, state) {
            continue;
        }
        let entry = policy.get(id).unwrap_or_else(|| {
            panic!("{domain}/{path}: policy HOLE — non-goal reachable state {id:?} has no entry")
        });
        assert!(
            !entry.outcomes.is_empty(),
            "{domain}/{path}: policy entry for {id:?} (action {}) has no outcomes",
            entry.action
        );
        for outcome in &entry.outcomes {
            if seen.insert(outcome.state.as_str()) {
                queue.push_back(outcome.state.as_str());
            }
        }
    }
}

/// The oracle leg of the three-way agreement, from the committed goldens.
/// Expected oracle signature per literature class (see module docs):
/// strong -> SOLVED, cyclic-only -> TIMEOUT (both flexible and fixed-ld),
/// unsolvable -> NOSOLUTION. The goldens must also record exactly the
/// flexible verdict each verdict-bearing run observed.
fn assert_oracle_signature(domain: &str, verdict: &Verdict, goldens: &Goldens) {
    let class = parse_class(&verdict.literature_class);
    let entry = goldens
        .results
        .get(domain)
        .unwrap_or_else(|| panic!("{domain}: missing from oracle-goldens.json"));
    match class {
        FondClass::Strong => {
            assert_eq!(
                entry.flexible.status, "SOLVED",
                "{domain}: oracle flexible must SOLVE a strong domain"
            );
        }
        FondClass::CyclicOnly => {
            assert_eq!(
                entry.flexible.status, "TIMEOUT",
                "{domain}: oracle flexible must diverge (no finite strong plan tree) on a cyclic-only domain"
            );
            let fixed_ld = entry.fixed_ld.as_ref().unwrap_or_else(|| {
                panic!("{domain}: cyclic-only domains need the fixed-ld cross-check recorded")
            });
            assert_eq!(
                fixed_ld.status, "TIMEOUT",
                "{domain}: oracle fixed-ld cross-check must also diverge on a cyclic-only domain"
            );
        }
        FondClass::Unsolvable => {
            assert_eq!(
                entry.flexible.status, "NOSOLUTION",
                "{domain}: oracle flexible must exhaust (NOSOLUTION) an unsolvable domain"
            );
        }
    }
}

/// Test-local mirror of `Goal::holds` (private in `planning_runtime`) for
/// the fact-only goals every fixture here uses. The numeric assertion is a
/// tripwire: if a fixture ever grows numeric goal bounds, this check must be
/// extended rather than silently ignoring them.
fn goal_holds(
    domain: &str,
    path: &str,
    goal: &Goal,
    state: &ferroplan::planning_runtime::State,
) -> bool {
    assert!(
        goal.numeric_min.is_empty() && goal.numeric_max.is_empty(),
        "{domain}/{path}: numeric goal bounds appeared; extend goal_holds"
    );
    goal.facts.is_subset(&state.facts)
}

fn differential_case(domain: &str) {
    let verdict = load_verdict(domain);
    let goldens = load_goldens();
    let literature = parse_class(&verdict.literature_class);
    assert_oracle_signature(domain, &verdict, &goldens);

    // Path 1: explicit PlanningProblem through the Fond dispatcher.
    let explicit_problem = load_explicit_problem(domain);
    let (explicit_class, explicit_plan) = classify_explicit(domain, &explicit_problem);
    assert_eq!(
        explicit_class, literature,
        "{domain}: explicit-encoding class disagrees with the literature class"
    );
    if literature != FondClass::Unsolvable {
        assert_outcome_closed(domain, "explicit", &explicit_problem, &explicit_plan);
    } else {
        assert!(
            explicit_plan.policy.is_empty(),
            "{domain}: unsolvable domain must produce no explicit policy"
        );
    }

    // Path 2: HDDL embedding through solve_hddl.
    let (domain_src, problem_src) = load_hddl(domain);
    let (hddl_class, hddl_plan) = classify_hddl(domain, &domain_src, &problem_src);
    assert_eq!(
        hddl_class, literature,
        "{domain}: HDDL-embedding class disagrees with the literature class"
    );
    if literature != FondClass::Unsolvable {
        let mirrored = mirrored_hddl_problem(&domain_src, &problem_src);
        assert_outcome_closed(domain, "hddl", &mirrored, &hddl_plan);
    } else {
        assert!(
            hddl_plan.policy.is_empty(),
            "{domain}: unsolvable domain must produce no hddl policy"
        );
    }
}

#[test]
fn tireworld_is_cyclic_only_across_both_engines() {
    differential_case("tireworld");
}

#[test]
fn triangle_tireworld_3cities_is_cyclic_only_across_both_engines() {
    differential_case("triangle-tireworld-3cities");
}

#[test]
fn islands_2_is_cyclic_only_across_both_engines() {
    differential_case("islands-2");
}

#[test]
fn wall_2rows_is_cyclic_only_across_both_engines() {
    differential_case("wall-2rows");
}

#[test]
fn faults_1bit_is_strong_across_both_engines() {
    differential_case("faults-1bit");
}

#[test]
fn boolean_not_is_strong_across_both_engines() {
    differential_case("boolean-not");
}

#[test]
fn coffee_is_cyclic_only_across_both_engines() {
    differential_case("coffee");
}

#[test]
fn river_unsafe_is_unsolvable_across_both_engines() {
    differential_case("river-unsafe");
}

#[test]
fn goldens_cover_every_fixture_domain_exactly() {
    let goldens = load_goldens();
    let golden_domains: BTreeSet<&str> = goldens.results.keys().map(String::as_str).collect();
    let fixture_domains: BTreeSet<&str> = DOMAINS.iter().copied().collect();
    assert_eq!(
        golden_domains, fixture_domains,
        "oracle-goldens.json must record verdicts for exactly the 8 fixture domains"
    );
}

/// Live re-falsification of the oracle goldens against the real external
/// harness. Ignored by default: it requires `/tmp/fond-oracle/oracle-run.sh`
/// (koala oracle, T01 flock-serialized runner) and burns real oracle wall
/// time; run explicitly with `cargo test ... -- --ignored`.
///
/// Cost control on purpose: the three fast verdicts (two SOLVED, one
/// NOSOLUTION) re-run in well under a second each, plus ONE cyclic-only
/// spot-check capped at 30s to re-demonstrate the divergence signature —
/// re-proving all five divergences on every explicit run would burn ~5
/// minutes of shared, flock-serialized oracle capacity for no extra
/// information (the goldens already froze the full sweep).
#[ignore = "requires the external /tmp/fond-oracle harness; run explicitly with --ignored"]
#[test]
fn oracle_live_reruns_match_the_golden_signature() {
    for (domain, expected) in [
        ("faults-1bit", "SOLVED"),
        ("boolean-not", "SOLVED"),
        ("river-unsafe", "NOSOLUTION"),
    ] {
        let status = live_oracle_status(domain, "flexible", 60);
        assert_eq!(
            status, expected,
            "{domain}: live oracle flexible verdict drifted from the golden signature"
        );
    }
    let status = live_oracle_status("tireworld", "flexible", 30);
    assert_eq!(
        status, "TIMEOUT",
        "tireworld: live oracle must still diverge (no finite strong plan tree) on the cyclic-only spot-check"
    );
}

/// One flock-serialized oracle invocation; returns the runner's status
/// string (the runner prints exactly one JSON line and exits 0 on any
/// planning verdict — non-zero exit is a harness failure, which panics).
fn live_oracle_status(domain: &str, mode: &str, timeout_sec: u32) -> String {
    let dir = fixture_dir(domain);
    let output = std::process::Command::new("/tmp/fond-oracle/oracle-run.sh")
        .arg(dir.join("domain.hddl"))
        .arg(dir.join("problem.hddl"))
        .args(["--mode", mode, "--timeout", &timeout_sec.to_string()])
        .output()
        .unwrap_or_else(|e| panic!("{domain}: spawn oracle-run.sh: {e} (harness missing?)"));
    assert!(
        output.status.success(),
        "{domain}: oracle-run.sh exited non-zero (harness failure): {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    #[derive(Deserialize)]
    struct Line {
        status: String,
    }
    let line: Line = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("{domain}: parse oracle JSON line {stdout:?}: {e}"));
    line.status
}
