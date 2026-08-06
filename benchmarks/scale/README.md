# Scale & complexity — the stress test

Three axes, pushed hard, the way a live game would push them, until the engine
tells you where the problem has to shrink. Generators:

- `gen.py N M [E]` — an `rpg-world` problem with **N locations** (a connected map:
  chain + E extra edges each, with `dist` + the `reachable` axiom), **M agents**,
  every resource fluent initialized, a 1-action goal so **grounding dominates**.
- `gen_domain.py K` — a domain with **K procedural craft action schemas** (+ a
  trivial matching problem), to stress raw domain breadth.

```sh
python3 benchmarks/scale/gen.py 200 4 > /tmp/p.pddl
ff -o examples/rpg-world/domain.pddl -f /tmp/p.pddl
python3 benchmarks/scale/gen_domain.py 1000 > /tmp/d.pddl
```

## Results (M4, contended; grounding runs the show)

**Domain complexity — action-schema count.** Costs nothing. Flat, linear, forgettable.

| K schemas | 100 | 300 | 1000 | 3000 |
|---|---|---|---|---|
| parse+ground+solve | 0.03s | 0.03s | 0.04s | 0.06s |

**Agents — per-agent action grounding** (fixed 60-location map). Linear. Gentle slope, no surprises.

| M agents | 2 | 5 | 10 | 20 | 40 |
|---|---|---|---|---|---|
| wall | 0.25s | 0.36s | 0.56s | 1.10s | 2.40s |

**Static content — map size + the `reachable` transitive-closure axiom.** Here's the wall.

| N locations | 20 | 50 | 100 | 150 | 200 | 300 | 500 |
|---|---|---|---|---|---|---|---|
| wall | 0.09s | 0.16s | 0.76s | 2.4s | 5.5s | **22s** | **97s** |

Growth curves at ~**O(N³·⁵)** — the reachability closure eats the clock at grounding
time (`crate::derived::compile`, a naïve datalog fixpoint that re-walks every
binding each round, its `exists` scanning every node in the graph).

## Verdict — where the cut goes

The engine shrugs off **arbitrarily wide domains** (3000 actions, near-zero cost)
and **crowds of agents** (linear, no complaint). One wall stands: the
**reachability axiom on large maps** (≈300+ POIs). Reductions, best first:

1. **Stop deriving reachability on huge maps — precompute it instead.** The game
   already owns the navigation graph; hand the planner `(reachable a b)` (or just
   the edges it needs) as init facts, not a `:derived` rule. Closure cost drops to zero.
2. **Plan per region.** A contract works a *local* sub-map, tens of POIs, never
   the whole world — the same decomposition that keeps crafting chains short.
3. **Engine work still on the table:** make the derived closure *semi-naïve*
   (derive only from last round's new facts), join-aware (index `link` by source
   so `exists` scans neighbours, not the whole graph). That pulls the closure from
   ~O(N⁴) toward **O(N·E)** and pushes the map ceiling from hundreds of POIs into
   the thousands.

Everything else — domain breadth, resource variety, agent count — never becomes
a bottleneck. Grow the domain as large as you like; shrink *map scope per plan*.
