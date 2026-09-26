//! WASI linear-memory ABI — the same shape as beam4pm's `rust4pm-wasm`
//! (`native/rust4pm-wasm/src/lib.rs`), so a BEAM host (wasmex/wasmtime) can
//! load ferroplan directly, no JS glue, no wasm-bindgen, no Component
//! Model. Built only for `wasm32-wasip1`; the browser build
//! (`wasm32-unknown-unknown` + `wasm-bindgen`, see `lib.rs`) is untouched
//! and keeps its own entry points.
//!
//! # Wire contract
//!
//! One dispatch export: `fp_call(ptr, len) -> u64` packed as
//! `(out_ptr << 32) | out_len`. Request and response are UTF-8 JSON. Every
//! failure returns `{"error":{"code":..,"message":..,"retryable":bool}}`
//! (matching the browser adapter's `err_json`/refusal shapes) rather than a
//! bare string, so a beam4pm host sees the same error envelope either way.
//!
//! Ops (`"op"` field of the request object):
//!   - `plan` `{domain, problem, mode?, flags?, search?}` -> `Solution` JSON
//!     (compatibility surface; single-threaded, same as the browser `plan`)
//!   - `plan_production` `{domain, problem, mode?, search?, max_evaluated?,
//!     max_plan_steps?, max_output_bytes?, request_id?}` ->
//!     `OperationEnvelope<Solution>` JSON
//!   - `htn_plan` `{domain, problem, limits?}` -> `UniversalPlan` JSON
//!     (`problem` is JSON text of a `PlanningProblem`, not PDDL; forces
//!     `PlanningType::Hierarchical` via `solve_planning_type`)
//!   - `fond_policy` `{domain, problem, limits?}` -> `UniversalPlan` JSON
//!     (same wire shape as `htn_plan`; forces `PlanningType::Fond`)
//!   - `hddl_solve` `{domain, problem, limits?}` -> `UniversalPlan` JSON.
//!     `domain`/`problem` are HDDL source text (not classical PDDL); parsed,
//!     grounded, and translated by `ferroplan_hddl`, then solved by the
//!     existing FOND solver (`ferroplan::solve_hddl`). `limits` is an
//!     optional partial `PlannerLimits` object (`max_depth`, `max_states`,
//!     `max_iterations`; any/all omitted fields fall back to their own
//!     defaults). Errors distinguish the failing pipeline stage:
//!     `FP_PARSE` (malformed HDDL), `FP_HDDL_GROUND` (grounding failed),
//!     `FP_HDDL_TRANSLATE` (ground IR -> planning-runtime IR failed),
//!     `FP_HDDL_ROOT_MISMATCH` (an Eve handoff conflicts with the problem's
//!     own `:htn` root network), `FP_MODEL` (the FOND solver itself
//!     rejected the translated problem), `FP_TIMEOUT` (bounded HDDL
//!     solving exceeded its limit), or `FP_WORKER_PANICKED` (a bounded
//!     worker terminated unexpectedly).
//!   - `readiness` `{}` -> capability manifest + fingerprint
//!   - `version` `{}` -> `{"version": "..."}`
//!   - `explain` `{domain, problem, plan}` (plan = a `Plan` object, not a
//!     JSON string) -> explanation JSON
//!   - `session_new` `{domain, problem}` -> `{handle}`
//!   - `session_fork` `{handle}` -> `{handle: new_handle}`
//!   - `session_free` `{handle}` -> `{freed:true}`
//!   - `session_set_goal` `{handle, goal}` -> `{ok:true}`
//!   - `session_restrict_prefix_claims` `{handle, prefix, claimed}` -> `{ok:true}`
//!   - `session_restrict_contains` `{handle, filter}` -> `{ok:true}`
//!   - `session_think` `{handle, evals, mem_mb}` -> `Solution` JSON
//!   - `session_replan_following` `{handle, evals, mem_mb}` -> `Solution` JSON,
//!     preserving the still-applicable prefix of the stashed plan when possible
//!   - `session_repair` `{handle, evals, mem_mb}` -> DfCM repair decision JSON:
//!     goal-met -> reuse-valid-suffix -> follow-biased replan -> full replan
//!   - `session_probe` `{handle, candidates, evals, mem_mb}` -> bounded
//!     counterfactual results over cheap forks; the parent session is unchanged
//!   - `session_valid` `{handle}` -> `{valid:bool}`
//!   - `session_step` `{handle}` -> the current step, or `null`
//!   - `session_suffix` `{handle}` -> the remaining steps array
//!   - `session_advance` `{handle}` -> `{ok:true}`
//!   - `session_drop_plan` `{handle}` -> `{ok:true}`
//!   - `session_has_plan` `{handle}` -> `{has_plan:bool}`
//!   - `session_set_fact` `{handle, name, value}` -> `{ok:true}`
//!   - `session_set_timed_fact` `{handle, dt, name, value}` -> `{ok:true}`
//!   - `session_observe` `{handle, sight:[[name,value],...]}` -> news JSON
//!   - `session_goal_met` `{handle}` -> `{goal_met:bool}`
//!   - `session_fact` `{handle, name}` -> `{value: bool|null}`
//!   - `session_apply_start` `{handle, name}` -> `{ok:true}`
//!   - `session_elapse` `{handle, dt}` -> fired-events JSON
//!   - `session_set_fluent` `{handle, name, value}` -> `{ok:true}`
//!   - `session_fluent` `{handle, name}` -> `{value: f64|null}`
//!   - `session_plan_valid` `{handle, plan, from}` -> `{valid:bool}`
//!   - `session_world_bytes` `{handle}` -> `{bytes: usize}`
//!   - `session_mind_bytes` `{handle}` -> `{bytes: usize}`
//!
//! # Handle space
//!
//! One monotonically increasing `u64` counter (starting at 1), one registry
//! (`SESSIONS: Mutex<Option<HashMap<u64, WasiSession>>>`), mirroring
//! beam4pm's `LOGS`/`NETS` "take -> compute lock-free -> re-insert"
//! discipline: the lock is never held across a solve, so a mid-compute trap
//! can't leave the registry poisoned in a way that bricks every other live
//! handle. Freed/consumed handles never come back.
//!
//! # Memory ownership contract
//!
//! Identical to beam4pm's: `std::alloc::alloc`/`dealloc`,
//! `Layout::from_size_align(len, 1)` throughout. `fp_call` CONSUMES the
//! request buffer (copies it out, frees it immediately — the host must not
//! dealloc it afterward); the response buffer is host-owned, freed via
//! `fp_dealloc(out_ptr, out_len)` after reading.

use ferroplan::planning_runtime::{solve_planning_type, PlanningProblem, UniversalPlanningRequest};
use ferroplan::planning_types::PlanningType;
use ferroplan::{
    capability_manifest, solve, solve_hddl, solve_production, HddlError, Mode, Options,
    PlannerLimits, ProductionLimits, Search,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::alloc::Layout;
use std::collections::HashMap;
use std::sync::Mutex;

const WASI_TEXT_FIELD_BYTES: usize = 1024 * 1024;
const WASI_JSON_FIELD_BYTES: usize = 16 * 1024 * 1024;
const WASI_MAX_THINK_EVALS: usize = 1_000_000;
const WASI_MAX_THINK_MEMORY_MB: usize = 2_048;
const WASI_MAX_PROBE_CANDIDATES: usize = 32;
const WASI_MAX_PROBE_OBSERVATIONS: usize = 1_024;

struct WasiSession {
    inner: ferroplan::Session,
    plan: Option<ferroplan::api::Plan>,
    cursor: usize,
}

static NEXT_HANDLE: Mutex<u64> = Mutex::new(1);
static SESSIONS: Mutex<Option<HashMap<u64, WasiSession>>> = Mutex::new(None);

fn next_handle() -> Result<u64, String> {
    let mut next = NEXT_HANDLE
        .lock()
        .map_err(|_| "internal: handle counter lock poisoned".to_string())?;
    let id = *next;
    *next += 1;
    Ok(id)
}

/// Take a session out of the registry (freeing the lock before any
/// compute), run `f`, then re-insert unless `f` consumed it. Never holds
/// `SESSIONS`'s mutex across a solve.
fn with_session<T>(handle: u64, f: impl FnOnce(&mut WasiSession) -> T) -> Result<T, String> {
    let mut session = {
        let mut guard = SESSIONS
            .lock()
            .map_err(|_| "internal: session registry lock poisoned".to_string())?;
        guard
            .get_or_insert_with(HashMap::new)
            .remove(&handle)
            .ok_or_else(|| format!("unknown session handle {handle}"))?
    };
    let out = f(&mut session);
    let mut guard = SESSIONS
        .lock()
        .map_err(|_| "internal: session registry lock poisoned".to_string())?;
    guard
        .get_or_insert_with(HashMap::new)
        .insert(handle, session);
    Ok(out)
}

fn insert_session(session: WasiSession) -> Result<u64, String> {
    let id = next_handle()?;
    let mut guard = SESSIONS
        .lock()
        .map_err(|_| "internal: session registry lock poisoned".to_string())?;
    guard.get_or_insert_with(HashMap::new).insert(id, session);
    Ok(id)
}

// ---------------------------------------------------------------------------
// Memory ABI
// ---------------------------------------------------------------------------

/// Allocate `len` bytes for the host to write a request into. `len == 0`
/// returns null (the host never allocates empty requests).
#[no_mangle]
pub extern "C" fn fp_alloc(len: usize) -> *mut u8 {
    if len == 0 {
        return std::ptr::null_mut();
    }
    match Layout::from_size_align(len, 1) {
        Ok(layout) => unsafe { std::alloc::alloc(layout) },
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a buffer previously produced by `fp_alloc` or returned by
/// `fp_call`. The exported wasm symbol and its argument layout are
/// unchanged (fond-htn-55, clippy::not_unsafe_ptr_arg_deref, clippy 1.97).
///
/// # Safety
///
/// Host contract: `ptr` must be null or a live pointer previously produced
/// by this module with the same `len` it was produced with.
#[no_mangle]
pub unsafe extern "C" fn fp_dealloc(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    if let Ok(layout) = Layout::from_size_align(len, 1) {
        unsafe { std::alloc::dealloc(ptr, layout) }
    }
}

/// One dispatch entry point: request JSON in linear memory -> response
/// JSON. Returns a packed u64: `(out_ptr << 32) | out_len`.
///
/// CONSUMES the request buffer (copies it out, then frees it) — the host
/// must not dealloc the request after this call. The response buffer is
/// the host's to free via `fp_dealloc(out_ptr, out_len)` after reading.
/// The exported wasm symbol and its argument layout are unchanged
/// (fond-htn-55, clippy::not_unsafe_ptr_arg_deref, clippy 1.97).
///
/// # Safety
///
/// Host contract: `ptr` must be null or a live pointer previously produced
/// by `fp_alloc`/`fp_call` with exactly this `len`.
#[no_mangle]
pub unsafe extern "C" fn fp_call(ptr: *mut u8, len: usize) -> u64 {
    let input: Vec<u8> = if ptr.is_null() || len == 0 {
        Vec::new()
    } else {
        let copied = unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec();
        fp_dealloc(ptr, len);
        copied
    };
    let out = dispatch(&input).unwrap_or_else(|message| {
        serde_json::to_vec(&err_json("FP_ADAPTER", &message)).unwrap_or_else(|_| {
            b"{\"error\":{\"code\":\"FP_ADAPTER\",\"message\":\"internal: response serialization failed\",\"retryable\":false}}".to_vec()
        })
    });
    let out_len = out.len();
    let out_ptr = Box::into_raw(out.into_boxed_slice()) as *mut u8 as u64;
    (out_ptr << 32) | (out_len as u64)
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

fn err_json(code: &str, msg: &str) -> Value {
    json!({ "error": { "code": code, "message": msg, "retryable": false } })
}

fn parse_mode(m: Option<&str>) -> Mode {
    match m.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("ff") => Mode::Ff,
        Some("pddl3") => Mode::Pddl3,
        Some("partition") => Mode::Partition,
        Some("temporal") => Mode::Temporal,
        Some("portfolio") => Mode::Portfolio,
        Some("optimal") => Mode::Optimal,
        _ => Mode::Auto,
    }
}

fn parse_mode_strict(m: Option<&str>) -> Result<Mode, String> {
    match m.map(|s| s.to_ascii_lowercase()).as_deref() {
        None | Some("auto") => Ok(Mode::Auto),
        Some("ff") => Ok(Mode::Ff),
        Some("pddl3") => Ok(Mode::Pddl3),
        Some("partition") => Ok(Mode::Partition),
        Some("temporal") => Ok(Mode::Temporal),
        Some("portfolio") => Ok(Mode::Portfolio),
        Some("optimal") => Ok(Mode::Optimal),
        Some(other) => Err(format!("unsupported mode `{other}`")),
    }
}

fn parse_search(s: Option<&str>) -> Search {
    match s.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("ehc") => Search::Ehc,
        Some("best-first") => Search::BestFirst,
        Some("ehc-then-bf") => Search::EhcThenBestFirst,
        _ => Search::Auto,
    }
}

fn parse_search_strict(s: Option<&str>) -> Result<Search, String> {
    match s.map(|s| s.to_ascii_lowercase()).as_deref() {
        None | Some("auto") => Ok(Search::Auto),
        Some("ehc") => Ok(Search::Ehc),
        Some("best-first") => Ok(Search::BestFirst),
        Some("ehc-then-bf") | Some("ehc-then-best-first") => Ok(Search::EhcThenBestFirst),
        Some(other) => Err(format!("unsupported search strategy `{other}`")),
    }
}

fn apply_flags(flags: Option<&str>) {
    let want: std::collections::HashSet<&str> = flags
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    ferroplan::features::set_overrides(
        want.contains("tdemand"),
        want.contains("tdecomp"),
        want.contains("tconc"),
    );
}

fn field_str<'a>(req: &'a Value, name: &str) -> Result<&'a str, String> {
    req.get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing or non-string field `{name}`"))
}

fn field_u64(req: &Value, name: &str) -> Result<u64, String> {
    req.get(name)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing or non-integer field `{name}`"))
}

fn dispatch(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() > WASI_JSON_FIELD_BYTES {
        return serde_json::to_vec(&err_json(
            "FP_LIMIT_INPUT",
            "request exceeds the WASI adapter input limit",
        ))
        .map_err(|e| e.to_string());
    }
    let req: Value = serde_json::from_slice(input).map_err(|e| format!("request JSON: {e}"))?;
    let op = req
        .get("op")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing `op` field".to_string())?;

    let response: Value = match op {
        "plan" => op_plan(&req)?,
        "plan_production" => op_plan_production(&req)?,
        "htn_plan" => op_htn_plan(&req)?,
        "fond_policy" => op_fond_policy(&req)?,
        "hddl_solve" => op_hddl_solve(&req)?,
        "readiness" => op_readiness()?,
        "version" => json!({ "version": env!("CARGO_PKG_VERSION") }),
        "explain" => op_explain(&req)?,
        "session_new" => op_session_new(&req)?,
        "session_fork" => op_session_fork(&req)?,
        "session_free" => op_session_free(&req)?,
        "session_set_goal" => op_session_set_goal(&req)?,
        "session_restrict_prefix_claims" => op_session_restrict_prefix_claims(&req)?,
        "session_restrict_contains" => op_session_restrict_contains(&req)?,
        "session_think" => op_session_think(&req)?,
        "session_replan_following" => op_session_replan_following(&req)?,
        "session_repair" => op_session_repair(&req)?,
        "session_probe" => op_session_probe(&req)?,
        "session_valid" => op_session_valid(&req)?,
        "session_step" => op_session_step(&req)?,
        "session_suffix" => op_session_suffix(&req)?,
        "session_advance" => op_session_advance(&req)?,
        "session_drop_plan" => op_session_drop_plan(&req)?,
        "session_has_plan" => op_session_has_plan(&req)?,
        "session_set_fact" => op_session_set_fact(&req)?,
        "session_set_timed_fact" => op_session_set_timed_fact(&req)?,
        "session_observe" => op_session_observe(&req)?,
        "session_goal_met" => op_session_goal_met(&req)?,
        "session_fact" => op_session_fact(&req)?,
        "session_apply_start" => op_session_apply_start(&req)?,
        "session_elapse" => op_session_elapse(&req)?,
        "session_set_fluent" => op_session_set_fluent(&req)?,
        "session_fluent" => op_session_fluent(&req)?,
        "session_plan_valid" => op_session_plan_valid(&req)?,
        "session_world_bytes" => op_session_world_bytes(&req)?,
        "session_mind_bytes" => op_session_mind_bytes(&req)?,
        other => {
            return serde_json::to_vec(&err_json("FP_UNKNOWN_OP", &format!("unknown op `{other}`")))
                .map_err(|e| e.to_string())
        }
    };
    serde_json::to_vec(&response).map_err(|e| e.to_string())
}

fn op_plan(req: &Value) -> Result<Value, String> {
    let domain = field_str(req, "domain")?;
    let problem = field_str(req, "problem")?;
    let mode = req.get("mode").and_then(Value::as_str);
    let flags = req.get("flags").and_then(Value::as_str);
    let search = req.get("search").and_then(Value::as_str);
    apply_flags(flags);
    let opts = Options {
        mode: parse_mode(mode),
        search: parse_search(search),
        threads: 1,
        ..Default::default()
    };
    match solve(domain, problem, &opts) {
        Ok(sol) => serde_json::to_value(sol).map_err(|e| e.to_string()),
        Err(e) => Ok(err_json("FP_ADAPTER", &e.to_string())),
    }
}

#[derive(Deserialize)]
struct PlanProductionReq {
    max_evaluated: Option<usize>,
    max_plan_steps: Option<usize>,
    max_output_bytes: Option<usize>,
    request_id: Option<String>,
}

#[derive(Clone, Deserialize)]
struct SessionProbeCandidate {
    id: String,
    goal: Option<String>,
    #[serde(default)]
    sight: Vec<(String, bool)>,
    restrict_contains: Option<String>,
}

fn op_plan_production(req: &Value) -> Result<Value, String> {
    let domain = field_str(req, "domain")?;
    let problem = field_str(req, "problem")?;
    let mode = req.get("mode").and_then(Value::as_str);
    let search = req.get("search").and_then(Value::as_str);
    let extra: PlanProductionReq =
        serde_json::from_value(req.clone()).map_err(|e| format!("request JSON: {e}"))?;

    let mode = match parse_mode_strict(mode) {
        Ok(mode) => mode,
        Err(message) => return Ok(adapter_refusal_json(extra.request_id.as_deref(), &message)),
    };
    let search = match parse_search_strict(search) {
        Ok(search) => search,
        Err(message) => return Ok(adapter_refusal_json(extra.request_id.as_deref(), &message)),
    };
    let defaults = ProductionLimits::default();
    let limits = ProductionLimits {
        max_evaluated: extra.max_evaluated.unwrap_or(defaults.max_evaluated),
        max_plan_steps: extra.max_plan_steps.unwrap_or(defaults.max_plan_steps),
        max_output_bytes: extra.max_output_bytes.unwrap_or(defaults.max_output_bytes),
        max_workers: 1,
        ..defaults
    };
    let options = Options {
        mode,
        search,
        threads: 1,
        max_evaluated: extra.max_evaluated,
        ..Default::default()
    };
    serde_json::to_value(solve_production(
        domain,
        problem,
        &options,
        &limits,
        extra.request_id.as_deref(),
    ))
    .map_err(|e| e.to_string())
}

/// `{op:"htn_plan", domain, problem[, limits?]}` -> `UniversalPlan` JSON.
/// `domain`/`problem` are UTF-8 JSON text of a `PlanningProblem` object
/// (same wire shape as `plan`/`plan_production`'s `domain`/`problem` fields,
/// but decoded into the typed universal-planning model rather than PDDL
/// text). `domain` is accepted and, if non-empty and not equal to
/// `problem`, merged in as an additional `PlanningProblem` fragment is NOT
/// supported here — ferroplan's universal-planning model has one combined
/// problem document, so `domain` is ignored when present and `problem`
/// alone is parsed as the full `PlanningProblem`. Forces
/// `PlanningType::Hierarchical`.
fn op_htn_plan(req: &Value) -> Result<Value, String> {
    solve_universal(req, PlanningType::Hierarchical)
}

/// `{op:"fond_policy", domain, problem[, limits?]}` -> `UniversalPlan` JSON.
/// Same wire shape as `op_htn_plan`; forces `PlanningType::Fond`.
fn op_fond_policy(req: &Value) -> Result<Value, String> {
    solve_universal(req, PlanningType::Fond)
}

fn solve_universal(req: &Value, planning_type: PlanningType) -> Result<Value, String> {
    let problem_text = field_str(req, "problem")?;
    bounded(problem_text, WASI_JSON_FIELD_BYTES, "problem")?;
    let problem: PlanningProblem = serde_json::from_str(problem_text)
        .map_err(|e| format!("problem: invalid PlanningProblem JSON: {e}"))?;
    let limits: PlannerLimits = match req.get("limits") {
        Some(v) => serde_json::from_value(v.clone()).map_err(|e| format!("limits: {e}"))?,
        None => PlannerLimits::default(),
    };
    let request = UniversalPlanningRequest {
        planning_type,
        problem,
        limits,
    };
    match solve_planning_type(&request) {
        Ok(plan) => serde_json::to_value(plan).map_err(|e| e.to_string()),
        Err(e) => Ok(err_json("FP_ADAPTER", &e.to_string())),
    }
}

fn adapter_refusal_json(request_id: Option<&str>, message: &str) -> Value {
    json!({
        "schema_version": "ferroplan.operation.v1",
        "request_id": request_id.unwrap_or("wasi-adapter-refusal"),
        "capability_id": "fp.wasi",
        "capability_version": env!("CARGO_PKG_VERSION"),
        "authority": "candidate_only",
        "outcome": "refused",
        "validation": "not_applicable",
        "payload": null,
        "error": {
            "code": "FP_INVALID_REQUEST",
            "message": message,
            "retryable": false
        }
    })
}

/// `hddl_solve`: HDDL domain+problem text -> `UniversalPlan` JSON, via
/// `ferroplan::solve_hddl` (parse -> ground -> translate, all
/// `ferroplan_hddl`, then the existing FOND solver). Same success/error
/// envelope shape as `op_plan`: `Ok(plan_json)` on success, `Ok(err_json)`
/// (not a `dispatch`-level `Err`) on a domain-level failure, so a malformed
/// HDDL document is a normal response, not an adapter fault.
fn op_hddl_solve(req: &Value) -> Result<Value, String> {
    let domain = field_str(req, "domain")?;
    let problem = field_str(req, "problem")?;
    bounded(
        domain,
        ProductionLimits::default().max_domain_bytes,
        "domain",
    )?;
    bounded(
        problem,
        ProductionLimits::default().max_problem_bytes,
        "problem",
    )?;
    let mut limits: PlannerLimits = match req.get("limits") {
        Some(v) => serde_json::from_value(v.clone()).map_err(|e| format!("limits: {e}"))?,
        None => PlannerLimits::default(),
    };
    // `solve_hddl`'s wall-clock watchdog (`max_wall_ms != 0`, the
    // `PlannerLimits::default()` case) spawns a real `std::thread` to race
    // against the timeout -- see `crates/ferroplan/src/hddl.rs`. WASI
    // preview1 (this crate's `wasm32-wasip1` target) has no OS threads:
    // `std::thread::spawn` there aborts the whole guest instance with an
    // `unreachable` trap instead of returning an `Err`, so every
    // `hddl_solve` call would panic before this function's own
    // `Ok(err_json)`-not-`Err` contract (see this fn's own doc comment) ever
    // had a chance to apply. Force the synchronous, non-threaded
    // `solve_hddl_inner` path (`max_wall_ms == 0`) at this ABI boundary --
    // the caller's own `limits` object (if any) is otherwise honored
    // unchanged, and a hung parse is still bounded on the BEAM side by this
    // op's own wasmex call timeout.
    limits.max_wall_ms = 0;
    match solve_hddl(domain, problem, &limits) {
        Ok(plan) => serde_json::to_value(plan).map_err(|e| e.to_string()),
        Err(e) => Ok(hddl_error_json(&e)),
    }
}

/// Map each `HddlError` variant to its own distinguishable error code —
/// which pipeline stage failed (parse/ground/translate/planner/timeout/worker)
/// is diagnostic information a caller needs, not something to collapse into
/// one generic message.
fn hddl_error_json(e: &HddlError) -> Value {
    match e {
        HddlError::Parse(msg) => err_json("FP_PARSE", &format!("HDDL parse error: {msg}")),
        HddlError::Ground(msg) => {
            err_json("FP_HDDL_GROUND", &format!("HDDL grounding error: {msg}"))
        }
        HddlError::Translate(msg) => err_json(
            "FP_HDDL_TRANSLATE",
            &format!("HDDL translation error: {msg}"),
        ),
        HddlError::RootTaskMismatch { .. } => err_json(
            "FP_HDDL_ROOT_MISMATCH",
            &format!("HDDL root-task mismatch: {e}"),
        ),
        HddlError::Planner(pe) => err_json("FP_MODEL", &format!("planner error: {pe}")),
        HddlError::Timeout {
            elapsed_ms,
            limit_ms,
        } => err_json(
            "FP_TIMEOUT",
            &format!("HDDL solve timed out after {elapsed_ms}ms (limit {limit_ms}ms)"),
        ),
        HddlError::WorkerPanicked(msg) => err_json(
            "FP_WORKER_PANICKED",
            &format!("HDDL solve worker panicked: {msg}"),
        ),
    }
}

fn op_readiness() -> Result<Value, String> {
    let manifest = capability_manifest();
    match manifest.fingerprint() {
        Ok(fingerprint) => Ok(json!({
            "schema_version": "ferroplan.readiness-contract.v1",
            "product_version": env!("CARGO_PKG_VERSION"),
            "manifest_fingerprint": fingerprint,
            "contract_valid": true,
            "admission_state": "declared",
            "admission_notice": "Admission is verifier-derived from exact-source evidence.",
            "manifest": manifest,
        })),
        Err(error) => Ok(err_json("FP_INVARIANT", &error.to_string())),
    }
}

fn op_explain(req: &Value) -> Result<Value, String> {
    let domain = field_str(req, "domain")?;
    let problem = field_str(req, "problem")?;
    let plan_value = req
        .get("plan")
        .ok_or_else(|| "missing field `plan`".to_string())?;
    let plan: ferroplan::api::Plan =
        serde_json::from_value(plan_value.clone()).map_err(|e| format!("plan: {e}"))?;
    match ferroplan::introspect::explain(domain, problem, &plan) {
        Ok(ex) => serde_json::to_value(ex).map_err(|e| e.to_string()),
        Err(e) => Ok(err_json("FP_VALIDATION", &e)),
    }
}

fn bounded(value: &str, max_bytes: usize, label: &str) -> Result<(), String> {
    if value.len() > max_bytes {
        Err(format!("{label} exceeds the {max_bytes}-byte limit"))
    } else {
        Ok(())
    }
}

fn op_session_new(req: &Value) -> Result<Value, String> {
    let domain = field_str(req, "domain")?;
    let problem = field_str(req, "problem")?;
    bounded(
        domain,
        ProductionLimits::default().max_domain_bytes,
        "domain",
    )?;
    bounded(
        problem,
        ProductionLimits::default().max_problem_bytes,
        "problem",
    )?;
    let opts = Options {
        threads: 1,
        max_evaluated: Some(ProductionLimits::default().max_evaluated),
        ..Default::default()
    };
    let session = WasiSession {
        inner: ferroplan::Session::new(domain, problem, &opts).map_err(|e| e.to_string())?,
        plan: None,
        cursor: 0,
    };
    let handle = insert_session(session)?;
    Ok(json!({ "handle": handle }))
}

fn op_session_fork(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let forked = with_session(handle, |s| WasiSession {
        inner: s.inner.fork(),
        plan: None,
        cursor: 0,
    })?;
    let new_handle = insert_session(forked)?;
    Ok(json!({ "handle": new_handle }))
}

fn op_session_free(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let mut guard = SESSIONS
        .lock()
        .map_err(|_| "internal: session registry lock poisoned".to_string())?;
    let existed = guard
        .get_or_insert_with(HashMap::new)
        .remove(&handle)
        .is_some();
    if !existed {
        return Err(format!("unknown session handle {handle}"));
    }
    Ok(json!({ "freed": true }))
}

fn op_session_set_goal(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let goal = field_str(req, "goal")?;
    bounded(goal, WASI_TEXT_FIELD_BYTES, "goal")?;
    with_session(handle, |s| s.inner.set_goal(goal))?.map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true }))
}

fn op_session_restrict_prefix_claims(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let prefix = field_str(req, "prefix")?.to_string();
    let claimed = field_str(req, "claimed")?.to_string();
    with_session(handle, |s| {
        let prefix = prefix.clone();
        let claimed_set: std::collections::HashSet<String> = claimed
            .split(',')
            .map(|s| s.trim().to_ascii_uppercase())
            .filter(|s| !s.is_empty())
            .collect();
        s.inner.restrict_ops(move |d| {
            d.starts_with(&prefix)
                && d.split_whitespace()
                    .nth(4)
                    .map(|y| !claimed_set.contains(y.trim_end_matches(')')))
                    .unwrap_or(true)
        });
    })?;
    Ok(json!({ "ok": true }))
}

fn op_session_restrict_contains(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let filter = field_str(req, "filter")?.to_string();
    with_session(handle, |s| {
        let filter = filter.clone();
        s.inner.restrict_ops(move |d| d.contains(&filter));
    })?;
    Ok(json!({ "ok": true }))
}

fn session_budget(req: &Value) -> Result<Result<(usize, usize), Value>, String> {
    let evals = req
        .get("evals")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing field `evals`".to_string())? as usize;
    let mem_mb = req
        .get("mem_mb")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing field `mem_mb`".to_string())? as usize;
    if evals == 0 || evals > WASI_MAX_THINK_EVALS {
        return Ok(Err(err_json(
            "FP_LIMIT_SEARCH",
            "evals must be within the WASI production budget",
        )));
    }
    if mem_mb == 0 || mem_mb > WASI_MAX_THINK_MEMORY_MB {
        return Ok(Err(err_json(
            "FP_LIMIT_MEMORY",
            "mem_mb must be within the WASI production budget",
        )));
    }
    Ok(Ok((evals, mem_mb)))
}

fn op_session_think(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let (evals, mem_mb) = match session_budget(req)? {
        Ok(budget) => budget,
        Err(refusal) => return Ok(refusal),
    };
    let sol = with_session(handle, |s| {
        let sol = s.inner.replan_budgeted(evals, Some(mem_mb));
        s.plan = if sol.solved { sol.plan.clone() } else { None };
        s.cursor = 0;
        sol
    })?;
    serde_json::to_value(sol).map_err(|e| e.to_string())
}

/// Follow-before-rethink over the stashed plan. This is CONSTRUCT-only:
/// it manufactures and stashes a candidate replacement plan but never
/// advances the cursor, applies an action, or grants authority.
fn op_session_replan_following(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let (evals, mem_mb) = match session_budget(req)? {
        Ok(budget) => budget,
        Err(refusal) => return Ok(refusal),
    };

    let result = with_session(handle, |s| {
        let Some(prior) = s.plan.clone() else {
            return Err("session has no stashed plan to follow".to_string());
        };
        let sol = s
            .inner
            .replan_following(&prior, s.cursor, evals, Some(mem_mb));
        s.plan = if sol.solved { sol.plan.clone() } else { None };
        s.cursor = 0;
        Ok(sol)
    })??;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

/// DfCM repair router: preserve the maximum reversible option before paying
/// for broader search. A valid suffix is reused with zero search; only a
/// broken suffix earns follow-biased replanning. With no prior plan, the
/// router falls back to the ordinary bounded replan. It never actuates.
fn op_session_repair(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let (evals, mem_mb) = match session_budget(req)? {
        Ok(budget) => budget,
        Err(refusal) => return Ok(refusal),
    };

    with_session(handle, |s| {
        if s.inner.goal_met() {
            return Ok(json!({
                "decision": "goal_met",
                "trigger": "goal_met",
                "plan_valid": Value::Null,
                "previous_suffix": [],
                "suffix": [],
                "solution": Value::Null,
            }));
        }

        match s.plan.clone() {
            Some(prior) => {
                let from = s.cursor.min(prior.steps.len());
                let previous_suffix = prior.steps[from..].to_vec();
                if s.inner.plan_still_valid(&prior, s.cursor) {
                    return Ok(json!({
                        "decision": "reuse_suffix",
                        "trigger": "none",
                        "plan_valid": true,
                        "previous_suffix": previous_suffix,
                        "suffix": previous_suffix,
                        "solution": Value::Null,
                    }));
                }

                let sol = s
                    .inner
                    .replan_following(&prior, s.cursor, evals, Some(mem_mb));
                let decision = if sol.solved {
                    "replanned_following"
                } else {
                    "replan_unsolved"
                };
                s.plan = if sol.solved { sol.plan.clone() } else { None };
                s.cursor = 0;
                let suffix = s
                    .plan
                    .as_ref()
                    .map(|plan| plan.steps.clone())
                    .unwrap_or_default();
                Ok(json!({
                    "decision": decision,
                    "trigger": "invalid_plan",
                    "plan_valid": false,
                    "previous_suffix": previous_suffix,
                    "suffix": suffix,
                    "solution": sol,
                }))
            }
            None => {
                let sol = s.inner.replan_budgeted(evals, Some(mem_mb));
                let decision = if sol.solved {
                    "replanned_full"
                } else {
                    "replan_unsolved"
                };
                s.plan = if sol.solved { sol.plan.clone() } else { None };
                s.cursor = 0;
                let suffix = s
                    .plan
                    .as_ref()
                    .map(|plan| plan.steps.clone())
                    .unwrap_or_default();
                Ok(json!({
                    "decision": decision,
                    "trigger": "no_plan",
                    "plan_valid": Value::Null,
                    "previous_suffix": [],
                    "suffix": suffix,
                    "solution": sol,
                }))
            }
        }
    })?
}

fn op_session_probe(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let (evals, mem_mb) = match session_budget(req)? {
        Ok(budget) => budget,
        Err(refusal) => return Ok(refusal),
    };
    let candidates_value = req
        .get("candidates")
        .ok_or_else(|| "missing field `candidates`".to_string())?;
    let candidates: Vec<SessionProbeCandidate> =
        serde_json::from_value(candidates_value.clone())
            .map_err(|e| format!("candidates: {e}"))?;
    if candidates.is_empty() || candidates.len() > WASI_MAX_PROBE_CANDIDATES {
        return Ok(err_json(
            "FP_LIMIT_CANDIDATES",
            "candidates must contain between 1 and 32 entries",
        ));
    }
    if candidates.iter().any(|candidate| {
        candidate.id.len() > 256
            || candidate
                .goal
                .as_ref()
                .is_some_and(|goal| goal.len() > WASI_TEXT_FIELD_BYTES)
            || candidate.sight.len() > WASI_MAX_PROBE_OBSERVATIONS
            || candidate
                .sight
                .iter()
                .any(|(fact, _)| fact.len() > 4_096)
            || candidate
                .restrict_contains
                .as_ref()
                .is_some_and(|filter| filter.len() > 4_096)
    }) {
        return Ok(err_json(
            "FP_LIMIT_CANDIDATE",
            "candidate id, goal, observation, or restriction exceeds the WASI probe limit",
        ));
    }

    let results = with_session(handle, |parent| {
        candidates
            .iter()
            .map(|candidate| {
                let mut mind = parent.inner.fork();

                if let Some(goal) = &candidate.goal {
                    if let Err(error) = mind.set_goal(goal) {
                        return json!({
                            "id": candidate.id,
                            "outcome": "refused",
                            "stage": "set_goal",
                            "error": error,
                        });
                    }
                }

                if let Some(filter) = &candidate.restrict_contains {
                    let filter = filter.clone();
                    mind.restrict_ops(move |display| display.contains(&filter));
                }

                let surprises = if candidate.sight.is_empty() {
                    Vec::new()
                } else {
                    let refs = candidate
                        .sight
                        .iter()
                        .map(|(fact, value)| (fact.as_str(), *value))
                        .collect::<Vec<_>>();
                    match mind.observe(&refs) {
                        Ok(news) => news,
                        Err(error) => {
                            return json!({
                                "id": candidate.id,
                                "outcome": "refused",
                                "stage": "observe",
                                "error": error,
                            })
                        }
                    }
                };

                let solution = mind.replan_budgeted(evals, Some(mem_mb));
                json!({
                    "id": candidate.id,
                    "outcome": if solution.solved { "solved" } else { "unsolved" },
                    "surprises": surprises,
                    "goal_met_before_search": mind.goal_met(),
                    "world_bytes": mind.world_bytes(),
                    "mind_bytes": mind.mind_bytes(),
                    "solution": solution,
                })
            })
            .collect::<Vec<_>>()
    })?;

    Ok(json!({
        "parent_handle": handle,
        "candidate_count": results.len(),
        "results": results,
    }))
}

fn op_session_valid(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let valid = with_session(handle, |s| {
        s.plan
            .as_ref()
            .is_some_and(|p| s.inner.plan_still_valid(p, s.cursor))
    })?;
    Ok(json!({ "valid": valid }))
}

fn op_session_step(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    with_session(handle, |s| {
        match s.plan.as_ref().and_then(|p| p.steps.get(s.cursor)) {
            Some(step) => serde_json::to_value(step).unwrap_or(Value::Null),
            None => Value::Null,
        }
    })
}

fn op_session_suffix(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    with_session(handle, |s| match s.plan.as_ref() {
        Some(p) => {
            let from = s.cursor.min(p.steps.len());
            serde_json::to_value(&p.steps[from..]).unwrap_or(json!([]))
        }
        None => json!([]),
    })
}

fn op_session_advance(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    with_session(handle, |s| {
        s.cursor = s.cursor.saturating_add(1);
    })?;
    Ok(json!({ "ok": true }))
}

fn op_session_drop_plan(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    with_session(handle, |s| {
        s.plan = None;
        s.cursor = 0;
    })?;
    Ok(json!({ "ok": true }))
}

fn op_session_has_plan(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let has_plan = with_session(handle, |s| s.plan.is_some())?;
    Ok(json!({ "has_plan": has_plan }))
}

fn op_session_set_fact(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let name = field_str(req, "name")?;
    let value = req
        .get("value")
        .and_then(Value::as_bool)
        .ok_or_else(|| "missing or non-bool field `value`".to_string())?;
    bounded(name, 4_096, "fact name")?;
    with_session(handle, |s| s.inner.set_fact(name, value))?.map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true }))
}

fn op_session_set_timed_fact(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let name = field_str(req, "name")?;
    let dt = req
        .get("dt")
        .and_then(Value::as_f64)
        .ok_or_else(|| "missing or non-numeric field `dt`".to_string())?;
    let value = req
        .get("value")
        .and_then(Value::as_bool)
        .ok_or_else(|| "missing or non-bool field `value`".to_string())?;
    bounded(name, 4_096, "fact name")?;
    with_session(handle, |s| s.inner.set_timed_fact(dt, name, value))?
        .map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true }))
}

fn op_session_observe(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let sight_value = req
        .get("sight")
        .ok_or_else(|| "missing field `sight`".to_string())?;
    let sight: Vec<(String, bool)> =
        serde_json::from_value(sight_value.clone()).map_err(|e| format!("sight: {e}"))?;
    if sight.len() > 100_000 {
        return Err("observation contains too many facts".to_string());
    }
    let news = with_session(handle, |s| {
        let refs: Vec<(&str, bool)> = sight.iter().map(|(f, v)| (f.as_str(), *v)).collect();
        s.inner.observe(&refs)
    })?
    .map_err(|e| e.to_string())?;
    serde_json::to_value(news).map_err(|e| e.to_string())
}

fn op_session_goal_met(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let goal_met = with_session(handle, |s| s.inner.goal_met())?;
    Ok(json!({ "goal_met": goal_met }))
}

fn op_session_fact(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let name = field_str(req, "name")?;
    if name.len() > 4_096 {
        return Ok(json!({ "value": null }));
    }
    let value = with_session(handle, |s| s.inner.fact(name))?;
    Ok(json!({ "value": value }))
}

fn op_session_apply_start(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let name = field_str(req, "name")?;
    bounded(name, 4_096, "action")?;
    with_session(handle, |s| s.inner.apply_start(name))?.map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true }))
}

fn op_session_elapse(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let dt = req
        .get("dt")
        .and_then(Value::as_f64)
        .ok_or_else(|| "missing or non-numeric field `dt`".to_string())?;
    let fired = with_session(handle, |s| s.inner.elapse(dt))?.map_err(|e| e.to_string())?;
    serde_json::to_value(fired).map_err(|e| e.to_string())
}

fn op_session_set_fluent(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let name = field_str(req, "name")?;
    let value = req
        .get("value")
        .and_then(Value::as_f64)
        .ok_or_else(|| "missing or non-numeric field `value`".to_string())?;
    bounded(name, 4_096, "fluent name")?;
    with_session(handle, |s| s.inner.set_fluent(name, value))?.map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true }))
}

fn op_session_fluent(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let name = field_str(req, "name")?;
    if name.len() > 4_096 {
        return Ok(json!({ "value": null }));
    }
    let value = with_session(handle, |s| s.inner.fluent(name))?;
    Ok(json!({ "value": value }))
}

fn op_session_plan_valid(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let from = field_u64(req, "from")? as usize;
    let plan_value = req
        .get("plan")
        .ok_or_else(|| "missing field `plan`".to_string())?;
    if serde_json::to_string(plan_value)
        .map(|s| s.len())
        .unwrap_or(0)
        > WASI_JSON_FIELD_BYTES
    {
        return Ok(json!({ "valid": false }));
    }
    let plan: Option<ferroplan::api::Plan> = serde_json::from_value(plan_value.clone()).ok();
    let valid = match plan {
        Some(plan) => with_session(handle, |s| s.inner.plan_still_valid(&plan, from))?,
        None => false,
    };
    Ok(json!({ "valid": valid }))
}

fn op_session_world_bytes(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let bytes = with_session(handle, |s| s.inner.world_bytes())?;
    Ok(json!({ "bytes": bytes }))
}

fn op_session_mind_bytes(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let bytes = with_session(handle, |s| s.inner.mind_bytes())?;
    Ok(json!({ "bytes": bytes }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Same fixture `ferroplan::hddl::solve_hddl`'s own end-to-end test uses
    // (crates/ferroplan/src/hddl.rs) — the FOND `oneof` domain. Going
    // through `dispatch` (not calling `op_hddl_solve` directly) exercises
    // the real wire path: JSON bytes in, JSON bytes out, exactly as a
    // beam4pm host would call `fp_call`.
    const FIXTURE_C_DOMAIN: &str = include_str!("../../ferroplan-hddl/fixtures/c/domain.hddl");
    const FIXTURE_C_PROBLEM: &str = include_str!("../../ferroplan-hddl/fixtures/c/problem.hddl");

    fn dispatch_json(req: &Value) -> Value {
        let bytes = dispatch(&serde_json::to_vec(req).unwrap()).expect("dispatch must not Err");
        serde_json::from_slice(&bytes).expect("dispatch response must be valid JSON")
    }

    /// Fixture C's root "reach" task genuinely has NO valid strong FOND
    /// policy once `ferroplan_hddl::translate` is decomposition-aware: its
    /// two methods for "reach" ("m-direct", one subtask; "m-two-step",
    /// "cross-bridge" then "walk") each leave one of the oneof
    /// `cross-bridge` outcomes stuck at a non-goal terminal (m-direct's
    /// task network is already exhausted regardless of outcome; m-two-step's
    /// "walk" only applies from l3, so its "success" outcome — landing
    /// directly at l2 — cannot execute the remaining "walk" step). See
    /// `ferroplan::hddl::tests::reach_htn_with_non_covering_methods_has_no_valid_fond_policy`
    /// for the full derivation. Before decomposition-awareness, this test
    /// asserted a solved plan — that was only reachable via the exact
    /// unsound "run any precondition-satisfying action regardless of the
    /// task network" shortcut this fix eliminates, so `FP_MODEL` (wrapping
    /// `PlannerError::NoPlan`) is the corrected, honest result.
    #[test]
    fn hddl_solve_reach_htn_with_non_covering_methods_reports_no_plan() {
        let response = dispatch_json(&json!({
            "op": "hddl_solve",
            "domain": FIXTURE_C_DOMAIN,
            "problem": FIXTURE_C_PROBLEM,
        }));
        assert_eq!(response["error"]["code"], json!("FP_MODEL"));
        assert!(response["error"]["message"]
            .as_str()
            .unwrap()
            .contains("NoPlan"));
    }

    #[test]
    fn hddl_solve_respects_a_partial_limits_override() {
        let response = dispatch_json(&json!({
            "op": "hddl_solve",
            "domain": FIXTURE_C_DOMAIN,
            "problem": FIXTURE_C_PROBLEM,
            "limits": { "max_iterations": 512 },
        }));
        // Same non-covering-methods HTN as the test above -- a permissive
        // `max_iterations` override cannot manufacture a policy that does
        // not exist; the request must still reach the planner (not be
        // rejected for a request-shape reason) and report the same
        // `FP_MODEL`/`NoPlan` outcome.
        assert_eq!(response["error"]["code"], json!("FP_MODEL"));
    }

    #[test]
    fn hddl_solve_reports_a_distinguishable_parse_error() {
        let response = dispatch_json(&json!({
            "op": "hddl_solve",
            "domain": "(define (domain broken",
            "problem": FIXTURE_C_PROBLEM,
        }));
        assert_eq!(response["error"]["code"], json!("FP_PARSE"));
        assert!(response["error"]["message"]
            .as_str()
            .unwrap()
            .contains("HDDL parse error"));
    }

    #[test]
    fn hddl_error_timeout_and_worker_panic_are_distinguishable() {
        let timeout = hddl_error_json(&HddlError::Timeout {
            elapsed_ms: 101,
            limit_ms: 100,
        });
        assert_eq!(timeout["error"]["code"], json!("FP_TIMEOUT"));
        assert!(timeout["error"]["message"]
            .as_str()
            .unwrap()
            .contains("101ms"));

        let worker = hddl_error_json(&HddlError::WorkerPanicked("worker-7".to_string()));
        assert_eq!(worker["error"]["code"], json!("FP_WORKER_PANICKED"));
        assert!(worker["error"]["message"]
            .as_str()
            .unwrap()
            .contains("worker-7"));
    }

    #[test]
    fn hddl_solve_missing_domain_field_is_a_dispatch_error_not_a_panic() {
        let bytes = dispatch(
            &serde_json::to_vec(&json!({ "op": "hddl_solve", "problem": FIXTURE_C_PROBLEM }))
                .unwrap(),
        );
        let err = bytes.unwrap_err();
        assert!(
            err.contains("domain"),
            "error should name the missing field: {err}"
        );
    }

    // ------------------------------------------------------------------
    // fond-htn-17 (wave v26.9.17): `hddl_solve` / `fond_policy` / `htn_plan`
    // op coverage. Every test below goes through the real wire path
    // (`dispatch_json`, JSON bytes in / JSON bytes out), because what it
    // asserts is that the *ABI surface* preserves the solver's result shape
    // -- not just that the solver works.
    //
    // Every request passes `limits: {"max_wall_ms": 0}` explicitly. Under
    // `wasm32-wasip1` (this module's only real target) `solve_hddl`'s
    // watchdog thread cannot exist and the op already forces the synchronous
    // path, but stating it per-request keeps these tests deterministic and
    // target-portable regardless of that op-level default.
    // ------------------------------------------------------------------

    /// Hand-authored for this ticket (fond-htn-17). Provenance: a
    /// self-written micro Transport-pattern FOND-HTN in the shape of this
    /// crate's own `ferroplan-hddl/fixtures/a` (pickup/drive/dropoff); no
    /// external corpus, no koala code. The FOND core is `drive`'s
    /// two-branch `oneof` whose second branch is the EMPTY effect `(and)` —
    /// the truck's move may silently fail — so the translated problem
    /// genuinely carries a non-deterministic action with two outcomes
    /// (arrive-at-l2 / nothing-changes), and the continuation stays solvable
    /// from both (`dropoff` is deliberately location-free, so the failed
    /// branch is a live branch, not a dead end).
    const TRANSPORT_ONEOF_EMPTY_DOMAIN: &str = r#"(define (domain transport-oneof)
  (:types loc)
  (:predicates
    (at ?l - loc)
    (connected ?a - loc ?b - loc)
    (has-package)
    (delivered))
  (:task deliver :parameters (?from - loc ?to - loc))
  (:action pickup
    :parameters (?l - loc)
    :precondition (at ?l)
    :effect (and (has-package)))
  (:action drive
    :parameters (?a - loc ?b - loc)
    :precondition (and (at ?a) (connected ?a ?b))
    :effect (oneof
      (and (not (at ?a)) (at ?b))
      (and)))
  (:action dropoff
    :parameters ()
    :precondition (has-package)
    :effect (and (not (has-package)) (delivered)))
  (:method m-deliver
    :parameters (?from - loc ?to - loc)
    :task (deliver ?from ?to)
    :ordered-subtasks (and
      (t1 (pickup ?from))
      (t2 (drive ?from ?to))
      (t3 (dropoff)))))"#;

    const TRANSPORT_ONEOF_EMPTY_PROBLEM: &str = r#"(define (problem transport-oneof-p1)
  (:domain transport-oneof)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (and (delivered))))"#;

    /// (a) `hddl_solve` on the micro Transport FOND-HTN above: the ABI must
    /// hand back a solved `UniversalPlan` whose *policy shape survives the
    /// wire* — exactly one policy entry (the `drive` choice) carries the
    /// action's two `oneof` outcomes, outcome mass sums to the full
    /// probability scale, and every entry keeps a non-empty outcomes array.
    #[test]
    fn hddl_solve_transport_oneof_with_empty_branch_solves_and_preserves_outcomes() {
        let response = dispatch_json(&json!({
            "op": "hddl_solve",
            "domain": TRANSPORT_ONEOF_EMPTY_DOMAIN,
            "problem": TRANSPORT_ONEOF_EMPTY_PROBLEM,
            "limits": { "max_wall_ms": 0 },
        }));
        assert!(
            response.get("error").is_none(),
            "micro Transport oneof domain must solve, got: {response}"
        );
        assert_eq!(response["solved"], json!(true), "{response}");
        assert_eq!(response["planning_type"], json!("fond"), "{response}");

        let policy = response["policy"]
            .as_array()
            .expect("solved FOND plan carries a policy array");
        assert!(!policy.is_empty(), "policy must not be empty: {response}");

        let two_outcome_entries: Vec<&Value> = policy
            .iter()
            .filter(|entry| {
                entry["outcomes"]
                    .as_array()
                    .map(|outcomes| outcomes.len() == 2)
                    .unwrap_or(false)
            })
            .collect();
        assert_eq!(
            two_outcome_entries.len(),
            1,
            "exactly the nondeterministic drive choice may carry two outcomes: {response}"
        );
        let drive_entry = two_outcome_entries[0];
        assert!(
            drive_entry["action"]
                .as_str()
                .is_some_and(|action| action.contains("drive")),
            "the two-outcome entry must be the drive choice: {drive_entry}"
        );
        let outcomes = drive_entry["outcomes"].as_array().unwrap();
        let mass: u64 = outcomes
            .iter()
            .map(|outcome| outcome["probability_ppm"].as_u64().unwrap_or(0))
            .sum();
        assert_eq!(
            mass, 1_000_000,
            "the two oneof outcomes must preserve the full probability mass: {drive_entry}"
        );
        let mut outcome_states = outcomes
            .iter()
            .map(|outcome| outcome["state"].as_str().unwrap_or_default())
            .collect::<Vec<_>>();
        outcome_states.sort();
        outcome_states.dedup();
        assert_eq!(
            outcome_states.len(),
            2,
            "the empty branch and the move branch must land in two distinct states: {drive_entry}"
        );

        for entry in policy {
            let outcomes = entry["outcomes"]
                .as_array()
                .unwrap_or_else(|| panic!("every policy entry carries outcomes: {entry}"));
            assert!(
                !outcomes.is_empty(),
                "no policy entry may drop its outcome set on the wire: {entry}"
            );
        }
    }

    /// The `fond_policy` op's wire problem for the retry-loop fixture: one
    /// nondeterministic `flip` from `s0` that either reaches the goal or
    /// loops back onto `s0` (same shape as planning_runtime's own
    /// `retry_loop_problem` unit fixture). No acyclic strong policy exists;
    /// the op must return the strong-cyclic solution through
    /// `solve_planning_type`'s fallback.
    fn retry_loop_problem_json() -> String {
        json!({
            "states": [
                { "id": "s0" },
                { "id": "g", "facts": ["done"] },
            ],
            "initial_states": ["s0"],
            "goal": { "facts": ["done"] },
            "transitions": [
                { "action": "flip", "from": "s0", "to": "g", "probability_ppm": 500_000 },
                { "action": "flip", "from": "s0", "to": "s0", "probability_ppm": 500_000 },
            ],
        })
        .to_string()
    }

    /// (b) `fond_policy` on the retry-loop problem: solved, and the policy
    /// that comes back through the ABI is *closed* — the single `flip` entry
    /// at `s0` still carries BOTH outcomes (goal + self-loop), and every
    /// outcome state is either policy-covered (`s0`) or the goal state
    /// (`g`), so the wire policy is a total controller, not a truncated
    /// fragment.
    #[test]
    fn fond_policy_op_solves_the_retry_loop_with_a_closed_policy() {
        let response = dispatch_json(&json!({
            "op": "fond_policy",
            "problem": retry_loop_problem_json(),
            "limits": { "max_wall_ms": 0 },
        }));
        assert!(
            response.get("error").is_none(),
            "retry-loop domain must solve via the strong-cyclic fallback, got: {response}"
        );
        assert_eq!(response["solved"], json!(true), "{response}");
        assert_eq!(response["planning_type"], json!("fond"), "{response}");
        assert!(
            response["notes"][0]
                .as_str()
                .is_some_and(|note| note.contains("strong-cyclic")),
            "a self-loop-only domain must be solved by the strong-cyclic fixpoint: {response}"
        );

        let policy = response["policy"].as_array().expect("policy array");
        assert_eq!(policy.len(), 1, "only s0 needs a choice: {response}");
        let entry = &policy[0];
        assert_eq!(entry["state"], json!("s0"), "{response}");
        assert_eq!(entry["action"], json!("flip"), "{response}");
        let mut outcome_states = entry["outcomes"]
            .as_array()
            .expect("entry outcomes array")
            .iter()
            .map(|outcome| outcome["state"].as_str().unwrap_or_default())
            .collect::<Vec<_>>();
        outcome_states.sort();
        assert_eq!(
            outcome_states,
            vec!["g".to_owned(), "s0".to_owned()],
            "the ABI must preserve both the goal and the self-loop outcome: {response}"
        );
        // Closure: every outcome state is either covered by a policy entry
        // or is the goal state (which needs no entry).
        let covered: std::collections::BTreeSet<&str> = policy
            .iter()
            .map(|entry| entry["state"].as_str().unwrap_or_default())
            .collect();
        for state in &outcome_states {
            assert!(
                covered.contains(*state) || *state == "g",
                "policy must be closed: outcome {state} is neither covered nor the goal: {response}"
            );
        }
    }

    /// (b, negative) `fond_policy` on the dead-end variant: `flip`'s second
    /// outcome lands in `dead`, a non-goal state with no outgoing edges, so
    /// neither the strong nor the strong-cyclic fixpoint can cover both
    /// outcomes. Per this ABI's error convention a solver-level refusal is a
    /// *normal response* (`Ok` bytes, typed `error` envelope), never a trap
    /// or panic — and the message must still name `NoPlan`.
    #[test]
    fn fond_policy_op_reports_a_typed_no_plan_on_the_dead_end_variant() {
        let problem = json!({
            "states": [
                { "id": "s0" },
                { "id": "g", "facts": ["done"] },
                { "id": "dead" },
            ],
            "initial_states": ["s0"],
            "goal": { "facts": ["done"] },
            "transitions": [
                { "action": "flip", "from": "s0", "to": "g", "probability_ppm": 500_000 },
                { "action": "flip", "from": "s0", "to": "dead", "probability_ppm": 500_000 },
            ],
        })
        .to_string();
        let response = dispatch_json(&json!({
            "op": "fond_policy",
            "problem": problem,
            "limits": { "max_wall_ms": 0 },
        }));
        assert_eq!(
            response["error"]["code"],
            json!("FP_ADAPTER"),
            "solver-level refusals surface as the adapter error envelope: {response}"
        );
        assert!(
            response["error"]["message"]
                .as_str()
                .is_some_and(|message| message.contains("NoPlan")),
            "the NoPlan shape must survive the ABI mapping: {response}"
        );
        assert_eq!(
            response["error"]["retryable"],
            json!(false),
            "a structural dead end is not retryable: {response}"
        );
    }

    /// (c) `htn_plan` on a single-method two-level hierarchy: `top` (level 0,
    /// abstract) decomposes via its only method into `mid` (level 1, still
    /// abstract) then `p2` (primitive); `mid` decomposes into `p1`
    /// (primitive). Single-method only by design — this branch's
    /// `hierarchical_plan` is first-method (backtracking lands via a frozen
    /// branch post-wave) — and the assertion is that the decomposition ORDER
    /// survives the ABI: depth-first, `a1` before `a2`.
    #[test]
    fn htn_plan_op_preserves_the_decomposition_order_of_a_two_level_hierarchy() {
        let problem = json!({
            "tasks": [
                { "id": "top" },
                { "id": "mid" },
                { "id": "p1", "primitive_action": "a1" },
                { "id": "p2", "primitive_action": "a2" },
            ],
            "root_tasks": ["top"],
            "methods": [
                { "id": "m-top", "task": "top", "subtasks": ["mid", "p2"] },
                { "id": "m-mid", "task": "mid", "subtasks": ["p1"] },
            ],
        })
        .to_string();
        let response = dispatch_json(&json!({
            "op": "htn_plan",
            "problem": problem,
            "limits": { "max_wall_ms": 0 },
        }));
        assert!(
            response.get("error").is_none(),
            "single-method hierarchy must decompose, got: {response}"
        );
        assert_eq!(response["solved"], json!(true), "{response}");
        assert_eq!(
            response["planning_type"],
            json!("hierarchical"),
            "{response}"
        );
        assert_eq!(
            response["decomposition"],
            json!(["a1", "a2"]),
            "decomposition must come back depth-first in method order: {response}"
        );
        let steps = response["steps"].as_array().expect("steps array");
        let step_actions: Vec<&str> = steps
            .iter()
            .map(|step| step["action"].as_str().unwrap_or_default())
            .collect();
        assert_eq!(
            step_actions,
            vec!["a1", "a2"],
            "the plan steps must mirror the decomposition order: {response}"
        );
    }

    /// (d) ABI negative: a malformed HDDL string must come back as a *typed*
    /// error envelope through the normal wire path — parseable JSON bytes,
    /// `FP_PARSE`, non-retryable — never a trap or panic. Covers both the
    /// domain text and the problem text (each is parsed by its own
    /// `parse_domain`/`parse_problem` call).
    #[test]
    fn hddl_solve_malformed_hddl_text_is_a_typed_error_not_a_trap() {
        let malformed_domain = dispatch_json(&json!({
            "op": "hddl_solve",
            "domain": "(define (domain broken",
            "problem": TRANSPORT_ONEOF_EMPTY_PROBLEM,
            "limits": { "max_wall_ms": 0 },
        }));
        assert_eq!(
            malformed_domain["error"]["code"],
            json!("FP_PARSE"),
            "{malformed_domain}"
        );
        assert_eq!(malformed_domain["error"]["retryable"], json!(false));
        assert!(malformed_domain["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("HDDL parse error")));

        let malformed_problem = dispatch_json(&json!({
            "op": "hddl_solve",
            "domain": TRANSPORT_ONEOF_EMPTY_DOMAIN,
            "problem": "this is not HDDL ]]",
            "limits": { "max_wall_ms": 0 },
        }));
        assert_eq!(
            malformed_problem["error"]["code"],
            json!("FP_PARSE"),
            "{malformed_problem}"
        );
        assert!(malformed_problem["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("HDDL parse error")));
    }

    const REPAIR_DOMAIN: &str = r#"(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))"#;

    const REPAIR_PROBLEM: &str = r#"(define (problem repair)
  (:domain rooms)
  (:objects a b c - room)
  (:init (at a) (link a b) (link c b))
  (:goal (at b)))"#;

    fn repair_session() -> u64 {
        let created = dispatch_json(&json!({
            "op": "session_new",
            "domain": REPAIR_DOMAIN,
            "problem": REPAIR_PROBLEM,
        }));
        created["handle"].as_u64().expect("session handle")
    }

    #[test]
    fn session_repair_reuses_a_still_valid_suffix_without_search() {
        let handle = repair_session();
        let first = dispatch_json(&json!({
            "op": "session_think",
            "handle": handle,
            "evals": 10_000,
            "mem_mb": 64,
        }));
        assert_eq!(first["solved"], json!(true), "{first}");

        let repaired = dispatch_json(&json!({
            "op": "session_repair",
            "handle": handle,
            "evals": 10_000,
            "mem_mb": 64,
        }));
        assert_eq!(repaired["decision"], json!("reuse_suffix"), "{repaired}");
        assert_eq!(repaired["trigger"], json!("none"), "{repaired}");
        assert_eq!(repaired["plan_valid"], json!(true), "{repaired}");
        assert_eq!(repaired["solution"], Value::Null, "{repaired}");
        assert_eq!(
            repaired["previous_suffix"], repaired["suffix"],
            "zero-search reuse must preserve the exact suffix: {repaired}"
        );
        assert!(
            !repaired["suffix"].as_array().unwrap().is_empty(),
            "the fixture must carry a real remaining plan: {repaired}"
        );

        let freed = dispatch_json(&json!({"op": "session_free", "handle": handle}));
        assert_eq!(freed["freed"], json!(true));
    }

    #[test]
    fn session_repair_uses_follow_biased_replanning_after_world_drift() {
        let handle = repair_session();
        let first = dispatch_json(&json!({
            "op": "session_think",
            "handle": handle,
            "evals": 10_000,
            "mem_mb": 64,
        }));
        assert_eq!(first["solved"], json!(true), "{first}");

        let observed = dispatch_json(&json!({
            "op": "session_observe",
            "handle": handle,
            "sight": [["(at a)", false], ["(at c)", true]],
        }));
        assert!(
            observed.as_array().is_some_and(|news| !news.is_empty()),
            "the drift must be a real Ferroplan surprise: {observed}"
        );

        let valid = dispatch_json(&json!({"op": "session_valid", "handle": handle}));
        assert_eq!(valid["valid"], json!(false), "{valid}");

        let repaired = dispatch_json(&json!({
            "op": "session_repair",
            "handle": handle,
            "evals": 10_000,
            "mem_mb": 64,
        }));
        assert_eq!(
            repaired["decision"],
            json!("replanned_following"),
            "{repaired}"
        );
        assert_eq!(repaired["trigger"], json!("invalid_plan"), "{repaired}");
        assert_eq!(repaired["plan_valid"], json!(false), "{repaired}");
        assert_eq!(repaired["solution"]["solved"], json!(true), "{repaired}");
        assert!(
            !repaired["previous_suffix"].as_array().unwrap().is_empty(),
            "the old candidate must remain visible for lineage: {repaired}"
        );
        assert!(
            !repaired["suffix"].as_array().unwrap().is_empty(),
            "the replacement candidate must be stashed: {repaired}"
        );

        let valid_after = dispatch_json(&json!({"op": "session_valid", "handle": handle}));
        assert_eq!(valid_after["valid"], json!(true), "{valid_after}");

        let freed = dispatch_json(&json!({"op": "session_free", "handle": handle}));
        assert_eq!(freed["freed"], json!(true));
    }

    #[test]
    fn session_repair_with_no_prior_plan_uses_bounded_full_replan() {
        let handle = repair_session();
        let repaired = dispatch_json(&json!({
            "op": "session_repair",
            "handle": handle,
            "evals": 10_000,
            "mem_mb": 64,
        }));
        assert_eq!(repaired["decision"], json!("replanned_full"), "{repaired}");
        assert_eq!(repaired["trigger"], json!("no_plan"), "{repaired}");
        assert_eq!(repaired["solution"]["solved"], json!(true), "{repaired}");

        let freed = dispatch_json(&json!({"op": "session_free", "handle": handle}));
        assert_eq!(freed["freed"], json!(true));
    }

    #[test]
    fn session_probe_evaluates_counterfactual_forks_without_mutating_the_parent() {
        let handle = repair_session();

        let probed = dispatch_json(&json!({
            "op": "session_probe",
            "handle": handle,
            "evals": 10_000,
            "mem_mb": 64,
            "candidates": [
                {
                    "id": "baseline",
                    "goal": "(at b)"
                },
                {
                    "id": "counterfactual-c",
                    "goal": "(at b)",
                    "sight": [["(at a)", false], ["(at c)", true]]
                },
                {
                    "id": "unreachable-c",
                    "goal": "(at c)"
                }
            ]
        }));
        assert_eq!(probed["candidate_count"], json!(3), "{probed}");
        let results = probed["results"].as_array().expect("probe results");
        assert_eq!(results[0]["outcome"], json!("solved"), "{probed}");
        assert_eq!(results[1]["outcome"], json!("solved"), "{probed}");
        assert!(
            results[1]["surprises"]
                .as_array()
                .is_some_and(|news| !news.is_empty()),
            "the counterfactual must differ from parent belief: {probed}"
        );
        assert_eq!(results[2]["outcome"], json!("unsolved"), "{probed}");

        let parent_plan = dispatch_json(&json!({
            "op": "session_has_plan",
            "handle": handle
        }));
        assert_eq!(
            parent_plan["has_plan"],
            json!(false),
            "probing must not stash a candidate into the parent: {parent_plan}"
        );
        let parent_a = dispatch_json(&json!({
            "op": "session_fact",
            "handle": handle,
            "name": "(at a)"
        }));
        let parent_c = dispatch_json(&json!({
            "op": "session_fact",
            "handle": handle,
            "name": "(at c)"
        }));
        assert_eq!(parent_a["value"], json!(true), "{parent_a}");
        assert_eq!(parent_c["value"], json!(false), "{parent_c}");

        let freed = dispatch_json(&json!({"op": "session_free", "handle": handle}));
        assert_eq!(freed["freed"], json!(true));
    }

    #[test]
    fn session_probe_refuses_oversized_candidate_sets_before_forking() {
        let handle = repair_session();
        let candidates = (0..=WASI_MAX_PROBE_CANDIDATES)
            .map(|i| json!({"id": format!("c{i}")}))
            .collect::<Vec<_>>();
        let refused = dispatch_json(&json!({
            "op": "session_probe",
            "handle": handle,
            "evals": 10_000,
            "mem_mb": 64,
            "candidates": candidates
        }));
        assert_eq!(
            refused["error"]["code"],
            json!("FP_LIMIT_CANDIDATES"),
            "{refused}"
        );

        let parent_plan = dispatch_json(&json!({
            "op": "session_has_plan",
            "handle": handle
        }));
        assert_eq!(parent_plan["has_plan"], json!(false));

        let freed = dispatch_json(&json!({"op": "session_free", "handle": handle}));
        assert_eq!(freed["freed"], json!(true));
    }

    // ------------------------------------------------------------------
    // fond-htn-29 (wave v26.9.17): 40-thread mixed-workload concurrency
    // stress through the dispatch surface.
    //
    // WHY HOST-SIDE DISPATCH, NOT ONE WASMTIME ENGINE PER THREAD:
    // wasm32-wasip1 — this module's only real target — has no OS threads:
    // `std::thread::spawn` inside the guest traps the whole instance (the
    // same hazard `op_hddl_solve` documents for `solve_hddl`'s watchdog
    // thread), and the wave's established gate (`wasmtime run` over
    // `cargo test --target wasm32-wasip1`) executes tests inside ONE
    // single-threaded guest, so a guest can never drive 40 workers itself.
    // A per-thread wasmtime Engine on the host side was also rejected: it
    // would stress wasmtime's loader, not ferroplan, and would drag a
    // heavyweight `wasmtime` dev-dependency plus an in-test guest build
    // into every host gate run. The production host (beam4pm/wasmex)
    // already owns instance creation — one fresh instance per call — so
    // the shared component that actually runs concurrently is THIS
    // dispatch code: byte-for-byte the same Rust that compiles into the
    // wasm guest. These tests drive it from 40 real host threads;
    // `not(target_family = "wasm")` keeps them out of the wasip1 build,
    // where threads cannot exist.
    // ------------------------------------------------------------------
    #[cfg(not(target_family = "wasm"))]
    mod concurrency_stress {
        use super::super::dispatch;
        use super::{
            retry_loop_problem_json, TRANSPORT_ONEOF_EMPTY_DOMAIN, TRANSPORT_ONEOF_EMPTY_PROBLEM,
        };
        use serde_json::{json, Value};
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use std::time::{Duration, Instant};

        /// Ticket scope: N=40 worker threads, 100 iterations per thread.
        const THREADS: usize = 40;
        const ITERS_PER_THREAD: usize = 100;
        /// Ticket invariant: no deadlock, test wall <= 120 s.
        const WALL_BUDGET: Duration = Duration::from_secs(120);

        // -- fixtures (hand-authored here, same shapes as the single-threaded
        // op tests above; no external corpus) --

        /// fond_policy dead-end variant: `flip`'s second outcome lands in
        /// `dead`, a non-goal sink, so the solver must refuse with a typed
        /// NoPlan error envelope (identical input to
        /// `fond_policy_op_reports_a_typed_no_plan_on_the_dead_end_variant`).
        fn fond_dead_end_problem_json() -> String {
            json!({
                "states": [
                    { "id": "s0" },
                    { "id": "g", "facts": ["done"] },
                    { "id": "dead" },
                ],
                "initial_states": ["s0"],
                "goal": { "facts": ["done"] },
                "transitions": [
                    { "action": "flip", "from": "s0", "to": "g", "probability_ppm": 500_000 },
                    { "action": "flip", "from": "s0", "to": "dead", "probability_ppm": 500_000 },
                ],
            })
            .to_string()
        }

        /// htn_plan single-method two-level chain (identical input to
        /// `htn_plan_op_preserves_the_decomposition_order_of_a_two_level_hierarchy`).
        fn htn_chain_problem_json() -> String {
            json!({
                "tasks": [
                    { "id": "top" },
                    { "id": "mid" },
                    { "id": "p1", "primitive_action": "a1" },
                    { "id": "p2", "primitive_action": "a2" },
                ],
                "root_tasks": ["top"],
                "methods": [
                    { "id": "m-top", "task": "top", "subtasks": ["mid", "p2"] },
                    { "id": "m-mid", "task": "mid", "subtasks": ["p1"] },
                ],
            })
            .to_string()
        }

        // -- wire helpers --

        /// One wire call: JSON bytes in, RAW response bytes out (no `Value`
        /// round-trip), so byte-stability is asserted on exactly what a
        /// beam4pm host would read out of linear memory.
        fn call(req: &Value) -> Vec<u8> {
            dispatch(&serde_json::to_vec(req).expect("request must serialize"))
                .expect("dispatch must not Err for a well-formed request")
        }

        fn hddl_solve_request() -> Value {
            json!({
                "op": "hddl_solve",
                "domain": TRANSPORT_ONEOF_EMPTY_DOMAIN,
                "problem": TRANSPORT_ONEOF_EMPTY_PROBLEM,
                "limits": { "max_wall_ms": 0 },
            })
        }

        fn fond_retry_request() -> Value {
            json!({
                "op": "fond_policy",
                "problem": retry_loop_problem_json(),
                "limits": { "max_wall_ms": 0 },
            })
        }

        fn fond_dead_end_request() -> Value {
            json!({
                "op": "fond_policy",
                "problem": fond_dead_end_problem_json(),
                "limits": { "max_wall_ms": 0 },
            })
        }

        fn htn_chain_request() -> Value {
            json!({
                "op": "htn_plan",
                "problem": htn_chain_problem_json(),
                "limits": { "max_wall_ms": 0 },
            })
        }

        // -- goldens --

        struct Goldens {
            hddl_solve: Vec<u8>,
            fond_retry: Vec<u8>,
            fond_dead_end: Vec<u8>,
            htn_chain: Vec<u8>,
        }

        /// Single-threaded goldens, computed in the test's own thread BEFORE
        /// any worker exists. The double-compute pre-flight falsifies
        /// single-threaded nondeterminism first, so a later cross-thread
        /// byte mismatch can only mean concurrency trouble, never an
        /// unstable serializer hiding behind the thread count.
        fn compute_goldens() -> Goldens {
            let build = || Goldens {
                hddl_solve: call(&hddl_solve_request()),
                fond_retry: call(&fond_retry_request()),
                fond_dead_end: call(&fond_dead_end_request()),
                htn_chain: call(&htn_chain_request()),
            };
            let first = build();
            let second = build();
            assert_eq!(
                first.hddl_solve, second.hddl_solve,
                "single-threaded hddl_solve is not byte-stable -- a determinism finding on its own; golden comparison would be meaningless"
            );
            assert_eq!(
                first.fond_retry, second.fond_retry,
                "single-threaded fond_policy(retry) is not byte-stable"
            );
            assert_eq!(
                first.fond_dead_end, second.fond_dead_end,
                "single-threaded fond_policy(dead-end) is not byte-stable"
            );
            assert_eq!(
                first.htn_chain, second.htn_chain,
                "single-threaded htn_plan is not byte-stable"
            );
            first
        }

        /// Cross-contamination check: every worker response must equal the
        /// pre-spawn single-threaded golden BYTE FOR BYTE. Any planner,
        /// registry, or serializer state leaking between threads shows up
        /// here as a divergence with the offending thread, iteration, and
        /// op named.
        fn assert_matches_golden(
            actual: &[u8],
            golden: &[u8],
            thread: usize,
            iter: usize,
            op: &str,
        ) {
            if actual != golden {
                panic!(
                    "fond-htn-29 cross-thread divergence: thread {thread} iteration {iter} op {op} \
                     expected {} bytes {:?}, got {} bytes {:?}",
                    golden.len(),
                    String::from_utf8_lossy(golden),
                    actual.len(),
                    String::from_utf8_lossy(actual)
                );
            }
        }

        /// Deadlock falsifier: the ticket bounds the test wall at 120 s and
        /// Rust's harness has no per-test timeout, so a detached monitor
        /// hard-exits the test binary if the budget blows — a hang must
        /// fail the gate loudly, never wedge it.
        fn arm_deadlock_watchdog(done: Arc<AtomicBool>) {
            std::thread::spawn(move || {
                let start = Instant::now();
                while !done.load(Ordering::Relaxed) {
                    if start.elapsed() > WALL_BUDGET {
                        eprintln!(
                            "fond-htn-29: concurrency stress exceeded the {WALL_BUDGET:?} wall \
                             budget — deadlock falsifier tripped"
                        );
                        std::process::exit(101);
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            });
        }

        /// The mixed workload: hddl_solve (micro Transport oneof-empty
        /// FOND-HTN), fond_policy on the retry loop (strong-cyclic solve),
        /// fond_policy on the dead-end variant (typed NoPlan refusal), and
        /// htn_plan on the single-method chain — interleaved round-robin,
        /// staggered by thread id so at any instant different threads sit in
        /// different ops (including two DISTINCT fond inputs alternating, so
        /// a cross-thread response mixup cannot pass unnoticed).
        fn mixed_round_robin(thread: usize, goldens: &Goldens) {
            for iter in 0..ITERS_PER_THREAD {
                match (thread + iter) % 4 {
                    0 => {
                        let out = call(&hddl_solve_request());
                        assert_matches_golden(
                            &out,
                            &goldens.hddl_solve,
                            thread,
                            iter,
                            "hddl_solve",
                        );
                    }
                    1 => {
                        let out = call(&fond_retry_request());
                        assert_matches_golden(
                            &out,
                            &goldens.fond_retry,
                            thread,
                            iter,
                            "fond_policy/retry",
                        );
                    }
                    2 => {
                        let out = call(&fond_dead_end_request());
                        assert_matches_golden(
                            &out,
                            &goldens.fond_dead_end,
                            thread,
                            iter,
                            "fond_policy/dead-end",
                        );
                    }
                    _ => {
                        let out = call(&htn_chain_request());
                        assert_matches_golden(&out, &goldens.htn_chain, thread, iter, "htn_plan");
                    }
                }
            }
        }

        /// Ticket scope 1+2: 40 threads x 100 mixed round-robin iterations;
        /// every result byte-matches the single-threaded golden; wall under
        /// 120 s (watchdog enforced, then asserted post-hoc for the record).
        #[test]
        fn stress_forty_threads_mixed_round_robin_every_result_matches_the_single_threaded_golden()
        {
            let goldens = compute_goldens();
            let done = Arc::new(AtomicBool::new(false));
            arm_deadlock_watchdog(done.clone());
            let started = Instant::now();
            std::thread::scope(|scope| {
                let goldens = &goldens;
                for thread in 0..THREADS {
                    scope.spawn(move || mixed_round_robin(thread, goldens));
                }
            });
            let wall = started.elapsed();
            done.store(true, Ordering::Relaxed);
            assert!(
                wall < WALL_BUDGET,
                "stress wall {wall:?} must stay under the ticket's 120 s budget"
            );
        }

        /// Ticket scope 3 (adversarial interleave): while 20 threads run the
        /// full mixed solving workload, the other 20 hammer the dispatcher
        /// with malformed traffic. Every fault must surface as a TYPED
        /// outcome — a dispatch `Err` naming the failing stage, or parseable
        /// JSON error-envelope bytes with the right code and
        /// `retryable:false` — never a panic, never a silent success; and
        /// after the error barrage each error thread must still get a
        /// byte-perfect success, proving error traffic poisons nothing.
        #[test]
        fn stress_half_malformed_traffic_cannot_corrupt_the_adjacent_solving_threads() {
            let goldens = compute_goldens();
            let done = Arc::new(AtomicBool::new(false));
            arm_deadlock_watchdog(done.clone());
            let started = Instant::now();
            std::thread::scope(|scope| {
                let goldens = &goldens;
                for thread in 0..THREADS {
                    scope.spawn(move || {
                        if thread % 2 == 0 {
                            for iter in 0..ITERS_PER_THREAD {
                                match iter % 4 {
                                    0 => {
                                        // truncated, non-JSON request bytes
                                        let err = dispatch(b"{\"op\":")
                                            .expect_err("truncated JSON must be a dispatch Err");
                                        assert!(
                                            err.contains("request JSON"),
                                            "thread {thread} iter {iter}: Err must name the JSON \
                                             parse stage: {err}"
                                        );
                                    }
                                    1 => {
                                        // unknown op: typed refusal envelope
                                        let out = call(&json!({ "op": "no-such-op" }));
                                        let v: Value = serde_json::from_slice(&out)
                                            .expect("unknown-op refusal must still be JSON bytes");
                                        assert_eq!(
                                            v["error"]["code"],
                                            json!("FP_UNKNOWN_OP"),
                                            "thread {thread} iter {iter}: {v}"
                                        );
                                        assert_eq!(v["error"]["retryable"], json!(false), "{v}");
                                    }
                                    2 => {
                                        // malformed HDDL domain text: FP_PARSE
                                        let out = call(&json!({
                                            "op": "hddl_solve",
                                            "domain": "(define (domain broken",
                                            "problem": TRANSPORT_ONEOF_EMPTY_PROBLEM,
                                            "limits": { "max_wall_ms": 0 },
                                        }));
                                        let v: Value = serde_json::from_slice(&out).expect(
                                            "a malformed-HDDL response must be JSON bytes, not a trap",
                                        );
                                        assert_eq!(
                                            v["error"]["code"],
                                            json!("FP_PARSE"),
                                            "thread {thread} iter {iter}: {v}"
                                        );
                                        assert_eq!(v["error"]["retryable"], json!(false), "{v}");
                                    }
                                    _ => {
                                        // fond_policy whose problem text is not
                                        // PlanningProblem JSON: dispatch Err naming the field
                                        let err = dispatch(
                                            &serde_json::to_vec(&json!({
                                                "op": "fond_policy",
                                                "problem": "not a problem document {{{",
                                                "limits": { "max_wall_ms": 0 },
                                            }))
                                            .expect("request must serialize"),
                                        )
                                        .expect_err(
                                            "unparseable problem text must be a dispatch Err",
                                        );
                                        assert!(
                                            err.contains("invalid PlanningProblem JSON"),
                                            "thread {thread} iter {iter}: Err must name the \
                                             failing field: {err}"
                                        );
                                    }
                                }
                            }
                            // post-barrage liveness: the same thread that just
                            // sent 100 malformed requests must still get a
                            // byte-perfect success
                            let out = call(&fond_retry_request());
                            assert_matches_golden(
                                &out,
                                &goldens.fond_retry,
                                thread,
                                ITERS_PER_THREAD,
                                "fond_policy/retry-after-errors",
                            );
                        } else {
                            mixed_round_robin(thread, goldens);
                        }
                    });
                }
            });
            let wall = started.elapsed();
            done.store(true, Ordering::Relaxed);
            assert!(
                wall < WALL_BUDGET,
                "adversarial stress wall {wall:?} must stay under the ticket's 120 s budget"
            );
        }
    }
}
