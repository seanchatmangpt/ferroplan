# Dispatch — the run to v0.2.1 ("The Bridge") — CLOSED CASE

> **Archived. Every phase below landed** (shipped in v0.2.1–v0.4.0): the proven
> flags graduated to defaults, temporal depth came online (TILs, duration
> inequalities, the escalation ladder), the decomposer + `Session` API shipped, the
> MCP server followed it out the door, crates.io publishing closed the loop. Then
> 0.4.0 picked up the IPC-5 preference track (see the
> [scoreboard](../benchmarks/ipc5-scoreboard.md)). This file stays on record as the
> plan that ran. **Live work** is in the [0.5 roadmap](roadmap-0.5.md)
> ("First Place"), which folds in the scoreboard's "Path to climb" (large-instance
> tails; rovers completion-aware pricing) and the
> [ESPC spec](espc-preferences-spec.md).

ferroplan v0.1 came out of the gate a mature FF-family engine, running on one bet
staked in the README: **an LLM should author and supervise a planner, not live
inside it as its runtime.** At the time that bet was still just talk — the engine
ran deep, but the *bridge* the thesis promised (natural-language goal →
decomposed, solvable contracts → an agent working the author→run→read→fix loop
against a real tool) hadn't been poured.

v0.2.1 closes the gap. Harden what's already proven, land one engine-depth win,
**pour the bridge**, ship it. Four phases, ordered so each one derisks the next:
cheap hardening first — low exposure, compounds — then a credibility win, then
the bridge itself (needs a solid engine underneath it), then distribution last,
so what goes out the door is already solid.

```
Phase 1: Graduate proven flags ──► Phase 2: Temporal depth (TILs) ──┐
         (hardening, low risk)              (credibility)            │
                                                                     ▼
                                            Phase 3: Decomposer ──► Phase 4: MCP + publish
                                                     (the bridge)            (reach)
```

Why "0.2.1" and not "1.0": the README reserves 1.0 for API stability ("APIs may
shift before 1.0"). This drop adds a public `decompose` surface and rewrites
default heuristics — exactly the churn 0.x exists to soak up. 1.0 comes after
0.2's APIs have sat in real use and held.

---

## Phase 1 — Graduate the proven opt-in flags to defaults

**Progress:** `FF_TDEMAND` made the cut (the numeric half). It split into tiers
rather than take a blind flip — a blind default regressed makespan on
renewable-resource concurrency domains (the `crew` pool serialized ~5→~10),
because the predicate-goal-threshold seeding misread a net-zero pool guard as
accumulation demand. The default now sits at the `Numeric` tier (numeric-goal
demand only — the full measured +8, RPG suite+hard 26→34/39, **no regression**);
the predicate/structural half + relevance pruning still rides explicit
`FF_TDEMAND` (`Full`); `FF_NO_TDEMAND` opts out clean (bit-identical to 0.1.0).
The override layer runs tri-state now (`features::DemandMode` /
`clear_overrides`). **Still open in Phase 1:** ESPC's latency trade — its outer
loop runs wall-clock-bounded, so "default on" isn't free on deadline-structured
domains (call it: always-on-where-it-bites, or a smaller default budget) — and
confirming the temporal landmark term needs nothing further (it's already
always-on).

**Why run it:** `FF_TDEMAND`, ESPC, and the temporal landmark term are all
measured wins (`FF_TDEMAND`: RPG 26→34/39, all validated; ESPC: openstacks p08
608→227) carrying "bit-identical when off" guarantees. They've been sitting
behind env vars, which means (a) the default planner runs needlessly weaker than
the engine underneath it, and (b) the Phase-3 decomposer would otherwise have to
memorize which magic vars to set. Flipping them on by default is the cheapest
high-value move in the release.

**Scope:**
- Flip the defaults in `crates/ferroplan/src/features.rs` (the `tdemand()` /
  `tdecomp()` / `tconc()` getters and `set_overrides`) and the ESPC entry in
  `pddl3.rs:791`. Keep an **escape hatch** — invert each to an `FF_NO_*` opt-*out*
  so anything depending on the old default can recover it, and the byte-identical
  regression baseline stays reachable.
- Decide per-flag whether "default on" means *always* or *only when the cheap
  applicability check fires* (TDEMAND already goes inert on domains without the
  converging-DAG shape; ESPC goes inert without make-deadline structure — so
  "always on" really means "on where it does anything").
- Run the full corpus (`benchmarks/`, `examples/rpg-world/suite/`, `hard/`) under
  the new defaults and confirm: no coverage regressions, every plan still
  validates.

**Acceptance:** default-mode coverage ≥ current opt-in coverage on every suite;
every produced plan validates (in-crate + VAL where available); `FF_NO_*`
recovers the old byte-identical behavior; CHANGELOG carries the default change.

**Touches:** `features.rs`, `pddl3.rs`, `temporal.rs`, the tests in
`tests/tdemand.rs` / `tests/espc.rs` (flip set-var → assert-default + opt-out
test), `benchmarks/`, `CHANGELOG.md`, `README.md` (Limitations section).

---

## Phase 2 — Temporal depth: timed initial literals + duration inequalities

**Progress: closed.** Duration inequalities — `:duration` parses `=` / `>=` /
`<=` / `and`-ranges into a `types::Duration { min, max }`; the search commits to
the shortest feasible duration (lower bound), the validator accepts `[min,
max]`. Timed initial literals — `(at <time> <literal>)` parses (told apart from
the `(at ?x ?y)` predicate by a numeric first arg), compiles to a synthetic
applier (grounds the fact, keeps the relaxed heuristic from dead-ending), fires
off a pre-seeded agenda, floors the STN re-timing so gated actions can't slide
before their gate, replays clean in the validator. Self-contained tests cover
both; the fixed-duration RPG corpus holds unchanged (26/27 suite, full lib suite
green). IPC temporal domains aren't vendored (licences), so coverage runs
through crafted domains + the in-crate validator. **Left on the table for a
future pass:** the decision-epoch search timeout on large instances, and
continuous (`#t`) effects.

**Why run it:** the highest-credibility engine addition on the table, and far
more tractable than continuous `#t` effects. TILs (`(at <t> (fact))` in the
init) and duration inequalities (`(<= ?duration N)` instead of `(= ?duration
N)`) unlock a real slice of the IPC temporal suite ferroplan couldn't even
express before. Pairs naturally with the decision-epoch search timeout — both
live in `temporal.rs` / `tsched.rs` / `tresolve.rs`.

**Scope:**
- **Parser:** accept timed initial literals in `:init` and `<=`/`>=` duration
  constraints in `:durative-action` (`lexer.rs` / `parser.rs`).
- **Search:** TILs become scheduled exogenous events on the decision-epoch
  timeline; duration inequalities turn a fixed-duration commitment into a
  bounded choice the scheduler resolves (start shortest-feasible, the FF
  default).
- **Validation:** extend the in-crate temporal validator (`temporal::validate`)
  to honor TILs and variable durations; keep VAL cross-checks green.
- **Search timeout (stretch):** profile the large temporal instances timing out
  today; cheapest win is likely tighter relevance pruning, or handing the
  Phase-3 decomposer the default temporal path for over-budget instances.

**Acceptance:** parse + solve + validate a TIL domain and a duration-inequality
domain from the IPC temporal suite that fail today; no regression on the 44/45
already-valid temporal plans; new coverage numbers land in
`benchmarks/temporal-results.md`.

**Touches:** `lexer.rs`, `parser.rs`, `temporal.rs`, `tsched.rs`, `tresolve.rs`,
`features.rs` (a `--temporal` capability surface), `benchmarks/temporal-results.md`,
README Limitations.

---

## Phase 3 — The bridge: the contract decomposer

**Progress: first cut landed.** `decompose(domain, problem, &Options)` and
`ff --decompose` now surface the partition-and-resolve engine (previously
gated behind the `FF_TDECOMP` flag, which handed back only the flat plan) as a
first-class, typed, serde-serializable `Decomposition { contracts, plan,
monolithic }`: each `Contract` carries its sub-goal, its sub-plan, its offset
in the stitched timeline; an un-splittable goal falls back to one monolithic
contract, reported honestly, no pretending. `tresolve::solve` now delegates to
a recording `decompose` (the `FF_TDECOMP` plan path stays byte-unchanged).
Proved out on `hard/order-8` & `order-12` (8 / 12 contracts, stitched +
validated) — instances the one-shot search flat fails on; self-contained
decompose tests + the full suite green. **Open follow-ups:** drive the split
straight off `BORDERS.md`'s op-count / converging-join rules (today it runs
through interaction-graph partition + conflict-merge instead), and a
natural-language → PDDL front end that emits the contracts.

**Why run it:** this is the thesis, full stop. `examples/BORDERS.md` is already
a *measured, precise* ruleset for when a single contract solves whole versus
must be split, and how to split it (op-count ceiling ≈2000; converging-
contributions ceiling; per-shape split rules). So this is "encode a known
spec," not "go do research." The subproblem-maker has been a human reading a
table — turn it into an actual tool.

**Scope:**
- A `decompose(domain, problem) -> Vec<Contract>` surface in the library: fed a
  goal past the borders, emit a **sequenced** list of solvable sub-contracts
  (each a `(domain, sub-problem)` whose goal sits inside the borders), staging
  and ordering dependencies attached.
- Drive each contract through the engine to confirm it actually solves; stitch
  the partial plans into one plan against the original goal and validate the
  whole run.
- Encode the `BORDERS.md` rules as the splitting policy (op-count ceiling,
  converging ≥2-input joins → stage all-but-one input, jobshop-by-jobs-never-
  by-machine, and so on). `FF_TDECOMP` (the existing partition-and-resolve
  path) is the engine-level primitive underneath; this phase adds the
  goal-level, cross-contract planner on top.
- CLI: `ff --decompose -o domain -f problem` prints the contract sequence + the
  stitched plan. Library: typed, serde-serializable `Contract` / `Decomposition`.

**Acceptance:** a goal from `BORDERS.md`'s "MUST SPLIT" column that one-shot
search fails on gets solved end-to-end via the decomposer, the stitched plan
validating against the original problem; the decomposition runs deterministic
and inspectable.

**Touches:** new `decompose.rs` module, `api.rs`, `partition.rs`/`resolve.rs`
(reuse), `ferroplan-cli`, `examples/BORDERS.md` (link the implementing module),
new `examples/` showcase, README ("make the thesis real").

---

## Phase 4 — The bridge, shipped: MCP server + publish

**Progress.** *crates.io setup* — version bumped to 0.2.1, CLI dep pin synced,
CHANGELOG cut to `[0.2.1]`, the full `RELEASING.md` pre-flight green
(`fmt --check`, `clippy -D warnings`, `RUSTDOCFLAGS=-D warnings cargo doc`,
`cargo package -p ferroplan` packages + verifies); both crates stand ready to
publish but **not yet pushed live** (held by request). *MCP server* — closed:
`crates/ferroplan-mcp` exposes `solve` / `validate` / `decompose` over stdio
(self-contained newline-delimited JSON-RPC 2.0, no async runtime), handing back
the structured `Solution` / `Decomposition`; integration tests drive the built
binary end to end; `publish = false` for now. **Still owed:** the actual
`cargo publish` + `v0.2.1` tag, and the **PyPI** wheel for `ferroplan-py`.

**Why run it:** the MCP server is the artifact that makes the whole bet
*usable by an agent* — `solve` / `validate` / `decompose` as tools an LLM calls
inside the author→run→read→fix loop the `ferroplan` skill already walks
through. Packaging (crates.io + PyPI) is the same distribution story, so ship
them together as one release: the engine goes public *and* an agent can drive
it the moment 0.2.1 lands.

**Scope:**
- **MCP server** (likely a new `crates/ferroplan-mcp` or a mode of the CLI)
  exposing `solve`, `validate`, `decompose`, and the feature table from the
  skill, with the structured JSON the library already returns.
- **crates.io:** publish `ferroplan` + `ferroplan-cli` (resolve the workspace
  version bump to 0.2.1, doc-test the public API, dependency audit).
- **PyPI:** publish the `ferroplan-py` wheel (maturin; it already builds
  standalone).
- Release mechanics per `RELEASING.md`; tag `v0.2.1`; CHANGELOG release notes
  telling the "bridge is real" story.

**Acceptance:** `cargo install ferroplan` / `pip install ferroplan` work clean
off a fresh machine; an agent can `decompose` then `solve` a too-big goal
end-to-end through the MCP server; `v0.2.1` tagged and out.

**Touches:** new `crates/ferroplan-mcp` (or CLI mode), `Cargo.toml` (version,
publish metadata), `crates/ferroplan-py`, `RELEASING.md`, `CHANGELOG.md`, README
(install + MCP quickstart).

---

## The 0.2.1 story

> v0.1 proved the engine. **v0.2 makes the bet real:** the proven heuristics run
> on by default, temporal coverage cuts deeper, a goal too big for one-shot
> search gets *automatically decomposed* into solvable contracts, the whole rig
> installs and runs under an agent over MCP. The README's thesis — LLM as
> author and supervisor, PDDL as the auditable interface, a fast solver as the
> runtime — stops being an argument and turns into a tool you `pip install`.

---

## Signal Intercepts: Frontier Research

Chatter off the arXiv wire, 2026. Six transmissions worth logging — not because
ferroplan needs to become something else, but because they mark where the
symbolic-planning perimeter is moving, and where the receipt-bound engine here
can absorb the gains without giving up the one thing that makes it trustworthy:
a plan you can verify, not a plan you have to take on faith.

The through-line across all six: nobody serious is proposing an LLM *replace*
the solver as runtime. The move is LLM-as-heuristic-author — evolve a heuristic
or pattern generator offline, freeze it into deterministic code, run *that* at
solve time. Soundness survives because the artifact that ships is still C++ or
Rust, still checked against the same validator. That's the same shape as
ferroplan's own thesis (README: "an LLM should be the author and supervisor of
a planner, not its runtime") — these six papers are independent confirmation
the field is converging on the same split.

- **[LLM-Evolved Domain-Independent Heuristics for Symbolic AI Planning](https://arxiv.org/html/2605.29649v1)**
  — an LLM evolves domain-independent heuristic *functions*, not domain-specific
  hacks. The output is code, not a policy — the kind of artifact a `decompose`
  or `solve` call could swap in behind a flag, same as `FF_TDEMAND` graduated
  from opt-in to measured default here. Relevant to Phase 1's pattern (measure,
  gate behind a flag, graduate what holds).

- **[LLM-Evolved Pattern Generators for Optimal Classical Planning](https://arxiv.org/pdf/2606.02438)**
  — pattern databases (PDBs) generated by an evolved LLM search instead of
  hand-tuned or purely combinatorial generation, aimed at *optimal* planning.
  ferroplan's own heuristic stack (RPG relaxation, `FF_TDEMAND`'s tiered
  demand estimation) is satisficing, not optimal — but the evolve-then-freeze
  pipeline generalizes: the frozen pattern generator is still an ordinary
  deterministic artifact once evolution stops.

- **[Hierarchical Task Network Planning with LLM-Generated Heuristics](https://arxiv.org/pdf/2605.07707)**
  — LLM-generated heuristics guiding HTN decomposition. Adjacent territory to
  Phase 3's contract decomposer: `BORDERS.md`'s split rules are hand-derived
  today (op-count ceiling, converging-join thresholds); an LLM-evolved
  heuristic *for choosing splits* is a plausible future generator for that
  table, not a replacement for the deterministic decomposer that executes it.

- **[Frontier Large Language Models Rival State-of-the-Art Planners](https://arxiv.org/pdf/2511.09378)**
  — LLMs directly generating plans, benchmarked against classical planners.
  Read carefully: "rival" is a benchmark claim about generation, not a
  soundness claim. A generated plan still needs the same validator ferroplan
  already runs (`temporal::validate`, VAL cross-checks, `bind_plan_receipt`) —
  this paper is evidence for where LLM-as-generator *fails silently* without
  that check, not evidence the check can be skipped.

- **[Can LLM-Reasoning Models Replace Classical Planning? A Benchmark Study](https://arxiv.org/pdf/2507.23589)**
  — a direct benchmark answer, and the title states its own conclusion's
  shape. The honest reading for this codebase: no, and the receipt-bound
  architecture (`bind_plan_receipt` / `verify.rs`) is exactly the
  infrastructure that stays load-bearing regardless of how the answer moves
  over time — determinism and verifiability aren't a stage the field is
  passing through, they're the property an LLM-reasoning planner doesn't yet
  offer.

- **[LLMs as Planning Formalizers: A Survey](https://arxiv.org/html/2503.18971v2)**
  — a survey of LLMs translating natural language into PDDL, not into plans.
  This is the closest existing literature to the front end Phase 3 flagged as
  an open follow-up ("a natural-language → PDDL front that emits the
  contracts") — worth a close read before that follow-up starts, since the
  survey catalogs known failure modes for exactly that translation step.

**What this doesn't change:** the runtime stays the deterministic FF-family
search this repo already runs. What it might feed, eventually: a heuristic
module, a pattern generator, or a split-policy generator, evolved offline by
an LLM, frozen to a deterministic artifact, checked in like any other module,
measured against the corpus the same way `FF_TDEMAND` was — bit-identical
opt-out included. No phase above is blocked on any of this landing.
