# Changelog

All notable changes to this project are documented here.

## [Unreleased]

### Added

- **`ferroplan-wasm` ontology + ggen host**: RDF description of the WASI ABI
  and host contract under `ontology/`, ggen-generated registry and Elixir
  beam-host outputs, `HANDWRITTEN.md` ledger (fp_* shell and op bodies are
  `UNSUPPORTED(generator-capability)`), and an op-set drift test. Authority
  ceiling NONE.

## [0.29.0] - 2026-09-28 — Mechanism, not authority: replan ops, a runtime crate, an offline demo

Cut label **v26.9.28**. Every new surface below returns *evidence* (plans,
verdicts, validation reports) and grants no execution authority.

### Added

- **Replan mechanism ops** (`ferroplan-wasm` WASI ABI, `ferroplan`):
  `Session::think_following(prior, from_step, ThinkBudget)` replays the held
  plan's still-applicable suffix and searches only the broken tail, falling
  back to the orbit-aware `think` on the remaining budget; `Exhausted` only
  ever comes from the fallback's complete search, so a budget burned in the
  follow leg reads as `Capped`. New ABI op `fond_policy_validate
  {problem, plan}` returns an independent `PolicyValidationReport`;
  `fond_policy` responses now carry a `validation` field; `session_fork`
  takes `{keep_plan}`; `session_think` returns `verdict`
  (`solved | capped | exhausted`), `capped` and `spent_evals`, and accepts
  `prefer_follow`. A plan-parity gate (`tests/think_following.rs`) pins
  `replan_budgeted` against `think` on six fixtures. New `session_fork` bench
  (world bytes shared once; per-fork mind bytes and latency).
- **`ferroplan-runtime`**, a new workspace crate: a multi-plan provider
  switching control plane (authority, budget, circuit, lease, scheduler,
  receipt/OCEL evidence, replay, TLA and trim-tab evidence).
- **DfCM session repair and probes** (`ferroplan-wasm`): `session_repair`
  (reuse a valid suffix with zero search, else follow-biased replan, else full
  replan), `session_replan_following`, bounded counterfactual `session_probe`
  over cheap forks (the parent is untouched), `fond_validate` with structured
  `FP_*` refusals, and browser parity for all of them. Construct-only: none
  advance the cursor or actuate.
- `ferroplan-hddl` `validate_files` example: parse, static validation and
  method grounding over HDDL files; exit 2 names the failing stage.

### Fixed

- Dependency advisories: `crossbeam-epoch` 0.9.21 (RUSTSEC-2026-0204),
  `event-listener` 5.4.2 (RUSTSEC-2026-0221), `spin` 0.10.1 (unyank).
- `cargo doc -D warnings` was red on `main`: 14 intra-doc links (private
  items, an ambiguous `trace`, an unescaped `[0,1]`) across `ferroplan`,
  `ferroplan-cli` and `ferroplan-wasm`. Docs only.
- `ferroplan-py`'s `pyproject.toml` was still `0.28.0` beside a `0.29.0` crate.

### Upstream sync — browser demo offline copy

- Merged upstream `hhh42/ferroplan@9d8f16abc1738bf34c5f9488c6bba7e29b713621` into the downstream line.
  The browser planner can now be explicitly saved for offline use: a service worker
  precaches the demo shell and WASM bundle, reports cache/build status, refreshes
  stale entries in the background, and supports removal. The Pages workflow stamps
  the service worker with the deploying commit and validates the worker + manifest.

## [0.28.1] - 2026-09-22 — The fork reconciles with upstream's 0.28.0

Upstream cut its own 0.28.0 while this fork had already tagged `v0.28.0`
(17e38bb, the FOND-HTN wave-6 land), so the two version lines collided.
This release merges upstream `main` @ a05b604 (the 0.28 cut: tcompress,
mem, temporal/pddl3/search/api, crucible select) into the fork line and
bumps the workspace to **0.28.1** to disambiguate going forward. The fork's
HDDL/FOND front-end, its hddl dependency edge, and the v26.9.22 deep lane
(durable oracle/corpus homes, skip-by-name deep-lane tests, standing guard)
carry through unchanged.


### The FOND-HTN line — HDDL in, policy out

The unreleased cycle adds a second language front-end and a second solver
family. Ferroplan reads fully observable non-deterministic (FOND) HTN
problems written in an HDDL subset, grounds and translates them into its
flat FOND model, and solves them with strong / strong-cyclic policy
fixpoints over composite (task-network, state) pairs. The language surface,
semantics, architecture, and the external-oracle methodology behind the
test walls are in
[`docs/FOND-HTN.md`](https://github.com/hhh42/ferroplan/blob/main/docs/FOND-HTN.md).
Nothing below is cut into a version yet.

### Added

- **A native HDDL front-end** (`crates/ferroplan-hddl`, zero dependency on
  `ferroplan`): parse → ground → translate. Typed parameters and objects
  with a single-level type hierarchy, `:constants`, abstract tasks,
  primitive actions, and methods with optional `:precondition` and totally
  or partially ordered subtask networks; `and`/`or`/`imply`/`not`/
  `forall`/`exists` expanded at grounding time; `(:probabilistic w e …)`
  accepted as sugar and rewritten to a `oneof` block plus a side-channel
  weight map before tokenizing. `oneof` is legal in exactly one position —
  the whole top-level `:effect`, `k >= 1` — and every other placement is a
  typed refusal (`ParseError::MalformedOneof`), including the
  precondition position that used to fall through a silent mis-parse as an
  atom named `oneof`. `when` inside a `oneof` branch is refused outright:
  a deliberate deviation from a reference dialect that parses it and then
  silently drops the conditional effect downstream. `:durative-action`,
  `:functions`, `:constraints`, and numeric fluents are refused loudly at
  the stage that owns them, never silently dropped.
- **`solve_hddl`** (`crates/ferroplan/src/hddl.rs`): HDDL domain+problem
  text → `UniversalPlan`, with per-stage typed errors (`Parse`, `Ground`,
  `Translate`, `Model`, `Timeout`, `RootTaskMismatch`) and a watchdog
  thread over the whole call keyed off `limits.max_wall_ms` (default
  10 s) — the caller never waits past budget, even though a worker thread
  cannot be forcibly killed. The result reports `PlanningType::Fond`: the
  front-end reuses the FOND solver, and the report names the solver that
  actually ran.
- **WASM ops** `hddl_solve` / `htn_plan` / `fond_policy`
  (`crates/ferroplan-wasm/src/wasi_abi.rs`) with per-stage error codes
  `FP_PARSE` / `FP_HDDL_GROUND` / `FP_HDDL_TRANSLATE` / `FP_MODEL`.
  `hddl_solve` runs synchronously under wasm32-wasip1 (no thread spawn
  there); all three are covered end to end with wasmtime as the test
  runner.
- **Strong and strong-cyclic FOND policies.** The object a policy is
  defined over is the composite (ground facts, task-network frontier)
  state — the (TN, state)-policy formalization of Chen & Bercher
  (AAAI 2021) — and every policy entry carries the full outcome set of
  its chosen action. `fond_policy` grows the strong least fixpoint (no
  fairness assumption, bounded executions only);
  `fond_policy_strong_cyclic` runs the standard two-phase construction of
  Cimatti et al. (AIJ 2003) — weak backward reachability, then a
  greatest-fixpoint prune inside the reachable set — which is what admits
  retry loops. The `Fond` dispatch arm tries strong first and falls back
  to strong-cyclic only on `NoPlan`: the cyclic solver strictly extends
  coverage and never overturns a verdict the strong solver already made.
- **The Eve bridge.** `solve_hddl_from_eve(&EveHandoff, &PlannerLimits)`
  consumes the handoff's HDDL half; where the problem text declares no
  `:htn` root network, Eve's root task is spliced in, and a problem
  network that disagrees with Eve's root task is refused
  (`RootTaskMismatch`) instead of silently resolved.
  `capability_manifest()` now admits `fp.core.fond` and `fp.core.hddl`,
  whose evidence ids are real test names. The handoff's PPDDL half is
  explicitly out of scope for the bridge.

### Fixed

- **Strong-cyclic policies that could never reach the goal.** The
  property wave's `FOUND_BUG_1`: 79 of 320 generated instances received a
  policy whose reachable region was a goal-unreachable pure self-loop.
  `fond_policy_strong_cyclic` now alternates a committable
  goal-reachability sweep with the Phase 2 witness prune — a state
  survives only if it can reach a goal along edges whose action commits
  all outcomes into the region — so pure self-loops prune to `NoPlan`
  while retry loops still qualify. The shrunk reproducer runs always-on
  in `tests/fond_property.rs`.
- **`hierarchical_plan` was state-blind**: it now backtracks over all HTN
  methods (pinned in `tests/planning_runtime.rs`), so one dead
  decomposition branch no longer ends the search.
- **The `oneof` surface is aligned** with the reference dialect's refusal
  rules, and oneof outcome fidelity (branch weights, empty branches,
  overlap) is pinned in the translated explicit graph.
- **Method preconditions are real.** Ground methods carry a
  `:precondition` and `translate` gates decomposition offers on it.
  Previously every method matching a task name was offered regardless of
  world state — a correctness bug, not just a performance one.
- **Validation is reference-level** (`crates/ferroplan-hddl/src/validate.rs`):
  cyclic type hierarchies; undeclared types on constants, objects, and
  quantifier binders; duplicate declarations; predicate arity at every
  use; method heads that are not declared compound tasks; subtask arity;
  argument subtyping; dangling or cyclic ordering constraints; init/goal
  atoms that are undeclared or unground; and an unusable root task
  network are all typed refusals before grounding starts. Compound tasks
  that are referenced but unrefinable come back through a new warnings
  channel (`validate_*_with_warnings`), computed as a reachability +
  nullability fixpoint ignoring preconditions.
- **Smaller correctness**: `contingent_policy`'s negative memo is
  depth-aware, so a depth-cached failure no longer fakes a `NoPlan`;
  probability-mass ceremony is no longer enforced on mass-blind planners;
  `total_order_ranks` returns `None` on a dangling order endpoint instead
  of panicking; `fond_policy`'s tautological post-check became a
  `debug_assert!` instead of dead code.

### Hardened

Malformed HDDL no longer panics. The adversarial-input pass replaced
panics with typed `ParseError` returns (`#![forbid(unsafe_code)]` at
`lib.rs:35`; `ParseError::NestedProbabilisticBlock` in `probabilistic.rs`;
`parser::parse_header`/`parser::section_value` no longer index/unwrap past
a short or malformed header/section). `grounder::BindingIter` replaces
eager Cartesian-product binding materialization with a lazy mixed-radix
odometer iterator, closing an unbounded-memory DoS on schemas with many
typed parameters. `GroundingLimits`/`TranslateLimits::max_wall` and
`PlannerLimits::max_wall_ms` (default 10 s) add a wall-clock refusal to
grounding, translation, and `solve_hddl`. A 40-thread concurrency test
(`solve_hddl_produces_correct_independent_results_under_concurrent_calls`)
proves independent concurrent `solve_hddl` calls don't cross-contaminate
state. Rustdoc coverage of the crate is 71% of public items (45/63
`pub fn`/`struct`/`enum`/`const`/`type`, excluding `pub mod`). Measured on
the real IPC2020 blocksworld fixture (`fixtures/f`): the method-
precondition gate cuts transitions built at a 120 s wall from 7,984,061 to
6,406,950 and the still-queued backlog from 1,359,726 to 307,752 states —
substantially less redundant branching — but this fixture still does not
finish translating to an explicit state graph within a 120 s wall-clock
bound either before or after the fix; not yet resolved.

### The test walls

Every wall below is a committed test file or a committed golden/RESULTS
record. The external planner used for differential testing runs only from
`/tmp` as a test oracle; its code never enters the repo — only its
recorded verdicts do.

- **Canonical verdicts.** Hand-authored flat-FOND domains pin strong,
  strong-cyclic-only, and unsolvable outcomes (`tests/fond_canonical.rs`);
  a hand-authored FOND-HTN micro-domain suite pins decomposition,
  backtracking, and policy shape end to end (`tests/fond_htn_micro.rs`).
- **IPC-2023 end to end.** All 13 curated IPC-2023 HTN instances run
  through `solve_hddl` (`tests/htn_ipc2023.rs`): 9 SOLVED with each
  returned policy verified outcome-closed against an independently
  re-derived ground IR, 4 honest typed limits (below), 0 parse or ground
  gaps, 0 panics, 0 hangs. Full table:
  `tests/fixtures/htn-ipc2023/RESULTS.md`.
- **Property equivalence against an independent reference**
  (`tests/fond_property.rs`): 320 instances generated from a fixed seed
  (splitmix64, replayable), judged against a reference that is
  deliberately NOT a fixpoint — it enumerates every policy (at most 3^8)
  and applies the Cimatti characterizations directly. A returned policy
  must be outcome-closed, must avoid unsafe states, and must reach the
  goal within its own region; `NoPlan` must match the enumeration;
  solving twice must be identical. This wall is what found `FOUND_BUG_1`.
- **Adversarial inputs** (`tests/hddl_adversarial.rs`): 27 cases pinning
  the typed-refusal surface (typed oneof, ordering, goal rejections),
  plus a runner over an external flawed-domain corpus. Known gaps ride as
  `#[ignore]`d tripwires with tickets, never silent skips.
- **Differential goldens.** Three corpora of recorded oracle verdicts are
  committed as facts: 21 deterministic HTN instances
  (`tests/fixtures/htn-oracle/` — 14/21 strict agreement, 6 admitted
  mismatches each carrying an `#[ignore]`d demonstration test, 1 open
  verdict where the oracle itself errors; table in its `RESULTS.md`), a
  31-run FOND-HTN oracle harvest (`tests/fixtures/fond-htn/
  oracle-harvest-full.json` — 18 SOLVED / 12 TIMEOUT / 1 UNSUPPORTED as
  the oracle said them), and 8 flat-FOND fixtures with three-way
  agreement (`tests/fixtures/fond-flat/`).
- **Concurrency and WASM.** The 40-thread `solve_hddl` isolation test and
  the wasip1+wasmtime op tests above. The wave receipt
  (`docs/jira/v26.9.17/_WAVE-RECEIPT.md`) records the combined gate at
  main: `cargo test -p ferroplan -p ferroplan-hddl` — 695 passed /
  0 failed, exit 0.

### Known limits (ticketed, not hidden)

- **Translate capacity.** `solve_hddl`'s internal translate wall
  (`TranslateLimits::default()`, 10 s) is hardcoded where the pipeline is
  assembled, and a caller's larger budget cannot raise it. Four IPC-2023
  instances hit it with roughly 16,800–46,661 composite states still
  queued (`tests/fixtures/htn-oracle/RESULTS.md`). Ticket fond-htn-23.
- **Conditional-effect grounding.** Action-level conditional effects are
  an honest refusal (`nested 'when' is out of scope`), and method-
  `:effect` domains fail grounding (`unbound variable`): loud typed
  errors, not panics. Two PANDA corpus pairs are admitted mismatches on
  exactly this. Ticket fond-htn-24.
- **Parse depth.** The parser recurses per nesting level with no depth
  budget: 250-deep effects parse, 500 abort, 1000 kill the process. It is
  carried as an `#[ignore]`d adversarial tripwire until fixed. Ticket
  fond-htn-22.
- **`(= …)` in goals.** The built-in term-equality predicate parses and
  validates but never evaluates true in `translate`, so affected
  deterministic fixtures return `NoPlan`; the mismatch goldens are
  committed. Ticket fond-htn-21.

### Finished hardening

Landed across the closing waves of this cycle; every claim below is backed
by the named commit, test file, or committed RESULTS record.

- **Duplicate (multi-inheritance) type declarations are accepted**
  (ticket fond-htn-44; branch `fix/duplicate-type-dedup`, merged `0f39d2d`):
  a type declared more than once in `:types` is no longer a validation
  refusal — a `ValidationWarning::DuplicateTypeDeclaration` names it and
  the subtype relation becomes the union of the declared parents
  (`crates/ferroplan-hddl/src/validate.rs`; the PO_UM-Translog corpus
  shape was the affected case).
- **Goal evaluation and parsing are depth-budgeted** (tickets fond-htn-42
  and fond-htn-22): ground-goal evaluation refuses past a nesting budget
  of `DEFAULT_MAX_GOAL_DEPTH` = 256 with `GroundError::GoalTooDeep`
  (`crates/ferroplan-hddl/src/grounder.rs`; branch `fix/goal-eval-recursion`,
  merged `f0b8b9c` — before it, a hand-built depth-50k `GroundGoal`
  SIGABRTed the process), and the parser bounds s-expression nesting at
  `DEFAULT_MAX_PARSE_DEPTH` = 256 with `ParseError::NestingTooDeep`
  carrying the offending `(`'s line/column (`crates/ferroplan-hddl/src/parser.rs`;
  branch `fix/parse-depth-budget`, merged `982ce3e` — the 1000-deep
  `(and …)` process abort is gone, pinned by
  `deep_nesting_1000_and_returns_typed_nesting_error_never_aborts`).
- **Grounding caps are caller-plumbable** (ticket fond-htn-43; branch
  `fix/ground-caps-plumbing`, merged `833994f`): `solve_hddl` derives
  `GroundingLimits` from the caller's `PlannerLimits`
  (`grounding_limits_from`, `crates/ferroplan/src/hddl.rs`) — ground
  action/method caps = `max_states` ÷ 10, calibrated so the default
  100,000 reproduces the historical 10,000/10,000 envelope exactly, and
  `max_wall` follows `max_wall_ms` (`0` → unbounded). The raised-caps
  re-run of the 17 ground-cap-refused IPC domains is committed
  (`crates/ferroplan/tests/ground_caps_ipc_addendum.rs`,
  `tests/fixtures/ipc-sweep/RESULTS-wavec.md`): 16 of 17 still refuse at
  1,000,000-instance caps — true counts exceed 1M; hierarchical
  task-relevance pruning has since landed as an opt-in flag
  (`GroundingLimits::prune_irrelevant`, ticket fond-htn-60; flipping the
  default envelope awaits the 16-domain default-caps re-run) — and the
  17th progressed past grounding to the translate wall.
- **Two stress walls landed.** The seeded 2000-case HDDL round-trip fuzz
  (`tests/hddl_fuzz_roundtrip.rs`, ticket fond-htn-31; branch
  `fuzz/hddl-roundtrip`, merged `8e54704`) completed with zero findings:
  no panics, every refusal typed, determinism holds across the double
  run, and the file's `KNOWN_FINDINGS` table is empty. The 5000-instance
  property scale-up (`tests/fond_property_scaleup.rs`, ticket fond-htn-33;
  branch `test/property-scaleup`, merged `2377802`) judged the solver
  against an independent label-correcting reference and **found
  FOUND_BUG_2** — strong-cyclic returns Phase-2 witness-first
  goal-unreachable loop choices when the Phase-3 region closes without a
  choice rewrite (882 of 3676 reference-solvable instances). **Fixed**
  (ticket fond-htn-57): Phase 3 records reach-discovery ranks and rewrites
  every surviving non-goal state's choice to a committable advancing
  action at fixpoint close; the shrunk reproducer runs always-on and the
  sweep's loop-policy carve-out was removed — the 5,000-instance sweep now
  fails on any recurrence (0 mismatches on the landing run).
- **Wave-6 hardening (landed on the integration line, coordinator lands to
  main):**
  - **Drop-retry re-decomposition seam fixed** (ticket fond-htn-58,
    commit `4964b74`): after a no-change `oneof` outcome the translator
    re-offers the pending abstract task instead of consuming the
    decomposition offer once — the committed koala-contrast pin
    `ORACLE_MISMATCH_micro_drop_retry` is now the always-on
    `micro_drop_retry_agrees_with_oracle` agreement test
    (`tests/fond_htn_oracle.rs`), goldens healed.
  - **Iterative `Drop` for `GroundGoal`** (ticket fond-htn-59, commit
    `304fa66`): any depth drops in O(1) stack via explicit-worklist tail
    destruction — a refused depth-50k goal no longer SIGABRTs on the way
    down; the `mem::forget` dodges in the depth-budget tests are gone.
  - **Hierarchical task-relevance pruning** (ticket fond-htn-60, opt-in
    `GroundingLimits::prune_irrelevant`): an iterative task-relevance
    fixpoint keeps only actions/methods on some path from the initial task
    network, so >1M-instance groundings can fit the caps; soundness
    falsifiers re-pinned (count reduction only). The 16-domain default-caps
    re-run harness is committed
    (`tests/grounding_prune_ipc_rerun.rs`, `#[ignore]`d long-run).
  - **TranslateLimits plumbing** (ticket fond-htn-65, commits `9bbe2c3` +
    `6757249`): `translate_limits_from` derives the translate wall/state
    budget from `PlannerLimits` (default-identical calibration,
    `max_states` ÷ 2), and the 9 wave-4 `LIMIT:translate-wall` IPC
    instances re-run at 60 s caller walls now bind at the cumulative
    pipeline watchdog, not the old 10 s translate wall
    (`tests/fixtures/ipc-sweep/RESULTS-wavec.md`, addendum 2).
  - **Differential fuzz harness** (ticket fond-htn-61): the committed
    generator's VALID draws run through both engines with a 100-pair
    ledger and per-seed divergence minimization
    (`tests/differential_fuzz.rs` + `tests/common/mod.rs`). The pre-hotfix
    ledger harvest found 14 divergences/3 incomparable; after the wave-6
    soundness fixes land, the live-vs-ledger tripwire correctly fires on
    8 verdict flips (6× SOLVED→NOSOLUTION = false solves removed by
    fond-htn-57, 2× NOSOLUTION→SOLVED = dead branches re-decomposed by
    fond-htn-58) — the ledger is parked as
    `tests/fixtures/differential-fuzz/ledger.pre-wave6-hotfix-harvest.json`
    until the oracle harness is rebuilt and a fresh ≥80-consumable harvest
    re-arms the tripwire.

The parse-depth row of "Known limits" above (ticket fond-htn-22) is
superseded by the parser depth budget in this block; the grounded-caps
mapping likewise supersedes the hard-coded-caps behavior the
"Translate capacity" row describes for the grounding stage only.

---

Older releases: [`CHANGELOG-ARCHIVE.md`](CHANGELOG-ARCHIVE.md) (31 earlier releases, 0.1.0–0.28.0).
