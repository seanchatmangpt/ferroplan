# Scenarios — problem spaces that stress different mechanics

Beyond the per-subsystem `contracts/`, each of these instances leans on a
*different* pressure point of the engine, same `rpg-world` domain underneath.
All clear fast.

| scenario | stresses | makespan |
|---|---|---|
| `bootstrap-a-workshop` | self-built workstations + a build-then-use dependency: no sawmill exists, so the plan must `build-sawmill` (from pre-cut components) **before** it can `saw-planks` | 8 |
| `logistics-run` | durative **travel** + the derived **reachability axiom** + multi-site accumulation: the quarry is two hops from camp (via a junction), so the agent chops at the forest and mines at the quarry | 25 |
| `mana-cycle` | a **renewable resource consume/regenerate loop**: brewing costs mana, meditation restores it, so three potions force `meditate`→`brew` interleaving | 11 |
| `guild-order` | a **multi-part (conjunctive) goal** — ingots + blocks + meals in one order — the temporal search's known weak spot; solves because each part is short | 6 |

```sh
ff -o examples/rpg-world/domain.pddl -f examples/rpg-world/scenarios/mana-cycle.pddl
```

## A modelling note surfaced by `guild-order`

Roles (`smith`, `mason-skill`, `cook`, …) are **yield bonuses, not gates**. The
domain carries **no agent-exclusion** — a durative action never reserves its
agent — so the planner is content to run a whole order through one craftsman
instead of splitting the work. Want NPCs one-task-at-a-time, division of labor
forced? Add a per-agent `(free ?a)` token: consume it `at start`, restore it
`at end` on every action. One change, and the model turns agent-exclusive —
`guild-order` becomes a real parallel-crew problem.
