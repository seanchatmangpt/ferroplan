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
//!     `FP_HDDL_TRANSLATE` (ground IR -> planning-runtime IR failed), or
//!     `FP_MODEL` (the FOND solver itself rejected the translated problem).
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

use ferroplan::{
    capability_manifest, solve, solve_hddl, solve_production, HddlError, Mode, Options,
    PlannerLimits, ProductionLimits, Search,
};
use ferroplan::planning_runtime::{solve_planning_type, PlanningProblem, UniversalPlanningRequest};
use ferroplan::planning_types::PlanningType;
use serde::Deserialize;
use serde_json::{json, Value};
use std::alloc::Layout;
use std::collections::HashMap;
use std::sync::Mutex;

const WASI_TEXT_FIELD_BYTES: usize = 1024 * 1024;
const WASI_JSON_FIELD_BYTES: usize = 16 * 1024 * 1024;
const WASI_MAX_THINK_EVALS: usize = 1_000_000;
const WASI_MAX_THINK_MEMORY_MB: usize = 2_048;

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
/// `fp_call`.
#[no_mangle]
pub extern "C" fn fp_dealloc(ptr: *mut u8, len: usize) {
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
#[no_mangle]
pub extern "C" fn fp_call(ptr: *mut u8, len: usize) -> u64 {
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
            return Ok(serde_json::to_vec(&err_json(
                "FP_UNKNOWN_OP",
                &format!("unknown op `{other}`"),
            ))
            .map_err(|e| e.to_string())?)
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
    let limits: PlannerLimits = match req.get("limits") {
        Some(v) => serde_json::from_value(v.clone()).map_err(|e| format!("limits: {e}"))?,
        None => PlannerLimits::default(),
    };
    match solve_hddl(domain, problem, &limits) {
        Ok(plan) => serde_json::to_value(plan).map_err(|e| e.to_string()),
        Err(e) => Ok(hddl_error_json(&e)),
    }
}

/// Map each `HddlError` variant to its own distinguishable error code —
/// which pipeline stage failed (parse/ground/translate) is diagnostic
/// information a caller needs, not something to collapse into one generic
/// message.
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

fn op_session_think(req: &Value) -> Result<Value, String> {
    let handle = field_u64(req, "handle")?;
    let evals = req
        .get("evals")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing field `evals`".to_string())? as usize;
    let mem_mb = req
        .get("mem_mb")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing field `mem_mb`".to_string())? as usize;
    if evals == 0 || evals > WASI_MAX_THINK_EVALS {
        return Ok(err_json(
            "FP_LIMIT_SEARCH",
            "evals must be within the WASI production budget",
        ));
    }
    if mem_mb == 0 || mem_mb > WASI_MAX_THINK_MEMORY_MB {
        return Ok(err_json(
            "FP_LIMIT_MEMORY",
            "mem_mb must be within the WASI production budget",
        ));
    }
    let sol = with_session(handle, |s| {
        let sol = s.inner.replan_budgeted(evals, Some(mem_mb));
        s.plan = if sol.solved { sol.plan.clone() } else { None };
        s.cursor = 0;
        sol
    })?;
    serde_json::to_value(sol).map_err(|e| e.to_string())
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
}
