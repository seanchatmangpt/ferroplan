# Performance

ferroplan is [data-oriented by design](./architecture.md), but a fast layout
buys nothing if the hot paths grind through redundant work. Three fixes went
in this session, each one turning an instance that used to be *un-finishable*
or *un-scoreable* into a routine solve — each measured, each
correctness-preserving.

## Grounding — static-precondition parameter-domain restriction

Untyped domains used to enumerate the full cartesian product of every parameter,
then string-match nearly all of it back out. gripper's `pick(?obj ?room ?gripper)`
was throwing off 154³ ≈ 3.6M bindings *per action* and discarding 99.98% of them
on the floor.

The fix: restrict each parameter's domain by its **static unary preconditions**
*before* enumerating — `?gripper` only ever ranges over grippers now, never
every object in the world. The blowup dies at the source. Produced ground ops
come out bit-identical; only the search to find them shrinks.

| instance | before | after |
|---|---|---|
| gripper p02 | 658 µs | 247 µs (2.65×) |
| 150-ball untyped, 1-step | 1.56 s | ~0 |
| gripper-250 (partition mode) | 11.9 s | 3.96 s (3×) |

(`crates/ferroplan/src/ground.rs`)

## EHC work cap — scaled by operator count

Enforced hill-climbing carried a fixed work cap. Large-but-easy instances blew
through it and bailed into the *unpruned* best-first arm — millions of
evaluations spent on a problem EHC's near-greedy descent would have walked
straight through. Scale the cap by operator count and those instances finish
in the cheap arm instead; the heuristic never moves, the plan stays valid.

| instance | before | after |
|---|---|---|
| gripper-250 `--mode ff` | 2.16M evals / 33 s | 32k evals / 0.86 s (38×) |

Small and genuinely-hard instances don't move — they never touched the old
cap, or they legitimately need the fallback (deep plateaus are still on the
backlog; see [perf-notes](#how-the-wins-are-measured)).
(`crates/ferroplan/src/search.rs`)

## Metric optimizer — monotone numeric-term folding

The [metric optimizer](./metric-quality.md) runs an anytime branch-and-bound
over the `:metric`. It used to see only the preference-violation terms — so on
domains where the metric also charges a **monotone numeric quantity**, like
rovers' `(sum-traverse-cost)`, it scored a bogus `0`. The numeric part was
invisible; the search had nothing left to optimize.

It now folds monotone numeric metric terms into total-cost and optimizes the
**full** metric. rovers went from un-scoreable to a real metric of **935.3** —
the move that unlocked the sixth IPC-5 domain (see
[Metric quality](./metric-quality.md)).
(`crates/ferroplan/src/pddl3.rs`)

## How the wins are measured

Two harnesses, division of labor deliberate:

- **`cargo bench -p ferroplan --bench planning`** (criterion) is the reference
  for wall-time deltas. Its `solve/` group covers small typed/numeric instances
  (gripper, blocks, rovers); `solve_large/` covers the scale-sensitive
  grounding- and search-dominated cases. Criterion is the *only* noise-robust
  timer on a loaded machine — trust it.
- **`benchmarks/perf.py`** reports deterministic evaluated-state counts. A
  constant-factor win leaves these bit-identical — proof the work shrank, not
  the strategy. A search-strategy win moves them, and demands a re-baseline.

> Raw wall-clock here runs noise-dominated below ~15% — the same binary has
> ranged 11.5–14 s under background load. Treat any single timed run with
> suspicion. Let criterion arbitrate.

The ranked backlog of remaining optimizations (generation-counter `Scratch`
reset, preferred-operator best-first, `apply_into` clone-on-survival) sits in
[`docs/perf-notes.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/docs/perf-notes.md),
alongside the methodology caveats learned the hard way — notably: `atos`
mis-attributes inlined hot code on optimized builds. Trust the de-noised
profile, not the raw top symbols.
