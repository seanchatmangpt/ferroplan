# jobshop — a scheduling domain (machine-exclusion)

Each **job** runs a fixed sequence of operations (s1→s2→…). Each operation claims a
**machine** that handles **one op at a time** — machine-exclusion modeled with a
`(free ?m)` token, consumed at-start, restored at-end. This is the
resource-exclusion pattern rpg-world deliberately leaves out. Durations run
per-(job,stage). Goal: every job complete. The planner overlaps different jobs on
whatever machines sit free.

Regenerate the ladder with `../../benchmarks/scale/gen_jobshop.py`.

![jobshop in the ferroplan-bevy visualizer — stage nodes joined by amber routing edges, machines as octagons, and jobs as boxes](../../book/src/images/jobshop-graph.png)

## The ladder & the border

| problem | size (jobs×stages×machines ≈ groundings) | result |
|---|---|---|
| `p1`–`p5` | 1×2×2 … 5×5×5 | ✅ (makespan 6–19) |
| `s10` | 10×10×10 ≈ 1k | ✅ 40 |
| `s20` | 20×10×10 ≈ 2k | ✅ 84 |
| `s50` | 50×10×10 ≈ 5k | ✅ 177 (1.2s) |
| `s50w` | 50×20×20 ≈ 20k | ✅ 210 (6s) |
| `s100` | 100×20×20 ≈ 40k | ✅ **382 (45s)** |
| `s100g` | 100×30×30 ≈ 90k | ❌ grounding wall |

**Jobshop scales hugely — that's the finding.** 100 jobs, 20 stages, 20 machines,
full machine-exclusion, and it schedules to makespan 382 in 45s. Reason: jobs run
as **independent linear chains that never converge** — squarely the engine's
strong suit (see the *unifying law* in [`BORDERS.md`](../BORDERS.md)). None of
rpg-world's heuristic killers show up here. The **only** ceiling is grounding-table
size (~40k operate instances clears, ~90k hits a wall). For a subproblem-maker: a
whole job-shop is safe under ~40k tuples; past that, **partition by jobs** — never
by machine or stage, those are what tie the schedule together.

```sh
ff -o examples/jobshop/domain.pddl -f examples/jobshop/s100.pddl   # 100 jobs, 45s
```
