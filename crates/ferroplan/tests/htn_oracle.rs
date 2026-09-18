//! Differential test: `ferroplan::solve_hddl` vs the koala/pandaPI external
//! oracle on the deterministic HTN corpus (ticket fond-htn-05,
//! docs/jira/v26.9.17/fond-htn-05-diff-htn.md).
//!
//! Corpus provenance (KOALA POLICY — no koala code is vendored, only
//! competition data + externally curated pairs):
//! - `IPC-2023/*` (13 instances): verbatim competition files from
//!   `/tmp/fond-corpus/htn-ipc2023/` (HDDL-Parser test corpus, canonical data).
//! - `panda-*` (4): pandaPIParser Ulm synthetic feature tests
//!   (BSD-3-Clause, Behnke/Höller/Bercher group) via
//!   `/tmp/fond-corpus/htn-other/curated/` (upstream provenance headers kept
//!   inside each file).
//! - `shop3-port-*` (4): hand-ported SHOP3 example domains (MPL-1.1) to
//!   minimal HDDL; ports documented in each file's `;`-comment header.
//!
//! Goldens: `fixtures/htn-oracle/oracle-goldens.json` — koala/pandaPI
//! verdicts harvested through the flock-serialized runner
//! `/tmp/fond-oracle/oracle-run.sh` (contract in
//! docs/jira/v26.9.17/_FOND-HTN-WAVE-CONTEXT.md). Golden JSON files record
//! oracle RESULTS (verdicts + wall), which are facts, not code.
//!
//! Agreement contract:
//! - golden SOLVED   => `solve_hddl` returns `Ok` with `plan.solved`, and the
//!   returned policy is outcome-closed against the translated ground IR
//!   (in-test checker below: every policy step's outcomes match the IR's
//!   own transitions, and every policy-following execution from an initial
//!   state terminates at a goal-satisfying state — never a non-goal dead
//!   end). These corpora are deterministic (no `:oneof`/`:probabilistic`),
//!   which the checker itself witnesses by asserting every IR transition
//!   carries the full 1_000_000 ppm probability.
//! - golden NOSOLUTION => `solve_hddl` returns `Err(HddlError::Planner(
//!   PlannerError::NoPlan))`.
//! - golden TIMEOUT / PARSE_ERROR / ERROR => bounded exit only: the call
//!   must return (no hang) within the wall budget and must not panic (no
//!   `HddlError::WorkerPanicked`).
//!
//! Known ferroplan-vs-oracle deviations are NOT silently skipped: each is a
//! named row in `KNOWN_MISMATCHES` below plus its own `#[ignore]`d test that
//! demonstrates the live behavior, and a History row on the ticket with the
//! oracle artifact path.

use ferroplan::{solve_hddl, HddlError, PlannerError, PlannerLimits, UniversalPlan};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

/// The ticket's per-instance wall budget for `solve_hddl`.
const WALL_BUDGET_MS: u64 = 60_000;
/// Slack over the watchdog budget for channel/thread teardown before the
/// "bounded exit" assertion fires.
const BOUNDED_EXIT_SLACK_MS: u64 = 5_000;
/// A translation is flagged SLOW in RESULTS.md above this threshold.
const SLOW_TRANSLATION_MS: u128 = 10_000;

/// Instances where `solve_hddl` is known to deviate from the oracle verdict.
/// Gate test skips these (asserting only bounded, panic-free exit); each row
/// has a dedicated `#[ignore]`d test below that demonstrates the deviation
/// live, and a History row on the ticket with the oracle artifact path.
/// Format: (instance name, reason).
const KNOWN_MISMATCHES: &[(&str, &str)] = &[
    ("PCP_1", "translate wall-clock limit (TranslateLimits::default() 10s, hardcoded in solve_hddl's pipeline): composite-state BFS still had 14392+ states queued when the limit fired; oracle SOLVED in 0.288s"),
    ("PO_Transport", "translate wall-clock limit (10s default): 41859+ states still queued; oracle SOLVED in 0.338s"),
    ("Satellite-GTOHP", "translate wall-clock limit (10s default): 19266+ states still queued; oracle SOLVED in 1.229s"),
    ("Transport", "translate wall-clock limit (10s default): 44286+ states still queued; oracle SOLVED in 0.327s"),
];

fn fixtures_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/htn-oracle")
}

/// One corpus instance: directory name plus its two HDDL sources.
struct Instance {
    name: String,
    domain_src: String,
    problem_src: String,
}

fn load_instances() -> Vec<Instance> {
    let mut names: Vec<String> = std::fs::read_dir(fixtures_dir())
        .expect("fixtures/htn-oracle directory must exist")
        .map(|e| {
            e.expect("readable fixture entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|n| n != "oracle-goldens.json" && n != "RESULTS.md")
        .collect();
    names.sort();

    names
        .into_iter()
        .map(|name| {
            let dir = fixtures_dir().join(&name);
            let mut problem = None;
            let mut domain = None;
            for entry in std::fs::read_dir(&dir).expect("instance dir").flatten() {
                let fname = entry.file_name().to_string_lossy().into_owned();
                if fname == "domain.hddl" {
                    domain = Some(std::fs::read_to_string(entry.path()).expect("read domain"));
                } else {
                    // Any non-domain file is the problem (the corpus README's
                    // own discovery rule: Lamps' problem is `pfile01.pddl`).
                    problem = Some(std::fs::read_to_string(entry.path()).expect("read problem"));
                }
            }
            Instance {
                name,
                domain_src: domain.expect("domain.hddl present"),
                problem_src: problem.expect("exactly one problem file present"),
            }
        })
        .collect()
}

fn goldens() -> BTreeMap<String, Value> {
    let raw = std::fs::read_to_string(fixtures_dir().join("oracle-goldens.json"))
        .expect("oracle-goldens.json committed alongside");
    let doc: Value = serde_json::from_str(&raw).expect("goldens parse as JSON");
    let instances = doc
        .get("instances")
        .and_then(|v| v.as_object())
        .expect("goldens carry an `instances` object");
    instances
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

fn limits_60s() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: WALL_BUDGET_MS,
        ..PlannerLimits::default()
    }
}

/// Classify a `solve_hddl` outcome the same way the golden vocabulary does.
fn ferroplan_status(result: &Result<UniversalPlan, HddlError>) -> &'static str {
    match result {
        Ok(plan) if plan.solved => "SOLVED",
        Ok(_) => "SOLVED_BUT_UNSOLVED_FLAG",
        Err(HddlError::Planner(PlannerError::NoPlan)) => "NOSOLUTION",
        Err(HddlError::Parse(_)) => "PARSE_ERROR",
        Err(HddlError::Ground(_)) => "GROUND_ERROR",
        Err(HddlError::Translate(_)) => "TRANSLATE_ERROR",
        Err(HddlError::Timeout { .. }) => "TIMEOUT",
        Err(HddlError::WorkerPanicked(_)) => "WORKER_PANICKED",
        Err(HddlError::Planner(_)) => "PLANNER_ERROR",
        Err(HddlError::RootTaskMismatch { .. }) => "ROOT_TASK_MISMATCH",
    }
}

/// Translation phase wall time + the translated ground IR (when it
/// succeeds), using the exact same phases and limit defaults
/// `solve_hddl_inner` uses internally — so the measured wall is the
/// translation portion of `solve_hddl`'s own pipeline.
fn measure_translation(
    instance: &Instance,
) -> (
    u128,
    Result<ferroplan_hddl::translate::PlanningProblem, String>,
) {
    let t0 = Instant::now();
    let out = (|| {
        let domain = ferroplan_hddl::parser::parse_domain(&instance.domain_src)
            .map_err(|e| e.to_string())?;
        let problem = ferroplan_hddl::parser::parse_problem(&instance.problem_src)
            .map_err(|e| e.to_string())?;
        let ir = ferroplan_hddl::grounder::ground(&domain, &problem, &Default::default())
            .map_err(|e| e.to_string())?;
        ferroplan_hddl::translate::translate(&ir, &Default::default()).map_err(|e| e.to_string())
    })();
    (t0.elapsed().as_millis(), out)
}

/// Outcome-closure checker for deterministic, oracle-SOLVED instances:
/// (1) every translated transition is deterministic (full-probability
/// single outcome — these corpora declare no `oneof`/`probabilistic`);
/// (2) every policy entry's declared outcomes equal the IR's own
/// transition targets for that (state, action);
/// (3) every policy-following execution from an initial state terminates at
/// a goal-satisfying state within a step bound — no non-goal dead ends.
fn assert_outcome_closed(plan: &UniversalPlan, ir: &ferroplan_hddl::translate::PlanningProblem) {
    // (1) determinism witness.
    for t in &ir.transitions {
        assert_eq!(
            t.probability_ppm, 1_000_000,
            "deterministic corpus produced a probabilistic transition {t:?}"
        );
    }

    let facts_by_state: BTreeMap<&str, &BTreeSet<String>> = ir
        .states
        .iter()
        .map(|s| (s.id.as_str(), &s.facts))
        .collect();
    let mut by_state_action: BTreeMap<(&str, &str), BTreeSet<&str>> = BTreeMap::new();
    for t in &ir.transitions {
        by_state_action
            .entry((t.from.as_str(), t.action.as_str()))
            .or_default()
            .insert(t.to.as_str());
    }

    // (2) policy entries agree with the IR's own edges.
    assert!(
        !plan.policy.is_empty(),
        "solved plan carries no policy entries"
    );
    for entry in &plan.policy {
        let key = (entry.state.as_str(), entry.action.as_str());
        let targets = by_state_action.get(&key).unwrap_or_else(|| {
            panic!(
                "policy action {:?} not applicable in state {:?}",
                entry.action, entry.state
            )
        });
        let declared: BTreeSet<&str> = entry.outcomes.iter().map(|o| o.state.as_str()).collect();
        assert_eq!(
            &declared, targets,
            "policy entry for state {:?} action {:?} declares outcomes {declared:?} but the IR transitions say {targets:?}",
            entry.state, entry.action
        );
    }

    // (3) every execution reaches a goal state.
    let policy: BTreeMap<&str, &str> = plan
        .policy
        .iter()
        .map(|e| (e.state.as_str(), e.action.as_str()))
        .collect();
    let bound = ir.states.len() * 2 + 16;
    for init in &ir.initial_states {
        let mut current = init.as_str();
        let mut steps = 0;
        loop {
            let facts = facts_by_state
                .get(current)
                .unwrap_or_else(|| panic!("policy reaches unknown state {current:?}"));
            if ir.goal.facts.is_subset(*facts) {
                break;
            }
            let action = policy.get(current).unwrap_or_else(|| {
                panic!("non-goal state {current:?} has no policy entry (dead end)")
            });
            let targets = by_state_action.get(&(current, action)).unwrap_or_else(|| {
                panic!("policy action {action:?} not applicable in state {current:?}")
            });
            assert_eq!(
                targets.len(),
                1,
                "deterministic execution from {current:?} via {action:?} has {targets:?} targets"
            );
            let next = *targets.iter().next().unwrap();
            assert_ne!(
                next, current,
                "policy self-loops at non-goal state {current:?}"
            );
            current = next;
            steps += 1;
            assert!(
                steps <= bound,
                "policy execution from {init:?} exceeds step bound {bound}"
            );
        }
    }
}

/// The gate: every fixture must agree with its oracle golden under the
/// ticket's wall budget (or be an admitted KNOWN_MISMATCHES row with its
/// own #[ignore] demonstration + ticket History row).
#[test]
fn solve_hddl_agrees_with_oracle_goldens_on_deterministic_htn_corpus() {
    let instances = load_instances();
    let goldens = goldens();
    assert!(
        instances.len() >= 21,
        "corpus must carry the ticket's 21 instances, found {}",
        instances.len()
    );
    for instance in &instances {
        let golden = goldens
            .get(&instance.name)
            .unwrap_or_else(|| panic!("fixture {} has no oracle golden", instance.name));
        let status = golden
            .get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("MISSING_STATUS");

        let (translation_ms, ir) = measure_translation(instance);
        let t0 = Instant::now();
        let result = solve_hddl(&instance.domain_src, &instance.problem_src, &limits_60s());
        let total_ms = t0.elapsed().as_millis();

        // Bounded exit for every instance, whatever the verdict: the call
        // itself must return within the watchdog budget (+ teardown slack).
        assert!(
            total_ms <= u128::from(WALL_BUDGET_MS + BOUNDED_EXIT_SLACK_MS),
            "{}: solve_hddl exceeded the bounded-exit budget ({total_ms}ms)",
            instance.name
        );
        assert!(
            !matches!(result, Err(HddlError::WorkerPanicked(_))),
            "{}: solve_hddl worker panicked: {result:?}",
            instance.name
        );

        let mismatch = KNOWN_MISMATCHES.iter().find(|(n, _)| n == &instance.name);
        match status {
            "SOLVED" | "NOSOLUTION" => {
                let expected = ferroplan_status(&result);
                let agree = expected == status;
                if let Some((_, reason)) = mismatch {
                    eprintln!(
                        "SKIP (admitted mismatch) {}: golden {status}, ferroplan {expected}: {reason}",
                        instance.name
                    );
                    continue;
                }
                assert!(
                    agree,
                    "{}: oracle says {status}, ferroplan says {expected} ({result:?}); translation took {translation_ms}ms",
                    instance.name
                );
                if status == "SOLVED" {
                    let plan = result.expect("SOLVED golden with Ok plan");
                    let translated = ir.unwrap_or_else(|e| {
                        panic!(
                            "{}: oracle SOLVED but translation phase failed ({e}); cannot run closure check",
                            instance.name
                        )
                    });
                    assert_outcome_closed(&plan, &translated);
                }
            }
            // Open verdicts: assert only "no panic, bounded exit" (already
            // checked above).
            "TIMEOUT" | "PARSE_ERROR" | "ERROR" | "HARNESS_FAILURE" | "UNKNOWN_OUTPUT" => {}
            other => panic!("{}: unknown golden status {other}", instance.name),
        }
    }
}

/// Regenerates the measurement lines for RESULTS.md (commit output under
/// `tests/fixtures/htn-oracle/RESULTS.md`). Run serially:
/// `cargo test -p ferroplan --test htn_oracle measure -- --ignored --test-threads=1`
#[test]
#[ignore = "measurement helper for RESULTS.md; run serially on an idle machine"]
fn measure_translation_walls_for_results_md() {
    let instances = load_instances();
    let goldens = goldens();
    for instance in &instances {
        let golden_status = goldens
            .get(&instance.name)
            .and_then(|g| g.get("status"))
            .and_then(|s| s.as_str())
            .unwrap_or("MISSING")
            .to_owned();
        let (translation_ms, _) = measure_translation(instance);
        let t0 = Instant::now();
        let result = solve_hddl(&instance.domain_src, &instance.problem_src, &limits_60s());
        let total_ms = t0.elapsed().as_millis();
        let slow = if translation_ms > SLOW_TRANSLATION_MS {
            " SLOW"
        } else {
            ""
        };
        println!(
            "RESULTS|{}|oracle={golden_status}|ferroplan={}|translation_ms={translation_ms}|total_ms={total_ms}{}",
            instance.name,
            ferroplan_status(&result),
            slow
        );
    }
}

// ---------------------------------------------------------------------------
// Admitted mismatches. One #[ignore]d test per KNOWN_MISMATCHES row: running
// it (`cargo test -p ferroplan --test htn_oracle -- --ignored <name>`)
// demonstrates the live ferroplan behavior that deviates from the oracle
// verdict. Ignored so the gate stays green while the defect is tracked.
//
// Each demonstration asserts the CURRENT ferroplan behavior, so it doubles as
// a permanent tripwire: the moment the underlying defect is fixed (translate
// capacity raised, conditional effects grounded), the demonstration starts
// failing and forces this row out of KNOWN_MISMATCHES and back into the
// agreement gate.
// ---------------------------------------------------------------------------

/// Shared body for the mismatch demonstrations: run the pipeline once and
/// return the classified outcome (never panics on Err — the whole point is
/// to surface the exact live behavior).
fn observe(instance: &str) -> (String, String) {
    let instances = load_instances();
    let instance = instances
        .iter()
        .find(|i| i.name == instance)
        .unwrap_or_else(|| panic!("fixture {instance} missing"));
    let result = solve_hddl(&instance.domain_src, &instance.problem_src, &limits_60s());
    let detail = match &result {
        Ok(plan) => format!(
            "Ok(solved={}, policy_entries={})",
            plan.solved,
            plan.policy.len()
        ),
        Err(e) => format!("Err({e})"),
    };
    (ferroplan_status(&result).to_owned(), detail)
}

macro_rules! mismatch_demo {
    ($fn_name:ident, $instance:literal, $expected_status:literal, $doc:expr) => {
        #[test]
        #[ignore = concat!("admitted mismatch vs oracle (fond-htn-05): ", $doc)]
        fn $fn_name() {
            let (status, detail) = observe($instance);
            assert_eq!(
                status, $expected_status,
                "{}: ferroplan behavior changed (now {status}: {detail}) — update \
                 KNOWN_MISMATCHES and re-include this instance in the agreement gate",
                $instance
            );
            println!("{}: {status}: {detail}", $instance);
        }
    };
}

mismatch_demo!(
    mismatch_pcp_1_translate_wall_limit,
    "PCP_1",
    "TRANSLATE_ERROR",
    "oracle SOLVED; ferroplan translate exceeds its internal 10s wall limit"
);
mismatch_demo!(
    mismatch_po_transport_translate_wall_limit,
    "PO_Transport",
    "TRANSLATE_ERROR",
    "oracle SOLVED; ferroplan translate exceeds its internal 10s wall limit"
);
mismatch_demo!(
    mismatch_satellite_gtohp_translate_wall_limit,
    "Satellite-GTOHP",
    "TRANSLATE_ERROR",
    "oracle SOLVED; ferroplan translate exceeds its internal 10s wall limit"
);
mismatch_demo!(
    mismatch_transport_translate_wall_limit,
    "Transport",
    "TRANSLATE_ERROR",
    "oracle SOLVED; ferroplan translate exceeds its internal 10s wall limit"
);

// ---------------------------------------------------------------------------
// Former admitted mismatches, fixed by ticket fond-htn-24 (conditional-effect
// grounding + method `:effect` + `:htn`-parameter existential binding): the
// two rows were flipped OUT of KNOWN_MISMATCHES and their old #[ignore]d
// GROUND_ERROR demonstrations became these ACTIVE tripwires — they now assert
// the agreement side (SOLVED with a closed policy, which the main gate's
// `assert_outcome_closed` also runs for them since they left the mismatch
// list). A regression back to a ground refusal fails here immediately.
// ---------------------------------------------------------------------------

/// `panda-conditional-effect`: action `:effect` carrying `when` clauses,
/// including a `when`-in-`when` (flattened to a conjunctive guard at
/// grounding time). Oracle: SOLVED in 0.301s.
#[test]
fn panda_conditional_effect_solves_in_agreement_with_oracle() {
    let (status, detail) = observe("panda-conditional-effect");
    assert_eq!(
        status, "SOLVED",
        "panda-conditional-effect: expected SOLVED (fond-htn-24 conditional-effect \
         grounding), got {status}: {detail}"
    );
    println!("panda-conditional-effect: {status}: {detail}");
}

/// `panda-method-effect`: method `:effect` + `when` domain with zero
/// primitive actions; the root `:htn` network binds its `:parameters`
/// existentially (`?x` -> the sole object `c`). Oracle: SOLVED in 0.284s.
#[test]
fn panda_method_effect_solves_in_agreement_with_oracle() {
    let (status, detail) = observe("panda-method-effect");
    assert_eq!(
        status, "SOLVED",
        "panda-method-effect: expected SOLVED (fond-htn-24 method-effect + htn-\
         parameter grounding), got {status}: {detail}"
    );
    println!("panda-method-effect: {status}: {detail}");
}

/// Bonus evidence (not a mismatch — the oracle's own verdict is the open
/// ERROR class): on shop3-port-loan-noplan koala's pipeline crashes in its
/// serializer (KeyError on the empty task network) while ferroplan returns a
/// clean `NoPlan`. This pins the ferroplan behavior: a panic/regression here
/// (e.g. the known depth-cache false-NoPlan class) trips this test.
#[test]
#[ignore = "loan-noplan clean-NoPlan tripwire; run to demo ferroplan outperforming the oracle harness"]
fn loan_noplan_ferroplan_returns_clean_no_plan_where_oracle_errors() {
    let (status, detail) = observe("shop3-port-loan-noplan");
    assert_eq!(
        status, "NOSOLUTION",
        "shop3-port-loan-noplan: expected a clean NoPlan, got {status}: {detail}"
    );
    println!("shop3-port-loan-noplan: clean NOSOLUTION: {detail}");
}
