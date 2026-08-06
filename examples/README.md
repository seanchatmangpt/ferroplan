# ferroplan examples

Worked PDDL domains and problems. Each one a self-contained dossier on a single
part of the engine. Every directory carries its own README: the command to run
it, the result to expect. Run any of them with the `ff` CLI:

```sh
ff -o examples/<dir>/domain.pddl -f examples/<dir>/problem.pddl
```

(Some temporal examples run only with feature flags set — each README names them.
See the [tuning reference](../book/src/tuning.md).)

## By feature — what to read to learn X

| you want to learn… | start here |
|---|---|
| STRIPS + typing, plans & goals | [`logistics`](logistics) — trucks, capacity, a train line |
| numeric fluents (a quantity gates actions) | [`cabin`](cabin) — a deep numeric build |
| **derived axioms** (`:derived`) | [`reachability`](reachability) — static transitive-closure reachability |
| full ADL (`when`, `forall`, `or`, negation) | [`village`](village) — the ADL stress test |
| **PDDL3 preferences / metric** optimization | [`villagers`](villagers) — a data-driven recipe planner scored by a metric |
| durative / temporal + resources | [`rpg`](rpg) — gather → craft → build with workers & materials |
| scheduling with machine exclusion | [`jobshop`](jobshop) — scales to 100 concurrent jobs |
| everything at once + **decomposition** | [`rpg-world`](rpg-world) — a ~120-action economy, solved as contracts |

## Suggested reading order

1. **[`rpg`](rpg)** — start here. Durative actions running against renewable
   (workers) and consumable (materials) resources. Everything downstream builds on
   this model.
2. **[`logistics`](logistics)** / **[`cabin`](cabin)** — the two classical
   foundations, STRIPS/typing and numeric fluents. `cabin` carries a durative "crew"
   twin clocking makespan against crew size.
3. **[`reachability`](reachability)** — a fact standing as *consequence* of static
   state, closed into init through a `:derived` axiom.
4. **[`village`](village)** — the full ADL surface (`when`, `forall`+`when`, `or`,
   negation) run against durative + numeric state, with a clear-eyed account of
   where the temporal heuristic gives out.
5. **[`villagers`](villagers)** — PDDL3 metric optimization (`--mode pddl3`), the
   planner embedded in a game loop.
6. **[`jobshop`](jobshop)** — durative scheduling under a machine-exclusion token.
7. **[`rpg-world`](rpg-world)** — the flagship. Everything above running together,
   plus the decomposition workflow (`ff --decompose`) that cuts an oversized goal
   into ordered contracts. See its [`suite/`](rpg-world/suite),
   [`hard/`](rpg-world/hard), and [`industrial-city/`](rpg-world/industrial-city).

## See also

- **[`BORDERS.md`](BORDERS.md)** — the measured map: where one-shot planning holds,
  where a goal has to break down into contracts.
- **Rust library examples** — [`crates/ferroplan/examples/`](../crates/ferroplan/examples):
  `solve`, `parse`, `json_api`, `replan` (the `Session` API), `decompose`, and
  `validate_plan`.
- **Minimal per-feature snippets** — the [ferroplan skill](../.claude/skills/ferroplan)
  carries one small domain+problem per feature, under its own `examples/`.
