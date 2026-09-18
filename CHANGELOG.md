# Changelog

All notable changes to this project are documented here.

## [Unreleased]

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

## [0.27.1] - 2026-09-11 — A budget the caller can set, and withdraw

No engine change. Coverage is unchanged from 0.27.0 (5,122/8,444) because
nothing here touches how the planner searches — the additions are inert
unless you set them.

### Added

Two fields on `Options`, both `None` by default:

- **`wall_ms: Option<u64>`** — this call's wall, in milliseconds. Armed at
  the top of `solve`, so it bounds parsing and GROUNDING as well as
  search. That is the budget the existing knobs could not express:
  `max_evaluated` caps evaluated states, and grounding runs before the
  first state exists, so a call capped at fifty thousand evaluations can
  still spend minutes. `FF_TIME_LIMIT` could not express it either, for
  the opposite reason — it is armed once per PROCESS from the first solve,
  so in a long-lived host it either bounds nothing or eventually refuses
  everything.
- **`should_continue: Option<Arc<AtomicBool>>`** — flip it to `false` to
  stop this call. Polled wherever the wall is: the grounding checkpoint
  (every 256 bindings), the best-first batch boundary, EHC's
  per-evaluation slice, and the temporal pop. `#[serde(skip)]`, since a
  live handle has no JSON form.

A stop from either returns `solved: false` with a note naming which budget
bound and where — `grounding stopped at the declared budget: …` or
`search stopped at the declared budget: …` — and never the word
"unsolvable". Running out of time is not a proof.

The budget is held per call, in a thread-local armed by an RAII guard, so
concurrent `solve` calls on different threads each carry their own and a
spent budget cannot leak into the next call on its thread.
`FF_NO_RUNG_WALLCAP` does not disable it: that hatch governs the
environment wall, and a budget passed in code outranks an environment
variable.

### Why this is a patch release

It answers a consumer blocked on it. `solve` was blocking and
uninterruptible, so a host that abandoned the work leaked that thread for
the life of the process; their own instrumentation recorded 213 solves
completed, then 2 abandoned, after which the pool was dead — 0 completed,
4 abandoned. The rest of the 0.28 cycle is unmeasured and stays on its
branch until it has been swept.

### Note

The search ladder rations a wall across its rungs, so a budget is not
simply "time until I stop" — each rung sees a fraction of it. A 30-block
instance that solves in 34 ms unbudgeted is stopped by a 120 ms wall and
solved under a 200 ms one. That is not new to this release: `FF_TIME_LIMIT`
of 0.12 s fails the same instance and 0.2 s solves it. Measure your own
domain rather than assuming a 250 ms wall buys 250 ms of search.

## [0.27.0] - 2026-09-10 — One lever in the engine, and an instrument that can finally finish

**61% coverage across 32 IPC boards** (5,122/8,444), **687 certified
optima** — **+134** over 0.26.0 on the same instrument. Full record:
[`docs/roadmap-0.27.md`](https://github.com/hhh42/ferroplan/blob/main/docs/roadmap-0.27.md).

The engine changed in exactly one place this cycle. Most of the work went
into the thing that measures it, and that is the honest summary of what
0.27 is.

### The engine

- **The anchored successor generator** (`PackedTask::applicable_ops`).
  Expansion used to scan every grounded op to find the applicable ones.
  Now each op is anchored at its RAREST positive precondition, candidates
  are gathered from the state's true facts, and the result is sorted back
  into the scan's order — so the search it feeds is byte-identical, the
  same plans found in the same number of evaluations. Expansion per
  evaluation: labyrinth 234 → 34 µs (7×), parking 258 → 21 µs (12×),
  markettrader 10 → 5.6 µs. Wired into the classical, LAMA and novelty
  rungs.
- `apply()` gains an allocation-free path for ops with no conditional and
  no numeric effects, which otherwise cost four temporaries per successor.
- **Recorded negative:** the counter-based relaxed-graph build (reached
  facts decrement the ops that need them, no per-layer scan) measured
  1.78 → 2.02 ms on labyrinth and 0.92 → 0.96 on parking. Slower.
  Removed. What remains on these boards is the relaxation floor itself,
  ~1.7–2 ms per evaluation at 60–80k ops; moving it means firing fewer
  ops or evaluating fewer states, not a faster scan.

### Where it moved

simple-preferences +8.5 pts (119/130), 2014 seq-agile +6.4 (172/280),
qualitative-preferences +5.0 (51/100), and 2014 seq-sat / 2023 seq-sat /
2023 classical +4.3 each. 2018 seq-sat at 94/240 now places ~1st of 25
entrants by rate. One track went backwards: tempo-sat −0.3 pts.

net-benefit stands at **270/270**, but that is NOT claimed as a 0.27
gain. The like-for-like backfill now running — the v0.26.0 tag rebuilt
and re-measured on this box under the current referee — reaches 270/270
as well, so its published +3 was the instrument, not the engine. The
same control has so far moved 2026 numeric-opt's +1 to 0 for the same
reason. **Expect the same-instrument total to land a little under +134**;
it will be recorded when the backfill completes, against 0.26.0's own
re-measured numbers rather than its published table.

### The instrument (crucible R2)

Not shipped to crates.io — it is the harness — but it is why the numbers
above are worth reading. **This is the first sweep in the project's
history to reach a terminal state: 8,444 of 8,444 instances banked, zero
owed.** The 0.26 cut was taken by decision after six passes and five days
sixteen hours with 232 rows still owed.

The referee now judges each row by ITS OWN process rather than by the
box, which is what makes a clean terminal state reachable at all. Four
defects it found the hard way, each with its receipt:

- `cpu_ms` was **41.67× low** on every row ever recorded — Mach absolute
  time read as nanoseconds.
- The throttle never reached the child for the whole 0.26 sweep: the
  control channel's sender was dropped at construction.
- The **E-core defect**: under POLITE the harness put planners in
  Darwin's background band, where the same instance took **59.28 s
  instead of 4.52 s** — and banked, because ρ 0.956 is CPU share and a
  demoted process has all the share of a slow core. 1,709 rows affected.
- The canary locked onto a 3%-frequency boost clock and refused 553 rows
  as thermal across eleven boards. Its baseline is now the 25th
  percentile of recent solo readings.

### Honesty notes

- **11 instances that 0.26 solved, 0.27 did not.** Each was re-opened and
  re-run solo on a quiet box under the cut rule before promotion; these
  are the ones that failed again and are counted as real. Eight others
  looked like regressions and were not — they solved on the re-run, which
  is a 42% false-regression rate in the raw sweep and the reason the
  re-check exists.
- **The referee changed during the final passes.** ρ is CPU over wall,
  and every process pays a fixed ~0.3 s of fork, exec, linking and
  teardown that is wall without CPU — so below ~8 s no process, however
  well served, can reach ρ ≥ 0.95. Runs under that floor were being
  refused forever, and the set could not have completed. They are now
  judged by the box-wide window, which is what 0.26's instrument did for
  every row. Verified against the database rather than asserted: the same
  rows banked under 0.26 as `window`. The solved count did not move
  across either change — what banked were honest non-solutions.
- Coverage is measured at 60 s (300 s where a board says ENTRY), against
  official budgets that are typically 30× longer. The comparison is to
  ferroplan 0.26.0 on the same box, not to the competition.

---

Older releases: [`CHANGELOG-ARCHIVE.md`](CHANGELOG-ARCHIVE.md) (28 earlier releases, 0.1.0–0.26.0).
