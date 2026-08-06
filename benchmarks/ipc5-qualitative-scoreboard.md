# IPC-5 (2006) qualitative-preferences scoreboard — ferroplan against the field

Vendored suite: `benchmarks/ipc/qualpref/{openstacks,rovers,storage,tpp,trucks}`
— the IPC-5 *qualitative-preferences* track, five domains deep, no pathways
entry in this cut of the archive. The instances carry PDDL3 `(:constraints ...)`
trajectory PREFERENCES — `always`, `sometime`, `at-most-once`,
`sometime-before`, every one `(preference name ...)`-wrapped, none of them
timed — layered on soft goals. Each problem's `(:metric minimize …)` counts
violated preferences (goal and constraint preferences sharing one
`(is-violated name)` namespace); **lower is better**.

One run: `ff -o qualpref/<domain>/domain.pddl -f qualpref/<domain>/pNN.pddl`
(the constraint gate lowers each constraint preference to monitor automata plus
a goal-side preference, then the PDDL3 metric optimizer prices what's left —
see `docs/roadmap-0.7.md` Phase 2).

## Reference status: GRAFTED from the official archive

The reference gap this board carried for three cycles is CLOSED
(2026-07-24). The official `IPC5-results.tgz` sits **vendored now at
`benchmarks/IPC5-results.tgz`** — pulled by hand off the old Brescia
site's live redirect after the Wayback Machine turned up nothing but
a 301, a corpse of a link, never the bytes themselves. Reference
metrics below come straight from the archive's per-instance
`; MetricValue` headers
(`RESULTS/<planner>/<domain>/QualitativePreferences/pNN.soln`). The
parser is cross-checked against the simple-preferences board: it
reproduces every committed SGPlan5 row there, exact, character for
character.

The qualitative field ran **SGPlan5** (the track winner, full 20/20
coverage across all five domains), **HPlan-P** (70/100), **MIPS-XXL**
(16/100), and **MIPS-BDD** (16/100). YochanPS never showed for this
track. Ferroplan's own numbers stay defaults-only, verified the same
way as always — reported equals verified on every oracle-checked
plan.

## The field, p01–p08 — grafted numbers, lower is better, **bold** marks ferroplan ≤ SGPlan5

| openstacks | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 | track cov. |
|---|---|---|---|---|---|---|---|---|---|
| ferroplan (0.16) | **66** | 68.6 | 77.8 | 89.2 | **122.5** | 121 | **283** | **617.7** | 8/8 run |
| SGPlan5 | 70 | 62.4 | 77 | 82.4 | 123.5 | 116.5 | 300 | 619.2 | 20/20 |
| HPlan-P | 76 | 71.2 | 88.8 | 94.2 | 147.5 | 144.5 | 294 | 618.5 | 18/20 |
| MIPS-XXL | 14 | 11.6 | — | — | — | — | — | — | 2/20 |
| MIPS-BDD | 68 | 66 | — | — | — | — | — | — | 2/20 |

| rovers | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 | track cov. |
|---|---|---|---|---|---|---|---|---|---|
| ferroplan (0.16) | **68.04** | **32.67** | **29.19** | **26.06** | 238.66 | **37.39** | **37.64** | **556** | 8/8 run |
| SGPlan5 | 88.08 | 40.44 | 39.31 | 43.43 | 236.32 | 75.43 | 87.96 | 674 | 20/20 |
| HPlan-P | 111.63 | 40.44 | 29.19 | 40.17 | 160.97 | 82.76 | 107.41 | 620 | 14/20 |
| MIPS-XXL | — | — | — | — | — | — | — | — | 0/20 |

| storage | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 | track cov. |
|---|---|---|---|---|---|---|---|---|---|
| ferroplan (0.16) | **0** | **1** | **2** | **5** | **47** | **90** | 200 | 261 | 8/8 run |
| SGPlan5 | 8 | 13 | 26 | 39 | 104 | 160 | 183 | 251 | 20/20 |
| HPlan-P | 0 | 1 | 17 | 36 | 78 | 149 | 240 | 337 | 14/20 |
| MIPS-XXL | 0 | 1 | 10 | 44 | — | — | — | — | 4/20 |
| MIPS-BDD | 0 | 1 | 2 | 15 | — | — | — | — | 4/20 |

| tpp | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 | track cov. |
|---|---|---|---|---|---|---|---|---|---|
| ferroplan (0.16) | **13** | **10** | **26** | **29** | **23** | **41** | 57 | **93** | 8/8 run |
| SGPlan5 | 13 | 12 | 32 | 32 | 27 | 64 | 49 | 126 | 20/20 |
| HPlan-P | 13 | 10 | 27 | 31 | 53 | 59 | 86 | 142 | 20/20 |
| MIPS-XXL | — | 33 | 52 | 73 | 199 | 229 | 273 | 317 | 9/20 |
| MIPS-BDD | 13 | 10 | 33 | 67 | 156 | 186 | 216 | 246 | 9/20 |

| trucks | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 | track cov. |
|---|---|---|---|---|---|---|---|---|---|
| ferroplan (0.16) | **0** | **1** | **0** | 2 | **0** | 4 | — | — | 6/8 run |
| SGPlan5 | 0 | 2 | 0 | 0 | 0 | 3 | 3 | 7 | 20/20 |
| HPlan-P | 0 | 1 | 5 | — | 13 | — | — | — | 4/20 |
| MIPS-XXL | 0 | — | — | — | — | — | — | — | 1/20 |
| MIPS-BDD | 1 | — | — | — | — | — | — | — | 1/20 |

## W/T/L against SGPlan5, the winner, p01–p08

Re-measured 2026-07-25 against the current engine. (The first graft,
run against the stale 0.8-era ledger, read 12W/3T/23L — a verdict
against a planner seven cycles dead, superseded now.)

| domain | W | T | L | note |
|---|---|---|---|---|
| openstacks | 4 | 0 | 4 | dead even with the winner |
| rovers | 7 | 0 | 1 | ahead of the winner (p05 the lone loss, 238.7 vs 236.3) |
| storage | 6 | 0 | 2 | ahead of the winner (p07/p08 the tail losses) |
| tpp | 6 | 1 | 1 | ahead of the winner (p07 the lone loss, 57 vs 49) |
| trucks | 1 | 3 | 2 | plus p07/p08 ferroplan no-run (coverage gap) |
| **total** | **24** | **4** | **10** | + 2 no-runs |

Read the numbers straight: **ferroplan, on today's defaults, beats the
IPC-5 qualitative-preferences winner 24–10 across the 38 comparable
instances, takes three domains outright (rovers, storage, tpp),
splits openstacks, and trails only on trucks** — and outruns
HPlan-P/MIPS-XXL/MIPS-BDD broadly on coverage and quality besides. A
correction goes on the record: the first graft, scored against a
stale ledger, blamed a tpp rout on an all-forgo plateau (the stale
row lined up exactly with MIPS-BDD's) — that diagnosis was true of
the 0.7/0.8-era engine, but the machinery that retired it (the 0.5.1
barrier default and the 0.6 selection layer, matured through 0.10's
DNF static resolution) had already shipped by then. The board simply
sat un-remeasured. What's left is small and named: tpp p07 (57 vs
49), trucks p04/p06 quality (2 vs 0, 4 vs 3), and the trucks p07/p08
600 s no-runs (⁶ below).

Two facts hold the numbers up even without a reference row:

- **reported equals verified, exactly, on every oracle-checked plan.** The
  independent verifier replays the plan over the original problem, folds
  every constraint preference's semantics over the trajectory (never the
  compiled monitors), grounds every inner quantifier, and recomputes the
  metric cold. `tests/ipc5_qual_metric.rs` asserts reported == verified on
  all five p01s in CI's heavy tier (value-independent, so engine
  improvements keep it green; the p01 regression ceilings in the same
  file stay re-locked to the current metrics), and the 0.7/0.8 spot
  checks via `examples/verify_plan.rs` (storage p03/p05/p07/p08,
  openstacks p05, tpp p08, trucks p05, rovers p08) all came back exact
  against the plans of their day.
- **Metrics agree at every thread count wherever both complete** (t1 ≡ t8 on
  all 34 instances with both runs inside budget — of the 36 with a metric,
  only storage p06 and trucks p06 lack a completed t1 run; the largest
  instances just need a longer wall budget at 1 thread — budget-bound,
  never divergent).

## ferroplan, p01–p08 — metric, wall seconds, 4-core box, pure defaults

Re-measured 2026-07-25 (the 0.16 standings cycle) on the 0.15.0
binary — defaults only, 600 s cap per instance, one sweep. **The
previous ledger ran the 0.7/0.8-era engine and had gone badly stale
on three domains**: seven cycles of engine work (the 0.10
precondition-DNF static resolution is the likely prime mover on the
`imply`/`exists`-heavy preference compilations, richer-h and the
optimizer maturation working behind it — per-cycle attribution not
reconstructed, this is a standings ledger, not a bisect) had quietly
dragged rovers 86.65→68.04 (p01) … 888→556 (p08), storage p02–p04
10/60/78→**1/2/5**, and tpp — the board's recorded rout —
24/42/60/78/156/186/216/246 → **13/10/26/29/23/41/57/93**. The
0.8-era numbers still live in this file's git history, buried but
not gone.

| domain | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 |
|---|---|---|---|---|---|---|---|---|
| openstacks | 66 | 68.6 | 77.8 | 89.2 | 122.5 | 121 | 283 | 617.7 |
| rovers | 68.04 | 32.67 | 29.19 | 26.06 | 238.66 | 37.39 | 37.64 | 556 |
| storage | 0 | 1 | 2 | 5 | 47 | 90 | 200 | 261 |
| tpp | 13 | 10 | 26 | 29 | 23 | 41 | 57 | 93 |
| trucks | 0 | 1 | 0 | 2 | 0 | 4 | —⁶ | —⁶ |
| *secs* | | | | | | | | |
| openstacks | 5.8 | 5.8 | 68 | 62 | 43 | 245 | 313 | 277 |
| rovers | 29 | 23 | 37 | 38 | 125 | 52 | 65 | 130 |
| storage | 0.0 | 0.1 | 19 | 24 | 126 | 178 | 322 | 328 |
| tpp | 0.1 | 10 | 17 | 19 | 48 | 57 | 68 | 90 |
| trucks | 0.1 | 19 | 15 | 42 | 61 | 422 | — | — |

⁶ trucks p07/p08: no metric yet — the 600 s cap runs out first. The
trucks tail was already the hardest simple-preferences draw (0.6
Phase-4 record: shared-timeline scheduling sitting outside
selection's reach); the qualitative variants pile `sometime-before`
ordering constraints on top of that. The 0.7 Phase-4 gate (temporal
selection, carried through to 0.9) is the recorded lever still to
pull.

## Coverage

**38 of 40 instances produce a plan and a metric on pure defaults**
(since 0.8): 33 inside 300 s at 8 threads, +2 (openstacks p07/p08)
inside 600 s, +1 (trucks p06) inside 600 s, +2 (storage p07/p08,
first covered in 0.8) inside 600 s on the 0.8 measurement box. Every
remaining gap carries a name: trucks p07/p08 blow through the 600 s
search budget. All 40 parse, gate, and compile clean — no rejections.
(The 0.7 ledger read 36/40, storage p05/p06 living under a documented
`FF_NO_ESPC=1` env and p07/p08 still uncovered — both walls came down
in 0.8; see the findings below.)

## The two scaling failures this suite forced (recorded 0.7; retired 0.8)

1. **Quadratic forall-preferences drowned grounding in its own weight.**
   Storage's crate²×storearea² always-preference (`forall (?c1 ?c2 - crate
   ?s1 ?s2 - storearea) (always (imply ...))`, named `p6A` in p03 and `p8A`
   in p05) blows out to thousands of instances, each one a monitor with a
   `When` transition riding every action; p03 and up killed a 15 GB
   container outright. Fixed as a default in 0.7: constraint-side static
   simplification (`constraints.rs`, `simplify_static`) drops
   statically-accepted instances before compilation ever starts — p05
   sheds 10,693 of 11,136 — the same `peval_static` move that made the
   simple-preferences storage instances tractable back in 0.5.
   `FF_PREF_NO_STATIC=1` restores the blind expansion, for anyone who
   wants to watch it die again.
2. **Wide-monitor states broke two memory budgets on the storage tail —
   both retired in 0.8** (`docs/roadmap-0.8.md` Phases 2–3). As recorded
   in 0.7, the survivors of the static drop each stacked facts onto every
   packed state and a `When` transition onto every action, producing two
   distinct exit-137s inside a 15 GB container:
   - **p05/p06 (443+ survivors): the ESPC monolithic pass.** One penalized
     tightening-B&B pass ran out of memory before its deterministic eval
     budget ever bit. Root cause, found in 0.8: ESPC's deadline-pair
     detection was pairing monitor artifacts (every action conditionally
     drops monitor bits into the priced preferences' collect
     preconditions), waking the pass on tasks with no real once-only
     achievement structure underneath. Since 0.8 the shared monitor block
     goes unscanned for deliverables — these tasks now take the closure
     optimizer on pure defaults, p05: 47, p06: 90, matching the old
     `FF_NO_ESPC=1` metrics exactly — and `FF_ESPC_TRAJ_PAIRS=1` restores
     the old pairing for anyone chasing the ghost. A deterministic search
     node cap (8 GiB byte model, `FF_SEARCH_NODE_CAP`) now backstops any
     wide-state pass regardless.
   - **p07/p08 (1,147+ survivors): grounding itself, the floor giving way.**
     The monitor × ground-action product ran out of memory before search
     ever got a turn. Retired in 0.8 by the shared monitor block: the
     transition block is byte-identical across every ground op, so it's
     ground once and shared (`Domain.monitors` plus per-op bits;
     `FF_NO_COND_SHARE=1` restores per-op copies) — p07 grounds in 313 ms
     at 109 MB peak, p08 in 676 ms at 174 MB, and both throw first-ever
     metrics (200 / 261, reported == verified exact).

## Provenance

- Binary: p01–p08 columns come from the 0.7 Phase-2-head sweep (release,
  frozen); the 0.8 additions (storage p05–p08 defaults confirmation and
  the first p07/p08 rows) come from the 0.8 Phase-3 head on a 4-core /
  15 GB box — metrics identical wherever both measured it, walls not
  comparable across the boxes.
- Runs: the 0.7 sweep ran one per (instance, thread count) ∈ {1, 8} at
  300 s defaults; every timeout or failure row got re-run sequentially on
  an idle box at 600 s (storage p05–p08 then under the documented env,
  since 0.8 on pure defaults). Container wall clock is advisory only —
  the metrics, not the times, are the locked quantity; the heavy locks
  live in `tests/ipc5_qual_metric.rs`.
- Instances: potassco mirror `ipc-2006/domains/<d>-preferences-qualitative/`
  (`instances/instance-N.pddl` → `pNN.pddl`), see
  `benchmarks/ATTRIBUTION.md`.
