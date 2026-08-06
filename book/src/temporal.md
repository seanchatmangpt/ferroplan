# Temporal planning

ferroplan runs **PDDL2.1 durative actions**. Any `:durative-action` in the
domain trips auto-detection; the problem gets routed to a decision-epoch
forward search. The CLI prints the IPC temporal plan format.

## What's supported

- `:durative-action` with `at start`, `over all`, and `at end`
  **conditions** and **effects**.
- **Durations** that are constants *or* **parameter-dependent**, e.g.
  `:duration (= ?duration (/ (distance ?a ?b) (speed ?v)))` — evaluated per
  grounded action against the initial state, or **per expansion against the
  current state** when the duration reads a fluent some action modifies
  (state-dependent durations, since 0.12). `?duration` is also accepted
  inside numeric *effect* expressions (duration-dependent effects like
  `(increase (energy ?x) (* ?duration (recharge-rate ?x)))`, since 0.10).
- **Duration inequalities** — `(>= ?duration L)` / `(<= ?duration U)` and `and`
  ranges; the search commits to the shortest feasible duration.
- **Timed initial literals** — `(at <time> <literal>)` in `:init`; each becomes a
  synthetic exogenous applier fired from a pre-seeded agenda at its time (so a goal
  reachable only via a TIL is not pruned as a dead end).
- **Required concurrency** — actions whose intervals must overlap (the classic
  "match / mend-fuse": the fuse can only be mended while the match is lit).

## How it works

Every durative action gets split into two instantaneous **snap-actions** — the
existing grounder and relaxed-plan heuristic keep working unmodified:

- `A-START` takes the `at start` condition (plus the `over all` invariant) and
  applies the `at start` effects plus a `(RUNNING-A …)` token;
- `A-END` requires the `at end` condition, the invariant, and that token; it
  applies the `at end` effects and drops the token.

Duration and the `over all` invariant live in a side table the temporal search
reads at run time. A decision-epoch search walks time over an agenda of
pending end-events — `A-END` only fires `duration` after its matching
`A-START` — and checks the invariant at both happenings, **and on every
happening between** (0.14). Any happening whose effects would delete a
running interval's invariant fact gets refused on the spot: no delete-and-
re-add can sneak through the endpoint checks, no bake spanning the gap between
two kiln firings. Nodes whose agenda head can never legally fire get pruned at
birth. Since 0.13 the pending-interval agenda runs **symmetry-reduced**
(canonical ordering, redundant identical-interval elimination — `FF_NO_TSYMM=1`
reverts it): same-epoch starts of interchangeable intervals stop multiplying
the visited space. 0.14 stacks **object-symmetry orbits** on top
(`FF_NO_ORBIT=1` reverts): when a problem's objects or goal pairs are
interchangeable — identical init profiles, symmetric goals, a grounded task
closed under relabeling — the visited key gets canonicalized under member
permutation. Machine-shop's "which identical piece is which" blowup collapses
to nothing.

## Output

Plans render in the IPC temporal format, `start: (action args) [duration]`,
tagged with the overall **makespan**:

```
0.000: (fly plane1 city-a city-b) [3.000]
3.000: (fly plane1 city-b city-c) [4.000]
```

Through the library, temporal solutions carry `time` on each `Step`, `makespan`
on the `Plan`.

## Usage

```sh
ff -o temporal-domain.pddl -f problem.pddl            # auto-detected
ff -o temporal-domain.pddl -f problem.pddl --mode temporal --json
```

## Resource scheduling (renewable + consumable)

Durative actions over numeric fluents give **resource allocation over time**
for free — the case that matters when you're scheduling crews, machines,
tools, power, mana. Model a **renewable** resource as a pool: taken at start,
returned at end, guarded by an at-start check.

```pddl
(:functions (workers))
(:durative-action chop-tree
  :duration (= ?duration 3)
  :condition (at start (>= (workers) 1))
  :effect (and (at start (decrease (workers) 1))     ; held over the interval…
               (at end   (increase (workers) 1))     ; …released at the end
               (at end   (increase (wood) 1))))
```

The decrement holds until the matching end fires — the decision-epoch search
locks the resource across the whole `[start, end]` interval. A pool of 1 forces
tasks to serialize; a larger pool lets them overlap. **Consumable** resources
(materials) run the same logic without the release — produced and burned by a
crafting chain (`wood → planks`, `planks + stone → house`).

See [`examples/rpg/`](https://github.com/seanchatmangpt/ferroplan/tree/main/examples/rpg)
for a full gather → craft → build example: the same problem plans serially
(makespan ~19) with `(= (workers) 1)`, in parallel (~13) with `3`. Plans are
satisficing, not makespan-optimal — fast and good, built for an agent that
plans, acts, and replans as the world moves under it.

## Validation against VAL

Every plan runs through [VAL](https://github.com/KCL-Planning/VAL), the IPC plan
validator. On the full IPC-2008/2011 tempo-sat corpus — 630 instances, 30 s
each — ferroplan solves **399, and every one of the 399 checks out VAL-valid**
under PDDL2.1 continuous-time semantics. That confirms the snap-action
compilation, `over all` invariants, required concurrency, ε-separation — all
correct. (VAL is what surfaced the ε-separation requirement in the first
place. Since 0.10 the pass totally ε-orders execution: same-instant mutexes —
conditional-effect ones included — can't happen, by construction.) The
unsolved remainder is **search-limited**: the recorded walls are guidance
problems, not semantics failures. See
[`benchmarks/ipc67-temporal.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc67-temporal.md).

## Not yet supported

**Continuous** (`#t`) effects don't run (discrete duration-dependent effects
via `?duration` do — see above). PDDL3 trajectory constraints
(`(:constraints …)`) are enforced on the *classical* path (untimed operators,
since 0.7), not on the temporal path — a durative-action domain that declares
them gets rejected outright, never silently dropped. Temporal **search
guidance** on the recorded wall domains (machine-shop, storage, model-train)
is the open item.
