//! Fuzz: seeded random HDDL generator — parse → validate → ground → translate
//! round-trip (ticket `fond-htn-31-fuzz-hddl-roundtrip`).
//!
//! A seeded, dependency-free SplitMix64 generator draws grammar-faithful
//! HDDL domain+problem pairs from the SUPPORTED surface only (see
//! `ferroplan-hddl/src/lib.rs` for that boundary): typed parameters/objects
//! over a small single-level type hierarchy, predicates, abstract tasks,
//! primitive actions with conjunctive preconditions and conjunctive or
//! top-level-`oneof` effects (branches may be empty `(and)` or overlap),
//! methods with implicit/`<`-style orderings and optional `:precondition`,
//! and problems with `:init`/`:goal`/`:htn` sections. Sizes are
//! parameterized (2–6 tasks, 2–8 actions, 2–10 objects) and a per-case
//! mutation switch (~30% of draws, from the same seeded stream) applies
//! grammatically-valid-but-semantically-odd mutations: shadowing/duplicate
//! names, unused method parameters, singleton types (declared, but with no
//! objects), and duplicate-ish method bodies.
//!
//! Every case runs parse → validate → ground → translate under small caps
//! and asserts:
//! 1. no panic (`catch_unwind` — a panic anywhere in the pipeline is a
//!    finding, never an accepted outcome);
//! 2. any `Err` is a typed error of the crate (`ParseError`,
//!    `ValidationError`, `GroundError`, `TranslateError`) with a non-empty
//!    `Display`; the pipeline's signatures enforce the typing, so the live
//!    assertion is the internal-consistency one: an explicit
//!    `validate_domain`/`validate_problem` pass must never be followed by a
//!    `GroundError::Validation` (ground runs exactly those two checks
//!    first), and a VALID (non-mutated) draw must never fail to parse or
//!    validate at all (that would be generator drift off the SUPPORTED
//!    surface, which is itself a finding about the generator);
//! 3. ~10% of cases additionally run end-to-end through
//!    `ferroplan::hddl::solve_hddl` on a bounded budget: `Ok` plans must be
//!    `solved` with a non-empty, OUTCOME-CLOSED policy (every outcome of
//!    every chosen action lands in a policy state or a goal state, checked
//!    against an independently re-run `translate` of the same case), and
//!    `Err(HddlError::WorkerPanicked(_))` — a disguised pipeline panic — is
//!    a finding; every other `HddlError` variant (including the honest
//!    `Planner(PlannerError::NoPlan)`) is a legitimate typed outcome;
//! 4. determinism: the whole sweep runs twice and the per-case outcome
//!    vectors must be identical (wall-clock `Timeout` outcomes are
//!    normalized to one token before comparison — they are honest refusals
//!    whose firing depends on machine load, not on the seed).
//!
//! Finding discipline (wave-4 rules addenda): on any unexpected panic the
//! case is minimized by halving the draw's size parameters until the panic
//! no longer reproduces, the shrunken domain+problem are written under
//! `tests/fixtures/fuzz-found/` with their seed, and the sweep fails naming
//! the seed and the fixture paths. Committed findings are recorded in
//! `KNOWN_FINDINGS` below together with an `#[ignore]`d reproducer test —
//! the sweep then asserts the known seed STILL panics (a live regression
//! tripwire), never a silent skip. There are no known findings yet; when a
//! fixture + reproducer are added, `KNOWN_FINDINGS` gains a row in the same
//! commit.
//!
//! The generator itself lives in `tests/common/` (extracted verbatim by
//! ticket fond-htn-61 so the differential fuzz reuses the same machinery);
//! this file owns the sweep, the caps, the typed-outcome assertions and the
//! finding discipline.
//!
//! Gates: `cargo test -p ferroplan --test hddl_fuzz_roundtrip` (default 500
//! cases, run twice for the determinism proof, well under the 120 s wall);
//! the full 2000-case sweep is `#[ignore]`d behind `-- --ignored`.
//!
//! Provenance: the generator, and every fixture it writes, are self-authored
//! in-repo draws — no koala/corpus content is copied (KOALA POLICY).

mod common;

use common::{draw_for, halve_sizes, mutation_of, Sizes};
use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits, UniversalPlan};
use ferroplan_hddl::grounder::{ground, GroundError, GroundingLimits};
use ferroplan_hddl::parser::{parse_domain, parse_problem, ParseError};
use ferroplan_hddl::translate::{
    translate, PlanningProblem as TrProblem, TranslateError, TranslateLimits,
};
use ferroplan_hddl::validate::{validate_domain, validate_problem, ValidationError};

use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::time::Duration;

/// Fixed sweep seed (ticket fond-htn-31, 2026-09-17). Case `i` gets seed
/// `BASE_SEED + i`, so the 2000-case sweep is a strict superset of the
/// 500-case one and every case is independently replayable from its seed.
const BASE_SEED: u64 = 0x2026_0917_0031;


// ---------------------------------------------------------------------------
// Small caps (ticket: parse -> validate -> ground -> translate under caps)
// ---------------------------------------------------------------------------

fn fuzz_ground_limits() -> GroundingLimits {
    GroundingLimits {
        max_ground_actions: 2_000,
        max_ground_methods: 2_000,
        prune_unreachable: false,
        max_wall: Some(Duration::from_millis(1_500)),
    }
}

fn fuzz_translate_limits() -> TranslateLimits {
    TranslateLimits {
        max_task_network_depth: 32,
        max_wall: Some(Duration::from_millis(1_500)),
        max_states: Some(10_000),
    }
}

fn fuzz_planner_limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 1_000,
        ..PlannerLimits::default()
    }
}

// ---------------------------------------------------------------------------
// Error variant tags (stable digest tokens, one per typed variant)
// ---------------------------------------------------------------------------

fn parse_tag(e: &ParseError) -> &'static str {
    match e {
        ParseError::Syntax(_) => "Syntax",
        ParseError::UnsupportedConstruct(_) => "UnsupportedConstruct",
        ParseError::MalformedOneof(_) => "MalformedOneof",
        ParseError::NestedProbabilisticBlock(_) => "NestedProbabilisticBlock",
        ParseError::NestingTooDeep { .. } => "NestingTooDeep",
    }
}

fn val_tag(e: &ValidationError) -> &'static str {
    match e {
        ValidationError::UnknownTaskOrAction(_) => "UnknownTaskOrAction",
        ValidationError::ArityMismatch { .. } => "ArityMismatch",
        ValidationError::UndefinedPredicate(_) => "UndefinedPredicate",
        ValidationError::UndefinedType(_) => "UndefinedType",
        ValidationError::DuplicateDefinition { .. } => "DuplicateDefinition",
        ValidationError::CyclicTypeHierarchy { .. } => "CyclicTypeHierarchy",
        ValidationError::PredicateArityMismatch { .. } => "PredicateArityMismatch",
        ValidationError::MethodHeadNotCompoundTask { .. } => "MethodHeadNotCompoundTask",
        ValidationError::UnknownMethodVariable { .. } => "UnknownMethodVariable",
        ValidationError::UndefinedOrderRef { .. } => "UndefinedOrderRef",
        ValidationError::CyclicOrdering { .. } => "CyclicOrdering",
        ValidationError::MissingTaskNetwork => "MissingTaskNetwork",
        ValidationError::UnknownConstant { .. } => "UnknownConstant",
        ValidationError::ArgumentTypeMismatch { .. } => "ArgumentTypeMismatch",
        ValidationError::NonGroundInitAtom { .. } => "NonGroundInitAtom",
        ValidationError::NonGroundRootSubtaskArg { .. } => "NonGroundRootSubtaskArg",
    }
}

fn ground_tag(e: &GroundError) -> &'static str {
    match e {
        GroundError::Validation(_) => "Validation",
        GroundError::TypeCycle(_) => "TypeCycle",
        GroundError::UnboundVariable(_) => "UnboundVariable",
        GroundError::UnsupportedPrecondition(_) => "UnsupportedPrecondition",
        GroundError::LimitExceeded(_) => "LimitExceeded",
        GroundError::UnsupportedNumericFluent(_) => "UnsupportedNumericFluent",
        GroundError::UnsupportedConstraint(_) => "UnsupportedConstraint",
        GroundError::Timeout { .. } => "TIMEOUT",
        GroundError::GoalTooDeep { .. } => "GoalTooDeep",
    }
}

fn translate_tag(e: &TranslateError) -> &'static str {
    match e {
        TranslateError::UnsupportedNegativeGoal => "UnsupportedNegativeGoal",
        TranslateError::UnsupportedGoalConnective(_) => "UnsupportedGoalConnective",
        TranslateError::UnboundVariable(_) => "UnboundVariable",
        TranslateError::TaskNetworkDepthExceeded { .. } => "TaskNetworkDepthExceeded",
        TranslateError::Timeout { .. } => "TIMEOUT",
        TranslateError::MemoryLimitExceeded { .. } => "MemoryLimitExceeded",
        TranslateError::MalformedTermEquality { .. } => "MalformedTermEquality",
        TranslateError::Ground(_) => "Ground",
    }
}

fn hddl_tag(e: &HddlError) -> &'static str {
    match e {
        HddlError::Parse(_) => "Parse",
        HddlError::Ground(_) => "Ground",
        HddlError::Translate(_) => "Translate",
        HddlError::Planner(_) => "Planner",
        HddlError::RootTaskMismatch { .. } => "RootTaskMismatch",
        HddlError::Timeout { .. } => "TIMEOUT",
        HddlError::WorkerPanicked(_) => "WorkerPanicked",
    }
}

fn planner_tag(e: &PlannerError) -> &'static str {
    match e {
        PlannerError::EmptyInitialState => "EmptyInitialState",
        PlannerError::UnknownState { .. } => "UnknownState",
        PlannerError::InvalidProbabilityMass { .. } => "InvalidProbabilityMass",
        PlannerError::ResourceBound { .. } => "ResourceBound",
        PlannerError::NoPlan => "NoPlan",
        PlannerError::HierarchyCycle { .. } => "HierarchyCycle",
        PlannerError::UnknownTask { .. } => "UnknownTask",
        PlannerError::NoMethod { .. } => "NoMethod",
        PlannerError::WorkflowCycle => "WorkflowCycle",
        PlannerError::WipBoundExceeded { .. } => "WipBoundExceeded",
        PlannerError::CapabilityUncovered { .. } => "CapabilityUncovered",
        PlannerError::AuthorityUnbound { .. } => "AuthorityUnbound",
        PlannerError::VerifierUnbound { .. } => "VerifierUnbound",
        PlannerError::ReceiptUnbound { .. } => "ReceiptUnbound",
        PlannerError::InvalidRdfProjection { .. } => "InvalidRdfProjection",
        PlannerError::Timeout { .. } => "TIMEOUT",
    }
}

// ---------------------------------------------------------------------------
// Per-case pipeline
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
struct CaseOutcome {
    seed: u64,
    mutated: bool,
    /// First stage that answered: "dparse" | "pparse" | "validate" |
    /// "ground" | "translate" | "translate-ok" | "known-finding".
    stage: &'static str,
    /// Error variant tag, or "states=N;trans=M" for a translated model.
    tag: String,
    /// "" when not sampled; "solve:ok:policy=N" / "solve:err:<tag>" otherwise.
    solve: String,
}

/// Run one case through parse -> validate -> ground -> translate (+ ~10%
/// through solve_hddl). Panics (any `assert!` here, or any pipeline panic)
/// are findings, caught by the sweep's `catch_unwind`.
fn run_case(seed: u64, sizes: Sizes, solve: bool) -> CaseOutcome {
    let (domain_src, problem_src) = draw_for(seed, sizes);
    let mutated = mutation_of(seed);
    let outcome = |stage: &'static str, tag: String| CaseOutcome {
        seed,
        mutated,
        stage,
        tag,
        solve: String::new(),
    };

    let domain = match parse_domain(&domain_src) {
        Ok(d) => d,
        Err(e) => {
            assert!(!e.to_string().is_empty(), "seed {seed}: empty ParseError");
            // A grammar-faithful generator must always parse: a parse error
            // on a VALID draw means generator drift off the SUPPORTED
            // surface — that is a finding about the generator.
            assert!(
                mutated,
                "seed {seed}: VALID draw failed to parse (generator drifted off \
                 the SUPPORTED surface): {e}"
            );
            return outcome("dparse", parse_tag(&e).to_owned());
        }
    };
    let problem = match parse_problem(&problem_src) {
        Ok(p) => p,
        Err(e) => {
            assert!(!e.to_string().is_empty(), "seed {seed}: empty ParseError");
            assert!(
                mutated,
                "seed {seed}: VALID draw failed to parse (generator drifted off \
                 the SUPPORTED surface): {e}"
            );
            return outcome("pparse", parse_tag(&e).to_owned());
        }
    };

    if let Err(e) = validate_domain(&domain) {
        assert!(
            mutated,
            "seed {seed}: VALID draw failed domain validation (generator drift): {e}"
        );
        assert!(!e.to_string().is_empty(), "seed {seed}: empty ValidationError");
        return outcome("validate", val_tag(&e).to_owned());
    }
    if let Err(e) = validate_problem(&domain, &problem) {
        assert!(
            mutated,
            "seed {seed}: VALID draw failed problem validation (generator drift): {e}"
        );
        assert!(!e.to_string().is_empty(), "seed {seed}: empty ValidationError");
        return outcome("validate", val_tag(&e).to_owned());
    }

    let ir = match ground(&domain, &problem, &fuzz_ground_limits()) {
        Ok(ir) => ir,
        Err(e) => {
            // internal consistency: ground re-runs exactly
            // validate_domain + validate_problem first; both just passed.
            assert!(
                !matches!(e, GroundError::Validation(_)),
                "seed {seed}: explicit validation passed but ground refused with \
                 GroundError::Validation ({e}) — the pipeline's validation front \
                 door and its caller disagree"
            );
            return outcome("ground", ground_tag(&e).to_owned());
        }
    };
    let tp = match translate(&ir, &fuzz_translate_limits()) {
        Ok(tp) => tp,
        Err(e) => {
            assert!(!e.to_string().is_empty(), "seed {seed}: empty TranslateError");
            return outcome("translate", translate_tag(&e).to_owned());
        }
    };
    let mut outcome = outcome(
        "translate-ok",
        format!("states={};trans={}", tp.states.len(), tp.transitions.len()),
    );

    if solve {
        outcome.solve = solve_sample(seed, &domain_src, &problem_src, &tp);
    }
    outcome
}

/// The ~10% end-to-end sample: `solve_hddl` on a bounded budget, asserting
/// typed outcomes and outcome-closed policies.
fn solve_sample(seed: u64, domain_src: &str, problem_src: &str, tp: &TrProblem) -> String {
    match solve_hddl(domain_src, problem_src, &fuzz_planner_limits()) {
        Err(e) => {
            assert!(
                !matches!(e, HddlError::WorkerPanicked(_)),
                "seed {seed}: solve_hddl worker panicked ({e:?}) — a pipeline panic \
                 in disguise, never an acceptable outcome"
            );
            assert!(!e.to_string().is_empty(), "seed {seed}: empty HddlError");
            if let HddlError::Planner(pe) = &e {
                format!("solve:err:Planner:{}", planner_tag(pe))
            } else {
                format!("solve:err:{}", hddl_tag(&e))
            }
        }
        Ok(plan) => {
            assert!(
                plan.solved,
                "seed {seed}: Ok plan with solved=false — the solver's failure mode \
                 is Err(NoPlan), never a falsified Ok"
            );
            assert!(
                !plan.policy.is_empty(),
                "seed {seed}: solved plan with an empty policy"
            );
            check_policy_outcome_closure(seed, tp, &plan);
            format!("solve:ok:policy={}", plan.policy.len())
        }
    }
}

/// Outcome closure: for every policy entry, the translated problem must
/// actually carry the chosen action out of that state, every branch outcome
/// of that choice must be listed, and every outcome state must itself be a
/// policy state or a goal state.
fn check_policy_outcome_closure(seed: u64, tp: &TrProblem, plan: &UniversalPlan) {
    let facts_by_state: BTreeMap<&str, &BTreeSet<String>> =
        tp.states.iter().map(|s| (s.id.as_str(), &s.facts)).collect();
    let empty: &BTreeSet<String> = &BTreeSet::new();
    let is_goal =
        |state: &str| tp.goal.facts.is_subset(facts_by_state.get(state).unwrap_or(&empty));
    let in_policy: BTreeSet<&str> = plan.policy.iter().map(|e| e.state.as_str()).collect();
    for entry in &plan.policy {
        let edges: Vec<_> = tp
            .transitions
            .iter()
            .filter(|t| t.from == entry.state && t.action == entry.action)
            .collect();
        assert!(
            !edges.is_empty(),
            "seed {seed}: policy picks action '{}' in state '{}' but the translated \
             problem has no such outgoing transition",
            entry.action,
            entry.state
        );
        assert_eq!(
            edges.len(),
            entry.outcomes.len(),
            "seed {seed}: policy entry for state '{}' action '{}' lists {} outcomes \
             but the problem has {} branches — outcome set is not closed",
            entry.state,
            entry.action,
            entry.outcomes.len(),
            edges.len()
        );
        for t in edges {
            assert!(
                in_policy.contains(t.to.as_str()) || is_goal(&t.to),
                "seed {seed}: policy not outcome-closed: state '{}' action '{}' \
                 outcome state '{}' is neither a policy state nor a goal state",
                entry.state,
                entry.action,
                t.to
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Minimizer + fixture writer (finding discipline)
// ---------------------------------------------------------------------------

/// Known, committed findings: seeds the sweep EXPECTS to panic (a live
/// regression tripwire — never a silent skip). Each row must have matching
/// fixtures under `tests/fixtures/fuzz-found/` and an `#[ignore]`d
/// reproducer test below. Empty until a first finding lands.
const KNOWN_FINDINGS: &[(u64, &str)] = &[];

fn case_panics(seed: u64, sizes: Sizes, solve: bool) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let _ = run_case(seed, sizes, solve);
    }))
    .is_err()
}

/// Halve the draw's sizes until the panic no longer reproduces; return the
/// last reproducing size vector (with its exact draw text — the committed
/// artifact).
fn minimize(seed: u64, start: Sizes, solve: bool) -> (Sizes, (String, String)) {
    let mut best_sizes = start;
    let mut best_text = draw_for(seed, start);
    let mut cur = start;
    loop {
        let next = halve_sizes(cur);
        if next == cur {
            break;
        }
        if case_panics(seed, next, solve) {
            cur = next;
            best_sizes = next;
            best_text = draw_for(seed, next);
        } else {
            break;
        }
    }
    (best_sizes, best_text)
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fuzz-found")
}

fn write_finding_fixtures(seed: u64, sizes: Sizes) -> Vec<PathBuf> {
    let dir = fixtures_dir();
    std::fs::create_dir_all(&dir).expect("create tests/fixtures/fuzz-found");
    let (domain, problem) = draw_for(seed, sizes);
    let stem = format!("hddl-roundtrip-seed{seed}");
    let dpath = dir.join(format!("{stem}.domain.hddl"));
    let ppath = dir.join(format!("{stem}.problem.hddl"));
    std::fs::write(&dpath, domain).expect("write finding domain fixture");
    std::fs::write(&ppath, problem).expect("write finding problem fixture");
    vec![dpath, ppath]
}

// ---------------------------------------------------------------------------
// The sweeps
// ---------------------------------------------------------------------------

fn run_sweep(n_cases: usize, label: &'static str) -> Vec<CaseOutcome> {
    let mut outcomes = Vec::with_capacity(n_cases);
    for i in 0..n_cases {
        let seed = BASE_SEED + i as u64;
        let sizes = common::sizes_for(seed);
        // ~10% end-to-end solve sample, deterministic in the case index.
        let solve = i % 10 == 0;
        let known = KNOWN_FINDINGS
            .iter()
            .find(|(s, _)| *s == seed)
            .map(|(_, w)| *w);
        let result = catch_unwind(AssertUnwindSafe(|| run_case(seed, sizes, solve)));
        let outcome = match (known, result) {
            (Some(why), Err(_)) => {
                // Known finding: it MUST still reproduce. This is the
                // tripwire row, not a skip.
                eprintln!("known fuzz finding seed {seed} still reproduces: {why}");
                CaseOutcome {
                    seed,
                    mutated: mutation_of(seed),
                    stage: "known-finding",
                    tag: "PANIC".to_owned(),
                    solve: String::new(),
                }
            }
            (Some(why), Ok(_)) => panic!(
                "known fuzz finding seed {seed} no longer reproduces ('{why}') — \
                 remove it from KNOWN_FINDINGS, delete its fixtures and its \
                 #[ignore]d reproducer, and append a paydown History row in the \
                 same change"
            ),
            (None, Ok(outcome)) => outcome,
            (None, Err(payload)) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "<opaque panic payload>".to_owned());
                let (min_sizes, _) = minimize(seed, sizes, solve);
                let paths = write_finding_fixtures(seed, min_sizes);
                panic!(
                    "NEW FUZZ FINDING in sweep '{label}': seed {seed} (mutated: {}) \
                     panicked: {msg}\nminimized sizes: {min_sizes:?}\nfixtures \
                     written: {paths:?}\nrequired in the same change: commit the \
                     fixtures, add a KNOWN_FINDINGS row for seed {seed}, add an \
                     #[ignore]d reproducer test, append a History row to \
                     docs/jira/v26.9.17/fond-htn-31-fuzz-hddl-roundtrip.md",
                    mutation_of(seed)
                );
            }
        };
        outcomes.push(outcome);
    }
    outcomes
}

/// Collapse wall-clock TIMEOUT tags before comparing runs: an honest refusal
/// whose firing depends on machine load is not part of the determinism proof
/// (the generator and every non-timeout outcome must still match exactly).
fn normalized(outcomes: &[CaseOutcome]) -> Vec<CaseOutcome> {
    outcomes
        .iter()
        .map(|o| CaseOutcome {
            seed: o.seed,
            mutated: o.mutated,
            stage: o.stage,
            tag: if o.tag == "TIMEOUT" {
                "TIMEOUT(normalized)".to_owned()
            } else {
                o.tag.clone()
            },
            solve: if o.solve == "solve:err:TIMEOUT" {
                "solve:TIMEOUT(normalized)".to_owned()
            } else {
                o.solve.clone()
            },
        })
        .collect()
}

const TIMEOUT_TAG: &str = "TIMEOUT(normalized)";
const TIMEOUT_SOLVE: &str = "solve:TIMEOUT(normalized)";

/// Pairwise determinism comparison. Everything must match exactly EXCEPT
/// pairs where either run hit a wall-clock budget (tag or solve watchdog):
/// those outcomes depend on machine load, not on the seed, so the honest
/// determinism claim is "identical modulo wall-clock refusals".
fn assert_deterministic(run_a: &[CaseOutcome], run_b: &[CaseOutcome]) {
    assert_eq!(
        run_a.len(),
        run_b.len(),
        "runs of the same sweep produced different case counts"
    );
    let a = normalized(run_a);
    let b = normalized(run_b);
    for (x, y) in a.iter().zip(b.iter()) {
        assert_eq!(x.seed, y.seed);
        assert_eq!(x.mutated, y.mutated, "seed {}: mutation flag differs", x.seed);
        if x.tag == TIMEOUT_TAG || y.tag == TIMEOUT_TAG || x.solve == TIMEOUT_SOLVE || y.solve == TIMEOUT_SOLVE {
            continue; // wall-clock boundary: tolerated in both directions
        }
        assert_eq!(
            (x.stage, x.tag.as_str(), x.solve.as_str()),
            (y.stage, y.tag.as_str(), y.solve.as_str()),
            "seed {}: same seed twice produced different outcomes — the \
             generator (or the pipeline) is not deterministic",
            x.seed
        );
    }
}

/// The gate: 500 seeded cases (70/30 valid/mutated from each case's own
/// stream), parse -> validate -> ground -> translate under small caps, ~10%
/// through solve_hddl with outcome-closed-policy checks; run TWICE for the
/// generator determinism proof. Budget: well under the 120 s wall.
#[test]
fn roundtrip_sweep_500_default() {
    let started = std::time::Instant::now();
    let run_a = run_sweep(500, "default-500-a");
    let run_b = run_sweep(500, "default-500-b");
    assert_deterministic(&run_a, &run_b);
    let translated = run_a.iter().filter(|o| o.stage == "translate-ok").count();
    let solved = run_a
        .iter()
        .filter(|o| o.solve.starts_with("solve:ok"))
        .count();
    let no_plan = run_a
        .iter()
        .filter(|o| o.solve.starts_with("solve:err:Planner:NoPlan"))
        .count();
    let sampled = run_a.iter().filter(|o| !o.solve.is_empty()).count();
    eprintln!(
        "roundtrip sweep (500, {:?}): translated-ok={translated}/{}; solve \
         sampled={sampled} (solved={solved}, honest-NoPlan={no_plan})",
        started.elapsed(),
        run_a.len()
    );
    // The sweep must actually exercise the pipeline's depth, not silently
    // refuse everything at the door: at least half the draws must reach a
    // translated model.
    assert!(
        translated * 2 >= run_a.len(),
        "only {translated}/{} draws reached translate — the generator is not \
         exercising the round-trip",
        run_a.len()
    );
    // Sampled cases are exactly the every-10th indices that reached
    // translate-ok (earlier-stage refusals have no plan to solve).
    let sampled_expected = (0..500)
        .step_by(10)
        .filter(|&i| run_a[i].stage == "translate-ok")
        .count();
    assert_eq!(
        sampled, sampled_expected,
        "solve sample must be exactly the every-10th translate-ok case"
    );
}

/// The full sweep: 2000 deterministic cases, same contract, run twice.
#[test]
#[ignore = "full 2000-case fuzz sweep — run with `cargo test -p ferroplan \
            --test hddl_fuzz_roundtrip -- --ignored`"]
fn roundtrip_sweep_2000_full() {
    let started = std::time::Instant::now();
    let run_a = run_sweep(2000, "full-2000-a");
    let run_b = run_sweep(2000, "full-2000-b");
    assert_deterministic(&run_a, &run_b);
    let translated = run_a.iter().filter(|o| o.stage == "translate-ok").count();
    eprintln!(
        "roundtrip sweep (2000, {:?}): translated-ok={translated}/{}",
        started.elapsed(),
        run_a.len()
    );
}
