# Configuration surface reference

Ferroplan's runtime configuration surface: every environment variable the
planner reads with a direct std::env::var / std::env::var_os read, plus
the string-key constants the shipped crates publish. Backlog [120]: env keys
are public config surface and belong in the doc-hdit coverage denominator,
documented here; the book's [tuning](../../../book/src/tuning.md) page stays
the experimental/restore-hatch tuning guide, and this page is the complete
per-module inventory.

Every env key named here is verified against the code-surface extraction
(scripts/gen_doc_surface.py code, ggen-marketplace hdit-v2-structs @
2de3d52fe): the key names below are the extraction's env_key items, not
hand-transcribed. Each meaning line was written from the read site in the
named module.

## Policy ([120] decision)

**Decision: env_key items stay IN the gated denominator and are documented
here; str_key items also stay in the denominator, and the genuinely-public
config constants are documented here — fixture/inline HDDL/PDDL string
literals lifted as str_key items by the extractor are recorded as an
extraction over-approximation, disclosed per [109]'s own rationale ("real
public configuration surface even when the known binding itself is private —
the VALUE is what docs name"), and NOT hand-documented** (documenting a test
fixture's HDDL text as "configuration" would be manufactured prose; the
extractor already renders them in the generated reference tables, where they
ground mechanically).

**No threshold change.** The gate thresholds (S 0.90 / Phi 0.001 / Q 0.65)
are unchanged by this decision; closure is achieved by documentation plus the
regenerated generated-reference scaffold, not by moving a gate.

The policy takes effect at the extraction that produced the denominators
below (ggen-marketplace 2de3d52fe, ferroplan @ 620f953, 2026-10-08):
1431 gated items (1709 raw; collapsed delta 278), of which 177 env_key items
(151 distinct keys) and 398 str_key items (347 distinct values).

## Environment keys by module

Meanings from the read sites. FF_* flags read with is_ok() are
presence-activated; FF_NO_* flags are restore hatches that remove a
default-on optimization. Test-only keys (the *_CHILD process-plumbing keys
the test harness sets for child processes, plus fixture keys) are listed
separately at the end.

### Search core (`crates/ferroplan/src/search.rs`)

| Key | Meaning |
|---|---|
| `FF_TIME_LIMIT` | Process wall budget in seconds; arms the budget-aware ladder checkpoints. |
| `FF_SEARCH_NODE_CAP` | Overrides the default node cap for classical search. |
| `FF_REPORT_RESERVE_SECS` | Wall reserve that stops optional quality work early enough to report a found plan. |
| `FF_NO_RUNG_WALLCAP` | Removes the per-rung wall checkpoints (the 0.21 shapes, kept as the RED record). |
| `FF_HTRACE` | Prints the best-first best-h trace (evaluation count and seconds per improvement) on stderr. |
| `FF_NOVELTY` / `FF_NO_NOVELTY` | Force / opt out of the width-1 novelty rung (default-on under a declared wall). |
| `FF_NOVELTY_ONLY` / `FF_NOVLIGHT_ONLY` / `FF_NOVDRIVER_ONLY` | Meaning-table hatches that run one novelty-family rung alone, whole wall. |
| `FF_NOVLIGHT` / `FF_NO_NOVLIGHT` | Force / opt out of the novelty-light rung (default-on under a declared wall). |
| `FF_NOV_OLD` | Restores the 0.21 novelty rung wholesale (the hatch swaps rungs). |
| `FF_NO_ENRICH` | Restores the plain classical fallback (no preference-op enrichment) in `plan_avoiding`. |
| `FF_CLM` | Arms the classical landmark-count guidance term (experimental, best-first fallback only). |
| `FF_NO_REFILL` | Restores the single-round think budget (disarms the 6-round refill loop). |
| `FF_NO_NODECAP_REFILL` | Restores the fixed node cap in the refill loop (no cap-raise re-entry). |
| `FF_NO_EHC_WALLCAP` | Removes EHC's wall slice checkpoint. |
| `FF_LEN_ANYTIME` | Marks the LAMA/anytime length-mode probe arm. |
| `FF_NO_LAMA` | Removes the LAMA rung from the ladder. |
| `FF_RESLM` | Arms the resource-landmark ordering term. |
| `FF_MEM_BUDGET_GB` | Retained-memory budget in GiB; sizes modelled node caps and arms the measured memory wall. |
| `FF_RES_DEBUG` | Resource debug narration on stderr (read in api.rs, constraints.rs, espc.rs, novelty.rs, temporal.rs, tresolve.rs). |

### Optimal search (`crates/ferroplan/src/optimal.rs`)

| Key | Meaning |
|---|---|
| `FF_OPT_NO_NUMH` | Disarms the numeric h arm unconditionally (pure hatch). |
| `FF_OPT_NO_NUMFOLD` | Pure hatch disabling the numeric-goal fold. |
| `FF_NO_LMCUT` | h^max only (full budget). |
| `FF_NO_HMAX_SPRINT` | LM-cut only; the gate never resurrects the sprint. |
| `FF_OPT_NO_ROOTGATE` | Removes the h^max/LM-cut root gate. |
| `FF_OPT_GATE_MARGIN` | Ratio threshold (default 1.4) deciding the h^max sprint slice. |
| `FF_OPT_NO_RESUME` | Restores the 0.21 throw-away handover (sprint dropped). |
| `FF_NO_INC_LMCUT` | Restores the 0.22 one-shot LM-cut probe. |
| `FF_NO_NODECAP_REFILL` | Restores the fixed node cap (duplicated with search.rs read site). |

### Heuristic / RPG (`crates/ferroplan/src/heuristic.rs`)

| Key | Meaning |
|---|---|
| `FF_NO_NUMH` | Restores the hole (no numeric achiever gradient). |
| `FF_NO_NUMPRE` | Restores the plateau regardless of the numeric achiever charge. |
| `FF_NUMPRE_NODAMP` | Restores the 0.21 charge exactly (both damping halves off). |
| `FF_NUMPRE_NOSKIP` | Charges even mover-covered preconditions (correction 2 off). |
| `FF_NUMPRE_NOSUM` | Selects first-wins instead of accumulating (correction 1 off). |
| `FF_NUMPRE_DEPTH` | Depth cap (default 4) on the charged-achiever recursion. |
| `FF_NO_NUMPRE_CHAIN` | Disables the charged-achiever recursion entirely. |
| `FF_NO_NEED_DIRS` | Restores the 0.27 relaxed-graph fixpoint test. |

### Grounding (`crates/ferroplan/src/ground.rs`)

| Key | Meaning |
|---|---|
| `FF_NO_JOIN_INDEX` | Restores the plain domain walk (no level index). |
| `FF_NO_MCV_JOIN` | Disables MCV join ordering (the 0.22 Phase 7 lever's hatch). |
| `FF_NO_STRAT_GROUND` | Disables stratified grounding (for A/B measurement). |
| `FF_NO_GOAL_FACTOR` | Restores the product compilation of the goal. |
| `FF_NO_FACT_COMPACT` | Restores raw packing (no monotone fact compaction). |
| `FF_NO_FIXPOINT_GROUND` | Disables fixpoint grounding. |
| `FF_NO_DNF_STATIC` | Disables static resolution inside precondition DNF expansion. |
| `FF_NO_FLUENT_FOLD` / `FF_NO_FLUENT_COMPACT` | Fluent fold / compact restore hatches. |
| `FF_NUMPRE_TEMPORAL` | Arms the numeric-precondition charge on temporal groundings (0.26 F3). |
| `FF_GROUND_PHASES` | Narrates grounding phases with elapsed time on stderr. |

### Temporal (`crates/ferroplan/src/temporal.rs`)

| Key | Meaning |
|---|---|
| `FF_TRPG` | Arms the TRPG-lite time-stamped relaxation tables (0.23 Phase 4 probe 2, opt-in). |
| `FF_TEVAL_BUDGET` | Caps temporal search evaluations (deterministic A/B stick, default unlimited). |
| `FF_TDEMAND_W` | Demand-tier weight (default 3). |
| `FF_NOREL` | Disables goal-relevance pruning alone. |
| `FF_TLAMA` | Opt-in TLAMA rung (recorded negative, 0.11). |
| `FF_LAX_HELPFUL` | Opt-in drift-repair rung (recorded negative, 0.11). |
| `FF_TAGENDA_W` / `FF_TAGENDA_W_PRUNE` | Agenda-size term weight (plain and pruned-pass variants). |
| `FF_TLIFO` | LIFO tie-break at equal temporal keys (measured worse on TMS). |
| `FF_TEMPORAL_NODE_CAP` | Overrides the temporal node cap (0 disables). |
| `FF_ORBIT_GEN` | Arms the orbit successor-generator arm. |
| `FF_NO_TSUCC` | Restores the full operator scan in temporal expansion. |
| `FF_TB_FREE_G` | Does not charge g for time-advance successors (experimental). |
| `FF_NO_TSYMM` | Restores arrival order for symmetric ends. |
| `FF_TEMPORAL_ABS_KEY` | Forces absolute time keys everywhere. |
| `FF_NO_SAT` | Collapses the SAT rung to byte-identity (the rung IS solve_ladder with it set). |
| `FF_NO_LADDER_DEDUP` | Restores the Full tier even when its demand is identical to the numeric tier. |
| `FF_H_ENDGATE` | End-gate probe (armed by the endgate test). |

### SAT (`crates/ferroplan/src/sat.rs`)

| Key | Meaning |
|---|---|
| `FF_SAT_BRANCH` | Selects the SAT branching heuristic. |
| `FF_NO_SAT_RATEBAIL` | Restores the 0.24 slice-spend shape (removes the rate bail). |
| `FF_NO_SAT_LAYERGEN` | Restores observed-placement-only clauses. |

### ESPC / preferences / constraints / costs

| Key | Meaning |
|---|---|
| `FF_ESPC_MONO` | Runs the monolithic ESPC loop instead of partitioned composition. |
| `FF_ESPC_TIME_MS` | Overrides the ESPC wall (measuring stick for the outer loop). |
| `FF_ESPC_TRAJ_PAIRS` | Trajectory-pair probe on the preference optimizer (pddl3.rs). |
| `FF_PREF_NO_STATIC` | Excludes init-satisfied preferences from guidance (the 0.4–0.5.0 behavior). |
| `FF_CONSTRAINTS_REJECT` | Forces constraint-reject verdicts (probe hatch). |
| `FF_NO_COND_SHARE` | Disables conditional-effect sharing. |
| `FF_NO_TRAJ_END` | Restores the exponential goal-DNF construction for trajectory monitors. |
| `FF_COST_SWEEP_EVALS` | Cost sweep eval budget (costs.rs). |
| `FF_LEN_SWEEP_EVALS` | Length sweep eval budget (costs.rs). |
| `FF_RES_WEIGHT` / `FF_RES_THRESH` | Resource guidance weight and threshold (pddl3.rs). |
| `FF_PREF_COST_WEIGHT` / `FF_DEADLINE_WEIGHT` | Preference cost / deadline weights (pddl3.rs). |
| `FF_PREF_NUMLEGACY` / `FF_PREF_COMPILED` | Legacy / compiled preference arms (pddl3.rs). |
| `FF_PREF_SEED` / `FF_PREF_SEED_BOUND` / `FF_PREF_SEED3` | Seed strategy selectors (pddl3.rs). |
| `FF_PREF_NO_SEED` | Disables incumbent zero (restores 0.27). |
| `FF_PREF_NO_ESCALATE` | Disables the preference escalate rung. |
| `FF_PREF_GREEDY` | Greedy preference arm. |
| `FF_PREF_NO_RESTARTS` | Disables the diversified restart ladder. |
| `FF_PREF_EVAL_BUDGET` | Preference optimizer eval budget. |
| `FF_PREF_NO_SELECT` | Disables preference selection. |
| `FF_PREF_NO_BARRIER` | Keeps init-satisfied preferences in guidance (0.5.1 default) vs exclude. |

### Feature tiers / orbit / parallel / portfolio

| Key | Meaning |
|---|---|
| `FF_TDEMAND` / `FF_NO_TDEMAND` | Force / remove the demand-tier heuristic (features.rs). |
| `FF_TDECOMP` | Arms the demand decomposition arm (features.rs). |
| `FF_NO_ESCALATE` | Removes the escalate rung (features.rs). |
| `FF_TCONC` | Arms temporal concurrency (features.rs). |
| `FF_NO_ESPC` | Removes the ESPC arm (features.rs). |
| `FF_ORBIT_ISO` | Goal-isomorphism orbit arm (0.23 probe 1, default OFF). |
| `FF_NO_ORBIT` | Disables orbit detection entirely. |
| `FF_NO_ORBIT_CLASSICAL` | Kills only the classical orbit consumer. |
| `FF_ORBIT_DEBUG` | Orbit debug narration. |
| `FFDP_THREADS` | Worker count for the data-parallel primitives (falls back to core count). |
| `FF_PORTFOLIO_SLICED` | Restores the pure doubling portfolio schedule. |
| `FF_SAT_CLASSICAL` | Forces the classical SAT arm (api.rs, planner.rs). |
| `FF_WALL_DEBUG` | Narrates every wall and memory decision on stderr. |
| `FF_NO_MEM_WALL` | Disables the measured memory wall (the 0.27 restore). |
| `FF_MEM_TRIP_FRAC` | Overrides the memory-wall trip fraction (default 0.75). |
| `FF_NUMNOV` | Arms the numeric novelty envelopes (novelty.rs). |
| `FF_NOV_R_CAP` | Width-2 novelty R-pair cap (default 256). |
| `FF_LAMA_EXT_ARRIVAL` | Opt-in ARRIVAL extension rule for LAMA extension (0.24 Phase 6). |
| FF_ESPC (not a direct read; the arm answers through `FF_NO_ESPC`) | see features.rs. |

### Example / fixture keys

| Key | Meaning |
|---|---|
| `FIXTURE_F_WALL_SECS` | Sets TranslateLimits max_wall for the fixture_f_stats example (default 30 s). |
| `THINK_EVALS` | Overrides the think budget in the village example. |

### Test-harness keys (*_CHILD process plumbing)

The test harness sets a per-child environment key so a forked child takes a
different code path — one key per test file:

`CALL_BUDGET_CHILD`, `DIFFERENTIAL_FUZZ_FRESH`, `ENRICH_CHILD`, `ENRICH_K`,
`ENRICH_CAP`, `GROUND_WALL_CHILD`, `INC_LMCUT_CHILD`, `LADDER_DEDUP_CHILD`,
`LADDER_WALL_CHILD`, `MCV_ROUTE_CHILD`, `MCV_STRAT_CHILD`, `MEM_WALL_CHILD`,
`MFW_PLANNER_ORACLE_PYTHONPATH`, `FERROPLAN_REQUIRE_MFW_ORACLE`,
`NODE_CAP_RLIMIT_CHILD`, `NOVDRIVER_CHILD`, `NUMFOLD_CHILD`,
`NUMOPT_ARM_CHILD`, `OPT_WALL_CHILD`, `PREF_CHASE_WALL_CHILD`,
`REFILL_CHILD`, `SAT_PROMO_CHILD`, `FERROPLAN_VAL`, `SCALING_LADDER_RESULTS`,
`SCALING_LADDER_RSS_KB`, `TCOMPRESS_TCONC_CHILD`, `ESCALATION_CHILD`,
`TGROUND_WALL_CHILD`, `TSEARCH_WALL_CHILD`, `HOME`,
`FERROPLAN_ORACLE_DIR`, `FERROPLAN_CORPUS_DIR`, `FERROPLAN_RUN_DIR`.

These ride in a plain list (not a table) on purpose: they are harness keys,
not tuning surface, and a table row per key would be manufactured prose.

## String-key constants

The shipped crates publish string constants that act as public config /
protocol surface — receipt domains and schema-version strings:

| Constant | Meaning |
|---|---|
| `urn:ferroplan:session-state:v1` | Session-state receipt domain (session.rs). |
| `urn:chatman:ferroplan-session-chain:v1` | Session-chain receipt domain (ferroplan-mcp session.rs). |
| `session_open` | Session-open receipt action key (ferroplan-mcp session.rs). |
| `ferroplan.handoff` | Webhandoff postMessage key (ferroplan-bevy webhandoff.rs). |
| `ferroplan-observation-pack/v1` | Observation-pack schema version (ferroplan-cli harvest). |
| `ferroplan-harvest-admission/v1` | Harvest admission schema version (ferroplan-cli harvest). |
| `ferroplan-harvest-receipt/v1` | Harvest receipt schema version (ferroplan-cli harvest). |
| `ferroplan-method-catalog/v1` | Method-catalog schema version (ferroplan-cli harvest). |
| `drop-retry` | Retry-pattern name (tests; also the wave-58 ticket name). |

Fixture/inline HDDL/PDDL literals lifted as str_key items are out of policy
scope for hand documentation (see Policy above); they render mechanically in
the generated reference tables.

## Known extraction-recall gaps (disclosed, not hand-filled)

The extractor surfaces direct std::env::var / var_os reads. Keys read
through helpers (crate::search::wall_frac_env with an FF_… literal,
the espc.rs env_num indirection) and set_var sites in tests are outside
extractor recall; the book's tuning page names several such keys
(FF_EHC_WALL_FRAC, FF_ESPC, FF_ESPC_EVAL_BUDGET, FF_TCOMPRESS_WALL_FRAC,
FF_TCOMPRESS_CHASE_FRAC, FF_PREF_SEED_WALL_FRAC, FF_PREF_SEED_EHC_FRAC,
FF_PREF_BARRIER — see the tuning tables for their meanings). They are
therefore de-backticked on the tuning page so they do not emit phantom
symbol claims, and they belong in the extractor's accessor list in a future
backlog item. FF_ESPC is answered through the FF_NO_ESPC read
(features.rs); FF_ESPC_EVAL_BUDGET is set_var-set by the espc test. All are
verified real on disk (grep). Disclosure, not a silent hole.

## Public re-export surface (coverage closure [120])

Every `pub use` re-export statement in the shipped crates, quoted
verbatim as the extractor lifts it (multi-identifier spans normalized
to one line, per backlog [105]): a re-export statement is itself
public API — the crate's canonical path for each re-exported item.
Verified byte-for-byte against the code-surface extraction at
ferroplan @ 620f953; meanings live in the module docs, the paths here
are the crate-root wasm/harvest/sat/root re-export map.

- `ferroplan-cli` `crates/ferroplan-cli/src/harvest/mod.rs`: `compile::{compile_pack, replay_pack}`
- `ferroplan-sat` `crates/ferroplan-sat/src/clause.rs`: `activity::{decay_clause_activities, ClauseActivity}`
- `ferroplan-sat` `crates/ferroplan-sat/src/clause.rs`: `alloc::{ClauseAlloc, ClauseRef}`
- `ferroplan-sat` `crates/ferroplan-sat/src/clause.rs`: `db::{ClauseDb, Tier}`
- `ferroplan-sat` `crates/ferroplan-sat/src/lib.rs`: `cnf::{CnfFormula, ExtendFormula}`
- `ferroplan-sat` `crates/ferroplan-sat/src/lib.rs`: `lit::{Lit, Var}`
- `ferroplan-sat` `crates/ferroplan-sat/src/lib.rs`: `solver::{Solver, SolverError}`
- `ferroplan-sat` `crates/ferroplan-sat/src/prop.rs`: `assignment::{backtrack, enqueue_assignment, full_restart, restart, Assignment, Trail}`
- `ferroplan-sat` `crates/ferroplan-sat/src/prop.rs`: `graph::{Conflict, ImplGraph, Reason}`
- `ferroplan-sat` `crates/ferroplan-sat/src/prop.rs`: `watch::{enable_watchlists, Watch, Watchlists}`
- `ferroplan-wasm` `crates/ferroplan-wasm/src/lib.rs`: `browser_impl::{ explain, fond_validate, plan, plan_production, readiness, version, WasmSession, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `api::{ decompose, parse, solve, Contract, Decomposition, DomainSummary, Metric, Mode, Options, ParseReport, Plan, ProblemSummary, Search, Solution, SolveError, Statistics, Step, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `eve::{ Activator, CapabilityTarget, Eve, EveError, EveHandoff, EveRequest, EveStage, GenesisProjection, GenesisWorld, GgenManufacturingRequest, GroundedGoal, HddlDecompositionRequest, HddlSurface, HumanPurpose, ManufactureTarget, McpPlusHandoff, PlanningRegime, PpddlPolicyRequest, PpddlSurface, SplitDirective, TruexContinuation, MAX_PRIMARY_ACTIVATORS, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `hddl::{solve_hddl, HddlError}`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `operator_compiler::{ compile_operator, CompiledOperator, OperatorCompileError, OperatorEffects, OperatorSpec, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `planner::{run_ff, run_planner}`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `planning_runtime::{ solve_planning_type, Agent, Goal as UniversalGoal, Method as PlanningMethod, PlanStep, PlannerError, PlannerLimits, PlanningProblem, PolicyEntry as UniversalPolicyEntry, PolicyOutcome as UniversalPolicyOutcome, QueueState, RdfTriple, State as UniversalState, Task as PlanningTask, Tool, Transition as UniversalTransition, UniversalPlan, UniversalPlanningRequest, WorkflowEdge, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `planning_types::{ route_planning_request, PlanningCapability, PlanningRail, PlanningRequest, PlanningRoute, PlanningRouteError, PlanningType, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `policy_validation::{ validate_fond_policy, PolicyGuarantee, PolicyIssue, PolicyValidationReport, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `ppddl::{ parse_ppddl, simulate_ppddl, solve_ppddl, validate_ppddl_policy, InitialStateProbability, PolicyDecision, PolicyOutcome, PolicyValidation, PpddlError, PpddlParseReport, ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution, ProbabilisticState, ProbabilisticStatistics, SimulationReport, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `production::{ parse_production, solve_ppddl_production, trace_production, validate_plan_production, PlanValidationEvidence, ProductionSession, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `production_explain::{decompose_production, explain_production}`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `readiness::{ capability_manifest, evaluate_readiness, production_input_fingerprint, solve_production, AuthorityClass, BuildIdentity, CapabilityContract, CapabilityEvaluation, CapabilityManifest, CompatibilityClass, DeterminismClass, InterfaceKind, ManifestError, OperationEnvelope, OutcomeClass, ProductionLimits, PublicError, ReadinessReport, ReadinessState, ReplayClass, SecurityClass, ValidationStatus, CANDIDATE_AUTHORITY, CAPABILITY_MANIFEST_SCHEMA, OPERATION_ENVELOPE_SCHEMA, }`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `session::{Session, Think, ThinkBudget, ThinkVerdict}`
- `ferroplan` `crates/ferroplan/src/lib.rs`: `trace::{trace, StateSnapshot}`
