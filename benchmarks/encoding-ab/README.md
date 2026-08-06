# Encoding A/B/C — does generic PDDL outrun action-specific?

Two ways to model the same crafting world. This benchmark settles it with
controlled measurement: **which encoding lets ferroplan search faster** — and
where the lines cross.

- **action-specific** — one `:action` per recipe (`craft_0`, `craft_1`, …). The
  "chop / mine / smelt" style: every verb gets its own schema. (cf. the durative
  `examples/rpg`, `examples/village`, `examples/rpg-world`.)
- **generic / data-driven** — the per-verb variation moves into data, one action
  covers every recipe. The "use-item over constants" style (cf. the original
  `Prohibited` domain's `consume` action over `(consumable ?type ?verb)`). Two flavors:
  - **data-table** — `(:action craft ?rec ?in ?out)` gated by a static `(recipe ?rec
    ?in ?out)` table; recipes/resources ride as `:constants`. Ground actions stay thin
    (1 input → 1 output).
  - **forall-numeric** — `(:action craft ?rec)` quantifying over *all* resources
    via per-recipe `(need ?rec ?res)` / `(make ?rec ?res)` quantity functions. One
    fat ADL operator; handles multi-input recipes natively.

A fair test needs one trick: all three encodings model the **same content** in
**the same planner mode**, so encoding is the only variable left standing.
(The domains already living in the repo differ in content *and* mode — a
confounded comparison; `asis.py` runs that one separately, as a raw data point.)

## Layout

| file | what |
|---|---|
| `gen.py` | emits the matched domains+problems for any `{encoding}×{mode}×{content}` |
| `run_experiment.py` | solves every corpus, writes `metrics/*.json` + `RESULTS.md` |
| `asis.py` | Part A: the *confounded* as-is comparison of the existing domains |
| `selfcheck.py` | runs the new `ff --validate` over real durative domains (validator demo + regression guard) |
| `proto/` | tiny hand-verified prototypes of all 6 domains (the "run it before you trust it" set) |
| `asis/prohibited-consume.pddl` | a working instance of the existing `Prohibited` domain (the shipped problem is stale) |
| `corpora/` | generated (gitignored); rebuilt by `gen.py emit-corpora` |
| `metrics/` | generated metric JSONs (perf.py schema; `perf.py compare`-able) |
| `RESULTS.md` | generated tables + the head-to-head verdict |

## Content models

- **chain** — `r0 → r1 → … → rK` (single-input recipes). Knobs: `K` chain length,
  `N` goal quantity. Linear, accumulative — the FF heuristic keeps a clean gradient all the way down.
- **converge** — a balanced binary assembly tree of depth `D` (two-input recipes); the
  goal item wants ≥2 sub-assemblies, each wanting its own sub-chain. Here the
  delete-relaxed heuristic "goes flat" (≥2 contributions converging on one goal
  quantity — see `../../examples/BORDERS.md`), and that's where the encodings pull apart.
- **techtree** — a realistic RPG crafting tech-tree (17 recipes, 26 resources): multi-input
  recipes with quantities (`house = 2 frame + 1 cutstone + 1 window`, `frame = 2 plank`, …),
  shared intermediates (plank feeds frame/tool/cart/sword), a couple of distractor
  recipes sitting off the goal path. Knob: `N` settlements. This is the case that needs
  the data-table's **arity-family** (`craft1/craft2/craft3`) and shows where its grounding
  cost bites against forall-numeric's one-action generality.

## Metrics

Pulled straight from `ff … --json --threads 1`:

- **`evaluated_states`** — node expansions, the headline number for search
  efficiency. Populated only on the **classical FF path** (instantaneous actions);
  the temporal path reports zero, so durative runs get compared on **makespan** instead.
- **plan length** — solution quality, the equivalence check (every encoding must
  return the same length on the same problem, since they model identical content).
- **coverage** — solved inside the node cap / timeout, nothing more.
- **grounded_actions / grounded_facts** — the data-table's grounding cost laid bare
  (its `craft` schema enumerates `K·R²` (chain) or `R³` (converge) candidate
  groundings, then prunes down to the `K` carrying a true `(recipe …)` fact).
- **wall-clock ms** — machine-dependent. Profiling color only, never the verdict.

`--threads 1` keeps `evaluated_states` and timings honest and deterministic.

## Reproduce it yourself

```sh
cd ferroplan
cargo build --release -p ferroplan-cli        # -> target/release/ff

# the controlled experiment (writes metrics/*.json and RESULTS.md)
python3 benchmarks/encoding-ab/run_experiment.py --contents chain converge

# the confounded as-is comparison (Part A)
python3 benchmarks/encoding-ab/asis.py

# the realistic tech-tree (slower; N>=2 hits the monolithic-search border), then
# reassemble the full RESULTS.md from all the saved metrics (no re-solving)
python3 benchmarks/encoding-ab/run_experiment.py --contents techtree --max-evaluated 300000 --timeout 150
python3 benchmarks/encoding-ab/run_experiment.py --report-only --contents chain converge techtree

# validate a plan under ferroplan's OWN semantics (auto-detects classical vs temporal)
ff -o domain.pddl -f problem.pddl --mode temporal > plan.txt
ff -o domain.pddl -f problem.pddl --validate plan.txt          # -> Plan valid / Plan invalid: ...
python3 benchmarks/encoding-ab/selfcheck.py                    # validator over real durative domains

# one-offs: emit a single domain/problem to stdout
python3 benchmarks/encoding-ab/gen.py domain  --encoding forall --mode inst --content converge --depth 3
python3 benchmarks/encoding-ab/gen.py problem --encoding forall --mode inst --content converge --depth 3 --qty 2

# pairwise diff any two metric files with the existing harness
python3 benchmarks/perf.py compare benchmarks/encoding-ab/metrics/converge-specific-inst.json \
                                   benchmarks/encoding-ab/metrics/converge-forall-inst.json
```

## Findings

Full tables live in `RESULTS.md` (committed run: `ff @ dbb9bb9`, threads=1,
cap=2M nodes, 45s timeout). Plan length matches across all three encodings on
every solved instance — they model the same content, no asterisk, so the
comparison holds.

**1. The data-table generic encoding costs nothing in search — it ties the
hand-written action-specific encoding exactly.** Identical `evaluated_states`
on *every* instance, chain and converge alike, identical coverage, identical
plan length:

| | chain (inst) total_eval | converge (inst) total_eval |
|---|---|---|
| specific | 8523 | 51465 |
| data-table | 8523 (+0.0%) | 51465 (+0.0%) |
| forall | 8523 (+0.0%) | 51668 (**+16.5% geomean**) |

So "generic vs action-specific" is **not** a search tradeoff, *provided you reach
for the data-table style*: folding N action schemas into one `craft ?rec ?in ?out`
plus a `(recipe …)` data table costs nothing in node expansions. Its only toll is
**grounding** — the `craft` schema enumerates `K·R²` (chain) / `R³` (converge)
candidate groundings before the static `(recipe …)` precondition prunes them down
to `K`. At these scales that's single-digit-to-low milliseconds (geomean_ms +69%
chain / +184% converge — still just milliseconds), but it compounds under temporal
mode: `data-table-temporal` on converge runs 11.7 s at d3·n2 and times out across
d4, where `specific-temporal` costs far less.

**2. The forall-numeric encoding is the one that builds "a very different, harder
search domain"** — the intuition holds, but only in two specific places:
- **A search penalty that grows with convergence.** Linear chains, it ties the
  others exactly (the heuristic gradient never changes); the convergent tree,
  it expands more states: +33% (d2·n2), **+65% (d2·n4)**, +19% (d3·n2), +25% (d3·n4).
  The delete-relaxed heuristic "goes flat" where ≥2 contributions converge
  (`BORDERS.md`); the fat `forall`-over-resources operator perturbs the relaxed
  plan harder than thin operators do.
- **Per-state CPU weight.** Even where node counts tie (chains), evaluating a
  `forall`-over-all-resources operator runs ~15–20× slower in wall-clock: chain
  k32·n4 = 611 ms against 30 ms (specific) / 40 ms (data-table). Temporal coverage
  pays for it (chain 10/12 vs 11/12).

**2b. On the realistic tech-tree (multi-input + quantities), the same picture holds
— and the data-table's feared grounding blow-up never shows up.** All three solve
N=1 with *identical* `evaluated_states` (187 261) and plan length (25); wall-clock
runs **specific 17 s · data-table 14 s · forall 136 s**. The data-table
**arity-family** (`craft1/craft2/craft3`) keeps pace with hand-written specific
even on three-input recipes — ferroplan's grounder constrains the `R^arity`
candidate enumeration through the static `(recipeN …)` facts instead of
materializing it — while forall runs ~8× slower per node. N≥2 defeats all three
monolithically (the realistic shared-intermediate tree hits the `BORDERS.md`
border — exactly why the production system decomposes a build into contracts
instead of solving it whole).

**3. Temporal makespan is a wash, noisy at best.** ferroplan's temporal search
is satisficing (first plan found, not makespan-optimal), so makespans don't compare
cleanly across encodings; `forall` even lands *shorter* makespans on converge
where it solves (d3·n1 = 6 vs 14), its atomic fat craft exposing more parallelism.
The signal that holds up is **coverage**, and it falls for all three past d3 —
temporal search runs out of road on big monolithic instances.

**4. As-is (Part A, confounded) — `metrics/asis.txt`.** The generic `Prohibited`
domain solves instantaneously (ev=6, len=5 via `claim → pick-up → consume×3`); the
action-specific durative `rpg`/`village`/`rpg-world` solve with makespans 7–25.
Different content *and* mode — not a fair comparison, just a data point on the wall.

### Verdict

- **Want generic *and* fast?** Reach for **data-table**. The maintainability win
  (one schema plus a data table, instead of one schema per recipe) costs **zero**
  search penalty against hand-written specific actions — the price is a small,
  bounded grounding tax and nothing more.
- **Skip forall-numeric if satisficing speed is the goal.** Most expressive of the
  three (native multi-input recipes and quantities), but it's the encoding that
  genuinely hardens the search once the recipe graph branches, and it runs
  CPU-heavy per node. Reach for it only when its expressiveness or its tighter
  temporal makespans are the point.
- **Action-specific** stays the search baseline; data-table matches it stride for
  stride. The real divergence lives in forall-numeric against convergent content,
  and in grounding cost against scale and temporal mode.

## Built-in validator (`ff --validate`)

External VAL was cut for textbook PDDL2.1 and picks a fight with ferroplan over
numeric-durative plans — it flags two durative actions touching `(workers)` at once
as a mutex, demands ε-separation to the letter — and fails perfectly good ferroplan
plans for it, even the shipped `examples/rpg` one. So ferroplan carries its **own**
validator now: `ff -o D -f P --validate plan.txt` replays a plan under the engine's
*own* `apply`/`op_applicable`/`goal_met` semantics (reusing `verify::verify` for
classical, `temporal::validate` for durative; library entry
`ferroplan::plan::validate_plan`). It reads the format for itself, classical
(`step N: …`) or temporal (`t: (…) [d]`), and prints `Plan valid` / `Plan invalid: <reason>`.

Two wins landed immediately (`selfcheck.py`, unit tests in `crates/ferroplan/src/plan.rs`):

1. **It accepts the resource-parallel plans VAL wrongly rejects.** The `rpg`
   3-worker plan, `build-house` ε-corrected to t=8.002, validates clean here while
   VAL still fails it on its concurrent-`(workers)` mutex — the original motive, proven out.
2. **It caught a real ferroplan bug on its first pass.** `selfcheck.py` flags 2 of
   5 real durative plans (`rpg/3workers` at t=8.001, `rpg-world/woodlot` at t=10.001):
   the temporal printer **under-separates ε** at a same-timestamp produce-at-end /
   consume-at-start boundary — a saw produces planks at its `at end`, the next
   action draws them at its `at start`, same printed tick. Nudge the consumer by
   +0.001 and the plan validates. The fix belongs in the temporal ε-separation /
   printer (`temporal.rs` `epsilon_separate`); logged here, not yet applied.
   *Resolved (verified 2026-07-22, 0.14 extension Phase 7):* fixed by the 0.10
   numeric-footprint work and the total-ε-ordering rewrite; `selfcheck.py` now
   reports **5 valid, 0 flagged**.

### Notes / caveats

- **VAL is advisory only for the durative domains** (see above): it rejects
  ferroplan's resource-parallel temporal plans on a strict concurrent-numeric mutex,
  so the durative comparison leans on ferroplan's own coverage/length/makespan
  (deterministic) and the built-in validator, not VAL.
- **The shipped `Prohibited` problem (`planner/bin/simple_problem.pddl`) has gone
  stale** — its goal simplifies to FALSE. `asis/prohibited-consume.pddl` stands in
  as a minimal working instance of the same, unmodified domain.
