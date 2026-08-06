# Village builder — full-feature stress test

A survival/village-builder, run to put load on ferroplan's whole **feature
surface** at once: durative actions, numeric fluents (graph distances +
material counts), the full **ADL** set. Everything a rich game domain would
throw at it, mixed in on purpose:

| feature | where in `domain.pddl` |
|---|---|
| Durative actions | every action (`travel`, `chop-wood`, `build-house`, …) |
| Numeric fluents | `(dist ?a ?b)` durations, `(wood)/(stone)/(sticks)` stockpiles, `(chops-left ?l)` |
| Disjunctive precondition | `travel`: `(or (road …) (trail …))` |
| Conditional effect (`when`) | `chop-wood`: skilled chopper gets `+1 wood` |
| Universal effect (`forall`+`when`) | `make-fire`: warms every agent at camp |
| Quantified precondition (`forall`) | `build-square`: needs **all** house slots built |
| Negation | `build-house`: `(not (built ?s))` |

## What this validated ✅

Parsing, grounding, solving — all three hold under the **combination** of ADL +
durative + numeric. `onesite.pddl` (a single-site village: gather, craft, build
the square, light the fire) clears end to end:

```sh
ff -o examples/village/domain.pddl -f examples/village/onesite.pddl   # solves
```

## What it surfaced (the honest limits)

1. ~~**No axioms / derived predicates.**~~ **Resolved.** When this example was
   first written, `:derived` came back rejected. Not anymore — ferroplan now
   carries static/stratified derived predicates, closed into the initial state
   via a datalog fixpoint, so `reachable`- / `village-complete`-style
   consequences of static state are expressible. See
   [`examples/reachability/`](../reachability) for the worked derived-axiom
   example. (Dynamic derived predicates — bodies over facts actions
   change — still off the table.)

2. **Temporal search is the bottleneck on a *graph map*.** `graph.pddl` (same
   goal across a 3-node forest/quarry/camp map, so the agent has to
   **travel**) does not clear within the node budget:

   ```sh
   ff -o examples/village/domain.pddl -f examples/village/graph.pddl   # exceeds the temporal search
   ```

   The cause is the temporal relaxed heuristic, not the features: delete-relaxation
   runs blind to agent *location*, weak on *numeric accumulation* — on a rich,
   many-action domain it hands back little guidance for the realistic "travel →
   gather → return → build" loop, and the decision-epoch search runs itself dry.
   A stripped 3-action version (travel + gather + fire) on the same map solves
   clean — it's the *combination* of richness + transport + accumulation that
   needs a stronger temporal heuristic (and the helpful-action pruning that
   depends on it).

The key result for using ferroplan as a game's planner: the **classical**
engine holds strong, durative + resources work, but **temporal search strength
on rich transport/crafting domains is the next engine investment** — the RPG
blocker.
