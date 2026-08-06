# The modern planning landscape, 2026 — the 0.17 frontier memo

Field dispatch, filed against the 0.17 Phase 1 deliverable
(`docs/roadmap-0.17.md`). I went back to where the competitions
ferroplan cut its teeth on (IPC-5/6/7, 2006–2011) left off, tracked
what the winners are actually running now, and came back with a
RANKED list of engine gaps — each one tagged with a mechanism
sketch, the evidence that it wins, and an honest in-THIS-engine
cost paragraph. Phase 3 takes its shot at the top of this list;
Phase 2's baseline sweeps stand as referee.

## The field, competition by competition

**IPC 2014.** 66 track-variant directories, pulled in cold via
potassco/pddl-instances — a sequential-MULTI-CORE track among them,
so ferroplan's mco entry runs backward-compatible too. Satisficing
went to the portfolio planners (IBaCoP family), clean past the
LAMA-2011 baseline. The lesson written on the wall that year was
PORTFOLIOS — ferroplan already carries that weapon
(`FF_PORTFOLIO`, budget-aware since 0.9).

**IPC 2018.** 12 satisficing domains sitting local: agricola,
caldera, data-network, flashfill, nurikabe, organic-synthesis,
settlers, snake, spider, termes, ... Fast Downward Stone Soup took
satisficing; **BFWS(pref) took agile, and BFWS variants ran up
close behind in satisficing.** The idea underneath the breakout:
**width/novelty-based search** (Lipovetzky & Geffner). Novelty of a
state — the size of the smallest atom tuple showing up for the
first time along the search (w=1: one single atom is new; w=2: a
pair is). BFWS orders its open list by ⟨novelty, unachieved-goals⟩,
heuristics riding along only as tie-breaks; novelty gets computed
RELATIVE to a partition (goal count, relevance count), which keeps
the tables lean and the signal sharp. The polynomial variants
(k-BFWS) prune w>k outright and still clear a startling fraction of
the corpus — exploration structure doing the work here, not
heuristic accuracy.

**IPC 2023 classical.** 7 new domains local, official per-instance
reference PLANS riding alongside and a `bounds.json` of best-known
costs: folding, labyrinth, quantum-layout, recharging-robots,
ricochet-robots, rubiks-cube, slitherlink. Satisficing AND agile
both fell to **Scorpion Maidu** (Scorpion plus width search "with
forgetting" — novelty tables wiped clean on a cycle) and
**Levitron** (Scorpion Maidu paired with **PowerLifted**, a LIFTED
planner, run in portfolio); DALAI (disjunctive action landmarks)
took a track of its own. The organizers' own retrospective names
the field's deepest wound as PDDL FEATURE SUPPORT — quantifiers,
disjunctions, `imply`, negative goal conditions — and that happens
to be a ferroplan STRENGTH (the 0.10 DNF-static fix; full ADL
standing on the 2008 openstacks board). Two of the three winning
ingredients are moves ferroplan doesn't have in its hand yet
(novelty, lifted search); the third — strong classical
heuristics/portfolios — it's already carrying.

**IPC 2023 numeric.** 20 domains local, official sat/opt result
CSVs attached: counters, farmland, sailing, drone, expedition,
hydropower, markettrader, settlersnumeric, sugar, zenotravel, fo-*
linear variants, ... Swept clean by **NLM-CutPlan** (Kuroiwa,
Shleyfman, Beck) — numeric LM-cut, landmark-cut stretched to cover
simple numeric conditions/effects (linear expressions;
constant-delta effects) — running past Numeric Fast Downward, with
an **Orbit** variant (symmetry orbit-space search, the same lineage
as ferroplan's 0.14 orbits, validated now from the optimal side).
The satisficing baseline of record is ENHSP (interval-based
relaxation / subgoaling heuristics, Scala et al.); Kuroiwa's lazy
greedy BFS with subgoaling relaxation is its satisficing-side
sibling. The "simple numeric" class covers most of the RPG's
resource math — quantities and money moved by constant or recipe
amounts — which means this track's heuristics are the village's
heuristics too.

## The ranked gap list

1. **Novelty/width signal in the classical ladder** (BFWS-class).
   Mechanism: per-state novelty against seen-atom tables
   partitioned by (unachieved-goal count[, relevance count]); order
   or prune by it; reset-on-restart ("forgetting") keeps tables
   honest across rungs. Evidence: agile winner 2018, inside both
   2023 winners; the single most proven post-LAMA satisficing idea.
   In-engine cost: MODERATE — a novelty table beside the visited
   set (facts are already dense bit-indices; a w=1 table is one
   bitset per partition cell, w=2 capped or skipped), a new rung in
   the classical ladder (the portfolio/ladder plumbing exists), no
   changes to h^FF. The going-in favorite, confirmed.
2. **Numeric heuristic upgrade** (subgoaling/AIBR class, NLM-cut's
   satisficing siblings). Evidence: the entire 2023 numeric podium;
   ferroplan's own audit (metric-time-2006 55/200; model-train-t
   0/30 last-mile-numeric wall — the SAME shape the 0.15 probe
   named). In-engine cost: SIGNIFICANT — a second numeric
   relaxation beside the FF-extension h (interval propagation per
   fluent; subgoal decomposition of comparisons), engine-visible on
   both corpora and the village. The 2023 numeric baseline sweep
   prices the distance first.
3. **Lifted / lazily-grounded search** (PowerLifted-class).
   Evidence: Levitron's winning half; the entire big-object
   problem class the village lives in. In-engine cost: LARGE
   (successor generation over schemas via joins instead of ground
   op tables) — priced by Phase 1's big-catalog stress test before
   any commitment; the cheap intermediate (lazy/on-demand
   grounding within the current architecture) may capture most of
   the village's need.
4. **Deferred evaluation + open-list alternation** (LAMA/FD
   machinery). Evidence: two decades of FD satisficing. In-engine
   cost: SMALL-MODERATE — evaluate-on-pop instead of on-generate
   under a flag; alternation between h^FF and novelty queues pairs
   naturally with bet #1. A supporting bet, not a headline.
5. **Dynamic derived predicates** (axioms). Evidence: several
   modern domains lean on them; ferroplan grounds static/stratified
   only. In-engine cost: MODERATE and long-deferred; goes in only
   if the Phase 2 sweeps show concrete coverage priced against it
   (the failure classifier will say).

## Assets now local (fetch scripted in `benchmarks/get-ipc.sh`)

- IPC 2014 (66 variants, potassco mirror) — includes seq-agile,
  seq-sat, seq-mco.
- IPC 2018 sat (12 domains, official bitbucket) + `cost_bounds.json`.
- IPC 2023 classical agl+opt (7 domains) + official reference
  plans per instance + `bounds.json`.
- IPC 2023 numeric (20 domains) + official sat/opt result CSVs.
- (Vendored earlier: the official IPC-5 results archive.)

The receipts are already in the drawer for the 2018/2023 classical
and 2023 numeric sweeps — no coverage-only asterisks on the new
standings tables, except where WE fail to record the currency (the
makespan runner debt, still open).

## The big-catalog stress test (the village priced before it is built)

`benchmarks/bench/gen_catalog.py`: ONE gather rule, ONE make rule,
the whole catalog laid down as static init data (the game's exact
contract), recipes forming a layered binary DAG, goal sitting at
the top item. `make` runs syntactically N³; a grounder that
resolves the static needs1/needs2 joins grounds ~N ops.

**Monotone variant (pure grounding pressure):**

| N items | wall | peak RSS |
|---|---|---|
| 100 | 0.36 s | 10 MB |
| 1,000 | 0.35 s | 10 MB |
| 3,000 | 3.3 s | 15 MB |
| 10,000 | 37 s | 42 MB |

Static resolution holds the line — the curve reads ~quadratic, not
cubic, and a 10,000-kind catalog grounds and solves clean at 37 s,
42 MB. A REALISTIC village (hundreds of item kinds) is through the
door in well under a second. **Verdict on gap #3: lifted search is
NOT the village's blocker** — demoted to a watch item, the
quadratic term earning one profile look somewhere down the road.

**Consume variant (inputs deleted — the game's real semantics):**

| N items | wall | note |
|---|---|---|
| 30 | 0.00 s | len 111 |
| 100 | 1.5 s | len **895** — wandering re-gathering |
| 300 | TIMEOUT 60 s | the wall |
| 1,000 | TIMEOUT 120 s | — |

**Correction, filed the same day.** The first read pinned this on
h^FF's consumption-blindness — the fixture was rigged. Its recipe
DAG grew DEEPER with N, and under consumption every make re-makes
its whole input subtree: the MINIMAL plan is exponential in depth
(the 895 steps at N=100 were largely forced marching, not
wandering) — that measures plan length, not search quality. Cap the
depth at the shape real crafting actually has (4 layers, width
scaling — `--depth`, now the generator default) and the picture
flips:

| N items (consume, depth 4) | wall | plan |
|---|---|---|
| 300 | 0.03 s | 15 |
| 1,000 | 0.33 s | 15 |
| 3,000 | 2.65 s | 15 |

**The village profile — wide consumable catalogs at honest
depth — is comfortable for the current engine out to N=3000 and
beyond.** Neither grounding nor consumption blocks the village at
realistic scale; both scares dissolved the moment fixture
discipline got applied. The novelty rung's case rests where the
literature already put it: the 2018/2023 corpus baselines are its
referee, not the village. (The rung itself was built and behaves —
the A/B on the depth-4 fixture reads exactly neutral, which is what
it should read on instances the earlier rungs already own.)
