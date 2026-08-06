# Coverage borders & the decomposition ruleset

Field map for the **subproblem-maker**: the exact size a contract can carry before
ferroplan's one-shot temporal search stalls out — the line the decomposer works,
whole handoff on one side, forced break-up on the other. Every number below is
measured, not estimated (see `rpg-world/suite/`, `rpg-world/hard/`, `logistics/`,
`jobshop/`).

## The unifying law

> ferroplan's delete-relaxed temporal heuristic keeps a clean gradient on **linear /
> accumulative** work, and goes **flat the instant ≥2 contributions must converge
> onto one goal quantity.**

Trace any failure and it reads the same: *converging-contributions ≥ 2*. The
relaxation logs the first contribution, marks the `>=` goal satisfied, goes dark on
the rest. A contract holds together whole iff **(i)** it's one accumulating/
processing chain inside the op budget, or **(ii)** every goal quantity in it takes
**at most one** converging contribution.

> **Update (temporal landmark term).** The temporal search carries a
> numeric-threshold *landmark deficit* in its phase-1 key now, and the gradient
> comes back for **single-round** converging DAGs — a join whose inputs are each
> produced *once* from cold solves clean (the from-scratch ingot, the metallurgy
> benchmark), and big linear accumulations run faster too. What's left standing is
> **multi-round** convergence (a goal wanting N of a product, so each intermediate
> gets pulled N times — `steel ≥ 2` from cold) plus the other shapes below; those
> still route to decomposition. The subproblem-maker's rule holds unchanged as *safe*
> default — stage inputs — but the engine gives more room now for one converging step
> left inside a contract.
>
> **Update 2 (`FF_TDEMAND` converging-resource demand term).** Opt-in: the temporal
> search regresses the numeric goal down the recipe DAG to a total per-resource
> demand and steers on cumulative availability — the gradient the relaxation never
> had for **multi-round** convergence. Measured lift: RPG coverage 26→34/39, all
> validated, multi-round converging DAGs solving now (`steel≥2` from cold), cyclic
> regen (`grain≥10`), multi-path numeric goals (`coin≥15`). Under `FF_TDEMAND` the
> **converging-contributions ceiling is no longer 1** for numeric goals — a contract
> can carry a full multi-round numeric chain. Still routes to the decomposer:
> **predicate/structural conjunctions** (the monolithic "village shape" `built-wall`,
> multi-structure `found-village`, big mixed `order-8/12`) — the demand term is
> numeric-only, doesn't touch those.
>
> **Update 3 (numeric demand is now default-on).** As of v0.2, the numeric half of
> Update 2 ships **default** (the `Numeric` tier) — the multi-round ceiling lift
> above applies with no flag set. Only the *predicate/structural* half
> (predicate-goal-threshold seeding + goal-relevance pruning) stays gated behind
> explicit `FF_TDEMAND` (the `Full` tier): seeding demand from a renewable-pool guard
> (`(>= (avail) 1)`) serializes concurrency domains, so that piece waits for an
> opt-in. `FF_NO_TDEMAND` kills all demand guidance if you need the old path back.
>
> **Update 4 (goal-relevance pruning is now default-on, v0.3.0).** Pruning cut loose
> from the `Full` tier and rides default now (`FF_NOREL` disables pruning alone;
> `FF_NO_TDEMAND` still restores the pristine pre-v0.2 path). The trigger case: on a
> fully-featured hub, even a 5-step chain (`flour ≥ 2`: till→plant→irrigate→
> harvest→mill) burned through the node budget chasing goal-irrelevant unbounded
> accumulators. Pruned, it solves in ~30 ms. The pass structure picked up an
> **unmasked complete backstop** (helpful/sound → full/tight → full/sound →
> full/unmasked) — completeness stops depending on the mask entirely. Full-corpus
> measurement (suite + hard + contracts + cabin + villagers): **65/75 → 67/75, zero
> losses, zero makespan changes.** The `gather-build` village shape solves under
> plain defaults now; only the *predicate demand seeding* half of the `Full` tier
> stays opt-in. Statically unproducible goals (a goal fact with no adder, a
> threshold no effect can raise) fail in microseconds instead of burning every pass.
>
> **Update 5 (on-failure escalation ladder, v0.3.0).** A default-tier monolithic
> search that fails now retries at the `Full` tier, then hands off to the decomposer
> — automatic, no flag. Each rung fires only on failure, so nothing that already
> solves changes; a would-be failure just gets more machinery thrown at it. Corpus:
> 67 → 73/75 (crew-solo/pair + skilled-specialists caught at the Full rung;
> order-8/12 + found-village caught at the decomposer rung; all validated). Practical
> read for a subproblem-maker: the "MUST SPLIT" rules below are advisory for
> *performance* now, not *coverage* — an oversized conjunction costs the ladder's
> extra minutes instead of failing outright. `crew-trio` and `skilled-crosstrained`
> stay the measured border — no rung reaches them. `FF_NO_ESCALATE` restores
> single-rung behavior.

## Border table (measured)

| shape | safe to hand whole | first fail | split unit |
|---|---|---|---|
| linear single-resource accumulate | ≤ **2000** primitive ops | 2001 (ore 3999→4000) | `ceil(ops/2000)` |
| deep **travel** (corridor) | ~**100** hops | ~200 (agent-location goes relaxed-"everywhere") | route in ≤100-hop legs |
| shallow (depth-1) **conjunction** | ≥ **10** independent parts | not the bottleneck | groups of ~10 |
| a single **depth≥2 chain**, alone | yes (sole goal) | **arity 2** — the moment any sibling is added | 1 chain per contract |
| **converging join** (2-input recipe) | ≤ **1** fresh sub-chain (others pre-staged); may accumulate N | ≥2 fresh sub-chains converging | 1 fresh input per contract |
| **farming harvest** (cyclic regen) | **1** harvest (≤3 grain) | the 2nd harvest — same whether cyclic *or* parallel fields | `ceil(N/3)` single-harvest |
| **multi-source numeric**, from scratch | **1** unit | 2 units (coin 2) | 1 unit / 1 chain per contract |
| multi-source numeric, **inputs in stock** | ≥ 30 units (makespan-linear) | none seen | stage inputs first |
| **logistics** leg (per-location goods) | **1** unit · 1 vehicle · any #hops | 2 units OR a 2nd vehicle/transshipment | per-unit, per-leg, per-package |
| **jobshop** (independent jobs) | ≤ ~**40k** operate groundings (100 jobs×20×20, 45s) | ~90k (100×30×30) | partition by jobs (never by machine/stage) |

Two anchors hold across every row: **(1)** op-count ceiling ≈ 2000 for a clean
linear chain; **(2)** converging-contributions ceiling = **1**.

## HAND WHOLE — a contract may contain

- one linear single-resource accumulation, ≤2000 ops;
- a wide-but-shallow conjunction (≥10 independent depth-1 deliverables), inputs staged;
- a single depth≥2 chain as the **sole** goal;
- a converging recipe where **all but one** input is pre-staged inventory (accumulate that one N times if needed);
- one farming harvest (≤3 grain);
- a multi-source numeric goal whose inputs are **already in stock** (scales to 30+);
- a logistics single-unit / single-vehicle leg over any number of hops;
- a whole job-shop under ~40k route-table tuples.

## MUST SPLIT — boundary → rule

- **>2000 ops** → `ceil(ops/2000)` contracts, sum the partials.
- **any depth≥2 chain conjoined with ≥1 sibling** → pull each multi-step deliverable into its own single-deliverable contract; depth-1 siblings may stay grouped (≤10).
- **a join needing ≥2 fresh sub-chains** → stage all-but-one input as inventory; one fresh input per contract; sequence stagers, then a final join contract.
- **≥2 harvests (grain≥4)** → `ceil(N/3)` single-harvest contracts (extra fields do *not* help).
- **from-scratch multi-source numeric >1 unit** → 1 unit / 1 chain per contract, or pre-stage inputs.
- **logistics beyond 1 unit / 1 vehicle** → split per-unit, per-leg at each transshipment, per-package.
- **jobshop over the tuple budget** → partition by **jobs** (jobs are independent; never slice by machine or stage).

## How the rules generalize across domains

- **rpg-world** (crafting/economy): where the numbers above were measured.
- **logistics** (per-location goods, trucks/trains, capacity): *same failure family,
  arrives sooner.* The per-location stock model is relaxation-hostile — almost any
  non-trivial delivery is already a converging-flow problem. Qualitative rules
  transfer verbatim; the quantitative allowances collapse to **multiplicity 1** (1
  unit, 1 vehicle, 1 leg). Deep travel stays free, same as rpg-world.
- **jobshop** (scheduling, machine-exclusion): the heuristic-shaped thresholds
  **don't apply** here — jobs are independent linear chains that never converge,
  which is the engine's strong suit (it clears **100 jobs** clean). The only ceiling
  is grounding-table size; slice by jobs when it's hit.

Bottom line for the subproblem-maker: **converging-contributions = 1** is the master
invariant across all three domains; op-count and travel-depth ceilings are
secondary. Ask "does this domain behave like logistics (collapse to 1) or like
jobshop (slice by independent units)?" — that call picks the quantitative budget.
