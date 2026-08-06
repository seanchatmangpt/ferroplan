# ESPC preference optimization — implementation spec (groundwork)

> **Dispatch — ESPC is live, and it's outrunning SGPlan5 on openstacks
> p04–p08** (opt-in `FF_ESPC`), see `crate::espc`, `benchmarks/ipc5-scoreboard.md`,
> and the CHANGELOG. Two increments got it there:
>
> - *Increment 1 (0.3-era):* the soft *occupancy* penalty was dead weight —
>   §"Conclusion" caught it inert and buried it. In its place: the loop hits,
>   on the **concrete** state, once-only conditional achievements that fire
>   *without delivering* (a product built while its orders still sit in the
>   queue), adapting a per-trigger penalty across the outer loop, iteration 0
>   left clean as a penalty-free floor. It narrowed the gap
>   (p01 63→42 … p08 608→227) but never closed on SGPlan's level — the
>   monolithic loop, still reachable today with `FF_ESPC_MONO=1`.
> - *Increment 2 (0.4.0):* the λ schedule stopped nursing one big problem and
>   started running a **partitioned composition** — one subproblem per
>   order-interaction component, the shared `stacks-avail` variable pulled
>   out of partition edges and priced instead as a **global constraint**,
>   never solved inside any single stage. p01–p08 dropped to
>   19/23/17/16/21/22/66/87 — **ahead of SGPlan5 (13/16/12/26/36/33/67/123) on
>   p04–p08**, the exact move this design record called in advance. What's
>   left standing is p01–p03, where the per-order grain is too coarse to bite.
>
> Everything below the line is the original design record, untouched.

How SGPlan5 (Hsu, Wah, Huang & Chen, IPC-2006) wrings good metrics out of
hard PDDL3 **preference** problems — traced back through the primary sources
(IJCAI-2007 #310; AIJ-2006 Wah & Chen; IPC-2006 booklet; ICAPS-06 workshop).
This is the *real* fix for ferroplan's IPC-5 **quality** gap: coverage is
already holding even, but metric quality is bleeding out (openstacks 70 vs
13). Not built yet. The `forbidden`/`plan_avoiding` plumbing in `search.rs`
and `Compiled.forgos` in `pddl3.rs` are the groundwork already down.

## Why our current approach can't close the gap (measured)

- Monolithic anytime B&B: run the eval budget up 10× and openstacks/p01 still
  sits at metric 70 — this is a *search-direction* wound (length-first can't
  reach the longer plans that satisfy more preferences), not a budget one.
- Per-preference "force-collect" (forbid a forgo, re-solve): buys a little on
  few-preference problems (rovers 698→646) but leaves openstacks cold (still
  70) and times out on the many-preference instances. Pulled.
- Root cause, per the research: in openstacks the *subproblems are trivial* —
  the whole fight is **joint global-constraint coordination across
  preferences**, and neither monolithic B&B nor per-preference forcing is
  built to run that fight.

## The SGPlan method

1. **Partition by guidance variables.** Nodes = multi-valued (SAS+/MDF) state
   variables appearing in goal-state constraints. Edges = constraints over them; a
   soft goal/preference is an **edge weight = its violation cost**, never its own
   node. METIS min-cut. #partitions = `min(#guidance-vars, #bottleneck-vars)`;
   grain chosen to minimize shared variables (→ fewer global constraints).
   Constraints spanning partitions become **global**; localized ones are **local**.

2. **Per-subproblem objective.** Subproblem *t* keeps only its stage's local
   constraints hard and folds **all global constraints into the objective** as
   penalty-weighted violations:
   `min_z(t)  J(z) + γᵀ|H(z)| + ηᵀ max(0, G(z))   s.t. local h(t)=0, g(t)≤0`.
   Solved by the modified-FF subplanner whose heuristic minimizes
   `Π(z) + τ·eᵀ + Σ_{k≠t} γ_{t,k}·em_{t,k}` (Π = Metric-FF heuristic for the
   subgoal; em = estimated active mutexes between subplans).

3. **ESPC resolution loop** (CPOPT partition-and-resolve):
   ```
   automated_partition()
   γ ← γ0 ; η ← η0
   repeat (OUTER):
     for t in 1..N:
       solve P_t with CURRENT FIXED γ, η          # modified-FF subplanner
       update_penalty()                            # raise penalties on violated
                                                   #   global constraints, INSIDE the loop
     recompute global metric; keep best plan as incumbent   # anytime
   until no global constraint is violated (an extended saddle point)
   ```

4. **Penalty update (reimplementable).** `γ ← γ + ρᵀ|H(z)|`, `η ← η + ϱᵀ max(0,G)`.
   Rate `ρ` is per-constraint and adapted multiplicatively by a **consecutive-
   violation counter**: when constraint *i* has been violated for *K* consecutive
   subproblem evaluations, increase its rate. Penalty multipliers are **separate
   from preference weights** (weights compute the metric; multipliers drive
   violations to zero).

5. **Preference classes.** Class 1 = final-state / `always` preferences → enforced
   as **local** constraints, solved by enumerating reachable values of each
   involved variable + backtracking on reachability. Class 2 = the rest →
   **relax-and-tighten** (ignore first, then penalize unsatisfied).

## Mapping to our Keyder–Geffner compilation (the actionable plan)

IPC-5 "simple-preferences" metrics answer to the **final state** alone, so per
the research the optimal collect/forgo assignment is computable straight from
final-state constraints + reachability:

1. Treat each `collect_i / forgo_i` decision as a binary guidance variable.
2. Build the interaction graph over the **objects/predicates** the preferences'
   `phi_i` share; partition into loosely-coupled groups (union-find or METIS).
3. Per group, find the max-weight jointly-satisfiable subset of its preferences
   (force those collects, via the existing `plan_avoiding` forbidden-forgo
   mechanism) — small groups keep this tractable where the monolithic problem
   chokes.
4. Resolve cross-group conflicts (satisfying group A's prefs forbids group B's)
   with the penalty loop: penalize the shared/global constraints, re-solve groups,
   iterate to an ESP; keep the best metric as an anytime incumbent.

### Open / ambiguous (flagged by the research)
- Exact `update_penalty` schedule + the consecutive-violation threshold *K*.
- Outer-loop termination beyond "no global violation".
- Grain-size selection is a stated objective, not a precise algorithm.

## Conclusion of the implementation study (decision: deferred)

A 4-design / adversarial-critique / synthesis study ran on top of this spec,
backed by six measured implementation attempts. The finding, no hedging:

**A general ESPC-style preference optimizer does NOT hold up in ferroplan as
it stands.** Every approach that fits the architecture (the Keyder–Geffner
collect/forgo representation + delete-relaxation heuristic) collapses to the
same "force-collect" lever — forbid `forgo_i` so the search must achieve
`phi_i` — and *every variant was built, run, and caught NOT moving the
metric*:

| variant | result |
|---|---|
| cost-aware heuristic + cost-first A*/WA* (×2) | suboptimal + timeouts; `h` blind |
| 10× B&B budget | openstacks unchanged (70) — search *direction*, not budget |
| per-preference greedy force-collect | no gain; timed out many-pref instances |
| all-forgo coverage floor | slow base search; hollow metrics |
| batch force-collect (top-{100/50/25}%) | no gain on pathways/rovers; regressed openstacks coverage via latency |

Two root causes, both architectural:
1. Under delete-relaxation the free `forgo_i` makes every `collected_i` trivially
   reachable, so the heuristic is **blind to which preferences a plan satisfies**.
2. The hardest gap (openstacks, 70 vs 13) is the **minimum-open-stacks scheduling**
   problem; its coupling lives in the `stacks-avail` resource, which appears in no
   preference `phi` and is **invisible to any phi-based partitioning**. A faithful
   ESPC needs SAS+/mutex-group guidance variables over `stacks-avail`.

**Two roads ahead, neither taken yet:**
- *General:* build a SAS+/mutex-group translation layer, then the real partition +
  penalty-resolution loop. Multi-week; the right architecture but a heavy lift.
- *Scoreboard-only:* a bespoke `openstacks` min-open-stacks oracle (detect the
  structure, schedule outside the relaxation-blinded search, inject as a
  fail-closed incumbent). ~3 days; lands around 20, not 13; domain-specific
  code, **not a general planner advance** — a scoreboard patch and nothing
  more.

Decision: **coverage already sits even with SGPlan6 (39/48); what's left
bleeding is metric quality on solved instances.** Neither road pays for
itself against the current milestone, so ESPC waits. The `forbidden`/`plan_avoiding`
plumbing and `Compiled.forgos` stay in the ground as groundwork for the general path.

## Revisit (2026-07) — the general path's blocker has since been built

Two facts have changed since the "deferred" call above, re-verified live:

1. **Root cause 2 no longer holds.** The multi-predicate (Helmert-style)
   monotonicity-invariant synthesis in `crates/ferroplan/src/invariants.rs`
   (see `docs/invariants-measurement.md`) turns up **exactly one mutex group
   on every openstacks instance: `(STACKS-AVAIL n)`** — the precise guidance
   variable this study said a faithful ESPC needs and phi-based partitioning
   was blind to (verified: `cargo run --release -p ferroplan --example
   invariants_coverage -- benchmarks/ipc/pref/openstacks`). The groups are
   already being consumed by classical partitioning
   (`partition::interaction_partition` → `resolve::solve`).

2. **The penalty loop is running** (`crate::espc`, opt-in `FF_ESPC`) but still
   chained to the bespoke make-deadline trigger on the *monolithic* search, and
   its payoff is **budget-bound**: on a 4-core box at the default 15 s budget only
   p01/p02/p06 improve (42/43/100); push `FF_ESPC_TIME_MS=90000` and the loop
   reproduces the recorded quality (e.g. p05 135→81).

So the "multi-week translation layer" half of the general path is built and
wired in; what's left is **increment 2** (named at the end of
`docs/invariants-measurement.md`): couple the `espc.rs` penalty schedule to the
partitioned search — subproblems from the goal-interaction components, global
constraints = cross-partition transitions of shared mutex variables
(openstacks: `stacks-avail`), λ raised per the existing per-trigger schedule.
That's also the fix the classical measurement predicts for the
resource-coupled partition regressions (gripper/logistics re-traversal).

## Closed (2026-07) — increment 2 built and measured

Increment 2 shipped on the PDDL3 metric path (opt-in `FF_ESPC`, default path
untouched). Each λ iteration now runs a **partitioned composition** instead of
the monolithic tightening B&B:

- Subproblems: interaction components over the real (non-`P3*`) goal
  (`partition::interaction_partition_of`), with the detected renewable-resource
  variables (`stacks-avail`) **excluded from edge formation** — priced as
  global constraints by the per-trigger λ schedule, exactly as prescribed
  above. On openstacks: one component per order.
- Per-stage quality pressure: stage goals are **enriched with the component's
  own preference deliverables** (a goal claims a deliverable when one of its
  achiever ops requires the deliverable's conditional-achievement condition —
  `ship-order(o)` requires `started(o)`, the condition under which
  `delivered(o,p)` fires), skipping deliverables already locked out. This
  replaces the monolithic B&B's cost bound, which cannot prune cost-flat stage
  plans; infeasible enrichment degrades to the bare goal, never a conflict.
- The `P3*` bookkeeping closes on an exact phase tail (`P3END`, then
  collect-iff-applicable-else-forgo per preference), and leftover budget goes
  to a monolithic polish B&B bounded by the incumbent (restores the plain-B&B
  floor). `FF_ESPC_MONO=1` reproduces the pre-increment monolithic loop.

Measured (release, 4 threads, `FF_ESPC_TIME_MS=90000`, 3 identical runs per
instance, stall/saddle-terminated well inside budget): openstacks p01–p08
42/43/55/66/81/90/151/227 → **19/23/17/16/21/22/66/87** — ahead of SGPlan5
(13/16/12/26/36/33/67/123) on p04–p08. The other five preference domains carry
no deadline pairs, so `FF_ESPC=1` stays a verified no-op there. See
`benchmarks/ipc5-scoreboard.md`.

## Follow-on (2026-07) — the closure optimizer generalizes the tail to the default path

The phase-tail machinery built for increment 2 became the core of the DEFAULT
preference-metric optimizer (see CHANGELOG "exact-closure metric optimizer"):
static preference simplification at compile, real-state search with
metric-bounded acceptance (`cost + closure(state) < bound`), the exact tail as
closure, and barrier-free full-DNF satisfaction guidance. Effects ripple across
the other IPC-5 preference domains: storage 2/8 coverage → 8/8 and ahead of
SGPlan5 on p01–p05; tpp/pathways draw level with SGPlan5 on their small
instances; trucks lifted across the row. The `FF_ESPC` openstacks path holds
steady (verified: locked results, t1≡t8); the openstacks DEFAULT dropped
63 → 49 riding the same closure optimizer. Remaining gaps and the next levers
are tracked in `benchmarks/ipc5-scoreboard.md` ("Path to climb" items 4–5).
