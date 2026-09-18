# FOND-HTN in ferroplan

Ferroplan accepts fully observable non-deterministic (FOND) planning problems
written in a subset of HDDL, grounds and translates them into its flat FOND
model, and solves them with the existing strong / strong-cyclic policy
fixpoints. This document covers the language surface, the semantics, the
architecture, the external-oracle methodology used to validate it, and the
deliberate deviations from the reference dialect.

Planner output is a candidate policy. It has no execution authority; BRCE
remains the exclusive consequential path.

The `oneof` surface described below landed on main from the frozen branch
`fix/oneof-koala-semantics` (tip `4d40f99`) and pins the reference-aligned
refusal rules described below. Sections of this document describing behavior
of the external reference planner rely on the wave context
(`docs/jira/v26.9.17/_FOND-HTN-WAVE-CONTEXT.md`), never on the reference
implementation's source, which is used strictly as an external test oracle.

## 1. Language surface

### HDDL subset

The front-end crate is `crates/ferroplan-hddl`. In scope:

- Typed parameters and objects, a single-level type hierarchy
  (subtype -> parent, transitively resolved during grounding), `:constants`.
- Predicates; abstract `:task` declarations; primitive `:action`s with a
  conjunctive precondition and effect; `:method`s with an optional
  `:precondition` (the HDDL `:method-preconditions` construct), a totally
  or partially ordered subtask network (explicit `<` ordering edges), and a
  `:effect` (below) applied at decomposition time.
- Problem files with `:objects`, `:init`, `:goal`, and the initial `:htn`
  task network — whose own `:parameters` are existentially bound: one ground
  root network (hence one initial state) per admissible binding of those
  variables over their declared types.
- Precondition/goal connectives: `and`, `or`, `imply`, `not` (of a single
  atom in preconditions; of compound descriptions in `:goal` via DNF
  expansion), `forall`, `exists` (expanded at grounding time). `or`/`imply`/
  quantifiers are refused inside `when`-effect conditions.
- The built-in `=` term-equality predicate (arity 2, no declaration needed):
  evaluated over ground terms — state-independent, so it never consults the
  fact set. In action/method preconditions it grounds to a dedicated
  equality node; in `when`-effect conditions it folds statically (a
  constantly-false equality pins the condition with a synthetic
  never-present marker, so that guarded effect can never fire); in `:goal`
  it folds at DNF expansion time. `(not (= …))` composes through the
  ordinary negation rules.
- Effects: literals, `and`, `when` (conditional, rules below), `oneof` (rules
  below), the empty effect `()`.
- The `(:probabilistic w1 e1 w2 e2 ...)` effect extension is accepted as pure
  syntactic sugar: a pure text pre-pass (`probabilistic::preprocess`) rewrites
  it into a standard `oneof` block plus a side-channel weight map, before
  tokenizing. Weights ride on each grounded outcome branch; translation splits
  transition probability mass proportionally to declared weights, evenly
  otherwise.

Out of scope, refused loudly at the correct pipeline stage rather than
silently dropped:

- `:durative-action` and `:functions` (parse-time `UnsupportedConstruct`);
  temporal/durative actions are a permanent non-goal.
- `:constraints` blocks and nested numeric-fluent declarations are lexed and
  parsed into real AST nodes, then refused the moment grounding starts
  (`GroundError::UnsupportedConstraint` / `GroundError::UnsupportedNumericFluent`).
  `increase`/`decrease` effects are the same: parsed, never grounded.
- Nested `oneof`, `oneof` under `when`, `oneof` in any goal-description
  position, and `oneof` in a method `:effect` (a decomposition effect is
  deterministic): hard, typed parse errors (below).
- `when` nested three or more condition levels deep (`(when c1 (when c2
  (when c3 e)))`): typed ground error (`nested 'when' is out of scope`).

### Conditional-effect (`when`) rules (landing semantics, ticket fond-htn-24)

`when` is grounded to the depth the corpus needs and no further, with every
boundary loud and typed:

| Rule | Meaning |
|---|---|
| Action `:effect` | A `when`'s condition is a ground goal over the action's binding (flat literal conjunction — `or`/`imply`/quantifiers refused there); its body is a conjunctive add/delete set. |
| Method `:effect` | Same grammar minus `oneof`, applied when the method is chosen for decomposition, before any subtask executes; guards evaluate against the pre-decomposition state. A method with no `:effect` behaves exactly as before (no-op). |
| One level of `when`-in-`when` flattens | `(when c1 (when c2 e))` grounds identically to `(when (and c1 c2) e)` — a second conditional whose guard conjoins both conditions. Sound because every guard is evaluated against the same source state. |
| Deeper nesting refused | A `when` at depth ≥ 3 keeps the existing typed refusal (`nested 'when' is out of scope`) rather than silently widening. |
| Guarded outcomes | Translation evaluates every guard against the transition's SOURCE state per outcome and applies the guarded del-then-add — the same compilation the in-repo `ppddl` compiler uses (`crates/ferroplan/src/ppddl/compile/part06.rs`); one shared `apply_effect_branch` serves action outcomes and method decompositions. |
| `when` inside a `oneof` branch | Still refused at parse time (deliberate deviation, table below). |

### `oneof` rules (landing semantics)

The reference dialect admits `oneof` in exactly one position: the *entire*
top-level `:effect` of an action, `(:effect (oneof e1 ... ek))`, with `k >= 1`.
Ferroplan's landing surface matches, with every refusal typed
(`ParseError::MalformedOneof`):

| Rule | Meaning |
|---|---|
| Top-level only | `oneof` is legal only as the whole `:effect` of an `:action`. Under a top-level `and` (`(:effect (and (p) (oneof ...)))`), under a `when`, in a precondition, a method condition, or `:goal` — refused. (A precondition-position `oneof` used to fall through to a silent mis-parse as an atom with predicate `oneof`; now a typed error.) |
| `k >= 1` | A bare `(oneof)` is refused: it would ground to an action with zero outcomes, an unexecutable dead-end. |
| `k == 1` degenerates | A single-branch `oneof` is normalized to its branch — a deterministic effect. The parsed AST's `Effect::Oneof` therefore always carries `k >= 2` genuinely non-deterministic branches. |
| Empty branch allowed | A branch may be the empty effect `()`; it survives as an `Effect::Empty` branch and grounds/translates to a genuine no-change outcome. |
| Overlap allowed | Branches need not be mutually exclusive. |
| Branch shapes | Plain literal effects, `and`-conjunctions of those, or `()`. |
| Nesting rejected | `oneof` inside a `oneof` branch: refused. |
| `when` in a branch rejected — deliberate deviation | The reference implementation *parses* `when` inside a `oneof` branch but silently drops the conditional effect downstream (a known open item there), manufacturing a silently-weaker domain. Ferroplan refuses loudly instead. This is a documented, deliberate deviation: a domain the reference accepts may be rejected by ferroplan, never silently mis-solved. |

## 2. Semantics

### Strong plans

A strong plan is a policy under which the goal is reached in a bounded number
of steps under every possible resolution of every non-deterministic choice.
Winning-set characterization: the least fixpoint grown from the goal states —
repeatedly admit any state from which some action exists whose outcomes
*all* land in the winning set. Cyclic solution structure cannot be expressed:
a state whose action self-loops would need itself already in the set at
admission time. No fairness assumption is needed. This is the classical
strong plan notion of Cimatti, Pistore, Roveri, and Traverso (AIJ 2003).

Ferroplan: `fond_policy` (`crates/ferroplan/src/planning_runtime.rs`, the
`"strong FOND fixed point"` note). Goal states need no policy entry; every
other winning state must have one, or the result is `NoPlan`.

### Strong-cyclic plans

A strong-cyclic plan is a policy under which the goal is reached with
probability 1 in the limit — every fair execution eventually reaches the goal,
possibly after unboundedly many retries. Winning-set characterization: the
greatest fixpoint *within* the weakly (OR-)reachable set — optimistically
keep every state from which the goal is reachable under some lucky run, then
repeatedly prune any non-goal state with no action whose outcomes all still
survive. Because the pessimistic set starts at the whole weakly-reachable set
rather than empty, a self-loop survives as long as its state does: this is
exactly what admits retry loops a least-fixpoint construction cannot express.

Every strong solution is also a strong-cyclic solution; the converse fails
precisely on domains whose only solutions contain retry loops. Reference:
Cimatti et al. (AIJ 2003), the standard two-phase symbolic model checking
construction.

Ferroplan: `fond_policy_strong_cyclic` (same file) runs three phases. The
`Fond` dispatch arm of `solve_planning_type` tries `fond_policy` first and
falls back to `fond_policy_strong_cyclic` only on `NoPlan`, so domains
`fond_policy` already solves are unchanged; the strong-cyclic solver strictly
extends coverage.

- **Phase 3 — committable goal-reachability, alternating with the witness
  prune.** The Phase 2 greatest fixpoint alone over-keeps: a state whose only
  closed action is a self-loop survives the prune with no path to the goal,
  and a closed policy over that region reaches the goal under *no* outcome
  resolution — not a strong-cyclic solution. Phase 3 therefore alternates two
  sweeps to a joint fixpoint: (a) the witness prune to its own fixpoint over
  the surviving set, then (b) committable backward reachability from the goal
  states — a surviving state enters only along an edge whose action commits
  *all* its outcomes into the region *and* at least one outcome lands closer
  to a goal. Retry loops qualify (their committed action advances toward the
  goal before retrying); pure self-loops do not. When the reachable core
  stops short of the surviving set, the set shrinks to it and both sweeps
  re-run — a pruned state may have been another state's all-outcomes witness.
  The solver finishes with an outcome-closure check computed on the *final*
  (surviving set, choices) pair — every surviving non-goal state must hold a
  choice whose every outcome lands in a goal state or another surviving state
  with a choice — so a witness validated early in a round cannot survive a
  prune later in the same round.
- **The FOUND_BUG_1 lineage, guarded always-on.** The property wave's
  independent policy enumeration caught the pre-Phase-3 solver accepting
  79/320 generated instances with a goal-unreachable self-loop policy. The
  fix landed in this Phase-3 hardening, and the reproducer
  `fond_property_FOUND_BUG_1_strong_cyclic_accepts_goal_unreachable_self_loop`
  in `crates/ferroplan/tests/fond_property.rs` runs on every test pass,
  asserting `Err(NoPlan)` on the 3-state self-loop-vs-unsafe-branch instance
  (the `FOUND_BUG_*` spelling is kept greppable by ticket convention). The
  same file's main property test no longer carves goal-unreachable regions
  out as a known-defect class: they gate.
- **Loop-bound policy.** Every structural fixpoint above (Phase 1 admission,
  Phase 2/3 prune alternation) is bounded by its own termination argument,
  *not* by the caller's `PlannerLimits::max_iterations`: each changing round
  admits or prunes at least one state, so each loop converges within
  `states + 1` changing rounds plus one confirming round. That `states + 1`
  failsafe breach surfaces as `Err(Timeout)` — never a silently truncated
  policy. `max_iterations` used to gate the prune; truncating a greatest
  fixpoint mid-convergence left stale witnesses in the returned policy (the
  wave-1 truncation dead-sink defect). Wall-clock checks still run every
  round, so `PlannerLimits::max_wall_ms` semantics are unchanged — for these
  structural fixpoints, caller round caps are advisory, not load-bearing.
  The same pattern is landing for `fond_policy`'s least fixpoint (each
  changing round admits ≥ 1 state, so the same `states + 1` bound applies) on
  branch `fix/fond-loop-failsafes` (`7fea6cc`, awaiting integration); until
  it merges, `fond_policy` remains gated by `max_iterations`
  (default 512), which can reject solvable domains when set below the
  fixpoint's convergence round.

### Fairness

Strong-cyclic soundness relies on the standard fairness assumption: every
non-deterministic outcome that is reachable infinitely often along an
infinite execution eventually occurs. Under fairness, a strong-cyclic policy
guarantees eventual goal reachability but no bounded step count; that weaker
guarantee is exactly what the retry-loop construction buys.

### Outcome-closed (TN, state) policies

For FOND *HTN* problems, the object a policy is defined over is not the bare
state but the pair (remaining task network, state): a policy maps each pair
reachable under the policy to an action, such that every execution under
every outcome resolution reaches a state whose task network is completed and
whose facts satisfy the goal — the policy must be closed under outcomes,
prescribing a move for every pair it can reach. The (TN, state)-policy
formalization for fully observable non-deterministic HTN planning is due to
Chen and Bercher (AAAI 2021), including the observation that method
commitment and outcome resolution interleave: committing to a method before
its non-deterministic outcomes are known is what makes flexible-HTN FOND
distinct from flattening the hierarchy first.

Ferroplan realizes the (TN, state) pair concretely as the *composite state*:
(ground-fact-set, task-network frontier). The returned `UniversalPlan`
policy is outcome-closed in the operational sense: each entry carries the
full outcome set of its chosen action (`PolicyOutcome` per edge).

### Known gaps

Of the wave-1 audit items (see the wave context), the strong-cyclic truncation
dead-sink is fixed — the structural `states + 1` failsafes plus the final
outcome-closure check above; the dead tautological post-check in `fond_policy`
is gone (kept as a debug-only tripwire asserting the admission invariant); and
the depth-cache false-`NoPlan` in `contingent_policy` is fixed (commit
`52c3501` lineage). Still open, each ticketed: `=` goal-literal evaluation
(ticket 21), parse-phase depth budget (ticket 22), translate capacity against
the hard-coded internal 10 s wall (ticket 23, below), conditional-effect
grounding limits on some PANDA shapes (wave-3 ticket 05), and
`probabilistic_policy`'s value iteration remaining `max_iterations`-gated
(noted on the field). This document describes the landed construction; the
open tickets bound what it currently guarantees.

## 3. Architecture

| Stage | Location | Role |
|---|---|---|
| `:probabilistic` text pre-pass | `crates/ferroplan-hddl/src/probabilistic.rs` (`preprocess`) | Rewrites weighted `:probabilistic` blocks into `oneof` + side-channel weight map before tokenizing. |
| Parse | `crates/ferroplan-hddl/src/parser.rs` (`parse_domain`, `parse_problem`) | HDDL text -> `ast::Domain`/`ast::Problem`. Typed refusals per Section 1. |
| Validate | `crates/ferroplan-hddl/src/validate.rs` (`validate_domain`, `validate_problem`) | Pre-grounding static checks: task/predicate/type resolution, arity, duplicates. |
| Ground | `crates/ferroplan-hddl/src/grounder.rs` (`ground`) | Type closure, object indexing, substitution; full combinatorial grounding by default, optional delete-relaxation pruning (`prune_unreachable`). `oneof`/`when` expand into grounded effect branches. |
| Translate | `crates/ferroplan-hddl/src/translate.rs` (`translate`) | BFS over the reachable *composite* (fact-set, TN-frontier) state space. Decomposition-aware: only actions reachable via an actual decomposition become transitions. `oneof` outcomes become probability-weighted transitions; method choice and execution order become ordinary `htn:decompose:*`/`htn:exec:*` transitions; TN completion is the synthetic `htn:done` fact. |
| Adapt | `crates/ferroplan/src/hddl.rs` (`adapt_problem`) | Field-for-field mapping into `planning_runtime::PlanningProblem`. Decomposition transitions get zero cost/duration: bookkeeping, not world action. |
| Solve | `crates/ferroplan/src/hddl.rs` (`solve_hddl`) -> `planning_runtime::solve_planning_type` | `PlanningType::Fond`: `fond_policy`, falling back to `fond_policy_strong_cyclic` on `NoPlan`. HDDL is a front-end, not an 18th+ paradigm, so the result reports `Fond` — honestly naming the solver that ran. |
| WASM | `crates/ferroplan-wasm/src/wasi_abi.rs` | Ops `hddl_solve` (HDDL text -> `UniversalPlan` JSON; per-stage error codes `FP_PARSE`/`FP_HDDL_GROUND`/`FP_HDDL_TRANSLATE`/`FP_MODEL`), `htn_plan` (JSON problem, forces `PlanningType::Hierarchical`), `fond_policy` (JSON problem, forces `PlanningType::Fond`). |
| Eve | `crates/ferroplan/src/eve.rs` (`EveStage::DecomposeHddl`) | Stage sits between `ProjectGenesis` and `GovernUncertaintyPpddl`. Currently no consumer — the Eve bridge is in flight (wave ticket T10). |

Boundedness: every phase after the parser enforces its own internal
state/iteration/wall-clock limits with loud typed refusals
(`TaskNetworkDepthExceeded`, `MemoryLimitExceeded`, `Timeout`,
`LimitExceeded`). `solve_hddl` adds a watchdog thread over the *entire* call
keyed off `limits.max_wall_ms` (the parser itself has no internal clock), so
the caller never waits past budget even though a worker thread cannot be
forcibly killed.

### Budget knobs

Three limit structs bound the pipeline. Defaults are what a plain
`Default::default()` caller gets; `0` / `None` conventions differ per struct
and are stated exactly.

| Knob | Location | Fields and defaults |
|---|---|---|
| `PlannerLimits` | `crates/ferroplan/src/planning_runtime.rs` (serde-visible via `UniversalPlanningRequest`) | `max_depth` 128 · `max_states` 100,000 · `max_iterations` 512 · `max_wall_ms` 10,000 ms — `max_wall_ms == 0` means unbounded; the wall deadline is checked once per fixpoint round, and `solve_hddl` keys its whole-call watchdog off the same field |
| `GroundingLimits` | `crates/ferroplan-hddl/src/grounder.rs` | `max_ground_actions` 10,000 · `max_ground_methods` 10,000 · `prune_unreachable` `false` · `max_wall` `Some(10 s)` — `None` means unbounded; each grounding sub-phase checks the wall against its own start time, so total ground wall is a small multiple of the per-phase budget |
| `TranslateLimits` | `crates/ferroplan-hddl/src/translate.rs` | `max_task_network_depth` 64 · `max_wall` `Some(10 s)` · `max_states` `Some(200,000)` interned composite states — `None` means unbounded; both are checked once per state popped off the BFS queue |

Honest caveat, current behavior (ticket 23 pending): `solve_hddl`'s pipeline
(`solve_hddl_inner`, `crates/ferroplan/src/hddl.rs`) hard-codes
`GroundingLimits::default()` and `TranslateLimits::default()`, so a caller's
`max_wall_ms` above 10 s **cannot** lift the internal translate wall — four
IPC-2023 instances (PCP_1, PO_Transport, Satellite-GTOHP, Transport) hit
`TranslateError::Timeout` at that wall with ~10k composite states interned and
more still queued (`crates/ferroplan/tests/fixtures/htn-ipc2023/RESULTS.md`,
the `LIMIT:translate-wall` rows), while the external oracle solves each in
under 1.3 s. Plumbling the caller budget (and a public composite-state budget)
through into the internal caps is ticket 23, in flight; the paragraph above
describes what the code does today, not what it is meant to do. Loud refusal
at the wall stays lawful either way: the translator never silently truncates.

### Capacity numbers

Wave-4 measured capacity (all numbers committed FACTS, reproduced from the
cited in-repo files; machine for every row: Apple M3 Max, 16 cores, 48 GB,
macOS 26.2):

- **Criterion bench** (`crates/ferroplan/benches/BENCH-FOND.md`, ticket
  fond-htn-26; `cargo bench -p ferroplan --bench fond -- --quick`, 10
  samples, seed `0x5EED_2026_0917`): strong fixpoint `chain_200` mean
  **15.53 ms ± 0.32**, `lattice_200` (20×10) mean **2.26 ms**, strong-cyclic
  `mixed_ladder_200` mean **22.40 ms ± 0.06** (wall includes the preceding
  strong-fixpoint NoPlan pass — production dispatch is strong first,
  strong-cyclic on `NoPlan`). `solve_planning_type` dispatch floor on a
  2-state chain: **899 ns ± 11 ns**. CI tripwires pin these shapes in
  `tests/fond_threshold.rs` (release bound 50 ms / dev 1200 ms).
- **IPC-2023 full sweep** (`crates/ferroplan/tests/fixtures/ipc-sweep/RESULTS.md`,
  ticket fond-htn-27; `cargo test -p ferroplan --test ipc_sweep -- --ignored`,
  2026-09-18T00:21:12Z): **12/43 SOLVED** end-to-end, 0 NOPLAN. Refusal
  histogram, 28 LIMIT refusals: `ground-actions` ×14, `translate-wall` ×9
  (the pre-ticket-23 internal 10 s translate wall), `ground-methods` ×3,
  `solve-wall` ×2; plus 3 `GAP:ground` validation refusals (duplicate type
  declaration; variable-arg root task-network subtasks). No panics, no
  garbage outcomes — every one of the 43 answers is an honest typed verdict.
- **Scaling ladder** (`crates/ferroplan/tests/fixtures/scaling-ladder/RESULTS.md`,
  ticket fond-htn-28; `cargo test -p ferroplan --test scaling_ladder --
  --ignored`, seed 20260917, max RSS ~93 MiB): **zero refusals through
  n=128** in both families (chain-world, transport-drop). Envelope: ground
  linear in n (transport) / quadratic (chain's n²+n schemas); translate
  linear in composite states but walls ≈×5 per doubling past n≈32
  (per-state fact-set cloning, quadratic in n); solve is the steepest stage
  at ≈×7.7–7.9 per doubling (≈ cubic — fixpoint rounds × full rescan) and is
  the **projected capacity knee**: the 60 s solve bound first bites at
  ≈n 200–210 (transport-drop) / ≈n 430 (chain-world). These are projections
  from measured doubling factors, not observed refusals.

Dependency direction: `ferroplan-hddl` has zero dependency on `ferroplan`;
`ferroplan` depends on `ferroplan-hddl`, never the reverse.

## 4. Oracle methodology

Ferroplan's FOND-HTN verdicts are differentially tested against an external
oracle — a third-party FOND-HTN planner used strictly as a test oracle. The
oracle lives only under `/tmp` (`/tmp/fond-review/Planner`, its own checkout,
plus `/tmp/fond-oracle`, our runner harness). Its code is never copied into
any git repo or worktree; only its *concepts* inform this document. The
binding rules are in the wave context (Section 6 below).

- **Runner contract.** `/tmp/fond-oracle/build.sh` is idempotent and
  flock-protected; `oracle-run.sh <domain> <problem> [--mode
  flexible|fixed-ld|fixed] [--timeout SEC] [--heur ...]` prints exactly one
  JSON verdict line: `status` (`SOLVED|NOSOLUTION|ERROR|TIMEOUT|PARSE_ERROR`),
  `mode`, `wall_s`, and an artifact path. A `NOSOLUTION` verdict is a
  *successful* oracle run; only harness failure exits non-zero.
- **Flock serialization is mandatory.** The oracle's own solve script writes
  fixed shared temp paths (parsed network, serialized output, result JSON),
  so concurrent invocations corrupt each other. All oracle invocations are
  therefore serialized through the flock-protected runner — never called
  concurrently, from any ticket, ever.
- **Golden verdict files are facts, not code.** Recorded oracle results
  (status/wall/policy shape) are committed as JSON fixtures and treated as
  evidence: they record what the oracle said, not behavior ferroplan must
  reproduce blindly. IPC/competition domain files under `/tmp/fond-corpus/`
  are competition data and may be vendored into repo fixtures with
  provenance; hand-authored fixtures carry a one-line provenance comment.
- **/tmp lifecycle.** `/tmp` is ephemeral. The build recipe (brew
  bison/flex/gengetopt; grounder via `make CXX=g++`; bliss unzip ->
  memleak-patch -> `__DATE__` sed -> `make CC=g++`, patched *after* cpddl's
  re-unzip) is recorded in the wave context for re-application after a wipe.
- **Differential protocol.** Solve with ferroplan, solve with the oracle,
  compare verdict class (policy found vs. none) and policy shape, per
  corpus instance. Disagreement is a defect report, not noise.
- **Consolidated results.** Every committed differential and harvest
  number — per corpus, with reproduction commands and the verdict-legend
  meanings — is tabulated in
  [`docs/BENCHMARKS.md`](BENCHMARKS.md); that page consolidates only
  committed results and never re-measures.
- **Known oracle quirks (expected; do not "fix").** `Success probability:
  4.0000` is a solved-leaf count, not a probability. The oracle's "flexible"
  mode finds strong plans; its strong-cyclic support is preliminary, so a
  `flexible + NOSOLUTION` verdict is treated carefully and cross-checked
  with `--fixed-ld` before it is allowed to overrule ferroplan.
- **Negative corpus.** Known-flawed-model domain files that live inside the
  oracle's parser repository run from `/tmp` as an external negative corpus;
  they are never vendored.

## 5. Migration deviations

| Reference concept | Ferroplan equivalent | Status |
|---|---|---|
| Goal = empty task network (TN-emptiness only) | Goal = `htn:done` synthetic fact (TN frontier empty) AND the domain `:goal` facts — a real HTN solution requires the *entire* network reduced to executed primitives with `:goal` as an additional requirement on the final facts, never `:goal` holding on a half-decomposed state | Implemented |
| Method preconditions compiled away | `MethodDef.precondition` retained as a first-class AST node; grounded form checked by `translate` — an unconditional method decomposes exactly as before | Implemented; stricter than the reference dialect |
| `when` inside a `oneof` branch parsed, then silently dropped downstream | `ParseError::MalformedOneof` — loud refusal; ferroplan never accepts a shape it would silently weaken | Deliberate deviation (Section 1) |
| `oneof` position surface (entire top-level `:effect`, `k >= 1`) | Same surface, landed on main from frozen `fix/oneof-koala-semantics` (`4d40f99`); earlier permissive behavior (`oneof` under a top-level `and`, silent goal-position mis-parse) refused | Implemented (aligned) |
| `:probabilistic w e ...` weighted-effect extension with side-channel weight map | `probabilistic::preprocess` — same shape in Rust: text pre-pass, weights per enclosing `:action` name, raw source text preserved | Implemented |
| Fixed vs flexible method commitment (commit the decomposition up front vs. allow commitment to vary with the execution branch) | Method choice is an explicit OR-branchpoint (`htn:decompose:<addr>:<method>`) inside the composite state space: the policy commits per composite state and may choose differently in different states — the flexible end of the spectrum, in the (TN, state)-policy sense of Chen & Bercher | Implemented (policy-level flexibility; no up-front commitment) |
| Flexible / fixed-ld / fixed oracle solving modes | Differential-testing inputs only — verdict classes to compare against, not ferroplan modes | External oracle only |
| Problem objects typed by an undeclared type (the oracle corpus's AssemblyHierarchical declares no `FaultyPort` type while its problems type objects `faultyCable-* - FaultyPort`; reference parser *and* grounder exit 0 with no diagnostic — harvest evidence, ticket fond-htn-02: 2 problem references to the undeclared type became 0 objects in the parsed model, recorded verbatim in `crates/ferroplan/tests/fixtures/fond-htn/oracle-harvest-full.json`) | `ValidationError::UndefinedType` at validation, before grounding (`crates/ferroplan-hddl/src/validate.rs`) — ferroplan rejects loudly what the reference silently drops. Kept over parity: a silently-weakened model can mis-solve; a typed refusal cannot. Ferroplan's rejection is confirmed correct by the same harvest records | Deliberate deviation — parity divergence |
| Strong-cyclic decision procedure (retry-loop policies) | `fond_policy` -> `fond_policy_strong_cyclic` fallback decides both strong and strong-cyclic classes | Parity divergence: the oracle is strong-only as a decision procedure — all five canonical cyclic-only domains TIMEOUT under both `--flexible` and `--fixed-ld` while ferroplan returns outcome-closed strong-cyclic policies (frozen evidence: `crates/ferroplan/tests/fixtures/fond-flat/oracle-goldens.json`, ticket fond-htn-06). Oracle TIMEOUT on a retry-style domain is the expected signature, never overruled by ferroplan without a cross-check |
| `Success probability: <n>` output line | — | Not a probability: the number is a solved-leaf count (Section 4) |

## 6. Compliance box

The KOALA POLICY, verbatim from the wave context
(`docs/jira/v26.9.17/_FOND-HTN-WAVE-CONTEXT.md`), binding on every ticket:

> - koala runs ONLY from `/tmp/fond-review/Planner` (its own checkout, fine)
>   and `/tmp/fond-oracle` (our runner harness).
> - NEVER copy koala code, grammar, build scripts, or verbatim files from
>   koala-planner org repos (`Planner`, `HDDL-Parser`, `domains`) into ANY git
>   repo or worktree — including fixtures. Use koala **concepts** only.
> - Ferroplan-repo fixtures are either (a) IPC/competition files from
>   canonical repos already fetched under `/tmp/fond-corpus/`
>   (ipc2023-htn, panda-planner-dev/ipc2020-domains — competition data, not
>   koala code; these MAY be copied into repo fixtures), or (b) hand-authored
>   by you (add a one-line provenance comment).
> - Golden JSON files recording oracle RESULTS (status/wall/policy shape) are
>   facts, not code — committing them is allowed and encouraged.
> - Sleath–Bercher flawed-model files live inside the koala HDDL-Parser repo
>   → run them from `/tmp` as an EXTERNAL negative corpus; do not vendor them.

This document cites the public literature and koala *concepts* only. Every
koala-behavioral claim above is anchored to the wave context or to this
repo's own landed code and commit messages, never to koala source.

## References

- Alessandro Cimatti, Marco Pistore, Marco Roveri, Paolo Traverso.
  *Weak, Strong, and Strong Cyclic Planning via Symbolic Model Checking.*
  Artificial Intelligence 147(1–2), 2003.
- Yuchen Chen, Daniel Bercher. *Fully Observable Nondeterministic
  Hierarchical Planning.* AAAI 2021.
- Daniel Höller, Gregor Behnke, Daniel Bercher, Susanne Biundo, Humbert
  Fiorino, Damien Pellier, Ronald Alford. *HDDL: An Extension to PDDL for
  Expressing Hierarchical Planning Tasks.* IJCAI 2020.
