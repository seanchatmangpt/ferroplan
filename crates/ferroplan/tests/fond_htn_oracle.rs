//! Differential: ferroplan `solve_hddl` vs the koala-planner oracle on
//! FOND-HTN (ticket fond-htn-04-diff-fondhtn, wave v26.9.17).
//!
//! Three layers live here:
//!
//! 1. **Agreement tests (in-repo fixtures)** — the four hand-authored
//!    Transport/Childsnack/Snake/Satellite-pattern fixture pairs under
//!    `fixtures/fond-htn/` are run through `solve_hddl` and their outcome
//!    must match the `ferroplan` field recorded in `oracle-goldens.json`
//!    (harvested facts: the oracle verdict comes from the koala-planner
//!    pandaPI pipeline via `/tmp/fond-oracle/oracle-run.sh`, the ferroplan
//!    verdict from the live run). `SOLVED` entries additionally get an
//!    **outcome-closure** check: every outcome state of every policy entry
//!    must itself be goal-holding or covered by another policy entry.
//! 2. **External-corpus tests** (`#[ignore]`d) — the 6 usable koala domains
//!    are read from absolute `/tmp` paths at TEST time only (KOALA POLICY:
//!    no koala files in any repo). Each asserts only when the files exist
//!    (skip-with-note otherwise) and never panics on parse gaps: a ferroplan
//!    parse failure is recorded in the goldens as
//!    `{"ferroplan": "parse-gap", "reason": "<verbatim error>"}` and the
//!    test asserts that gap classification stays true.
//! 3. **`ORACLE_MISMATCH_*` tests** (`#[ignore]`d) — one per harvested
//!    semantic disagreement. The goldens keep both sides' verdicts; these
//!    tests pin the *ferroplan* side live (the oracle side is the committed
//!    fact) so any behavior change on either reconciliation path trips.
//!
//! ## Compatibility matrix (parser grammar vs koala-dialect constructs)
//!
//! Evidence symbols are `crates/ferroplan-hddl/src/parser.rs` items at base
//! `d2faf4d`; every construct used by the 6 usable koala domains
//! (Transport, Childsnack, Snake, Satellite, Depots, Rover):
//!
//! | construct (koala use) | class | evidence |
//! |---|---|---|
//! | `:ordered-subtasks` (Childsnack, Snake, Depots, Rover; incl. empty `:ordered-subtasks( )`) | SUPPORTED | `parse_task_network` (`.get(":ordered-subtasks")`), sequential edges synthesized via `windows(2)`; `parse_subtasks` empty-list → `vec![]` |
//! | `:tasks` as `:htn` network key (Rover pfile01 `:tasks (and …)`) | SUPPORTED | `parse_task_network` (`.get(":subtasks").or_else(.get(":tasks"))`) |
//! | `(:task …)` domain sections (all 6) | SUPPORTED | `parse_domain` `":task"` arm → `parse_task_def` |
//! | bare subtask lists, no `(and …)` wrapper (Snake `:htn :subtasks (hunt)`) | SUPPORTED | `parse_subtasks` — unwrapped list = single entry; bare unlabeled calls get synthetic `t{i}` ids |
//! | method `:precondition`, incl. `not`, `forall` (Snake, Childsnack, Depots, Rover) | SUPPORTED | `parse_method_def` (`map.get(":precondition")` → full `GoalDesc` grammar via `parse_goal`); grounded into `grounder::GroundMethod::precondition` and enforced by `translate` before offering a decomposition branch |
//! | `=` term-equality in preconditions (Snake `move-base`, `strike`; micro-recurse) | SUPPORTED (parse, validate, evaluation) | `parse_goal` folds `=` into `GoalDesc::Atom`; `check_undefined_predicates` exempts `=` (arity 2); `grounder::ground_goal` lowers it to `GroundGoal::Eq` — state-independent term equality over ground constants (`evaluate_ground_goal`/`relaxed_satisfiable`: `Eq(x, y) ⇒ x == y`). `when`-conditions fold it statically (a constantly-false equality pins the never-present `never:` condition marker); `:goal` folds it at DNF time (ticket fond-htn-21; healed the harvested `micro-recurse` mismatch) |
//! | method `:constraints` (Satellite methods; Rover `:htn :constraints ()`) | PARSE-SUPPORTED, SEMANTICS IGNORED | `keyed_map` accepts any `:keyword` and `parse_method_def`/`parse_htn` never read `:constraints` — no `ConstraintDef` is produced from methods, so the constraint can never influence search (no error, silent drop) |
//! | `oneof` at top level of an action `:effect` (all 6 domains) | SUPPORTED | `parse_effect(…, allow_oneof = true)` from `parse_action_def`; `oneof` arm |
//! | `oneof` empty branch (`()` koala-dialect / `(and)`) (Transport, Snake) | SUPPORTED | empty sexp → `Effect::Empty` in `parse_effect`; both lex as an empty effect branch |
//! | `oneof` overlapping branches (Childsnack `(served ?c)` in both branches) | SUPPORTED (parse + semantics) | branches are arbitrary `Effect`s, no disjointness required; `translate` splits outcomes into probability-weighted transitions |
//! | `oneof` nested in `oneof`/`when` | UNSUPPORTED (loud) | `ParseError::MalformedOneof` in `parse_effect` (`allow_oneof` gate) |
//! | `:goal` alongside `:htn` in a problem (Depots p01) | SUPPORTED | `parse_problem` has both `":goal"` and `":htn"` arms; `translate` conjoins `:goal` DNF clauses with the `htn:done` root-completion marker |
//! | typing: `:types` hierarchies, `?v - t` params, `:constants` (all 6) | SUPPORTED | `parse_types`/`parse_typed_group`/`parse_typed_params`, `":constants"` arm |
//! | `:requirements` flags (`:method-preconditions`, `:universal-preconditions`, `:equality`, …) | ACCEPTED, IGNORED | `":requirements"` arm is `{}` — never an error |
//! | `:ordering` with explicit `<` operator (Transport, Satellite) | SUPPORTED | `parse_order_edges` accepts `(< a b)` and `(a < b)` |
//!
//! Not exercised by these 6 domains but refused loudly by the same parser:
//! `:functions` and `:durative-action` (`ParseError::UnsupportedConstruct`).
//!
//! ## Harvested disagreements (see goldens `note` fields + ticket History)
//!
//! Semantic (`ORACLE_MISMATCH_*` tests below):
//! - `micro-drop-retry` (+ Transport, same shape): koala "flexible" allows
//!   re-decomposition after a `oneof` outcome that dead-ends the task
//!   network; ferroplan at this base treats the exhausted network + unsat
//!   `:goal` as a dead terminal → `NoPlan`. Owned upstream by frozen branch
//!   `fix/oneof-koala-semantics` (tip `4d40f99`, NOT merged into this base).
//! - `micro-sense`: koala computes **strong** (acyclic) plans and refuses
//!   the calibrate/retry loop (NOSOLUTION under `flexible` AND `fixed-ld`);
//!   ferroplan's `fond_policy` strong fixpoint closes the fair loop →
//!   `SOLVED`. Both sound under their own semantics.
//! - `micro-recurse`: positive `=` term-equality never evaluated true in
//!   `evaluate_ground_goal` (parse/validate accepted it; evaluation did not)
//!   → ferroplan `NoPlan` on a deterministically solvable problem. HEALED by
//!   ticket fond-htn-21: the mismatch `#[ignore]`d pin was promoted to the
//!   always-on `micro_recurse_agrees_with_oracle` agreement test below.
//!
//! Resource-limit divergences (no semantic verdict on the ferroplan side at
//! `PlannerLimits::default()`; recorded in the goldens as `"error"` with the
//! verbatim reason, no mismatch test): koala-Transport and koala-Childsnack
//! exceed the 10 s default `max_wall_ms` inside `solve_hddl`'s translate
//! BFS; koala-Snake exceeds the default `max_ground_methods` (10000).
//!
//! Agreement: micro-overlap, koala-Satellite, koala-Depots, koala-Rover
//! (Depots' and Rover's `oneof` failure branches do NOT dead-end their task
//! networks, so no re-decomposition divergence surfaces there).

// The `ORACLE_MISMATCH_*` test names are mandated verbatim by ticket
// fond-htn-04 scope 5; they are deliberately not snake_case.
#![allow(non_snake_case)]

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits, UniversalPlan};
use ferroplan_hddl::grounder;
use ferroplan_hddl::parser::{parse_domain, parse_problem};
use ferroplan_hddl::translate::{self, PlanningProblem, TranslateLimits};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const GOLDENS_RAW: &str = include_str!("fixtures/fond-htn/oracle-goldens.json");
const FIXTURE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fond-htn");

// ---------------------------------------------------------------------------
// Classification helpers
// ---------------------------------------------------------------------------

/// How ferroplan classified one domain+problem pair, in the goldens'
/// status vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    /// `Ok(plan)` with `plan.solved` and a non-empty (closed) policy —
    /// maps to golden status `"SOLVED"`.
    Solved,
    /// `Err(HddlError::Planner(PlannerError::NoPlan))` — the typed no-plan
    /// answer, maps to golden status `"NOSOLUTION"`.
    NoPlan,
    /// `Err(HddlError::Parse(_))` — recorded in goldens as
    /// `{"ferroplan": "parse-gap", "reason": …}`.
    ParseGap(String),
    /// Any other `Err` (ground/translate/planner limits, timeouts) —
    /// recorded as `"error"` with a reason.
    Error(String),
    /// `Ok(plan)` that is neither solved nor a typed `NoPlan` — should not
    /// happen (an unsolved request must surface as `NoPlan`); kept distinct
    /// so it can never masquerade as agreement.
    UnsolvedOk(Vec<String>),
}

fn classify(domain_src: &str, problem_src: &str) -> Outcome {
    match solve_hddl(domain_src, problem_src, &PlannerLimits::default()) {
        Ok(plan) if plan.solved && !plan.policy.is_empty() => Outcome::Solved,
        Ok(plan) => Outcome::UnsolvedOk(plan.notes),
        Err(HddlError::Planner(PlannerError::NoPlan)) => Outcome::NoPlan,
        Err(e @ HddlError::Parse(_)) => Outcome::ParseGap(e.to_string()),
        Err(e) => Outcome::Error(e.to_string()),
    }
}

/// The full public parse → ground → translate pipeline (everything before
/// the FOND solver), for the outcome-closure check.
fn translated_problem(
    domain_src: &str,
    problem_src: &str,
) -> Result<PlanningProblem, String> {
    let domain = parse_domain(domain_src).map_err(|e| e.to_string())?;
    let problem = parse_problem(problem_src).map_err(|e| e.to_string())?;
    let ir = grounder::ground(&domain, &problem, &Default::default())
        .map_err(|e| e.to_string())?;
    translate::translate(&ir, &TranslateLimits::default()).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Outcome-closure checker (in-test, per ticket scope 4)
// ---------------------------------------------------------------------------

/// A solved plan's policy must be **outcome-closed**: every outcome state of
/// every policy entry either satisfies the translated problem's goal (which
/// includes the `htn:done` root-completion marker) or is itself covered by a
/// policy entry of its own; every policy entry must list at least one
/// outcome; and every initial state must be goal-holding or covered.
fn assert_outcome_closure(plan: &UniversalPlan, problem: &PlanningProblem) {
    let facts_by_id: BTreeMap<&str, &BTreeSet<String>> = problem
        .states
        .iter()
        .map(|s| (s.id.as_str(), &s.facts))
        .collect();
    let is_goal = |state: &str| {
        facts_by_id
            .get(state)
            .map(|facts| problem.goal.facts.is_subset(facts))
            .unwrap_or(false)
    };
    let covered: BTreeSet<&str> = plan.policy.iter().map(|e| e.state.as_str()).collect();

    for entry in &plan.policy {
        assert!(
            !entry.outcomes.is_empty(),
            "policy entry for state {} carries zero outcomes",
            entry.state
        );
        for outcome in &entry.outcomes {
            assert!(
                is_goal(&outcome.state) || covered.contains(outcome.state.as_str()),
                "outcome closure violated: policy entry (state {}, action {}) has \
                 outcome state {} that is neither goal-holding nor covered by any \
                 policy entry",
                entry.state,
                entry.action,
                outcome.state
            );
        }
    }
    for initial in &problem.initial_states {
        assert!(
            is_goal(initial) || covered.contains(initial.as_str()),
            "outcome closure violated: initial state {initial} is neither \
             goal-holding nor covered by any policy entry"
        );
    }
}

// ---------------------------------------------------------------------------
// Golden access helpers
// ---------------------------------------------------------------------------

fn goldens() -> Vec<Value> {
    let parsed: Value = serde_json::from_str(GOLDENS_RAW).expect("oracle-goldens.json parses");
    parsed["runs"].as_array().cloned().expect("goldens `runs` array")
}

fn golden_string<'a>(run: &'a Value, key: &str) -> &'a str {
    run[key].as_str().unwrap_or_else(|| {
        panic!("golden run {} is missing string field `{key}`", run["id"])
    })
}

/// The live ferroplan outcome for one golden run's files, read from
/// `dir` (fixture dir for in-repo runs, `None` for absolute external paths).
fn outcome_for_run(run: &Value, fixture_dir: Option<&Path>) -> Outcome {
            let (domain_field, problem_field) = match fixture_dir {
                    Some(_) => ("domain", "problem"),
                    None => ("domain_path", "problem_path"),
                };
    let domain_path = match fixture_dir {
        Some(dir) => dir.join(golden_string(run, domain_field)),
        None => std::path::PathBuf::from(golden_string(run, domain_field)),
    };
    let problem_path = match fixture_dir {
        Some(dir) => dir.join(golden_string(run, problem_field)),
        None => std::path::PathBuf::from(golden_string(run, problem_field)),
    };
    let domain_src = std::fs::read_to_string(&domain_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", domain_path.display()));
    let problem_src = std::fs::read_to_string(&problem_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", problem_path.display()));
    classify(&domain_src, &problem_src)
}

/// Assert the live outcome matches what the goldens recorded for ferroplan,
/// and that `SOLVED` entries are outcome-closed.
fn assert_matches_golden(run: &Value, live: &Outcome, fixture_dir: Option<&Path>) {
    let id = golden_string(run, "id");
    let recorded = golden_string(run, "ferroplan");
    match recorded {
        "parse-gap" => {
            assert!(
                matches!(live, Outcome::ParseGap(_)),
                "{id}: goldens record a ferroplan parse-gap but live outcome is {live:?}; \
                 the recorded gap classification no longer holds"
            );
            assert!(
                run.get("reason").and_then(Value::as_str).is_some(),
                "{id}: parse-gap golden entry must carry a verbatim `reason`"
            );
        }
        "error" => {
            assert!(
                matches!(live, Outcome::Error(_)),
                "{id}: goldens record a ferroplan error but live outcome is {live:?}"
            );
        }
        "SOLVED" => {
            assert!(
                matches!(live, Outcome::Solved),
                "{id}: goldens record SOLVED but live outcome is {live:?}"
            );
            if let Outcome::Solved = live {
                let (domain_field, problem_field) = match fixture_dir {
                    Some(_) => ("domain", "problem"),
                    None => ("domain_path", "problem_path"),
                };
                let domain_path = match fixture_dir {
                    Some(dir) => dir.join(golden_string(run, domain_field)),
                    None => std::path::PathBuf::from(golden_string(run, domain_field)),
                };
                let problem_path = match fixture_dir {
                    Some(dir) => dir.join(golden_string(run, problem_field)),
                    None => std::path::PathBuf::from(golden_string(run, problem_field)),
                };
                let domain_src = std::fs::read_to_string(&domain_path).unwrap();
                let problem_src = std::fs::read_to_string(&problem_path).unwrap();
                let problem = translated_problem(&domain_src, &problem_src)
                    .unwrap_or_else(|e| panic!("{id}: re-translate for closure check failed: {e}"));
                let plan = solve_hddl(&domain_src, &problem_src, &PlannerLimits::default())
                    .unwrap_or_else(|e| panic!("{id}: expected a solved plan, got {e:?}"));
                assert_outcome_closure(&plan, &problem);
            }
        }
        "NOSOLUTION" => {
            assert!(
                matches!(live, Outcome::NoPlan),
                "{id}: goldens record NOSOLUTION but live outcome is {live:?}"
            );
        }
        other => panic!("{id}: unknown golden ferroplan status `{other}`"),
    }
}

// ---------------------------------------------------------------------------
// Layer 1: in-repo fixture agreement
// ---------------------------------------------------------------------------

#[test]
fn in_repo_fixtures_match_recorded_ferroplan_outcomes() {
    let dir = Path::new(FIXTURE_DIR);
    for run in goldens().iter().filter(|r| golden_string(r, "kind") == "in-repo") {
        let live = outcome_for_run(run, Some(dir));
        assert_matches_golden(run, &live, Some(dir));
    }
}

/// Ledger consistency: where the goldens claim agreement, both recorded
/// statuses must be equal; where they claim disagreement, both
/// `ORACLE_MISMATCH` sides must differ — and every disagreement must have a
/// dedicated `#[ignore]`d `ORACLE_MISMATCH_*` test in this file.
#[test]
fn goldens_agreement_ledger_is_consistent() {
    let runs = goldens();
    assert_eq!(runs.len(), 10, "4 in-repo fixtures + 6 koala domains");
    assert_eq!(
        runs.iter().filter(|r| golden_string(r, "kind") == "in-repo").count(),
        4
    );
    assert_eq!(
        runs.iter().filter(|r| golden_string(r, "kind") == "external").count(),
        6
    );
    for run in &runs {
        let id = golden_string(run, "id");
        let oracle = run["oracle"]["status"].as_str().unwrap_or_else(|| {
            panic!("{id}: golden missing oracle status")
        });
        assert!(
            matches!(oracle, "SOLVED" | "NOSOLUTION" | "ERROR" | "TIMEOUT" | "PARSE_ERROR"),
            "{id}: oracle status `{oracle}` outside the runner vocabulary"
        );
        let ferroplan = golden_string(run, "ferroplan");
        let agree = run["agreement"].as_bool().unwrap_or_else(|| {
            panic!("{id}: golden missing boolean `agreement`")
        });
        assert_eq!(
            agree,
            oracle == ferroplan,
            "{id}: agreement={agree} but oracle={oracle} ferroplan={ferroplan}"
        );
        if !agree {
            let divergence = run["divergence"].as_str();
            if divergence != Some("semantic") {
                // Resource-limit divergences have no ferroplan semantic
                // verdict to pin — the golden `reason` carries them.
                continue;
            }
            // Rust idents cannot carry `-`; golden ids map hyphens to `_`
            // when they become `ORACLE_MISMATCH_*` fn names here.
            let expected_test = format!(
                "ORACLE_MISMATCH_{}",
                id.trim_start_matches("koala-").replace('-', "_")
            );
            let source = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fond_htn_oracle.rs"
            ));
            assert!(
                source.contains(&format!("fn {expected_test}")),
                "{id}: disagreement recorded without `fn {expected_test}` in this file"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Layer 2: external koala corpus (#[ignore]d, absolute /tmp paths at runtime)
// ---------------------------------------------------------------------------

/// Shared body for the 6 external-corpus cases: skip-with-note when the
/// koala files are absent, never panic on parse gaps.
fn external_corpus_case(id: &str) {
    let run = goldens()
        .into_iter()
        .find(|r| golden_string(r, "id") == id)
        .unwrap_or_else(|| panic!("golden run {id} missing"));
    let domain_path = Path::new(golden_string(&run, "domain_path"));
    let problem_path = Path::new(golden_string(&run, "problem_path"));
    if !domain_path.exists() || !problem_path.exists() {
        eprintln!(
            "skip-with-note: external corpus for {id} unavailable \
             ({} / {} missing) — KOALA POLICY keeps these outside the repo",
            domain_path.display(),
            problem_path.display()
        );
        return;
    }
    let live = outcome_for_run(&run, None);
    assert_matches_golden(&run, &live, None);
}

#[test]
#[ignore = "external corpus: reads koala domains from /tmp/fond-review/domains at test time"]
fn external_corpus_koala_transport() {
    external_corpus_case("koala-Transport");
}

#[test]
#[ignore = "external corpus: reads koala domains from /tmp/fond-review/domains at test time"]
fn external_corpus_koala_childsnack() {
    external_corpus_case("koala-Childsnack");
}

#[test]
#[ignore = "external corpus: reads koala domains from /tmp/fond-review/domains at test time"]
fn external_corpus_koala_snake() {
    external_corpus_case("koala-Snake");
}

#[test]
#[ignore = "external corpus: reads koala domains from /tmp/fond-review/domains at test time"]
fn external_corpus_koala_satellite() {
    external_corpus_case("koala-Satellite");
}

#[test]
#[ignore = "external corpus: reads koala domains from /tmp/fond-review/domains at test time"]
fn external_corpus_koala_depots() {
    external_corpus_case("koala-Depots");
}

#[test]
#[ignore = "external corpus: reads koala domains from /tmp/fond-review/domains at test time"]
fn external_corpus_koala_rover() {
    external_corpus_case("koala-Rover");
}

// ---------------------------------------------------------------------------
// Layer 3: harvested semantic disagreements (#[ignore]d per ticket scope 5)
// ---------------------------------------------------------------------------

/// DISAGREEMENT (drop-retry shape): koala "flexible" **re-decomposes** after
/// the empty-branch outcome of `drop` (task network exhausted, package still
/// in the truck) and finds a policy; ferroplan at this base advances the
/// task frontier past `drop` on BOTH outcomes, so the empty branch lands in
/// a terminal state where `:goal` is unsatisfiable → typed `NoPlan`.
/// Owned by frozen branch `fix/oneof-koala-semantics` (tip `4d40f99`, not
/// merged here); oracle side recorded in `oracle-goldens.json`
/// (`micro-drop-retry`: SOLVED, 0.306 s).
#[test]
#[ignore = "recorded oracle mismatch: koala flexible re-decomposition vs ferroplan dead-terminal (fix/oneof-koala-semantics)"]
fn ORACLE_MISMATCH_micro_drop_retry() {
    let dir = Path::new(FIXTURE_DIR);
    let run = goldens()
        .into_iter()
        .find(|r| golden_string(r, "id") == "micro-drop-retry")
        .expect("micro-drop-retry golden");
    let live = outcome_for_run(&run, Some(dir));
    assert_matches_golden(&run, &live, Some(dir));
    assert_eq!(
        golden_string(&run, "ferroplan"),
        "NOSOLUTION",
        "mismatch healed: if ferroplan now agrees with the oracle (SOLVED), \
         update the goldens + close this test"
    );
    assert_eq!(live, Outcome::NoPlan);
}

/// DISAGREEMENT (strong vs strong-cyclic): `micro-sense`'s calibrate/retry
/// loop is a fair strong-cyclic solution — ferroplan's `fond_policy` fixpoint
/// closes it (SOLVED, outcome-closed). koala computes **strong** (acyclic)
/// plans and correctly refuses the unbounded retry loop (NOSOLUTION under
/// both `flexible` and the `fixed-ld` cross-check). Neither side is wrong;
/// the semantics differ. Also documents: koala's grounder panics on the
/// bare-`:precondition (not …)` dialect (`no entry found for key`,
/// task_defs.rs:30) — the fixture uses `(and (not …))`.
#[test]
#[ignore = "recorded oracle mismatch: koala strong (acyclic) semantics vs ferroplan strong-cyclic fixpoint"]
fn ORACLE_MISMATCH_micro_sense() {
    let dir = Path::new(FIXTURE_DIR);
    let run = goldens()
        .into_iter()
        .find(|r| golden_string(r, "id") == "micro-sense")
        .expect("micro-sense golden");
    let live = outcome_for_run(&run, Some(dir));
    assert_matches_golden(&run, &live, Some(dir));
    assert_eq!(golden_string(&run, "ferroplan"), "SOLVED");
    assert_eq!(live, Outcome::Solved);
    assert_eq!(
        run["oracle"]["status"].as_str(),
        Some("NOSOLUTION"),
        "if koala ever accepts the retry loop, update the goldens + close this test"
    );
}

/// DISAGREEMENT (`=` evaluation gap), healed by ticket fond-htn-21: at base
/// `d2faf4d` the parser and validator accepted the built-in `=` term-equality
/// predicate but `grounder::evaluate_ground_goal` had no `=` arm, so
/// `micro-recurse`'s m-arrived method (`:precondition (= ?s ?goal)`) was
/// never offered and every `travel` decomposition dead-ended → typed
/// `NoPlan` while the oracle SOLVED. The fix lowers ground `=` to
/// `GroundGoal::Eq` — state-independent term equality over ground constants
/// — in the precondition, method-condition, `when`-condition, and `:goal`
/// paths, so the deterministic fixture now solves. Promoted from an
/// `#[ignore]`d `ORACLE_MISMATCH_*` pin to this always-on agreement test:
/// ferroplan must keep agreeing with the oracle (SOLVED, outcome-closed)
/// from here on.
#[test]
fn micro_recurse_agrees_with_oracle() {
    let dir = Path::new(FIXTURE_DIR);
    let run = goldens()
        .into_iter()
        .find(|r| golden_string(r, "id") == "micro-recurse")
        .expect("micro-recurse golden");
    let live = outcome_for_run(&run, Some(dir));
    assert_matches_golden(&run, &live, Some(dir));
    assert_eq!(live, Outcome::Solved);
    assert_eq!(
        run["oracle"]["status"].as_str(),
        Some("SOLVED"),
        "if koala ever stops solving micro-recurse, re-harvest the goldens"
    );
}
