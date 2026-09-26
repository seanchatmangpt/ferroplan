//! WebAssembly bindings — smuggle the planner past the glass, run it inside
//! the browser's own skull.
//!
//! [`plan`] is the historical entry: hand it a domain, a problem, an optional
//! mode/thread pick, and it hands back the structured [`ferroplan::Solution`]
//! as JSON. No threads out here — the sandbox doesn't allow it — so every
//! solve runs with `threads = 1`; the lib's data-parallel map just falls
//! back to a single lane, same result, no shortcuts skipped. It is retained
//! for compatibility; new hosts should use [`plan_production`], which returns
//! the same bounded, typed, candidate-only operation envelope exposed by the
//! native adapters.
//!
//! Build: `cargo build -p ferroplan-wasm --release --target wasm32-unknown-unknown`
//! then `wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/release/ferroplan_wasm.wasm`.
//! `web/index.html` runs the whole rig standalone — a self-contained demo.
//!
//! A second target lives alongside this browser one: `cargo build -p
//! ferroplan-wasm --release --target wasm32-wasip1` builds a plain
//! linear-memory JSON ABI (no wasm-bindgen, no JS glue) for a BEAM host
//! (wasmex/wasmtime) — see [`wasi_abi`], which mirrors beam4pm's own
//! `rust4pm-wasm` adapter shape exactly (`fp_alloc`/`fp_call`/`fp_dealloc`,
//! same handle-registry discipline). The two targets are mutually
//! exclusive at compile time (`cfg(target_os = "unknown")` vs `cfg(target_os
//! = "wasi")`); this file's `#[wasm_bindgen]` surface only compiles for the
//! browser target.

#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
pub mod wasi_abi;

#[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
use ferroplan::{
    capability_manifest, solve, solve_production, validate_fond_policy, Mode, Options,
    PlanningProblem, ProductionLimits, Search, UniversalPlan,
};
#[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
use wasm_bindgen::prelude::*;

#[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
pub use browser_impl::{
    explain, fond_validate, plan, plan_production, readiness, version, WasmSession,
};

#[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
mod browser_impl {
    use super::*;

    const WASM_HARD_INPUT_BYTES: usize = 64 * 1024 * 1024;
    const WASM_TEXT_FIELD_BYTES: usize = 1024 * 1024;
    const WASM_JSON_FIELD_BYTES: usize = 16 * 1024 * 1024;
    const WASM_MAX_THINK_EVALS: usize = 1_000_000;
    const WASM_MAX_THINK_MEMORY_MB: usize = 2_048;
    const WASM_MAX_PROBE_CANDIDATES: usize = 32;
    const WASM_MAX_PROBE_OBSERVATIONS: usize = 1_024;

    macro_rules! serialize_or_error {
        ($value:expr) => {
            serde_json::to_string($value)
                .unwrap_or_else(|error| err_json("FP_ADAPTER", &format!("serialize: {error}")))
        };
    }

    /// Compatibility solve surface. Comes back as a `Solution` in JSON, or
    /// `{"error": "..."}` when the parse or the search dies. `mode` reads "auto",
    /// "ff", "pddl3", "partition", "temporal" — case doesn't matter, unknown
    /// falls to Auto, and Auto already knows to route durative-action work to
    /// the temporal solver.
    ///
    /// `flags` is a comma-separated switchboard of the planner's env-gated
    /// feature toggles for this one solve (e.g. "tdemand,tdecomp"): `tdemand`
    /// wires in converging-resource demand guidance plus goal-relevance
    /// pruning; `tdecomp` is the partition-and-resolve decomposer — the tool
    /// the genuinely hard temporal jobs actually need. These live as env vars
    /// in the core lib, but WASM runs single-threaded, so we flip them
    /// in-process here and reset the whole set on every call — no leftover
    /// setting bleeds from one solve into the next.
    #[wasm_bindgen]
    pub fn plan(
        domain: &str,
        problem: &str,
        mode: Option<String>,
        flags: Option<String>,
        search: Option<String>,
    ) -> String {
        if let Err(message) = ensure_compat_input(domain, problem) {
            return err_json("FP_LIMIT_INPUT", &message);
        }
        apply_flags(flags.as_deref());
        let opts = Options {
            mode: parse_mode(mode.as_deref()),
            search: parse_search(search.as_deref()),
            threads: 1,
            ..Default::default()
        };
        match solve(domain, problem, &opts) {
            Ok(sol) => serialize_or_error!(&sol),
            Err(e) => err_json("FP_ADAPTER", &e.to_string()),
        }
    }

    /// Bounded browser production solve. The returned JSON is a versioned
    /// `OperationEnvelope<Solution>` and is always candidate-only.
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub fn plan_production(
        domain: &str,
        problem: &str,
        mode: Option<String>,
        search: Option<String>,
        max_evaluated: Option<usize>,
        max_plan_steps: Option<usize>,
        max_output_bytes: Option<usize>,
        request_id: Option<String>,
    ) -> String {
        let mode = match parse_mode_strict(mode.as_deref()) {
            Ok(mode) => mode,
            Err(message) => return adapter_refusal_json(request_id.as_deref(), &message),
        };
        let search = match parse_search_strict(search.as_deref()) {
            Ok(search) => search,
            Err(message) => return adapter_refusal_json(request_id.as_deref(), &message),
        };
        let defaults = ProductionLimits::default();
        let limits = ProductionLimits {
            max_evaluated: max_evaluated.unwrap_or(defaults.max_evaluated),
            max_plan_steps: max_plan_steps.unwrap_or(defaults.max_plan_steps),
            max_output_bytes: max_output_bytes.unwrap_or(defaults.max_output_bytes),
            max_workers: 1,
            ..defaults
        };
        let options = Options {
            mode,
            search,
            threads: 1,
            max_evaluated,
            ..Default::default()
        };
        serialize_or_error!(&solve_production(
            domain,
            problem,
            &options,
            &limits,
            request_id.as_deref(),
        ))
    }

    /// Independently validate a FOND UniversalPlan against the exact
    /// PlanningProblem JSON. Evidence only; no policy action is selected or
    /// executed by this function.
    #[wasm_bindgen]
    pub fn fond_validate(problem_json: &str, plan_json: &str) -> String {
        if problem_json.len() > WASM_JSON_FIELD_BYTES || plan_json.len() > WASM_JSON_FIELD_BYTES {
            return err_json(
                "FP_LIMIT_INPUT",
                "problem or plan exceeds the browser JSON limit",
            );
        }
        let problem: PlanningProblem = match serde_json::from_str(problem_json) {
            Ok(problem) => problem,
            Err(error) => {
                return err_json(
                    "FP_INVALID_PROBLEM",
                    &format!("problem: invalid PlanningProblem JSON: {error}"),
                )
            }
        };
        let plan: UniversalPlan = match serde_json::from_str(plan_json) {
            Ok(plan) => plan,
            Err(error) => {
                return err_json(
                    "FP_INVALID_POLICY",
                    &format!("plan: invalid UniversalPlan JSON: {error}"),
                )
            }
        };
        serialize_or_error!(&validate_fond_policy(&problem, &plan))
    }

    /// Canonical capability contract and deterministic manifest fingerprint.
    #[wasm_bindgen]
    pub fn readiness() -> String {
        let manifest = capability_manifest();
        match manifest.fingerprint() {
            Ok(fingerprint) => serde_json::json!({
                "schema_version": "ferroplan.readiness-contract.v1",
                "product_version": env!("CARGO_PKG_VERSION"),
                "manifest_fingerprint": fingerprint,
                "contract_valid": true,
                "admission_state": "declared",
                "admission_notice": "Admission is verifier-derived from exact-source evidence.",
                "manifest": manifest,
            })
            .to_string(),
            Err(error) => err_json("FP_INVARIANT", &error.to_string()),
        }
    }

    /// Translate the demo's short feature names into ferroplan's feature
    /// overrides for this solve — env vars panic on wasm, so the in-process
    /// override stands in. Resets the whole managed set on every call; nothing
    /// from the last pick rides along into this one.
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

    /// The build's serial number, stamped in the demo footer.
    #[wasm_bindgen]
    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Explain a plan for its domain + problem.
    #[wasm_bindgen]
    pub fn explain(domain: &str, problem: &str, plan_json: &str) -> String {
        if let Err(message) = ensure_compat_input(domain, problem) {
            return err_json("FP_LIMIT_INPUT", &message);
        }
        if plan_json.len() > WASM_JSON_FIELD_BYTES {
            return err_json(
                "FP_LIMIT_INPUT",
                "plan JSON exceeds the browser adapter input limit",
            );
        }
        let plan: ferroplan::api::Plan = match serde_json::from_str(plan_json) {
            Ok(p) => p,
            Err(e) => return err_json("FP_PARSE", &format!("plan: {e}")),
        };
        match ferroplan::introspect::explain(domain, problem, &plan) {
            Ok(ex) => serialize_or_error!(&ex),
            Err(e) => err_json("FP_VALIDATION", &e),
        }
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

    /// Translate the demo's search names into [`Search`]; unknown or `auto`
    /// hands the wheel back to the engine's own default.
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

    fn ensure_compat_input(domain: &str, problem: &str) -> Result<(), String> {
        if domain.len() > WASM_HARD_INPUT_BYTES || problem.len() > WASM_HARD_INPUT_BYTES {
            return Err("domain or problem exceeds the browser hard input limit".to_string());
        }
        Ok(())
    }

    fn adapter_refusal_json(request_id: Option<&str>, message: &str) -> String {
        serde_json::json!({
            "schema_version": "ferroplan.operation.v1",
            "request_id": request_id.unwrap_or("wasm-adapter-refusal"),
            "capability_id": "fp.wasm",
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
        .to_string()
    }

    fn err_json(code: &str, msg: &str) -> String {
        serde_json::json!({
            "error": {
                "code": code,
                "message": msg,
                "retryable": false
            }
        })
        .to_string()
    }

    /// A live [`ferroplan::Session`], caged for the browser: the in-page
    /// bazaar runs real minds — fork, scope, think, observe — all
    /// client-side, no round trip to a server. This wrapper keeps the mind's
    /// CURRENT PLAN and cursor so the JS loop matches the native `bazaar_live`
    /// shape exactly: think stashes the plan, `valid()` is a free replay of
    /// the suffix, `step_json()` / `advance()` walk it one beat at a time. All
    /// untrusted string/JSON boundaries are explicitly bounded.
    #[derive(serde::Deserialize)]
    struct BrowserProbeCandidate {
        id: String,
        goal: Option<String>,
        #[serde(default)]
        sight: Vec<(String, bool)>,
        restrict_contains: Option<String>,
    }

    #[wasm_bindgen]
    pub struct WasmSession {
        inner: ferroplan::Session,
        plan: Option<ferroplan::api::Plan>,
        cursor: usize,
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = error)]
        fn js_console_error(s: &str);
    }

    #[wasm_bindgen]
    impl WasmSession {
        /// Ground a world from nothing. Failure comes back as a JS string.
        #[wasm_bindgen(constructor)]
        pub fn new(domain: &str, problem: &str) -> Result<WasmSession, JsValue> {
            ensure_js_len(
                domain,
                ProductionLimits::default().max_domain_bytes,
                "domain",
            )?;
            ensure_js_len(
                problem,
                ProductionLimits::default().max_problem_bytes,
                "problem",
            )?;
            std::panic::set_hook(Box::new(|info| {
                js_console_error(&format!("wasm panic: {info}"));
            }));
            let opts = Options {
                threads: 1,
                max_evaluated: Some(ProductionLimits::default().max_evaluated),
                ..Default::default()
            };
            Ok(WasmSession {
                inner: ferroplan::Session::new(domain, problem, &opts).map_err(js_err)?,
                plan: None,
                cursor: 0,
            })
        }

        /// A cheap mind — shares the grounded world, keeps its own private
        /// state.
        pub fn fork(&self) -> WasmSession {
            WasmSession {
                inner: self.inner.fork(),
                plan: None,
                cursor: 0,
            }
        }

        pub fn set_goal(&mut self, goal: &str) -> Result<(), JsValue> {
            ensure_js_len(goal, WASM_TEXT_FIELD_BYTES, "goal")?;
            self.inner.set_goal(goal).map_err(js_err)
        }

        /// Scope one actor's moves by op-display prefix — the bazaar's
        /// `restrict_ops` shape (`"TRADE ALICE "`) — then blackout anything
        /// whose 5th token, the item RECEIVED, shows up in `claimed`
        /// (comma-separated; empty means nobody's staked a claim yet). The
        /// loop-side claims policy, run right here on the glass.
        pub fn restrict_prefix_claims(&mut self, prefix: String, claimed: String) {
            let prefix = bounded_string(prefix, 4_096);
            let claimed = bounded_string(claimed, WASM_TEXT_FIELD_BYTES);
            let claimed: std::collections::HashSet<String> = claimed
                .split(',')
                .map(|s| s.trim().to_ascii_uppercase())
                .filter(|s| !s.is_empty())
                .collect();
            self.inner.restrict_ops(move |d| {
                d.starts_with(&prefix)
                    && d.split_whitespace()
                        .nth(4)
                        .map(|y| !claimed.contains(y.trim_end_matches(')')))
                        .unwrap_or(true)
            });
        }

        /// A bounded think — burns its budget, stashes the plan internally,
        /// hands back the whole `Solution` as JSON for the display to chew on.
        pub fn think(&mut self, evals: usize, mem_mb: usize) -> String {
            if evals == 0 || evals > WASM_MAX_THINK_EVALS {
                return err_json(
                    "FP_LIMIT_SEARCH",
                    "evals must be within the browser production budget",
                );
            }
            if mem_mb == 0 || mem_mb > WASM_MAX_THINK_MEMORY_MB {
                return err_json(
                    "FP_LIMIT_MEMORY",
                    "mem_mb must be within the browser production budget",
                );
            }
            let sol = self.inner.replan_budgeted(evals, Some(mem_mb));
            self.plan = if sol.solved { sol.plan.clone() } else { None };
            self.cursor = 0;
            serialize_or_error!(&sol)
        }

        /// Follow-before-rethink: preserve the still-applicable prefix of the
        /// stashed plan and search only for the broken tail. This manufactures
        /// a candidate plan; it does not execute it.
        pub fn replan_following(&mut self, evals: usize, mem_mb: usize) -> String {
            if evals == 0 || evals > WASM_MAX_THINK_EVALS {
                return err_json(
                    "FP_LIMIT_SEARCH",
                    "evals must be within the browser production budget",
                );
            }
            if mem_mb == 0 || mem_mb > WASM_MAX_THINK_MEMORY_MB {
                return err_json(
                    "FP_LIMIT_MEMORY",
                    "mem_mb must be within the browser production budget",
                );
            }
            let Some(prior) = self.plan.clone() else {
                return err_json("FP_NO_PLAN", "session has no stashed plan to follow");
            };
            let sol = self
                .inner
                .replan_following(&prior, self.cursor, evals, Some(mem_mb));
            self.plan = if sol.solved { sol.plan.clone() } else { None };
            self.cursor = 0;
            serialize_or_error!(&sol)
        }

        /// DfCM repair router. Preserve the cheapest reversible option:
        /// goal-met -> valid suffix reuse -> follow-biased repair -> full
        /// bounded replan. It never advances the cursor or actuates.
        pub fn repair(&mut self, evals: usize, mem_mb: usize) -> String {
            if evals == 0 || evals > WASM_MAX_THINK_EVALS {
                return err_json(
                    "FP_LIMIT_SEARCH",
                    "evals must be within the browser production budget",
                );
            }
            if mem_mb == 0 || mem_mb > WASM_MAX_THINK_MEMORY_MB {
                return err_json(
                    "FP_LIMIT_MEMORY",
                    "mem_mb must be within the browser production budget",
                );
            }

            if self.inner.goal_met() {
                return serde_json::json!({
                    "decision": "goal_met",
                    "trigger": "goal_met",
                    "plan_valid": null,
                    "previous_suffix": [],
                    "suffix": [],
                    "solution": null,
                })
                .to_string();
            }

            match self.plan.clone() {
                Some(prior) => {
                    let from = self.cursor.min(prior.steps.len());
                    let previous_suffix = prior.steps[from..].to_vec();
                    if self.inner.plan_still_valid(&prior, self.cursor) {
                        return serde_json::json!({
                            "decision": "reuse_suffix",
                            "trigger": "none",
                            "plan_valid": true,
                            "previous_suffix": previous_suffix,
                            "suffix": previous_suffix,
                            "solution": null,
                        })
                        .to_string();
                    }

                    let sol = self
                        .inner
                        .replan_following(&prior, self.cursor, evals, Some(mem_mb));
                    let decision = if sol.solved {
                        "replanned_following"
                    } else {
                        "replan_unsolved"
                    };
                    self.plan = if sol.solved { sol.plan.clone() } else { None };
                    self.cursor = 0;
                    let suffix = self
                        .plan
                        .as_ref()
                        .map(|plan| plan.steps.clone())
                        .unwrap_or_default();
                    serde_json::json!({
                        "decision": decision,
                        "trigger": "invalid_plan",
                        "plan_valid": false,
                        "previous_suffix": previous_suffix,
                        "suffix": suffix,
                        "solution": sol,
                    })
                    .to_string()
                }
                None => {
                    let sol = self.inner.replan_budgeted(evals, Some(mem_mb));
                    let decision = if sol.solved {
                        "replanned_full"
                    } else {
                        "replan_unsolved"
                    };
                    self.plan = if sol.solved { sol.plan.clone() } else { None };
                    self.cursor = 0;
                    let suffix = self
                        .plan
                        .as_ref()
                        .map(|plan| plan.steps.clone())
                        .unwrap_or_default();
                    serde_json::json!({
                        "decision": decision,
                        "trigger": "no_plan",
                        "plan_valid": null,
                        "previous_suffix": [],
                        "suffix": suffix,
                        "solution": sol,
                    })
                    .to_string()
                }
            }
        }

        /// Compare bounded counterfactual candidates over cheap forks without
        /// mutating this parent session. Each candidate may retarget the goal,
        /// observe a bounded fact set, and optionally restrict its action
        /// surface by display substring.
        pub fn probe_json(&self, candidates_json: &str, evals: usize, mem_mb: usize) -> String {
            if candidates_json.len() > WASM_JSON_FIELD_BYTES {
                return err_json("FP_LIMIT_INPUT", "candidate JSON exceeds the browser limit");
            }
            if evals == 0 || evals > WASM_MAX_THINK_EVALS {
                return err_json(
                    "FP_LIMIT_SEARCH",
                    "evals must be within the browser production budget",
                );
            }
            if mem_mb == 0 || mem_mb > WASM_MAX_THINK_MEMORY_MB {
                return err_json(
                    "FP_LIMIT_MEMORY",
                    "mem_mb must be within the browser production budget",
                );
            }
            let candidates: Vec<BrowserProbeCandidate> = match serde_json::from_str(candidates_json)
            {
                Ok(candidates) => candidates,
                Err(error) => return err_json("FP_ADAPTER", &format!("candidates: {error}")),
            };
            if candidates.is_empty() || candidates.len() > WASM_MAX_PROBE_CANDIDATES {
                return err_json(
                    "FP_LIMIT_CANDIDATES",
                    "candidates must contain between 1 and 32 entries",
                );
            }
            if candidates.iter().any(|candidate| {
                candidate.id.len() > 256
                    || candidate
                        .goal
                        .as_ref()
                        .is_some_and(|goal| goal.len() > WASM_TEXT_FIELD_BYTES)
                    || candidate.sight.len() > WASM_MAX_PROBE_OBSERVATIONS
                    || candidate.sight.iter().any(|(fact, _)| fact.len() > 4_096)
                    || candidate
                        .restrict_contains
                        .as_ref()
                        .is_some_and(|filter| filter.len() > 4_096)
            }) {
                return err_json(
                    "FP_LIMIT_CANDIDATE",
                    "candidate id, goal, observation, or restriction exceeds the browser probe limit",
                );
            }

            let results = candidates
                .iter()
                .map(|candidate| {
                    let mut mind = self.inner.fork();
                    if let Some(goal) = &candidate.goal {
                        if let Err(error) = mind.set_goal(goal) {
                            return serde_json::json!({
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
                                return serde_json::json!({
                                    "id": candidate.id,
                                    "outcome": "refused",
                                    "stage": "observe",
                                    "error": error,
                                })
                            }
                        }
                    };
                    let solution = mind.replan_budgeted(evals, Some(mem_mb));
                    serde_json::json!({
                        "id": candidate.id,
                        "outcome": if solution.solved { "solved" } else { "unsolved" },
                        "surprises": surprises,
                        "goal_met_before_search": mind.goal_met(),
                        "world_bytes": mind.world_bytes(),
                        "mind_bytes": mind.mind_bytes(),
                        "solution": solution,
                    })
                })
                .collect::<Vec<_>>();

            serde_json::json!({
                "candidate_count": results.len(),
                "results": results,
            })
            .to_string()
        }

        /// Replay the stored plan's tail from the cursor forward — free, no
        /// search spent.
        pub fn valid(&self) -> bool {
            self.plan
                .as_ref()
                .is_some_and(|p| self.inner.plan_still_valid(p, self.cursor))
        }

        /// The step under the cursor right now, as JSON — `null` once the plan
        /// runs dry or was never there.
        pub fn step_json(&self) -> String {
            match self.plan.as_ref().and_then(|p| p.steps.get(self.cursor)) {
                Some(s) => serde_json::to_string(s).unwrap_or_else(|_| "null".into()),
                None => "null".into(),
            }
        }

        /// Whatever's left of the plan, as JSON — feeds the claims logic and
        /// the display alike.
        pub fn suffix_json(&self) -> String {
            match self.plan.as_ref() {
                Some(p) => serde_json::to_string(&p.steps[self.cursor.min(p.steps.len())..])
                    .unwrap_or_else(|_| "[]".into()),
                None => "[]".into(),
            }
        }

        pub fn advance(&mut self) {
            self.cursor = self.cursor.saturating_add(1);
        }

        pub fn drop_plan(&mut self) {
            self.plan = None;
            self.cursor = 0;
        }

        pub fn has_plan(&self) -> bool {
            self.plan.is_some()
        }

        pub fn set_fact(&mut self, name: &str, value: bool) -> Result<(), JsValue> {
            ensure_js_len(name, 4_096, "fact name")?;
            self.inner.set_fact(name, value).map_err(js_err)
        }

        pub fn set_timed_fact(&mut self, dt: f64, name: &str, value: bool) -> Result<(), JsValue> {
            ensure_js_len(name, 4_096, "fact name")?;
            self.inner.set_timed_fact(dt, name, value).map_err(js_err)
        }

        pub fn observe(&mut self, sight_json: &str) -> Result<String, JsValue> {
            ensure_js_len(sight_json, WASM_JSON_FIELD_BYTES, "observation JSON")?;
            let sight: Vec<(String, bool)> = serde_json::from_str(sight_json).map_err(js_err)?;
            if sight.len() > 100_000 {
                return Err(JsValue::from_str("observation contains too many facts"));
            }
            let refs: Vec<(&str, bool)> = sight.iter().map(|(f, v)| (f.as_str(), *v)).collect();
            let news = self.inner.observe(&refs).map_err(js_err)?;
            serde_json::to_string(&news).map_err(js_err)
        }

        pub fn goal_met(&self) -> bool {
            self.inner.goal_met()
        }

        /// What the mind currently believes the fact holds (`null` if the
        /// grounding never heard of it).
        pub fn fact(&self, name: &str) -> JsValue {
            if name.len() > 4_096 {
                return JsValue::NULL;
            }
            match self.inner.fact(name) {
                Some(v) => JsValue::from_bool(v),
                None => JsValue::NULL,
            }
        }

        pub fn restrict_contains(&mut self, filter: String) {
            let filter = bounded_string(filter, 4_096);
            self.inner.restrict_ops(move |d| d.contains(&filter));
        }

        pub fn apply_start(&mut self, name: &str) -> Result<(), JsValue> {
            ensure_js_len(name, 4_096, "action")?;
            self.inner.apply_start(name).map_err(js_err)
        }

        pub fn elapse(&mut self, dt: f64) -> Result<String, JsValue> {
            let fired = self.inner.elapse(dt).map_err(js_err)?;
            serde_json::to_string(&fired).map_err(js_err)
        }

        pub fn set_fluent(&mut self, name: &str, value: f64) -> Result<(), JsValue> {
            ensure_js_len(name, 4_096, "fluent name")?;
            self.inner.set_fluent(name, value).map_err(js_err)
        }

        pub fn fluent(&self, name: &str) -> JsValue {
            if name.len() > 4_096 {
                return JsValue::NULL;
            }
            match self.inner.fluent(name) {
                Some(v) => JsValue::from_f64(v),
                None => JsValue::NULL,
            }
        }

        pub fn plan_valid_json(&self, plan_json: &str, from: usize) -> bool {
            if plan_json.len() > WASM_JSON_FIELD_BYTES {
                return false;
            }
            serde_json::from_str::<ferroplan::api::Plan>(plan_json)
                .map(|p| self.inner.plan_still_valid(&p, from))
                .unwrap_or(false)
        }

        pub fn world_bytes(&self) -> usize {
            self.inner.world_bytes()
        }

        pub fn mind_bytes(&self) -> usize {
            self.inner.mind_bytes()
        }
    }

    fn ensure_js_len(value: &str, max_bytes: usize, label: &str) -> Result<(), JsValue> {
        if value.len() > max_bytes {
            Err(JsValue::from_str(&format!(
                "{label} exceeds the {max_bytes}-byte limit"
            )))
        } else {
            Ok(())
        }
    }

    fn bounded_string(mut value: String, max_bytes: usize) -> String {
        if value.len() <= max_bytes {
            return value;
        }
        let mut end = max_bytes;
        while end > 0 && !value.is_char_boundary(end) {
            end -= 1;
        }
        value.truncate(end);
        value
    }

    fn js_err(e: impl std::fmt::Display) -> JsValue {
        JsValue::from_str(&e.to_string())
    }
} // mod browser_impl
