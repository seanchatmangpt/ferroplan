//! IPC-2023 full-domain sweep — `solve_hddl` staged benchmark
//! (ticket `fond-htn-27-bench-ipc-full-sweep`, wave `v26.9.17-fond-htn-hardening`).
//!
//! Provenance: the fixtures under `tests/fixtures/ipc-sweep/` are verbatim
//! IPC-2023 hierarchical-track competition data — `domain.hddl` plus the
//! first-listed problem for each of the 43 domains — copied from the vendored
//! IPC corpus per the wave KOALA POLICY (competition data, not koala code;
//! copying instances into repo fixtures is explicitly admitted). Committed
//! footprint ≈ 1.2 MB, under the ticket's 2 MB cap.
//!
//! Two tests:
//!
//! - `sampled_heartbeat` (always-on, the ticket's CI heartbeat): the 6
//!   fastest SOLVED domains of the committed full-sweep run (see RESULTS.md
//!   totals), each required to still end-to-end solve within 10 s, the whole
//!   test within 30 s.
//! - `full_sweep_writes_results` (`#[ignore]`d long-run — run with
//!   `cargo test -p ferroplan --test ipc_sweep -- --ignored`): all 43
//!   domains, staged parse → ground → translate → solve, printing
//!   machine-readable `IPCRESULT|...` lines and writing
//!   `tests/fixtures/ipc-sweep/RESULTS.md`.
//!
//! Stage budgets (the ticket's staged spec, mapped onto the pipeline's real
//! bounds):
//!
//! - **parse**: 10 s outer wall. The parser has no internal cap (see
//!   `solve_hddl`'s own docs), so the outer wall is the only bound.
//! - **ground**: `GroundingLimits::default()` — the exact configuration
//!   `solve_hddl_inner` passes — under a 60 s anti-hang watchdog. Where the
//!   internal caps bind (10 s wall per grounding phase; 10k ground
//!   actions/methods) the verbatim refusal is the recorded result.
//! - **translate**: `TranslateLimits::default()` — again exactly what
//!   `solve_hddl_inner` passes — under a 60 s anti-hang watchdog. Ticket 23
//!   was **not** landed on this branch, so the internal 10 s translate wall
//!   (plus 200k states / depth 64 count caps) is the binding translate
//!   budget; where it binds, the verbatim
//!   `translate wall-clock limit exceeded: ... limit 10000ms` refusal is the
//!   recorded result, per the ticket.
//! - **solve**: `adapt_problem` + `solve_planning_type(PlanningType::Fond)`
//!   — the exact `solve_hddl_inner` tail — with
//!   `PlannerLimits::max_wall_ms = 30_000`, under a 120 s anti-hang
//!   watchdog. Budgets are per-stage (the ticket's staged spec), not
//!   `solve_hddl`'s cumulative watchdog.
//!
//! Contract (wave-4 rules: no cherry-picking, honest typed outcomes): every
//! domain yields SOLVED / NOPLAN / `LIMIT:*` (a named internal cap) /
//! `GAP:*` (a pipeline gap, recorded verbatim in RESULTS.md). The test
//! panics — and the gate fails — only on contract violations: a stage-thread
//! panic, a garbage `Ok` with `solved=false`, or a hang past an anti-hang
//! watchdog. `cargo test ... -- --ignored` exiting 0 therefore certifies "no
//! panic anywhere in the 43".

use ferroplan::hddl::adapt_problem;
use ferroplan::planning_runtime::{
    solve_planning_type, PlannerError, PlannerLimits, UniversalPlanningRequest,
};
use ferroplan::planning_types::PlanningType;
use ferroplan_hddl::grounder::{self, GroundError, GroundedIR, GroundingLimits};
use ferroplan_hddl::parser;
use ferroplan_hddl::translate::{
    self, PlanningProblem as IrProblem, TranslateError, TranslateLimits,
};
use std::sync::mpsc::RecvTimeoutError;
use std::time::{Duration, Instant};

/// Solve-stage wall budget (`PlannerLimits::max_wall_ms`), per ticket: 30 s.
const SOLVE_WALL_MS: u64 = 30_000;
/// Above this total, RESULTS.md flags the domain SLOW (htn_ipc2023 convention).
const SLOW_MS: u128 = 10_000;

type Entry = (&'static str, &'static str, &'static str);

/// All 43 IPC-2023 domains: (key, domain source, first-listed problem source).
const DOMAINS: &[Entry] = &[
    (
        "assemblyhierarchical",
        include_str!("fixtures/ipc-sweep/AssemblyHierarchical/domain.hddl"),
        include_str!("fixtures/ipc-sweep/AssemblyHierarchical/genericLinearProblem_depth01.hddl"),
    ),
    (
        "barman_bdi",
        include_str!("fixtures/ipc-sweep/Barman-BDI/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Barman-BDI/pfile01.hddl"),
    ),
    (
        "blocksworld_gtohp",
        include_str!("fixtures/ipc-sweep/Blocksworld-GTOHP/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Blocksworld-GTOHP/p01.hddl"),
    ),
    (
        "blocksworld_hpddl",
        include_str!("fixtures/ipc-sweep/Blocksworld-HPDDL/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Blocksworld-HPDDL/pfile_005.hddl"),
    ),
    (
        "depots",
        include_str!("fixtures/ipc-sweep/Depots/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Depots/p01.hddl"),
    ),
    (
        "factories_simple",
        include_str!("fixtures/ipc-sweep/Factories-simple/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Factories-simple/pfile01.hddl"),
    ),
    (
        "freecell_learned_ecai_16",
        include_str!("fixtures/ipc-sweep/Freecell-Learned-ECAI-16/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Freecell-Learned-ECAI-16/probfreecell-02-1.hddl"),
    ),
    (
        "hiking",
        include_str!("fixtures/ipc-sweep/Hiking/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Hiking/p01.hddl"),
    ),
    (
        "lamps",
        include_str!("fixtures/ipc-sweep/Lamps/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Lamps/pfile01.pddl"),
    ),
    (
        "logistics_learned_ecai_16",
        include_str!("fixtures/ipc-sweep/Logistics-Learned-ECAI-16/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Logistics-Learned-ECAI-16/probLOGISTICS-04-0.hddl"),
    ),
    (
        "minecraft_player",
        include_str!("fixtures/ipc-sweep/Minecraft-Player/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Minecraft-Player/p-003-003-003-003.hddl"),
    ),
    (
        "minecraft_regular",
        include_str!("fixtures/ipc-sweep/Minecraft-Regular/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Minecraft-Regular/p-003-003-003-003.hddl"),
    ),
    (
        "monroe_fo_1",
        include_str!("fixtures/ipc-sweep/Monroe_FO_1/domain.hddl"),
        include_str!(
            "fixtures/ipc-sweep/Monroe_FO_1/pfile01-p-0092-set-up-shelter-no-pref-tlt.hddl"
        ),
    ),
    (
        "monroe_fo_19",
        include_str!("fixtures/ipc-sweep/Monroe_FO_19/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Monroe_FO_19/pfile19-p-0037-clear-road-hazard-3-tlt.hddl"),
    ),
    (
        "monroe_fo_2",
        include_str!("fixtures/ipc-sweep/Monroe_FO_2/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Monroe_FO_2/pfile02-p-0063-clear-road-wreck-5-tlt.hddl"),
    ),
    (
        "monroe_fo_20",
        include_str!("fixtures/ipc-sweep/Monroe_FO_20/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Monroe_FO_20/pfile20-p-0037-clear-road-hazard-4-tlt.hddl"),
    ),
    (
        "monroe_po_1",
        include_str!("fixtures/ipc-sweep/Monroe_PO_1/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Monroe_PO_1/pfile01-p-0014-fix-power-line-4.hddl"),
    ),
    (
        "monroe_po_19",
        include_str!("fixtures/ipc-sweep/Monroe_PO_19/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Monroe_PO_19/pfile19-p-0084-provide-temp-heat-2.hddl"),
    ),
    (
        "monroe_po_2",
        include_str!("fixtures/ipc-sweep/Monroe_PO_2/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Monroe_PO_2/pfile02-p-0051-plow-road-3.hddl"),
    ),
    (
        "monroe_po_20",
        include_str!("fixtures/ipc-sweep/Monroe_PO_20/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Monroe_PO_20/pfile20-p-0080-provide-temp-heat-5.hddl"),
    ),
    (
        "multiarm_blocksworld",
        include_str!("fixtures/ipc-sweep/Multiarm-Blocksworld/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Multiarm-Blocksworld/pfile_01_005.hddl"),
    ),
    (
        "pcp_1",
        include_str!("fixtures/ipc-sweep/PCP_1/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_1/p-pcp01.hddl"),
    ),
    (
        "pcp_16",
        include_str!("fixtures/ipc-sweep/PCP_16/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_16/p-pcp16.hddl"),
    ),
    (
        "pcp_17",
        include_str!("fixtures/ipc-sweep/PCP_17/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_17/p-pcp17.hddl"),
    ),
    (
        "pcp_2",
        include_str!("fixtures/ipc-sweep/PCP_2/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PCP_2/p-pcp02.hddl"),
    ),
    (
        "po_barman_bdi",
        include_str!("fixtures/ipc-sweep/PO_Barman-BDI/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Barman-BDI/pfile01.hddl"),
    ),
    (
        "po_colouring",
        include_str!("fixtures/ipc-sweep/PO_Colouring/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Colouring/pfile01.hddl"),
    ),
    (
        "po_monroe_po_1",
        include_str!("fixtures/ipc-sweep/PO_Monroe_PO_1/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Monroe_PO_1/pfile01-p-0088-quell-riot-1.hddl"),
    ),
    (
        "po_monroe_po_2",
        include_str!("fixtures/ipc-sweep/PO_Monroe_PO_2/domain.hddl"),
        include_str!(
            "fixtures/ipc-sweep/PO_Monroe_PO_2/pfile02-p-0068-provide-medical-attention-4.hddl"
        ),
    ),
    (
        "po_monroe_po_24",
        include_str!("fixtures/ipc-sweep/PO_Monroe_PO_24/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Monroe_PO_24/pfile24-p-0059-clear-road-hazard-3.hddl"),
    ),
    (
        "po_monroe_po_25",
        include_str!("fixtures/ipc-sweep/PO_Monroe_PO_25/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Monroe_PO_25/pfile25-p-0050-clear-road-hazard-2.hddl"),
    ),
    (
        "po_rover",
        include_str!("fixtures/ipc-sweep/PO_Rover/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Rover/pfile01.hddl"),
    ),
    (
        "po_satellite",
        include_str!("fixtures/ipc-sweep/PO_Satellite/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Satellite/1obs-1sat-1mod.hddl"),
    ),
    (
        "po_transport",
        include_str!("fixtures/ipc-sweep/PO_Transport/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Transport/pfile01.hddl"),
    ),
    (
        "po_um_translog",
        include_str!("fixtures/ipc-sweep/PO_UM-Translog/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_UM-Translog/01-A-AirplanesHub.hddl"),
    ),
    (
        "po_woodworking",
        include_str!("fixtures/ipc-sweep/PO_Woodworking/domain.hddl"),
        include_str!("fixtures/ipc-sweep/PO_Woodworking/00--p01-variant.hddl"),
    ),
    (
        "robot",
        include_str!("fixtures/ipc-sweep/Robot/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Robot/pfile_01_001.hddl"),
    ),
    (
        "rover_gtohp",
        include_str!("fixtures/ipc-sweep/Rover-GTOHP/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Rover-GTOHP/p01.hddl"),
    ),
    (
        "satellite_gtohp",
        include_str!("fixtures/ipc-sweep/Satellite-GTOHP/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Satellite-GTOHP/p01.hddl"),
    ),
    (
        "snake",
        include_str!("fixtures/ipc-sweep/Snake/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Snake/pb-10slots-seed1.snake.hddl"),
    ),
    (
        "towers",
        include_str!("fixtures/ipc-sweep/Towers/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Towers/pfile_01.hddl"),
    ),
    (
        "transport",
        include_str!("fixtures/ipc-sweep/Transport/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Transport/pfile01.hddl"),
    ),
    (
        "woodworking",
        include_str!("fixtures/ipc-sweep/Woodworking/domain.hddl"),
        include_str!("fixtures/ipc-sweep/Woodworking/00--p01-variant.hddl"),
    ),
];

/// The ticket's always-on sampled subset: the 6 fastest SOLVED domains of the
/// committed full-sweep run (RESULTS.md totals). If the pipeline changes
/// shape enough to move these off the fastest-six list, re-derive the subset
/// from a fresh `-- --ignored` sweep.
const SAMPLED: &[&str] = &[
    "lamps",
    "robot",
    "po_satellite",
    "towers",
    "blocksworld_gtohp",
    "factories_simple",
];

// ---------------------------------------------------------------------------
// Staged runner
// ---------------------------------------------------------------------------

/// Typed stage failure — the variant determines LIMIT:* vs GAP:* labeling.
#[derive(Debug)]
enum StageError {
    Parse(String),
    Ground(GroundError),
    Translate(TranslateError),
    Solve(PlannerError),
}

/// One stage run. The input is moved into a fresh thread — the same watchdog
/// shape `solve_hddl` itself uses — so an outer-wall fire abandons the stage
/// honestly instead of hanging the sweep. `OuterWall` and `Panicked` are
/// contract violations, never outcomes.
enum Stage<Out> {
    Done { wall_ms: u128, out: Out },
    Failed { wall_ms: u128, err: StageError },
    OuterWall,
    Panicked,
}

fn run_stage<In, Out, F>(input: In, outer: Duration, f: F) -> Stage<Out>
where
    In: Send + 'static,
    Out: Send + 'static,
    F: FnOnce(In) -> Result<Out, StageError> + Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let start = Instant::now();
        let result = f(input);
        let wall_ms = start.elapsed().as_millis();
        // A failed send only means the receiver already timed out and moved
        // on — same convention as solve_hddl's worker.
        let _ = tx.send((wall_ms, result));
    });
    match rx.recv_timeout(outer) {
        Ok((wall_ms, Ok(out))) => Stage::Done { wall_ms, out },
        Ok((wall_ms, Err(err))) => Stage::Failed { wall_ms, err },
        Err(RecvTimeoutError::Timeout) => Stage::OuterWall,
        // The stage thread panicked before sending — contract violation.
        Err(RecvTimeoutError::Disconnected) => Stage::Panicked,
    }
}

fn contract_violation(key: &str, stage: &str, what: &str) -> ! {
    panic!(
        "CONTRACT VIOLATION in domain {key:?} at stage {stage:?}: {what} — \
         this is not an honest typed outcome, the sweep gate must fail"
    )
}

/// A domain's full staged record — one RESULTS.md row.
#[derive(Debug)]
struct DomainResult {
    key: &'static str,
    objects: usize,
    gactions: usize,
    gmethods: usize,
    parse_ms: u128,
    ground_ms: u128,
    translate_ms: u128,
    solve_ms: u128,
    total_ms: u128,
    /// SOLVED | NOPLAN | LIMIT:* | GAP:*
    label: String,
    plan_len: usize,
    policy_entries: usize,
    /// Which stages actually ran — a 0 ms wall is a real measurement, not a
    /// skip (lamps grounds in sub-millisecond time). `ground_done` further
    /// distinguishes a completed grounding (counts are real) from one that
    /// refused at a cap/validation (wall is real, counts are not).
    ground_ran: bool,
    ground_done: bool,
    translate_ran: bool,
    solve_ran: bool,
    /// Verbatim first error for non-solved outcomes; "-" when solved.
    verbatim: String,
    flags: String,
}

fn classify_ground(err: &GroundError) -> String {
    match err {
        GroundError::Timeout { .. } => "LIMIT:ground-wall".into(),
        GroundError::LimitExceeded(msg) => {
            if msg.contains("max_ground_actions") {
                "LIMIT:ground-actions".into()
            } else if msg.contains("max_ground_methods") {
                "LIMIT:ground-methods".into()
            } else {
                "LIMIT:ground".into()
            }
        }
        _ => "GAP:ground".into(),
    }
}

fn classify_translate(err: &TranslateError) -> String {
    match err {
        TranslateError::Timeout { .. } => "LIMIT:translate-wall".into(),
        TranslateError::MemoryLimitExceeded { .. } => "LIMIT:translate-states".into(),
        TranslateError::TaskNetworkDepthExceeded { .. } => "LIMIT:translate-depth".into(),
        _ => "GAP:translate".into(),
    }
}

fn classify_solve(err: &PlannerError) -> String {
    match err {
        PlannerError::NoPlan => "NOPLAN".into(),
        PlannerError::Timeout { .. } => "LIMIT:solve-wall".into(),
        PlannerError::ResourceBound { resource, .. } => format!("LIMIT:resource:{resource}"),
        _ => "GAP:plan".into(),
    }
}

fn finish(mut r: DomainResult, label: String, verbatim: String) -> DomainResult {
    r.total_ms = r.parse_ms + r.ground_ms + r.translate_ms + r.solve_ms;
    r.label = label;
    r.verbatim = verbatim.replace(['\n', '\r'], "; ");
    if r.total_ms > SLOW_MS {
        r.flags = "SLOW".into();
    }
    r
}

fn run_domain(entry: &Entry) -> DomainResult {
    let (key, domain_src, problem_src) = (entry.0, entry.1, entry.2);
    let blank = DomainResult {
        key,
        objects: 0,
        gactions: 0,
        gmethods: 0,
        parse_ms: 0,
        ground_ms: 0,
        translate_ms: 0,
        solve_ms: 0,
        total_ms: 0,
        label: String::new(),
        plan_len: 0,
        policy_entries: 0,
        ground_ran: false,
        ground_done: false,
        translate_ran: false,
        solve_ran: false,
        verbatim: String::new(),
        flags: String::new(),
    };

    // Stage 1: parse — 10 s outer wall (the parser has no internal cap).
    let (domain, problem, mut r) = match run_stage(
        (domain_src.to_owned(), problem_src.to_owned()),
        Duration::from_secs(10),
        |(d, p)| {
            let domain = parser::parse_domain(&d).map_err(|e| StageError::Parse(e.to_string()))?;
            let problem =
                parser::parse_problem(&p).map_err(|e| StageError::Parse(e.to_string()))?;
            Ok((domain, problem))
        },
    ) {
        Stage::Done {
            wall_ms,
            out: (domain, problem),
        } => {
            let objects = problem.objects.len();
            let r = DomainResult {
                parse_ms: wall_ms,
                objects,
                ..blank
            };
            (domain, problem, r)
        }
        Stage::Failed { wall_ms, err } => {
            let verbatim = match &err {
                StageError::Parse(msg) => msg.clone(),
                other => format!("{other:?}"),
            };
            let mut r = DomainResult {
                parse_ms: wall_ms,
                ..blank
            };
            return finish(r, "GAP:parse".into(), verbatim);
        }
        Stage::OuterWall => contract_violation(key, "parse", "hang past the 10 s outer wall"),
        Stage::Panicked => contract_violation(key, "parse", "stage thread panicked"),
    };

    // Stage 2: ground — solve_hddl's exact defaults (10 s internal wall per
    // phase, 10k actions/methods), 60 s anti-hang watchdog.
    let ir: GroundedIR = match run_stage((domain, problem), Duration::from_secs(60), |(d, p)| {
        grounder::ground(&d, &p, &GroundingLimits::default()).map_err(StageError::Ground)
    }) {
        Stage::Done { wall_ms, out } => {
            r.ground_ms = wall_ms;
            r.ground_ran = true;
            r.ground_done = true;
            r.gactions = out.actions.len();
            r.gmethods = out.methods.len();
            out
        }
        Stage::Failed { wall_ms, err } => {
            r.ground_ms = wall_ms;
            r.ground_ran = true;
            let label = match &err {
                StageError::Ground(e) => classify_ground(e),
                other => contract_violation(key, "ground", &format!("wrong stage error {other:?}")),
            };
            let verbatim = match err {
                StageError::Ground(e) => e.to_string(),
                other => format!("{other:?}"),
            };
            return finish(r, label, verbatim);
        }
        Stage::OuterWall => contract_violation(key, "ground", "hang past the 60 s anti-hang wall"),
        Stage::Panicked => contract_violation(key, "ground", "stage thread panicked"),
    };

    // Stage 3: translate — solve_hddl's exact defaults (10 s internal wall /
    // 200k states / depth 64 — ticket 23 NOT landed on this branch), 60 s
    // anti-hang watchdog. Where the internal 10 s cap binds, the verbatim
    // refusal is the recorded result, per the ticket.
    let ir_problem: IrProblem = match run_stage(ir, Duration::from_secs(60), |ir| {
        ferroplan_hddl::translate::translate(&ir, &TranslateLimits::default())
            .map_err(StageError::Translate)
    }) {
        Stage::Done { wall_ms, out } => {
            r.translate_ms = wall_ms;
            r.translate_ran = true;
            out
        }
        Stage::Failed { wall_ms, err } => {
            r.translate_ms = wall_ms;
            r.translate_ran = true;
            let label = match &err {
                StageError::Translate(e) => classify_translate(e),
                other => {
                    contract_violation(key, "translate", &format!("wrong stage error {other:?}"))
                }
            };
            let verbatim = match err {
                StageError::Translate(e) => e.to_string(),
                other => format!("{other:?}"),
            };
            return finish(r, label, verbatim);
        }
        Stage::OuterWall => {
            contract_violation(key, "translate", "hang past the 60 s anti-hang wall")
        }
        Stage::Panicked => contract_violation(key, "translate", "stage thread panicked"),
    };

    // Stage 4: solve — solve_hddl_inner's exact tail (adapt_problem +
    // solve_planning_type(Fond)), 30 s PlannerLimits wall, 120 s anti-hang
    // watchdog.
    match run_stage(ir_problem, Duration::from_secs(120), |ir| {
        let request = UniversalPlanningRequest {
            planning_type: PlanningType::Fond,
            problem: adapt_problem(ir),
            limits: PlannerLimits {
                max_wall_ms: SOLVE_WALL_MS,
                ..PlannerLimits::default()
            },
        };
        solve_planning_type(&request).map_err(StageError::Solve)
    }) {
        Stage::Done { wall_ms, out: plan } => {
            if !plan.solved {
                contract_violation(
                    key,
                    "solve",
                    "solve_planning_type returned Ok with solved=false — garbage, \
                     not an honest answer",
                );
            }
            r.solve_ms = wall_ms;
            r.solve_ran = true;
            r.plan_len = plan.steps.len();
            r.policy_entries = plan.policy.len();
            finish(r, "SOLVED".into(), "-".into())
        }
        Stage::Failed { wall_ms, err } => {
            r.solve_ms = wall_ms;
            r.solve_ran = true;
            let label = match &err {
                StageError::Solve(e) => classify_solve(e),
                other => contract_violation(key, "solve", &format!("wrong stage error {other:?}")),
            };
            let verbatim = match err {
                StageError::Solve(e) => format!("{e:?}"),
                other => format!("{other:?}"),
            };
            finish(r, label, verbatim)
        }
        Stage::OuterWall => contract_violation(key, "solve", "hang past the 120 s anti-hang wall"),
        Stage::Panicked => contract_violation(key, "solve", "stage thread panicked"),
    }
}

// ---------------------------------------------------------------------------
// Machine-readable lines + RESULTS.md
// ---------------------------------------------------------------------------

fn ipcresult_line(r: &DomainResult) -> String {
    format!(
        "IPCRESULT|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        r.key,
        r.label,
        r.parse_ms,
        r.ground_ms,
        r.translate_ms,
        r.solve_ms,
        r.total_ms,
        r.objects,
        r.gactions,
        r.gmethods,
        r.plan_len,
        r.flags
    )
}

fn cmd_line(program: &str, args: &[&str]) -> String {
    std::process::Command::new(program)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|_| "unknown".into())
}

fn write_results_md(results: &[DomainResult]) -> std::path::PathBuf {
    let cpu = cmd_line("sysctl", &["-n", "machdep.cpu.brand_string"]);
    let date = cmd_line("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]);

    let solved: Vec<&DomainResult> = results.iter().filter(|r| r.label == "SOLVED").collect();
    let noplan: Vec<&DomainResult> = results.iter().filter(|r| r.label == "NOPLAN").collect();
    let limits: Vec<&DomainResult> = results
        .iter()
        .filter(|r| r.label.starts_with("LIMIT:"))
        .collect();
    let gaps: Vec<&DomainResult> = results
        .iter()
        .filter(|r| r.label.starts_with("GAP:"))
        .collect();

    let sum = |f: fn(&DomainResult) -> u128| -> u128 { results.iter().map(f).sum() };

    let mut md = String::new();
    md.push_str("# IPC-2023 full sweep — `solve_hddl` staged (43 domains)\n\n");
    md.push_str(
        "Ticket `fond-htn-27-bench-ipc-full-sweep`, wave `v26.9.17-fond-htn-hardening`.\n\n",
    );
    md.push_str(&format!(
        "- machine: {cpu} (`sysctl -n machdep.cpu.brand_string`)\n"
    ));
    md.push_str(&format!("- date: {date} (UTC)\n"));
    md.push_str("- commands:\n");
    md.push_str(
        "  - `cargo test -p ferroplan --test ipc_sweep` (sampled 6-domain heartbeat, exit 0)\n",
    );
    md.push_str(
        "  - `cargo test -p ferroplan --test ipc_sweep -- --ignored` (this full sweep, exit 0)\n",
    );
    md.push_str(
        "- corpus: all 43 IPC-2023 hierarchical-track domains, verbatim competition data \
                 (`domain.hddl` + first-listed problem each), vendored under \
                 `tests/fixtures/ipc-sweep/` (≈1.2 MB, under the 2 MB ticket cap).\n",
    );
    md.push_str(
        "- stage budgets (per-stage, the ticket's staged spec — NOT solve_hddl's \
                 cumulative watchdog):\n",
    );
    md.push_str("  - parse: 10 s outer wall (the parser has no internal cap);\n");
    md.push_str(
        "  - ground: `GroundingLimits::default()` — exactly what `solve_hddl_inner` \
                 passes (10 s wall per grounding phase; 10k ground actions/methods) — under a \
                 60 s anti-hang watchdog; where an internal cap binds, its verbatim refusal is \
                 the recorded result;\n",
    );
    md.push_str(
        "  - translate: `TranslateLimits::default()` — exactly what `solve_hddl_inner` \
                 passes (10 s wall / 200k states / depth 64) — under a 60 s anti-hang watchdog. \
                 Ticket 23 was NOT landed on this branch, so the internal 10 s translate wall is \
                 the binding translate budget and `LIMIT:translate-wall` rows carry its verbatim \
                 `limit 10000ms` refusal, per the ticket;\n",
    );
    md.push_str(
        "  - solve: `adapt_problem` + `solve_planning_type(PlanningType::Fond)` (the \
                 exact `solve_hddl_inner` tail), `PlannerLimits::max_wall_ms = 30000`, under a \
                 120 s anti-hang watchdog.\n",
    );
    md.push_str(
        "- `objects` = problem `:objects` entries; `ground actions`/`ground methods` = \
                 grounded IR instance counts (`GroundingLimits` caps apply to these); walls in \
                 milliseconds; `-` = stage skipped after an earlier refusal, or (in the count \
                 columns) a grounding that refused at a cap/validation before counts existed.\n",
    );
    md.push_str(
        "- Contract: no domain panicked, no garbage `Ok(solved=false)`, no hang past an \
                 anti-hang wall — the sweep test exits 0 iff all 43 outcomes are honest typed \
                 answers (SOLVED / NOPLAN / LIMIT / GAP).\n\n",
    );
    md.push_str("| domain | objects | ground actions | ground methods | parse ms | ground ms | translate ms | solve ms | total ms | outcome | plan len | policy entries | flags |\n");
    md.push_str("|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in results {
        let cell = |v: u128, ran: bool| if ran { v.to_string() } else { "-".to_string() };
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            r.key,
            r.objects,
            if r.ground_done {
                r.gactions.to_string()
            } else {
                "-".into()
            },
            if r.ground_done {
                r.gmethods.to_string()
            } else {
                "-".into()
            },
            cell(r.parse_ms, true),
            cell(r.ground_ms, r.ground_ran),
            cell(r.translate_ms, r.translate_ran),
            cell(r.solve_ms, r.solve_ran),
            r.total_ms,
            r.label,
            if r.label == "SOLVED" {
                r.plan_len.to_string()
            } else {
                "-".into()
            },
            if r.label == "SOLVED" {
                r.policy_entries.to_string()
            } else {
                "-".into()
            },
            r.flags,
        ));
    }
    md.push_str(&format!(
        "| TOTALS (43) | | | | {} | {} | {} | {} | {} | {}/43 SOLVED, {} NOPLAN, {} LIMIT:*, {} GAP:* | | | |\n\n",
        sum(|r| r.parse_ms),
        sum(|r| r.ground_ms),
        sum(|r| r.translate_ms),
        sum(|r| r.solve_ms),
        sum(|r| r.total_ms),
        solved.len(),
        noplan.len(),
        limits.len(),
        gaps.len(),
    ));

    md.push_str("## Coverage (honest)\n\n");
    md.push_str(&format!("End-to-end solved: **{}/43**.\n\n", solved.len()));
    md.push_str(&format!(
        "Refused by limits: **{}** — by kind: {}\n\n",
        limits.len(),
        {
            let mut kinds: Vec<(String, usize)> = Vec::new();
            for r in &limits {
                match kinds.iter_mut().find(|(k, _)| *k == r.label) {
                    Some((_, n)) => *n += 1,
                    None => kinds.push((r.label.clone(), 1)),
                }
            }
            kinds
                .iter()
                .map(|(k, n)| format!("{k} ×{n}"))
                .collect::<Vec<_>>()
                .join(", ")
        }
    ));
    if !noplan.is_empty() {
        md.push_str(&format!(
            "NOPLAN (honest solver answer — both the strong fixpoint and the strong-cyclic \
             fallback refused): **{}**.\n\n",
            noplan.len()
        ));
    }
    md.push_str(&format!(
        "Pipeline gaps: **{}** — verbatim first error each:\n\n",
        gaps.len()
    ));
    for r in &gaps {
        md.push_str(&format!("- `{}` ({}): `{}`\n", r.key, r.label, r.verbatim));
    }
    md.push_str("\n### Limit refusals, verbatim\n\n");
    for r in &limits {
        md.push_str(&format!("- `{}` ({}): `{}`\n", r.key, r.label, r.verbatim));
    }
    if !noplan.is_empty() {
        md.push_str("\n### NOPLAN, verbatim\n\n");
        for r in &noplan {
            md.push_str(&format!("- `{}`: `{}`\n", r.key, r.verbatim));
        }
    }

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/ipc-sweep/RESULTS.md");
    std::fs::write(&path, &md).expect("write RESULTS.md");
    path
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Long-run: the full 43-domain staged sweep. Never runs in CI — run with
/// `cargo test -p ferroplan --test ipc_sweep -- --ignored`. Writes
/// `tests/fixtures/ipc-sweep/RESULTS.md` and prints `IPCRESULT|...` lines
/// (capture with `-- --nocapture`).
#[test]
#[ignore = "long-run: full 43-domain IPC-2023 staged sweep (ticket fond-htn-27)"]
fn full_sweep_writes_results() {
    let start = Instant::now();
    let results: Vec<DomainResult> = DOMAINS.iter().map(run_domain).collect();
    for r in &results {
        println!("{}", ipcresult_line(r));
        if r.label != "SOLVED" {
            println!("IPCERR|{}|{}|{}", r.key, r.label, r.verbatim);
        }
    }
    let path = write_results_md(&results);
    let solved = results.iter().filter(|r| r.label == "SOLVED").count();
    println!(
        "IPCSWEEP|total_domains|{}|solved|{}|sweep_wall_ms|{}|results|{}",
        results.len(),
        solved,
        start.elapsed().as_millis(),
        path.display()
    );
    assert_eq!(
        results.len(),
        43,
        "the sweep table must cover all 43 IPC-2023 domains"
    );
}

/// Always-on CI heartbeat: the 6 fastest SOLVED domains of the committed
/// sweep (see `SAMPLED`), each required to still end-to-end solve within
/// 10 s, the whole test within 30 s.
#[test]
fn sampled_heartbeat() {
    let start = Instant::now();
    for key in SAMPLED {
        let entry = DOMAINS
            .iter()
            .find(|e| e.0 == *key)
            .unwrap_or_else(|| panic!("sampled key {key:?} must be in the DOMAINS table"));
        let r = run_domain(entry);
        println!("{}", ipcresult_line(&r));
        assert_eq!(
            r.label, "SOLVED",
            "sampled heartbeat domain {key:?} no longer solves: {} ({})",
            r.label, r.verbatim
        );
        assert!(
            r.total_ms < 10_000,
            "sampled heartbeat domain {key:?} took {} ms (budget 10000 ms) — \
             re-derive the sampled subset from a fresh -- --ignored sweep",
            r.total_ms
        );
    }
    let total = start.elapsed();
    println!("IPCSWEEP|sampled_total_ms|{}", total.as_millis());
    assert!(
        total < Duration::from_secs(30),
        "sampled heartbeat must stay under 30 s total, took {total:?}"
    );
}
