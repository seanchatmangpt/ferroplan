//! Differential fuzz: the committed generator's VALID draws through BOTH
//! engines, divergences ledgered (ticket `fond-htn-61-differential-fuzz`).
//!
//! The generator is the exact machinery committed for ticket fond-htn-31
//! (now in [`common`], extracted verbatim): 100 forced-VALID seeded draws
//! (`draw_valid` — the ~30% semantic-oddity mutation switch is hard off),
//! so every pair is valid-by-construction and both engines are expected to
//! consume it cleanly. No koala/corpus content is copied (KOALA POLICY):
//! the draws are self-authored in-repo; the koala oracle is invoked as an
//! external process from `/tmp` only.
//!
//! Two layers:
//!
//! 1. **Offline classification (default gate)** — all 100 pairs through
//!    `solve_hddl` under a 10 s wall, recording SOLVED / NOSOLUTION /
//!    typed-limit (the ticket's ferroplan vocabulary). Panics are findings:
//!    a VALID draw that fails to parse, a `WorkerPanicked` (disguised
//!    pipeline panic), or an `Ok` plan with `solved == false` all abort the
//!    sweep naming the seed. When the harvested ledger
//!    (`tests/fixtures/differential-fuzz/ledger.json`) exists it is pinned:
//!    the live ferroplan verdict of every pair must match the ledger's
//!    recorded ferroplan verdict, except at the wall-clock boundary — a
//!    timeout-shaped typed limit is an honest machine-load-dependent
//!    refusal, not a verdict, so it is tolerated in both directions with a
//!    printed note (the same normalization discipline as
//!    `hddl_fuzz_roundtrip.rs`'s determinism proof). A SOLVED↔NOSOLUTION
//!    flip is a behavior change and fails the gate. When the ledger does
//!    not exist yet, the test prints a skip-with-note (the ledger is
//!    manufactured by the ignored driver below) and passes on the live
//!    classification alone — the oracle leg is never exercised offline.
//! 2. **Full differential run** (`#[ignore]`d external driver) — the same
//!    100 pairs, additionally: written under `/tmp/differential-fuzz-w61/`
//!    and run through the koala oracle
//!    (`/tmp/fond-oracle/oracle-run.sh --mode flexible --timeout 30`, the
//!    T01 flock-serialized runner — never `solve.py` directly, never
//!    concurrently). Each pair is classified:
//!    * `agreement` — both engines give the same solvability verdict
//!      (SOLVED/SOLVED or NOSOLUTION/NOSOLUTION);
//!    * `divergence` — opposite solvability verdicts from two engines that
//!      both consumed the input cleanly (the interesting class; each new
//!      one is minimized and committed as an `#[ignore]`d finding, never
//!      papered over);
//!    * `incomparable` — either side gave no solvability verdict
//!      (ferroplan typed-limit, koala TIMEOUT — expected for cyclic-only
//!      domains, which koala's strong-only decision procedure cannot
//!      decide);
//!    * `oracle-rejected` — koala could not consume the input at all
//!      (PARSE_ERROR / ERROR); counts against the ≥ 80 oracle-consumable
//!      gate, never as a divergence.
//!    The full 100-pair ledger is written to
//!    `tests/fixtures/differential-fuzz/ledger.json` (oracle verdicts are
//!    facts — committing them is the point) and the run asserts
//!    ≥ 80 oracle-consumable pairs.
//!
//! Oracle verdicts are cached under `/tmp/differential-fuzz-w61/` keyed by
//! seed + mode + timeout + a hash of the exact input texts, so minimization
//! loops and re-runs of the ignored driver only pay for uncached
//! invocations (the runner's own flock serializes the real `solve.py`
//! traffic). Set `DIFFERENTIAL_FUZZ_FRESH=1` to bypass the cache for a
//! gate run.
//!
//! Gates:
//! * `cargo test -p ferroplan --test differential_fuzz` — offline; exit 0.
//! * `cargo test -p ferroplan --test differential_fuzz -- --ignored` —
//!   full run on an oracle-equipped machine, exit 0 with
//!   ≥ 80 oracle-consumable pairs.
//!
//! Provenance: generator and fixtures are self-authored seeded draws; the
//! ledger records koala oracle RESULTS (facts, committable per KOALA
//! POLICY); no koala code, grammar, or fixture text enters this repo.

mod common;

use common::{draw_valid, halve_sizes, sizes_for, Sizes};

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits, UniversalPlan};
use ferroplan_hddl::grounder::{ground, GroundingLimits};
use ferroplan_hddl::parser::{parse_domain, parse_problem};
use ferroplan_hddl::translate::{translate, PlanningProblem as TrProblem, TranslateLimits};
use ferroplan_hddl::validate::{validate_domain, validate_problem};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Differential sweep seed (ticket fond-htn-61). Pair `i` gets seed
/// `BASE_SEED + i`; every draw is independently replayable from its seed.
const BASE_SEED: u64 = 0x2026_0917_0061;
/// The ticket's pair count.
const N_CASES: usize = 100;
/// Ticket wall for the ferroplan leg.
const FP_WALL_MS: u64 = 10_000;
/// Ticket oracle contract.
const ORACLE_MODE: &str = "flexible";
const ORACLE_TIMEOUT_SECS: u32 = 30;
const ORACLE_RUNNER: &str = "/tmp/fond-oracle/oracle-run.sh";
/// Where the driver writes pairs + the verdict cache (the oracle side stays
/// in /tmp, KOALA POLICY).
const RUN_DIR: &str = "/tmp/differential-fuzz-w61";
/// The committed 100-pair ledger (manufactured by the ignored driver).
const LEDGER_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/differential-fuzz/ledger.json"
);
/// The ticket's consumability floor for the full run.
const MIN_ORACLE_CONSUMABLE: usize = 80;
/// Committed both-clean divergence findings: seeds the driver EXPECTS to
/// diverge (live regression tripwires — never silent skips). Each row must
/// have minimized fixtures under
/// `tests/fixtures/differential-fuzz/findings/` and an `#[ignore]`d
/// reproducer test below. Empty until a first finding lands.
const KNOWN_DIVERGENCES: &[(u64, &str)] = &[];

fn diff_planner_limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: FP_WALL_MS,
        ..PlannerLimits::default()
    }
}

// ---------------------------------------------------------------------------
// The 100 pairs (deterministic; identical across both layers)
// ---------------------------------------------------------------------------

struct Pair {
    i: usize,
    seed: u64,
    sizes: Sizes,
    domain_src: String,
    problem_src: String,
}

fn all_pairs() -> Vec<Pair> {
    (0..N_CASES)
        .map(|i| {
            let seed = BASE_SEED + i as u64;
            let sizes = sizes_for(seed);
            let (domain_src, problem_src) = draw_valid(seed, sizes);
            Pair {
                i,
                seed,
                sizes,
                domain_src,
                problem_src,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Ferroplan leg — the ticket's SOLVED / NOSOLUTION / typed-limit vocabulary
// ---------------------------------------------------------------------------

/// How ferroplan classified one pair.
#[derive(Clone, Debug, PartialEq, Eq)]
enum FpVerdict {
    /// `Ok` plan, `solved`, non-empty policy, outcome-closure checked
    /// against an independent re-translate whenever that re-translate
    /// succeeds under the fuzz caps.
    Solved { policy_states: usize },
    /// The honest typed no-plan answer.
    NoSolution,
    /// Any other typed refusal (wall-clock Timeout, ground/translate caps,
    /// unsupported-surface refusals). A VALID draw must never be refused
    /// for Parse (generator drift) or WorkerPanicked (disguised panic) —
    /// those are findings and panic instead.
    TypedLimit { detail: String },
}

impl FpVerdict {
    fn status(&self) -> &'static str {
        match self {
            FpVerdict::Solved { .. } => "SOLVED",
            FpVerdict::NoSolution => "NOSOLUTION",
            FpVerdict::TypedLimit { .. } => "TYPED_LIMIT",
        }
    }
    /// A solvability verdict — the only ferroplan answers that can disagree
    /// with the oracle's.
    fn solvability(&self) -> Option<&'static str> {
        match self {
            FpVerdict::Solved { .. } => Some("SOLVED"),
            FpVerdict::NoSolution => Some("NOSOLUTION"),
            FpVerdict::TypedLimit { .. } => None,
        }
    }
    /// Wall-clock refusals are machine-load-dependent, not seed-dependent:
    /// tolerated (with a printed note) in the ledger tripwire.
    fn timeout_shaped(&self) -> bool {
        match self {
            FpVerdict::TypedLimit { detail } => detail.to_lowercase().contains("timeout"),
            _ => false,
        }
    }
}

/// One `solve_hddl` call under the differential wall; returns the verdict
/// and the live wall seconds. Finding panics: generator drift
/// (`Parse` on a VALID draw), disguised panics (`WorkerPanicked`), falsified
/// `Ok` (`solved == false`), and outcome-closure violations on SOLVED.
fn fp_verdict(pair: &Pair) -> (FpVerdict, f64) {
    let seed = pair.seed;
    let started = std::time::Instant::now();
    let result = solve_hddl(&pair.domain_src, &pair.problem_src, &diff_planner_limits());
    let wall = started.elapsed().as_secs_f64();
    let verdict = match result {
        Ok(plan) => {
            assert!(
                plan.solved,
                "seed {seed}: Ok plan with solved=false — a falsified Ok is a \
                 finding, the solver's failure mode is Err(NoPlan)"
            );
            assert!(
                !plan.policy.is_empty(),
                "seed {seed}: solved plan with an empty policy"
            );
            if let Some(tp) = independent_translate(pair) {
                check_policy_outcome_closure(seed, &tp, &plan);
            } else {
                eprintln!(
                    "seed {seed}: note — independent re-translate refused under the \
                     fuzz caps; SOLVED recorded without the closure cross-check"
                );
            }
            FpVerdict::Solved {
                policy_states: plan.policy.len(),
            }
        }
        Err(e) => match e {
            HddlError::Planner(PlannerError::NoPlan) => FpVerdict::NoSolution,
            HddlError::Parse(ref msg) => panic!(
                "seed {seed}: VALID draw failed to parse inside solve_hddl \
                 (generator drifted off the SUPPORTED surface): {msg}"
            ),
            HddlError::WorkerPanicked(ref msg) => panic!(
                "seed {seed}: solve_hddl worker panicked ({msg}) — a pipeline \
                 panic in disguise, never an acceptable outcome"
            ),
            other => {
                assert!(!other.to_string().is_empty(), "seed {seed}: empty HddlError");
                FpVerdict::TypedLimit {
                    detail: other.to_string(),
                }
            }
        },
    };
    (verdict, wall)
}

/// The full parse → ground → translate pipeline (everything before the
/// solver), under the roundtrip fuzz caps — for the outcome-closure check.
/// `None` when the pipeline refuses under those (tighter-than-solver) caps.
fn independent_translate(pair: &Pair) -> Option<TrProblem> {
    let domain = parse_domain(&pair.domain_src).ok()?;
    let problem = parse_problem(&pair.problem_src).ok()?;
    let ir = ground(
        &domain,
        &problem,
        &GroundingLimits {
            max_ground_actions: 2_000,
            max_ground_methods: 2_000,
            prune_unreachable: false,
            max_wall: Some(Duration::from_millis(1_500)),
        },
    )
    .ok()?;
    translate(
        &ir,
        &TranslateLimits {
            max_task_network_depth: 32,
            max_wall: Some(Duration::from_millis(1_500)),
            max_states: Some(10_000),
        },
    )
    .ok()
}

/// Outcome closure (same contract as `hddl_fuzz_roundtrip.rs`): every
/// outcome of every policy entry lands in a policy state or a goal state.
fn check_policy_outcome_closure(seed: u64, tp: &TrProblem, plan: &UniversalPlan) {
    let facts_by_state: BTreeMap<&str, &BTreeSet<String>> =
        tp.states.iter().map(|s| (s.id.as_str(), &s.facts)).collect();
    let empty: &BTreeSet<String> = &BTreeSet::new();
    let is_goal = |state: &str| tp.goal.facts.is_subset(facts_by_state.get(state).unwrap_or(&empty));
    let in_policy: BTreeSet<&str> = plan.policy.iter().map(|e| e.state.as_str()).collect();
    for entry in &plan.policy {
        let edges: Vec<_> = tp
            .transitions
            .iter()
            .filter(|t| t.from == entry.state && t.action == entry.action)
            .collect();
        assert!(
            !edges.is_empty(),
            "seed {seed}: policy picks action '{}' in state '{}' but the \
             translated problem has no such outgoing transition",
            entry.action,
            entry.state
        );
        assert_eq!(
            edges.len(),
            entry.outcomes.len(),
            "seed {seed}: policy entry for state '{}' action '{}' lists {} \
             outcomes but the problem has {} branches — outcome set not closed",
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
// Koala oracle leg — the T01 runner, /tmp only, flock-serialized
// ---------------------------------------------------------------------------

/// The koala side's verdict for one pair.
#[derive(Clone, Debug, PartialEq, Eq)]
enum KoVerdict {
    Solved,
    NoSolution,
    Timeout,
    ParseError,
    Error,
    /// Harness absent: the leg is skipped-with-note (offline layer).
    Skipped,
}

impl KoVerdict {
    fn from_status(status: &str) -> KoVerdict {
        match status {
            "SOLVED" => KoVerdict::Solved,
            "NOSOLUTION" => KoVerdict::NoSolution,
            "TIMEOUT" => KoVerdict::Timeout,
            "PARSE_ERROR" => KoVerdict::ParseError,
            "ERROR" => KoVerdict::Error,
            other => panic!("unknown oracle status {other:?}"),
        }
    }
    fn label(&self) -> &'static str {
        match self {
            KoVerdict::Solved => "SOLVED",
            KoVerdict::NoSolution => "NOSOLUTION",
            KoVerdict::Timeout => "TIMEOUT",
            KoVerdict::ParseError => "PARSE_ERROR",
            KoVerdict::Error => "ERROR",
            KoVerdict::Skipped => "SKIPPED",
        }
    }
    /// Pairs where koala consumed the input and gave a real answer — the
    /// ≥ 80 gate counts exactly these.
    fn consumable(&self) -> bool {
        matches!(
            self,
            KoVerdict::Solved | KoVerdict::NoSolution | KoVerdict::Timeout
        )
    }
    /// A solvability verdict — the only koala answers that can disagree
    /// with ferroplan's.
    fn solvability(&self) -> Option<&'static str> {
        match self {
            KoVerdict::Solved => Some("SOLVED"),
            KoVerdict::NoSolution => Some("NOSOLUTION"),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OracleRun {
    status: String,
    wall_s: f64,
}

fn oracle_available() -> bool {
    Path::new(ORACLE_RUNNER).exists()
}

fn input_hash(domain: &str, problem: &str) -> String {
    let mut h = DefaultHasher::new();
    h.write(domain.as_bytes());
    h.write(&[0]);
    h.write(problem.as_bytes());
    format!("{:016x}", h.finish())
}

/// The verdict cache (`RUN_DIR/oracle-cache.json`): keyed by seed + mode +
/// timeout + input-text hash, so only an actual input/contract change pays
/// for a fresh oracle call. `DIFFERENTIAL_FUZZ_FRESH=1` bypasses it.
fn cache_path() -> PathBuf {
    Path::new(RUN_DIR).join("oracle-cache.json")
}

fn load_cache() -> BTreeMap<String, OracleRun> {
    if std::env::var("DIFFERENTIAL_FUZZ_FRESH").is_ok() {
        return BTreeMap::new();
    }
    std::fs::read_to_string(cache_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn store_cache(cache: &BTreeMap<String, OracleRun>) {
    std::fs::create_dir_all(RUN_DIR).expect("create /tmp run dir");
    let json = serde_json::to_string_pretty(cache).expect("serialize oracle cache");
    std::fs::write(cache_path(), json).expect("write oracle cache");
}

/// One flock-serialized oracle invocation via the T01 runner. Panics on a
/// harness failure (the runner reserves non-zero exit for exactly that).
fn oracle_invoke(dpath: &Path, ppath: &Path) -> OracleRun {
    let output = std::process::Command::new(ORACLE_RUNNER)
        .arg(dpath)
        .arg(ppath)
        .args([
            "--mode",
            ORACLE_MODE,
            "--timeout",
            &ORACLE_TIMEOUT_SECS.to_string(),
        ])
        .output()
        .unwrap_or_else(|e| panic!("spawn {ORACLE_RUNNER}: {e} (harness missing?)"));
    assert!(
        output.status.success(),
        "oracle-run.sh exited non-zero (harness failure): {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    #[derive(Deserialize)]
    struct Line {
        status: String,
        wall_s: f64,
    }
    let line: Line = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("parse oracle JSON line {stdout:?}: {e}"));
    OracleRun {
        status: line.status,
        wall_s: line.wall_s,
    }
}

/// The koala verdict for one pair, cache-aware. The pair's files must
/// already be written under `RUN_DIR` (the oracle consumes them from /tmp).
fn ko_verdict(pair: &Pair, cache: &mut BTreeMap<String, OracleRun>) -> (KoVerdict, OracleRun) {
    let key = format!(
        "{}|{}|{}|{}",
        pair.seed,
        ORACLE_MODE,
        ORACLE_TIMEOUT_SECS,
        input_hash(&pair.domain_src, &pair.problem_src)
    );
    if let Some(hit) = cache.get(&key) {
        return (KoVerdict::from_status(&hit.status), hit.clone());
    }
    let dpath = pair_path(pair, "domain.hddl");
    let ppath = pair_path(pair, "problem.hddl");
    let run = oracle_invoke(&dpath, &ppath);
    cache.insert(key, run.clone());
    (KoVerdict::from_status(&run.status), run)
}

fn pair_path(pair: &Pair, file: &str) -> PathBuf {
    Path::new(RUN_DIR).join(format!("pair-{:03}-seed-{}.{file}", pair.i, pair.seed))
}

fn write_pair_files(pair: &Pair) {
    std::fs::create_dir_all(RUN_DIR).expect("create /tmp run dir");
    std::fs::write(pair_path(pair, "domain.hddl"), &pair.domain_src)
        .expect("write domain for oracle");
    std::fs::write(pair_path(pair, "problem.hddl"), &pair.problem_src)
        .expect("write problem for oracle");
}

// ---------------------------------------------------------------------------
// Classification (the ticket's vocabulary)
// ---------------------------------------------------------------------------

fn classify(fp: &FpVerdict, ko: &KoVerdict) -> &'static str {
    match (fp, ko) {
        (_, KoVerdict::Skipped) => "oracle-skipped",
        (_, KoVerdict::ParseError) | (_, KoVerdict::Error) => "oracle-rejected",
        (FpVerdict::TypedLimit { .. }, _) => "incomparable",
        (_, KoVerdict::Timeout) => "incomparable",
        (FpVerdict::Solved { .. }, KoVerdict::Solved)
        | (FpVerdict::NoSolution, KoVerdict::NoSolution) => "agreement",
        _ => "divergence",
    }
}

/// A both-clean divergence: two engines that each consumed the input and
/// each returned a solvability verdict — disagreeing.
fn both_clean_divergence(fp: &FpVerdict, ko: &KoVerdict) -> bool {
    fp.solvability().is_some()
        && ko.solvability().is_some()
        && classify(fp, ko) == "divergence"
}

// ---------------------------------------------------------------------------
// Ledger
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PairLedger {
    i: usize,
    seed: u64,
    ferroplan: FpLedgerEntry,
    /// `None` only in the degenerate offline shape; the driver always
    /// records the oracle column (possibly SKIPPED-with-note).
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle: Option<KoLedgerEntry>,
    classification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FpLedgerEntry {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_states: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    wall_s: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct KoLedgerEntry {
    status: String,
    wall_s: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ledger {
    ticket: String,
    generated_utc: String,
    machine: String,
    generator: Value,
    config: Value,
    summary: Value,
    pairs: Vec<PairLedger>,
}

fn machine_note() -> String {
    std::process::Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn utc_now() -> String {
    std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

// ---------------------------------------------------------------------------
// Layer 1: the offline gate
// ---------------------------------------------------------------------------

/// The default gate: live ferroplan classification of all 100 VALID pairs
/// (10 s wall each), then — when the harvested ledger exists — the ledger
/// tripwire. The oracle leg is never exercised here.
#[test]
fn differential_offline_classifies_all_100() {
    let started = std::time::Instant::now();
    let pairs = all_pairs();
    let legs: Vec<(FpVerdict, f64)> = pairs.iter().map(fp_verdict).collect();
    let verdicts: Vec<&FpVerdict> = legs.iter().map(|(v, _)| v).collect();

    let solved = verdicts
        .iter()
        .filter(|v| matches!(v, FpVerdict::Solved { .. }))
        .count();
    let no_solution = verdicts
        .iter()
        .filter(|v| matches!(v, FpVerdict::NoSolution))
        .count();
    let typed_limit = N_CASES - solved - no_solution;
    eprintln!(
        "differential ferroplan leg ({} pairs, {:?}): SOLVED={solved} \
         NOSOLUTION={no_solution} TYPED_LIMIT={typed_limit}",
        pairs.len(),
        started.elapsed()
    );
    // The sweep must actually exercise the solver, not silently refuse
    // everything at the door: at least half the pairs need a decisive
    // solvability verdict.
    assert!(
        solved + no_solution >= N_CASES / 2,
        "only {}/{} pairs got a decisive ferroplan verdict (SOLVED={solved} \
         NOSOLUTION={no_solution}) — the generator is not exercising the solver",
        solved + no_solution,
        N_CASES
    );

    ledger_tripwire(&pairs, &verdicts);
}

/// When the harvested ledger exists, the live ferroplan verdicts must match
/// it: SOLVED↔NOSOLUTION flips fail; the wall-clock boundary (a recorded
/// decisive verdict vs a live timeout-shaped refusal, or vice versa) is
/// tolerated with a printed note. Missing ledger: skip-with-note
/// (manufactured by the ignored driver).
fn ledger_tripwire(pairs: &[Pair], live: &[&FpVerdict]) {
    if !Path::new(LEDGER_PATH).exists() {
        eprintln!(
            "skip-with-note: ledger not yet harvested at {LEDGER_PATH} — run \
             `cargo test -p ferroplan --test differential_fuzz -- --ignored` \
             (the external oracle driver) to manufacture it"
        );
        return;
    }
    let raw = std::fs::read_to_string(LEDGER_PATH).expect("read ledger");
    let ledger: Ledger = serde_json::from_str(&raw).expect("parse ledger");
    assert_eq!(ledger.pairs.len(), N_CASES, "ledger pair count drifted");
    assert_eq!(
        ledger.generator["base_seed"].as_u64(),
        Some(BASE_SEED),
        "ledger was harvested from a different sweep seed"
    );
    assert_eq!(
        ledger.generator["mutation"], false,
        "ledger was harvested from mutated (non-VALID) draws"
    );

    let mut tolerated = 0usize;
    for (entry, (pair, lv)) in ledger.pairs.iter().zip(pairs.iter().zip(live)) {
        assert_eq!(entry.seed, pair.seed, "ledger pair {} seed mismatch", entry.i);
        let recorded = entry.ferroplan.status.as_str();
        if recorded == lv.status() {
            continue;
        }
        // Wall-clock boundary, both directions, printed — never silent.
        let boundary = (recorded == "TYPED_LIMIT" && lv.solvability().is_some())
            || (lv.timeout_shaped() && recorded != "TYPED_LIMIT");
        if boundary {
            tolerated += 1;
            eprintln!(
                "pair {} seed {}: ledger says {recorded}, live says {} — \
                 wall-clock refusal boundary, tolerated with note",
                entry.i,
                pair.seed,
                lv.status()
            );
            continue;
        }
        panic!(
            "pair {} seed {}: ledger recorded ferroplan {recorded} but the live \
             run now says {} — a verdict flip on the same seed is a behavior \
             change, never papered over (live detail: {lv:?})",
            entry.i, pair.seed, lv.status()
        );
    }
    if tolerated > 0 {
        eprintln!("ledger tripwire: {tolerated} wall-clock-boundary pairs tolerated with note");
    }
}

// ---------------------------------------------------------------------------
// Layer 2: the ignored full driver (ferroplan + koala oracle)
// ---------------------------------------------------------------------------

/// Minimize a both-clean divergence: halve the draw's sizes while the
/// divergence still reproduces (both legs re-run per step; the koala leg
/// pays real, cache-aware oracle calls). Returns the last diverging size
/// vector.
fn minimize_divergence(seed: u64, start: Sizes, cache: &mut BTreeMap<String, OracleRun>) -> Sizes {
    let mut cur = start;
    loop {
        let next = halve_sizes(cur);
        if next == cur || !divergence_reproduces(seed, next, cache) {
            return cur;
        }
        cur = next;
    }
}

fn divergence_reproduces(
    seed: u64,
    sizes: Sizes,
    cache: &mut BTreeMap<String, OracleRun>,
) -> bool {
    let (domain_src, problem_src) = draw_valid(seed, sizes);
    let pair = Pair {
        i: usize::MAX, // minimization probe; not a ledger row
        seed,
        sizes,
        domain_src,
        problem_src,
    };
    let (fp, _) = fp_verdict(&pair);
    write_pair_files(&pair);
    let (ko, _) = ko_verdict(&pair, cache);
    both_clean_divergence(&fp, &ko)
}

fn finding_fixtures(seed: u64, sizes: Sizes) -> Vec<PathBuf> {
    let dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/differential-fuzz/findings");
    std::fs::create_dir_all(&dir).expect("create findings dir");
    let (domain, problem) = draw_valid(seed, sizes);
    let stem = format!("diff-fuzz-seed{seed}");
    let dpath = dir.join(format!("{stem}.domain.hddl"));
    let ppath = dir.join(format!("{stem}.problem.hddl"));
    std::fs::write(&dpath, domain).expect("write finding domain fixture");
    std::fs::write(&ppath, problem).expect("write finding problem fixture");
    vec![dpath, ppath]
}

/// The full differential run: 100 VALID pairs through BOTH engines, the
/// 100-pair ledger manufactured, ≥ 80 oracle-consumable asserted, both-clean
/// divergences minimized + committed as findings (never papered over).
#[test]
#[ignore = "full differential run: drives the external /tmp/fond-oracle koala \
            oracle (flock-serialized, up to ~30 min) — run with \
            `cargo test -p ferroplan --test differential_fuzz -- --ignored`"]
fn differential_full_with_oracle() {
    let started = std::time::Instant::now();
    let pairs = all_pairs();
    let mut cache = load_cache();

    // Validity pre-flight: both engines are supposed to consume these
    // cleanly, so any VALID draw that fails ferroplan's parse/validate is
    // generator drift — surface it before burning oracle wall time.
    for pair in &pairs {
        let d = parse_domain(&pair.domain_src)
            .unwrap_or_else(|e| panic!("seed {}: VALID draw failed to parse: {e}", pair.seed));
        let p = parse_problem(&pair.problem_src)
            .unwrap_or_else(|e| panic!("seed {}: VALID draw failed to parse: {e}", pair.seed));
        validate_domain(&d)
            .unwrap_or_else(|e| panic!("seed {}: VALID draw failed validation: {e}", pair.seed));
        validate_problem(&d, &p).unwrap_or_else(|e| {
            panic!("seed {}: VALID draw failed validation: {e}", pair.seed)
        });
    }

    let oracle_offline = !oracle_available();
    if oracle_offline {
        eprintln!(
            "HARNESS_UNAVAILABLE: {ORACLE_RUNNER} absent — every oracle leg is \
             skipped-with-note; the >= {MIN_ORACLE_CONSUMABLE} consumable gate \
             cannot be met on this machine"
        );
    }

    let mut ledger_pairs = Vec::with_capacity(N_CASES);
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut new_divergences: Vec<(u64, Sizes)> = Vec::new();
    for pair in &pairs {
        let (fp, fp_wall) = fp_verdict(pair);
        let (ko, ko_run) = if oracle_offline {
            (
                KoVerdict::Skipped,
                OracleRun {
                    status: "SKIPPED".to_owned(),
                    wall_s: 0.0,
                },
            )
        } else {
            write_pair_files(pair);
            ko_verdict(pair, &mut cache)
        };
        let class = classify(&fp, &ko);
        *counts.entry(class).or_insert(0) += 1;
        eprintln!(
            "pair {:03} seed {}: ferroplan={} oracle={} -> {} (fp {fp_wall:.3}s, ko {:.3}s)",
            pair.i,
            pair.seed,
            fp.status(),
            ko.label(),
            class,
            ko_run.wall_s
        );
        let known = KNOWN_DIVERGENCES.iter().find(|(s, _)| *s == pair.seed);
        match (known, both_clean_divergence(&fp, &ko)) {
            (Some((_, why)), false) => panic!(
                "seed {}: known divergence no longer reproduces ('{why}') — \
                 remove it from KNOWN_DIVERGENCES, delete its fixtures and its \
                 #[ignore]d reproducer, and append a paydown History row in the \
                 same change",
                pair.seed
            ),
            (Some((_, why)), true) => {
                eprintln!("seed {}: known divergence still reproduces: {why}", pair.seed);
            }
            (None, true) => new_divergences.push((pair.seed, pair.sizes)),
            (None, false) => {}
        }
        ledger_pairs.push(PairLedger {
            i: pair.i,
            seed: pair.seed,
            ferroplan: FpLedgerEntry {
                status: fp.status().to_owned(),
                policy_states: match &fp {
                    FpVerdict::Solved { policy_states } => Some(*policy_states),
                    _ => None,
                },
                detail: match &fp {
                    FpVerdict::TypedLimit { detail } => Some(detail.clone()),
                    _ => None,
                },
                wall_s: fp_wall,
            },
            oracle: Some(KoLedgerEntry {
                status: ko_run.status.clone(),
                wall_s: ko_run.wall_s,
                note: (ko == KoVerdict::Skipped)
                    .then(|| "oracle harness absent — leg skipped-with-note".to_owned()),
            }),
            classification: class.to_owned(),
        });
    }
    let consumable = ledger_pairs
        .iter()
        .filter(|p| {
            p.oracle
                .as_ref()
                .map(|o| KoVerdict::from_status(&o.status).consumable())
                .unwrap_or(false)
        })
        .count();
    let mut fp_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for p in &ledger_pairs {
        *fp_counts.entry(p.ferroplan.status.as_str()).or_insert(0) += 1;
    }
    let mut ko_counts: BTreeMap<String, usize> = BTreeMap::new();
    for p in &ledger_pairs {
        *ko_counts
            .entry(p.oracle.as_ref().unwrap().status.clone())
            .or_insert(0) += 1;
    }
    eprintln!(
        "differential full run ({:?}): classifications={counts:?} ferroplan={fp_counts:?} \
         oracle={ko_counts:?} consumable={consumable}/{N_CASES}",
        started.elapsed()
    );

    // The ticket's floor.
    assert!(
        consumable >= MIN_ORACLE_CONSUMABLE,
        "only {consumable}/{N_CASES} pairs were oracle-consumable \
         (>= {MIN_ORACLE_CONSUMABLE} required) — PARSE_ERROR/ERROR verdicts \
         against the generator's VALID surface are findings about \
         dialect coverage, not silent exclusions"
    );

    // New both-clean divergences are findings: minimize, commit the
    // fixtures, tripwire the sweep — never paper over.
    if !new_divergences.is_empty() {
        for (seed, sizes) in &new_divergences {
            let min_sizes = minimize_divergence(*seed, *sizes, &mut cache);
            let paths = finding_fixtures(*seed, min_sizes);
            eprintln!("NEW DIVERGENCE seed {seed}: minimized sizes {min_sizes:?}, fixtures {paths:?}");
        }
        store_cache(&cache);
        let seeds: Vec<u64> = new_divergences.iter().map(|(s, _)| *s).collect();
        panic!(
            "NEW BOTH-CLEAN DIVERGENCE(S) at seeds {seeds:?} — required in the \
             same change: commit the minimized fixtures under \
             tests/fixtures/differential-fuzz/findings/, add KNOWN_DIVERGENCES \
             rows for each seed (with the observed verdicts), add #[ignore]d \
             reproducer tests, and append History rows to \
             docs/jira/v26.9.17/fond-htn-61-differential-fuzz.md"
        );
    }

    // Manufacture the committed ledger (oracle verdicts are facts).
    let ledger = Ledger {
        ticket: "fond-htn-61-differential-fuzz".to_owned(),
        generated_utc: utc_now(),
        machine: machine_note(),
        generator: json!({
            "source": "tests/common/mod.rs (extracted verbatim from tests/hddl_fuzz_roundtrip.rs, ticket fond-htn-31)",
            "entry_point": "draw_valid (mutation switch hard-off)",
            "base_seed": BASE_SEED,
            "cases": N_CASES,
            "mutation": false,
            "provenance": "self-authored in-repo seeded draws; no koala/corpus content (KOALA POLICY)",
        }),
        config: json!({
            "ferroplan_max_wall_ms": FP_WALL_MS,
            "oracle_runner": ORACLE_RUNNER,
            "oracle_mode": ORACLE_MODE,
            "oracle_timeout_s": ORACLE_TIMEOUT_SECS,
        }),
        summary: json!({
            "classification": counts,
            "ferroplan": fp_counts,
            "oracle": ko_counts,
            "oracle_consumable": consumable,
        }),
        pairs: ledger_pairs,
    };
    let path = Path::new(LEDGER_PATH);
    std::fs::create_dir_all(path.parent().unwrap()).expect("create ledger dir");
    std::fs::write(path, serde_json::to_string_pretty(&ledger).unwrap())
        .expect("write ledger");
    eprintln!("ledger written: {LEDGER_PATH}");
}
