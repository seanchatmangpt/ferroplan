# IPC-5 (2006) simple-preferences scoreboard — ferroplan against the field

Vendored suite: `benchmarks/ipc/pref/{openstacks,pathways,rovers,storage,tpp,trucks}`
— the IPC-5 *simple-preferences* (soft-goal) domains. Each problem's
`(:metric minimize …)` counts violated preferences (rovers folds in a numeric
`sum-traverse-cost` term); **lower is better**.

One run: `ff -o pref/<domain>/domain.pddl -f pref/<domain>/pNN.pddl` (the PDDL3
metric optimizer reports back `metric value N, K preferences`).

## Reference and scoring — verified from the official archive

The IPC-5 field for this subtrack ran **SGPlan5** (the winner, first in all 6
SP domains, 6/0), **MIPS-XXL**, **MIPS-BDD**, and **YochanPS**. Their
per-instance metrics come straight off the `; MetricValue` headers in the
official `IPC5-results.tgz`; instance `pNN` is the same physical problem
across every planner (our p01–p08 are the competition's p01–p08, unchanged).

IPC-5 ranked **per domain by place** (IPC-4 style): **coverage first, then plan
quality, then CPU** — not the IPC-2008 normalized ratio. SGPlan5 holds full
coverage in every domain, which makes it the natural quality benchmark below.
(MIPS-XXL's openstacks headers carry a known `0.00` reporting artifact —
coverage only; MIPS-BDD is optimal but runs at very low coverage.)

## ferroplan against SGPlan5, p01–p08 — lower is better, **bold** marks ferroplan ≤ SGPlan5

**openstacks** — satisfaction guidance cracked the all-forgo floor (70→63),
the exact-closure optimizer² pushed the default further still (63→42 on
p01), and the opt-in ESPC penalty loop (`FF_ESPC`, see
`docs/espc-preferences-spec.md`) chains its λ schedule to a **partitioned
search** ("increment 2": one subproblem per order-interaction component,
the shared `stacks-avail` variable priced as a global constraint instead of
solved inside any one stage) — **ferroplan beats SGPlan5 on p04–p08**, the
first domain where it leads the IPC-5 winner across the larger half of the
suite:

| inst | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 |
|---|---|---|---|---|---|---|---|---|
| ferroplan¹ (default) | 19 | 23 | 17 | **16** | **21** | **22** | **66** | **87** |
| `FF_NO_ESPC=1`² | 23 | 24 | 29 | 39 | 66 | 65 | 126 | 370 |
| SGPlan5 | 13 | 16 | 12 | 26 | 36 | 33 | 67 | 123 |

¹ **The default since 0.5** (graduated: `features::espc()` wakes wherever
deadline pairs exist — a verified no-op across the other five domains — and
the outer budget runs a deterministic eval pool, `FF_ESPC_EVAL_BUDGET`
default 6M, standing in for the wall clock; `FF_ESPC_TIME_MS` still hangs
around as an optional additional cap). The graduated default row reproduces
the old opt-in row exactly — 19/23/17/16/21/22/66/87, t1≡t4, worst case p04
at roughly 63 s wall, p01 in about 3 s. `FF_NO_ESPC=1` restores the
closure-only path; `FF_ESPC_MONO=1` brings back the earlier monolithic loop.

² Default path since 2026-07: the **exact-closure metric optimizer** (static
preference simplification at compile time, real-state search with
metric-bounded acceptance, the exact `P3END`/collect/forgo phase tail,
barrier-free DNF guidance), riding a **budget-escalating retry**: a
tightening probe that hits its 300k per-iteration eval cap without
improvement doesn't quit — it retries the same bound with whatever budget is
left, `FF_PREF_EVAL_BUDGET` (default 2M evals, deterministic, thread-count
independent) the real contract underneath. Second pass, 2026-07: **anytime
in-sweep tightening** (each sweep tightens its bound in place on every
acceptance instead of restarting per improvement — a restart now costs once
per cap, not once per improvement; `FF_PREF_GREEDY=1` restores
first-improvement sweeps) and a **diversified restart ladder** — a capped
no-improvement sweep is the current h-ordering saying it can't reach a
better plan (measured: same-direction retries just re-tread the same prefix
and change nothing), so the loop rotates the open-list weights through a
fixed half-cap profile ladder (h-greedy → h-heavy → g-heavy → pure-h) under
the same bound before the final all-remaining escalation
(`FF_PREF_NO_RESTARTS=1` turns the ladder off). Fully deterministic. Most
instances land inside 65 s wall at 4 cores; the trucks tail runs slowest
(p07 ~104 s, p08 ~154 s) because the escalated retries actually spend the
budget they're given. `FF_PREF_COMPILED=1` / `FF_PREF_NO_STATIC=1` /
`FF_PREF_BARRIER=1` / `FF_PREF_NO_ESCALATE=1` restore the pre-2026-07
pieces. Since 0.5 the folded numeric metrics (rovers) route through the
closure optimizer too (`FF_PREF_NUMLEGACY=1` restores the pre-0.5 legacy
split — see the rovers section).

**tpp** — the exact-closure optimizer² **ties SGPlan5 on p01–p04** (the whole
field ties SGPlan there); the restart ladder shaved the tail (97/116/131 →
93/104/117), but SGPlan5 holds it:

| inst | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 |
|---|---|---|---|---|---|---|---|---|
| ferroplan² | **16** | **24** | **29** | **35** | 80 | **101** | 103 | 129 |
| SGPlan5 | 16 | 24 | 29 | 35 | 79 | 101 | 100 | 105 |

(0.6: the selection layer — `docs/forensics-tpp.md` traced SGPlan5's p05 79
back to the closed-form end-state selection optimum; `selection.rs` now
solves that selection exactly and plans straight to it as a target. p05
89 → 80, the solver's bound reproducing the 79 optimum — the residual +1
is one `p-drive` application, a per-action preference living outside
end-state selection — p06 104 → **101, an exact tie**, p07 110 → 103.
Domain totals 517 vs 489.)

**storage** — full coverage now (was 2/8: the quadratic forall-preference
compiled to 1601–62191 instances and walled the search shut). Static
simplification strips the statically-satisfied ~90–97%, the exact-closure
optimizer² searches real states only, and the restart ladder² broke the
large-instance plateau (46/145/200/263 → 31/121/124/148) — **ferroplan now
beats SGPlan5 on p01–p07** (7 of 8) **and on the domain total** (447 vs
547):

| inst | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 |
|---|---|---|---|---|---|---|---|---|
| ferroplan² | **3** | **5** | **6** | **9** | **25** | **43** | **60** | **83** |
| SGPlan5 | 5 | 8 | 14 | 17 | 87 | 124 | 160 | 132 |

(0.5.1: keeping init-satisfied preferences inside the guidance — see
`docs/forensics-tpp.md` — took p05–p08 from 31/121/124/148 down to
25/43/60/83: **a full 8/8 domain sweep**, totals 234 vs 547.)

**trucks** — the closure optimizer² lifted the whole row (p08: 133 → 10, p07:
67 → 12) and the ladder² finished off p03 (1 → 0) and p06 (6 → 1); ferroplan
**wins p01 and p07**, ties p02–p05, and sits **ahead on the domain total**
(23 vs 31):

| inst | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 |
|---|---|---|---|---|---|---|---|---|
| ferroplan² | **0** | **0** | **0** | **0** | **0** | 1 | **12** | 10 |
| SGPlan5 | 1 | 0 | 0 | 0 | 0 | 0 | 24 | 6 |

**rovers** (MetricSimplePreferences — numeric metric via numeric-term folding)
— **a full domain lead since 0.5**: folded metrics now route through the
exact-closure optimizer (the 0.4.0 "closure churn" verdict turns out to have
been an artifact of first-improvement restarts, which the anytime sweeps
swept away; `FF_PREF_NUMLEGACY=1` restores the legacy split). Ferroplan
**wins p04/p06/p07/p08, ties p01/p05 exactly**, and leads the totals 5301.6
vs 5632.5:

| inst | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 |
|---|---|---|---|---|---|---|---|---|
| ferroplan² | **811.3** | 502.2 | 847.4 | **418.7** | **483.6** | **655.7** | **402.2** | **740.9** |
| SGPlan5 | 811.3 | 473.2 | 811.3 | 485.4 | 483.6 | 656.7 | 403.4 | 1007.6 |

(0.6: the selection layer generalizes to the numeric metric — it picks which
samples and images are worth their traverse cost as the target — p02 596.7 →
502.2, p03 935.3 → 847.4, p08 998.1 → **740.9**; the totals lead widens to
4862.0 vs 5632.5.)

**pathways** — **ties SGPlan5 on p01–p04** (was p01 alone) and the ladder²
**wins p05 outright** (8.5 → 6 against SGPlan's 6.5); SGPlan5 pulls ahead
after that:

| inst | p01 | p02 | p03 | p04 | p05 | p06 | p07 | p08 |
|---|---|---|---|---|---|---|---|---|
| ferroplan² | **2** | **3** | **3** | **2** | **6.5** | 11 | 12.5 | 20.2 |
| SGPlan5 | 2 | 3 | 3 | 2 | 6.5 | 10 | 8 | 12.9 |

(0.5.1: the guidance-barrier default cost p05's outright win — 6 → 6.5, now
an exact tie — and bought p06 12.9 → 11 in return; the trade is on the record
in `docs/forensics-tpp.md`.)

## Verdict — 0.5, everything below is the default configuration

Quality reads two ways — per-instance wins or domain totals — and the two
conventions used to disagree. As of 0.5 they agree on three domains. On
p01–p08, full 48/48 coverage everywhere, one configuration, no env vars,
deterministic at any thread count:

- **ferroplan leads SGPlan5 under both conventions on three of the six
  domains**: **openstacks** (wins p04–p08; totals 271 vs 326 — the ESPC loop
  doing the work is the default now, deterministically budgeted),
  **storage** (**an 8/8 domain sweep** since 0.5.1; totals 234 vs 547), and
  **rovers** (wins p04/p06/p07/p08, exact ties p01/p05; totals 5301.6 vs
  5632.5).
- **trucks splits the conventions**: ferroplan leads the totals (23 vs 31)
  while the instances draw (wins p01/p07, ties p02–p05, loses p06 by 1 and
  p08 by 4).
- **tpp** (ties p01–p04 and p06 since 0.6's selection layer; p05/p07/p08
  trail by 1/3/24, totals 517 vs 489) and **pathways** (ties p01–p05, tails
  off after) still belong to the IPC-5 winner — barely, on tpp.
- Instance tally across the 48: **19 wins / 16 ties / 13 losses** (0.4.0
  read 14/11/23). SGPlan5's original 6/0 domain sweep now reads **2/3/1** by
  instances-and-totals combined, its remaining edge carried entirely by
  tpp's p05/p07/p08 (gaps of 1/3/24) and the pathways tail.

Under the IPC-5 **coverage-first** rule this is an honest "**closing on
first**": three domains led under either reading of quality, a fourth on
totals — clear of MIPS-XXL, MIPS-BDD, and YochanPS everywhere, and no longer
behind SGPlan5 in aggregate instance count. What still separates second from
first comes down to two domain tails (tpp and pathways p05–p08), both
direction-bound (measured: identical metrics at 4× the eval budget) and both
resistant to the restart ladder and to composition-as-seeding alike — the
open research item for 0.6.

## Path to climb

1. ~~**openstacks resource loop**~~ — **done** ("increment 2", 2026-07): the
   ESPC λ schedule now drives a partitioned composition (one stage per order
   component, `stacks-avail` cut out of the edges and priced as a global
   constraint instead, per-stage goals enriched with their own deliverables),
   taking p01–p08 from 42/43/55/66/81/90/151/227 to 19/23/17/16/21/22/66/87 at
   the same budget — ahead of SGPlan5 on p04–p08. What's left: p01–p03 (small
   instances, 19/23/17 vs 13/16/12), where the per-order grain runs too
   coarse to matter and the polish B&B is the binding mechanism.
2. ~~**tpp/storage quality**~~ — **done** (2026-07): the exact-closure metric
   optimizer (real-state search, metric-bounded acceptance, exact phase
   tail) with barrier-free DNF guidance ties SGPlan5 on tpp p01–p03 and
   pathways p01–p04, beats it on storage p01–p05 and trucks p01/p07.
3. ~~**B&B scalability**~~ — **done** (2026-07): static preference
   simplification (statically-satisfied instances dropped at compile) plus
   the closure optimizer's instant init-tail incumbent hand storage full
   8/8 coverage (62k raw instances on p08) with every instance under 60 s.
4. ~~**Large-instance tails**~~ — **largely closed** (2026-07, second pass):
   the anytime-plus-ladder combination. Measured in two halves: **anytime
   in-sweep tightening alone changed nothing** (identical metrics, fewer
   iterations) — the plateau was never restart churn, it was a guidance
   limit. At the plateau bound the h-ordering runs out of budget without
   ever reaching a better plan, no matter how much contiguous budget it's
   handed. The **diversified restart ladder** is what actually broke it
   (same bound, rotated open-list weights, half-size rungs so the final
   full-budget escalation still lands hard — full-size rungs starved it and
   gave back tpp p04 / trucks p07): storage p05–p08 46/145/200/263 →
   31/121/124/148 (p06/p07 flip to wins), pathways p05 8.5 → 6 (a win), tpp
   p05–p07 −4/−12/−14, trucks p03 1→0 / p06 6→1, openstacks default p01
   42→23. It cost something: tpp p08 +1, openstacks p03 +1, rovers p02
   +56.8 (all instances that were losing already). What's left (tpp/pathways
   p05–p08) plateaus under every profile in the ladder; the next lever is
   partitioned closure search — ESPC-style composition, run on the closure
   path.
5. ~~**rovers numeric metric**~~ — **closed** (0.5): the domain flipped to a
   full lead on the third lever tried. The record, in order: (a) cost-aware
   open-list ordering (`SearchCfg::w_c`) — a dead end, collapses quality at
   every weight (cost only grows along a path, never shrinks); (b)
   forgo-aware seeding (`FF_PREF_SEED=1`, prices each preference's
   completion via `heuristic::relaxed_plan_cost`) — neutral, the EHC seed
   already lands at the same incumbent (the machinery stays, default off);
   (c) **numeric closure routing** — the 0.4.0 verdict that folded metrics
   measure worse on the closure path turns out to have been an artifact of
   first-improvement restart churn, swept away by the anytime sweeps.
   Routing folded metrics through the closure optimizer (default since 0.5;
   `FF_PREF_NUMLEGACY=1` restores the split) ties SGPlan5 exactly on p01/p05
   and beats it on p04/p06/p07/p08 — 4 wins, 2 ties, 2 losses, and the
   totals lead besides. The completion is priced by the closure acceptance
   test itself (`cost-so-far + closure < bound` sums the real traverse cost
   with the exact forgo weight) — the thing items (a) and (b) were groping
   toward the whole time.

> Reproduce: `for p in p01..p08; do ff -o pref/<domain>/domain.pddl -f pref/<domain>/$p.pddl; done`
