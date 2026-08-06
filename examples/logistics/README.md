# logistics — a transshipment domain (per-location goods)

The truck → depot → train shape rpg-world **can't** express — its stockpile stays
global. Here goods sit **per-location**: packages wait at locations, vehicles
(capacity-bound trucks, a train line) **move** between connected locations carrying
them, load/unload happens at depots. Goal: packages land at their destinations.
Routing, capacity, transshipment — a distinct class of sub-problem.

![logistics in the ferroplan-bevy visualizer — location circles joined by gray road edges and a blue rail edge, with a package box and truck/train mobiles](../../book/src/images/logistics-graph.png)

## Problems & the border

| problem | what | result |
|---|---|---|
| `p1` | single hop, 1 box | ✅ 4 |
| `p2` | 3-hop corridor, 1 box | ✅ 10 |
| `border-probe-4c` | 1 truck cap 3, **1** box A→C | ✅ 7 |
| `border-probe-train1` | train only, 1 box railA→railB | ✅ 6 |
| `border-probe-4b` | 1 truck cap 3, **2** boxes A→C | ❌ |
| `border-probe-handoff` | **2-truck** relay via a mid depot, 1 box | ❌ |
| `p3` | truck→train→truck transshipment, 1 box | ❌ |
| `p4` | capacity batching, 1 truck cap 3, 3 boxes | ❌ |
| `p5`–`p8` | multi-package / star-hub / scaling | ❌ |

**Border: multiplicity 1.** A single package on a single vehicle solves over any
number of hops. Add **a 2nd package**, **a 2nd vehicle**, or a **transshipment
hand-off**, and it fails — per-location delivery is a converging-flow problem
(≥2 contributions landing on one "delivered" goal), the exact
[`BORDERS.md`](../BORDERS.md) *converging-contributions ≥ 2* failure. Deep travel
still runs free, matching rpg-world. For a subproblem-maker: **one package, one
vehicle, one leg per contract** — split at every transshipment point.

```sh
ff -o examples/logistics/domain.pddl -f examples/logistics/p1.pddl
```
