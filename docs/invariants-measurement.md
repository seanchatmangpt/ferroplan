# Mutex-group synthesis — coverage measurement (increment 1)

No commit before the read. Point the instrument at the SAS+ slice
(`crates/ferroplan/src/invariants.rs`) and find out cold whether the cheap
**single-predicate** monotonicity-invariant synthesis is strong enough to feed
SGPlan/ESPC subgoal partitioning (Plan B) — or whether it's a dead end.

Tool: `cargo run -p ferroplan --example invariants_coverage -- <domain-dir>...`

## Results (in-repo + parent IPC benchmarks)

| domain | groups | coverage | biggest group |
|---|---|---|---|
| blocks | **0** | 0% | — |
| logistics | **0** | 0% | — |
| gripper | 1 | 2–7% | `(at-robby …)` |
| elevator | 1 | 22–29% | `(lift-at …)` |
| rovers | 1 | 5% | `(at rover0 …)` |
| satellite | 1 | 35% | `(pointing sat0 …)` |
| trucks | 1 | 2–5% | `(time-now …)` |

## Finding

The single-predicate pass surfaces **only** the clean cases — variables where
every value is the same predicate, the "X is at exactly one Y" position
variables (`at-robby`, `lift-at`, `pointing`, truck/rover `at`, `time-now`).
That's it. Coverage tops out at 2–35% (usually under 10%), and **blocks and
logistics come back empty**.

Everything else it can't see is **multi-predicate** — a variable whose values
span several predicates:
- blocks: block support `{on(b,·), ontable(b), holding(b)}`; hand `{handempty, holding(·)}`
- logistics: package location `{at(pkg,·), in(pkg,·)}`
- gripper: ball location `{at(ball,·), carry(ball)}`; gripper `{free(g), carry(·,g)}`

The balance check throws these out — an action adds into the variable through
one predicate, deletes through another (`unload` adds `(at pkg loc)`, deletes
`(in pkg truck)`), and a single-predicate candidate reads as unbalanced. Which
is the sting: these "where is X / what is held" variables are exactly the
guidance variables ESPC partitioning needs.

## Implication for Plan B (ESPC)

Verdict on the cheap slice: **insufficient**. Won't feed ESPC as-is. To make
Plan B viable the synthesis has to be upgraded to **multi-predicate
monotonicity invariants** — the real Helmert refinement. When an action
unbalances a candidate, extend the candidate with the facts that action
deletes (so `{holding(?x)}` grows to `{holding(?x), handempty}`, which
`pickup` balances), then re-verify. That's the documented "weeks" core the
SAS+ investigation flagged.

The single-predicate pass earns its keep as a correct, useful base — it
already nails the position variables. The multi-predicate refinement is the
next increment, if Plan B is a go.

## Update — multi-predicate refinement implemented

`synthesize` now runs Helmert-style branch-and-verify: an action's add comes
back unbalanced, the candidate gets extended with a deleted-**and-required**
fact — the precondition guarantees the removed unit was the true one, which
is what keeps it sound — then re-verifies to a fixpoint. Coverage on the same
instances:

| domain | single-pred | multi-pred | biggest group |
|---|---|---|---|
| blocks | 0%, 0 grp | **100%, 9 grp** | block support `{on,ontable,holding}` |
| logistics | 0%, 0 grp | **93%, 9 grp** | object location `{at,in}` |
| gripper | 7%, 1 grp | **71%, 7 grp** | gripper hand `{free,carry}` |
| rovers | 5% | 20% | `(at rover0 …)` |
| trucks | 5% | 12% | `(time-now …)` |
| elevator / satellite | 29% / 35% | 29% / 35% | (already single-pred) |

The multi-predicate variables — block support, package/object location, the
gripper hand, exactly the guidance variables ESPC partitions on — come back
recovered. **Verdict: Plan B is fed.** Next move: consume these groups in the
SGPlan/ESPC partitioning (`resolve`/`partition`).

## Update — wired into partitioning (ESPC consumer, increment 1)

`synthesize` now feeds `resolve::solve`. The initial partition seeds off a
**goal-interaction graph** over the mutex variables
(`partition::interaction_partition` — goals link when an operator achieves
one's variable while disturbing another's; connected components become the
initial groups), and on a conflict the resolver **merges the actual
conflicting pair** (`merge_at`) instead of grabbing a positional neighbor.
Sound throughout — worst case, the resolver still collapses back to plain
search.

Plan length, partition vs FF (release, in-repo benchmarks):

| instance | ff | partition |
|---|---|---|
| blocks p01/p02/p03 | 14 / 18 / 14 | **10 / 14 / 10** |
| gripper p01/p02/p03 | 17 / 25 / 33 | 19 / 27 / 35 |
| logistics p01/p02/p03 | 24 / 23 / 19 | 32 / 29 / 20 |

**Finding:** valid plans across the board; partitioning **shortens blocks
~25%** where goals share structure without being resource-coupled. On
**resource-coupled** domains — gripper's two grippers, logistics' shared
trucks — naive decomposition re-traverses the shared resource and plans run
longer. Fixing that, and the openstacks metric gap with it, needs **resource
coordination across subproblems: the ESPC penalty loop** (multiplier updates
on shared/global constraints). That's increment 2 — the guidance variables
are wired end-to-end through the pipeline now.
