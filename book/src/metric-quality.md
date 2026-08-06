# Metric quality & invariants

Two fronts of the IPC-5 / SGPlan-class work: a satisfaction-guided metric optimizer
running the preference (soft-goal) track, and Helmert-style mutex-group synthesis
feeding the partition-and-resolve mode.

## IPC-5 / metric quality

Six domains, all cleared: ferroplan runs the full IPC-5 (2006) *simple-preferences*
soft-goal set — openstacks, tpp, storage, trucks, rovers, pathways. The `:metric`
charges for every violated preference (rovers also tacks on a numeric traverse
cost), and **lower is better**. Preferences get compiled away (Keyder & Geffner);
an exact-closure optimizer with budget-escalating branch-and-bound drives the
metric from there (see [PDDL3 preferences](./pddl3.md)).

rovers held out longest. Its metric also charges a **monotone numeric quantity**
(`sum-traverse-cost`) — the optimizer used to ignore it outright, scoring a bogus
`0`. Fold monotone numeric terms into total-cost and the *full* metric comes into
view: a real **935.3**. See [Performance](./performance.md).

The trap: delete-relaxation hides what forgoing a soft goal actually costs. The
free Keyder–Geffner forgo makes every preference look reachable from anywhere, so
on `openstacks-soft` the metric search had nothing pulling it toward delivering —
it just sat on the **all-forgo floor** (metric 70 on p01). Two engine changes
closed most of that gap.

1. **Satisfaction-guided ordering** (`search::SatGuidance`) — a heap penalty
   counting preferences forgone in the *concrete* state, giving the search a
   reason to deliver instead of coast. It cracked the floor (70 → 63 on
   openstacks p01) with zero regressions — it only reorders nodes, and
   branch-and-bound keeps the best plan found regardless.
2. **The exact-closure metric optimizer** (0.4.0, now the default) — real-state
   search, metric-bounded acceptance, an exact `collect`/`forgo` phase tail,
   static preference simplification at compile, barrier-free full-DNF guidance,
   budget-escalating branch-and-bound. Pushed openstacks p01 further (63 → 42),
   lifted whole domains: storage to full 8/8 coverage, trucks p08 from 133 down
   to 10.

One piece stayed stuck: openstacks needed *scheduling* of the shared
`stacks-avail` resource, invisible to the satisfaction term since it shows up in
no preference. The **ESPC penalty loop** (default-on since 0.5; `FF_NO_ESPC`
opts out) closes it — its λ schedule drives a partitioned composition that prices
`stacks-avail` as a global constraint, and openstacks moves **ahead of SGPlan5 on
p04–p08**.

**Standing against SGPlan5, the IPC-5 winner:** full 48/48 coverage, a
domain-level lead on two of six (openstacks with `FF_ESPC`; storage on defaults),
parity on the small instances almost everywhere else — a strong 2nd under the
coverage-first rule. Per-instance tables, the ESPC method, reproduction commands:
[`benchmarks/ipc5-scoreboard.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc5-scoreboard.md).

## Mutex groups & SAS+

For the SGPlan-style [partition mode](./architecture.md), ferroplan synthesizes
**mutex-group "guidance variables"** — Helmert-style monotonicity invariants that
recover a task's SAS+ multi-valued variables: the "where is X / what is held"
facts that can never hold at once.

A cheap single-predicate pass catches only variables whose values are all the
same predicate — the clean position variables (`at-robby`, `lift-at`,
`pointing`) — and comes back with **nothing** on blocks or logistics. The real
work is the **multi-predicate** refinement: when an action unbalances a
candidate — adds into the variable through one predicate, deletes through
another — the candidate absorbs the deleted-*and-required* fact and gets
re-verified to a fixpoint (`crates/ferroplan/src/invariants.rs`). That recovers
exactly what the partitioner needs:

| domain | single-pred | multi-pred | biggest group |
|---|---|---|---|
| blocks | 0%, 0 grp | **100%, 9 grp** | block support `{on, ontable, holding}` |
| logistics | 0%, 0 grp | **93%, 9 grp** | object location `{at, in}` |
| gripper | 7%, 1 grp | **71%, 7 grp** | gripper hand `{free, carry}` |

These groups feed SGPlan-style partitioning: seed the initial partition from a
goal-interaction graph over the mutex variables, and on conflict the resolver
merges the actual conflicting pair. The payoff — **blocks plans shorten ~25%**
where goals share structure but aren't resource-coupled. Resource-coupled domains
are a different fight: naive decomposition keeps re-traversing the shared
resource, which is exactly what the default-on ESPC penalty loop (`FF_NO_ESPC`
opts out) now prices as a global constraint instead. Method, coverage numbers,
findings:
[`docs/invariants-measurement.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/docs/invariants-measurement.md).
