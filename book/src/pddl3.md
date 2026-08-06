# PDDL3 preferences

ferroplan strips soft-goal preferences at compile time (Keyder & Geffner, JAIR 2009)
and drives the `:metric` down from there.

- **Goal preferences**, including `(forall (?x) (preference p phi))`, are
  expanded into one instance per binding; `(is-violated p)` counts violated
  instances.
- **Precondition preferences** become satisfied/violated action variants, so a
  violation is charged exactly once per application.
- The metric must be linear in `(is-violated …)` and `(total-cost)` (the IPC-5
  *simple-preferences* shape) — plus any monotone numeric term (e.g. rovers'
  `sum-traverse-cost`), which is folded into `total-cost`; maximize / negative /
  scaled metrics fall back to a satisficing plan with a clear note.

## The optimizer (0.4.0)

Default weapon: an **exact-closure metric optimizer**. It walks real states under
metric-bounded acceptance, then closes out the compiled preference bookkeeping with
a provably-optimal `collect`/`forgo` tail. Three pieces carry the scale:

- **Static preference simplification** at compile — statically-satisfied preference
  instances are dropped before grounding (storage's 62k-instance quadratic `forall`
  collapses ~97%).
- **Barrier-free full-DNF guidance** — the search sees a preference's forgone cost
  directly instead of behind a compilation barrier.
- **A budget-escalating branch-and-bound** — a tightening probe that hits its
  per-iteration eval cap without improvement retries the same bound with the
  remaining budget rather than giving up. The deterministic, thread-count-independent
  budget is `FF_PREF_EVAL_BUDGET` (default 2M evals) — a real quality dial.
- **Anytime sweeps + a diversified restart ladder** — each sweep tightens its
  bound in place on every acceptance (a restart happens once per cap, not once
  per improvement), and a capped sweep that fails to improve rotates the
  open-list weights through a fixed profile ladder before the final
  full-budget escalation — a stuck h-ordering is a direction problem, not a
  budget problem. This is what broke the storage/tpp large-instance plateau
  (storage now beats SGPlan5 on p01–p07).

For resource-coupled domains a **default-on ESPC penalty loop** (opt out with `FF_NO_ESPC`; after
Hsu–Wah's extended-saddle-point method) prices a shared resource as a global
constraint across a partitioned search — this is the lever that puts openstacks ahead of
SGPlan5 on its larger instances. Every knob has a restore hatch (`FF_PREF_COMPILED`,
`FF_PREF_NO_STATIC`, `FF_PREF_BARRIER`, `FF_PREF_NO_ESCALATE`, `FF_ESPC_MONO`); see
the [tuning reference](./tuning.md).

Past a certain size the search runs out the clock. On the largest instances exact
optimization hands back the best plan found (flagged *not proven optimal*) when the
budget dies. Full per-instance results vs SGPlan5:
[`benchmarks/ipc5-scoreboard.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc5-scoreboard.md).

## Trajectory constraints (`(:constraints ...)`) — enforced since 0.7

The six untimed modal operators — `always`, `sometime`, `at-most-once`,
`sometime-after`, `sometime-before`, `at end` — hold on the classical path.
Each ground constraint instance compiles down to a small monitor automaton:
fresh 0-ary monitor facts, flipped by conditional effects on every action,
checked at the goal. `forall` outside a `(preference ...)` multiplies
instances (so `(is-violated name)` counts violated instances); `and`/`forall`
*inside* a preference body stay ONE instance, violated at most once — the
PDDL3 instance boundary.

- **Hard** constraints gate the goal through a forced-terminal END action
  (since 0.8): acceptance latches through conditional effects on one
  synthetic `TRAJ-END` step — stripped from reported plans — so the
  compiled goal stays literal-only and grounding cost runs LINEAR in the
  monitor count (the 0.7 goal-conjunct compilation went exponential in the
  worst case; `FF_NO_TRAJ_END=1` restores it). Violate a hard constraint
  and there is no plan. Not a bad plan — no plan.
- **Soft** `(preference name ...)` constraints drop to ordinary goal
  preferences, priced by the metric machinery above — the optimizer stack
  runs unchanged, and `(is-violated name)` reads across goal and
  constraint preferences in one namespace.
- The monitor transition block grounds ONCE, shared across every ground
  action (since 0.8, `FF_NO_COND_SHARE=1` restores per-op copies). Instances
  that once burned through 15 GB during grounding now ground in under a
  second at ~100–200 MB.
- An independent verifier (`ferroplan::verify`) replays the ORIGINAL
  constraint semantics over the trajectory — never the compiled monitors.
  Reported metrics are cross-checked by construction; `validate_plan`
  rejects any constraint-violating plan on sight.
- Statically decidable instances get simplified away before grounding ever
  starts (quadratic `forall` constraints over static relations stay
  tractable); `FF_PREF_NO_STATIC=1` restores the blind expansion.

The timed operators (`within`, `always-within`, `hold-during`,
`hold-after`) and constraints on durative-action domains get **rejected by
name** — never silently dropped on the floor. `FF_CONSTRAINTS_REJECT=1`
restores the pre-0.7 blanket rejection. Measured results on the IPC-5
qualitative-preferences track:
[`benchmarks/ipc5-qualitative-scoreboard.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc5-qualitative-scoreboard.md).
