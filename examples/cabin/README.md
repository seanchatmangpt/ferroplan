# cabin — raise a log cabin from standing trees

One goal (`cabin-finished`), one long ordered sequence of work — a deliberately
**deep, linear crafting chain**. Chop, process, build, in order, no shortcuts.

```
fell-tree ─┬─ saw-planks ──┐
           ├─ hew-beam ─────┤
           └─ split-shingles┤
mine-ore ── smelt-ingot ── forge-nails ┤   ← everything feeds the build…
dig-sand ── fire-glass ────────────────┤
quarry-stone ──────────────────────────┤
                                        ▼
  lay-foundation → raise-walls → frame-roof → lay-floor
      → build-door → hang-door → set-window-frames → glaze-windows → finish-cabin
```

The build stages are a **strict linear chain** — each waits on the last — so the
plan is a forced march the planner just has to follow. The full cabin runs
**~52 steps**: fell a dozen-odd trees, mill them to planks/beams/shingles, forge
nails from ore, fire window glass from sand, quarry stone, then raise it.

## Why classical (not durative)?

Modeled as a **numeric classical** domain — instantaneous actions, each one
ticking its cost onto `(total-time)` — not `:durative-actions`. ferroplan's
metric/FF search carries a ~50-step numeric plan without strain; the temporal
decision-epoch search **can't** — it runs dry around ~20 steps on a chain this
deep, tuned instead for shorter, more-concurrent durative work like
[`../rpg-world`](../rpg-world). The second lesson buried in this example: match
the encoding to the solver — long sequential numeric builds go classical,
concurrent durative work goes temporal.

## Run it

```sh
# the shell — foundation, walls, roof (~26 steps, instant)
ff -o examples/cabin/domain.pddl -f examples/cabin/raise-frame.pddl

# the whole cabin — ~52 steps end to end (a few seconds)
ff -o examples/cabin/domain.pddl -f examples/cabin/raise-cabin.pddl
```

In the web demo, "The whole log cabin" is flagged slow — run it in **Web Worker**
mode, keeps the page responsive while it grinds through (~7s).

## Parallel crew — `crew.pddl` (makespan drops with more workers)

`crew.pddl` is the **durative** twin. Same job, but now actions take time, and the
planner's **scheduling phase** packs them onto a crew of workers, one job per
worker at a time. Independent work — chopping, mining, digging, firing glass —
starts to overlap, and **more workers finish sooner**. Same 34-step job, different
makespan:

```sh
ff -o examples/cabin/crew.pddl -f examples/cabin/crew-solo.pddl --mode temporal   # 1 worker  -> makespan 109
ff -o examples/cabin/crew.pddl -f examples/cabin/crew-pair.pddl --mode temporal   # 2 workers -> makespan 63
ff -o examples/cabin/crew.pddl -f examples/cabin/crew-trio.pddl --mode temporal   # 3 workers -> makespan 47
```

This needs the concurrent scheduler, which is gated: set `FF_TDEMAND=1 FF_TCONC=1`
(or, in the web demo, the example carries flags `tdemand,tconc`). Why a separate
phase? "One job per worker at a time" is a convention the DOMAIN does not state —
it is lockless — so nothing but the scheduler enforces it. Without the flags you
get what the PDDL says, not what a crew can do: through 0.27 that was the search's
own sequential layout (109 / 152 / 198, the serial sum, *growing* with crew size);
since 0.28 the default route left-shifts independent work, and on a lockless
domain that overlaps everything — makespan 47 for ANY crew, one worker on four
jobs at once. Legal PDDL, and not a crew schedule. With `FF_TCONC=1` the
left-shift stands aside and the numbers above are unchanged. The scheduler (`crate::tsched`) searches a single-actor reduction for
*what* to do, then repacks it across the crew for *who does what, when* — validated,
and only kept if it's genuinely shorter. The crew domain is **lockless** (workers
interchangeable) so the search stays small and the scheduler owns the parallelism.

## Skilled crew — `crew-skilled.pddl` (tasks need the right specialist)

Workers aren't always interchangeable. `crew-skilled.pddl` gates tasks on
**skills**: only a `(sawyer ?w)` mills (saw/hew/split), only a `(smith ?w)` smelts
and forges. The scheduler reads a task's actor-referencing precondition as its
required skill and routes the task **only to a worker who holds it** — location
works the same way — and `validate()` confirms the routing held.

```sh
# specialists: 1 sawyer (ana), 1 smith (ben), 1 labourer (cal)
ff -o examples/cabin/crew-skilled.pddl -f examples/cabin/skilled-specialists.pddl --mode temporal
#   -> every SAW-PLANKS is ANA, every SMELT/FORGE is BEN
# cross-trained: all three have both skills -> the skilled work spreads
ff -o examples/cabin/crew-skilled.pddl -f examples/cabin/skilled-crosstrained.pddl --mode temporal
```

(Run with `FF_TDEMAND=1 FF_TCONC=1`.) The single-actor search reduction turns into
a *super-worker* holding the union of all skills, so the plan still gets found; the
scheduler then hands each task off to a real worker carrying the needed skill. Ask
for a skill no worker has, and the problem is correctly unsolvable.

### Skill scarcity bites — `forge-*.pddl` (forge 80 nails)

Put the skilled work on the critical path, and a missing specialist shows up
straight in the makespan. These forge a keg of nails — smelt and forge are
smith-only, mining is labour:

| crew | makespan |
|---|---|
| `forge-1smith` — 1 smith, 3 workers | **65** |
| `forge-1smith-crowd` — 1 smith, **5** workers | 62 |
| `forge-2smith` — **2 smiths**, 3 workers | **44** |
| `forge-3smith` — 3 smiths, 3 workers | 38 |

Two extra *labourers* barely move the number (65 → 62) — the lone smith is the
cap. A second *smith*, same crew size, shaves off a third (65 → 44). A third
smith buys less: ore supply and the smelt→forge dependency start to bind. Skill
scarcity, not headcount.

## Files
- `domain.pddl` — the cabin domain (harvest + mill + smith + glass + masonry + the
  9-stage linear build), classical/numeric, solo.
- `raise-frame.pddl` — goal `roof-on`: the weather-tight shell.
- `raise-cabin.pddl` — goal `cabin-finished`: the complete cabin, door and windows.
- `crew.pddl` — the durative twin; `crew-{solo,pair,trio}.pddl` — 1/2/3-worker crews
  for the makespan comparison (run with `FF_TDEMAND=1 FF_TCONC=1`).
- `crew-skilled.pddl` + `skilled-{specialists,crosstrained}.pddl` — skill-gated tasks
  (sawyer/smith) showing the scheduler route work to the right specialist.
