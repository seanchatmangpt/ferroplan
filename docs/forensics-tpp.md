# Forensics: what the tpp/pathways tail gap actually is

*2026-07, post-0.5.0. Case open: the remaining SGPlan5 gap
(tpp/pathways p05–p08) reads direction-bound — identical metrics at 4× the
eval budget, deaf to the restart ladder, deaf to composition-as-seeding.
Nothing else gets touched until we know what the better plans actually DO.*

*(The official `IPC5-results.tgz` plans went dark — the ICAPS'06 host and
known mirrors 403, no way in. So the optimum gets derived cold, from first
principles. Turned out to be the sharper method anyway: it hands over the
exact target, not a specimen plan to reverse-engineer.)*

## The decisive observation: tpp actions cost nothing

The tpp metric is a pure weighted sum of violated preferences; `drive`, `buy`,
`load`, `unload` — free, all of them, zero cost. A plan's quality lives or
dies on the end state it stops in. This was never a search problem. It's a
**selection problem**: pick the jointly-satisfiable preference subset of
maximum weight, then walk to it — any route that arrives is as good as any
other.

The end-state space is small, closed-form, countable on the fingers. Per
goods `g`, the mutex group `stored(g, level1..4)` holds exactly one level at
a time, and that level decides which of the p0A/p1A/p2A/p3A instances
(weights 1/2/4/8) get violated:

| final level | violated weight (of 15) |
|---|---|
| none | 15 |
| L1 | 14 |
| L2 | 13 |
| L3 | 11 |
| L4 | 7 |

Levels sit under a hard ceiling — total market supply (`on-sale` init): on
p05, goods1 ≤ 3, goods3/6/7 ≤ 2, goods2/4/5 ≤ 4. The 16-weight constraint
preferences chain the choices together: `p4D/p4E` forbid goods1 at L3/L4,
`p4B/p4C` tie goods1's level to goods4's, `p4A`/`p4F` demand goods3/goods7
and goods5/goods6 sit at EQUAL levels.

## SGPlan5's 79 is the assignment optimum — derived, not copied

Run the assignment by hand:

- goods2, goods4 @ L4 (cap 4): 7 + 7
- goods3, goods6, goods7 @ L2 (cap 2): 13 + 13 + 13 — `p4A` holds because
  goods3 and goods7 sit at the SAME level
- goods1 @ **L2**, not its cap of L3: 13 — L3 would shave 2 more off p2A but
  trips the 16-weight `p4D`; L2 also clears `p4C` for free since goods4
  already sits at L4
- goods5 @ **L2**, not its cap of L4: 13 — L4 would shave 6 but `p4F` demands
  goods6 match it, and goods6 is capped at 2; matching at L2 costs 6, saves
  16

Total: 7+7+13+13+13+13+13 = **79**. Exactly SGPlan5's score. The IPC-5
winner's number on this instance is the closed-form selection optimum —
nothing about its *search* needed replicating, only its *selection*.

## Our 93, diffed: the whole gap is ONE decision

Pull the 0.5.0 plan's end state from `--json`: goods2@4, goods3@2, goods4@4,
goods5@2, goods6@2, goods7@2 — **six of seven choices optimal** — and
goods1@**L3**: 11 violated plus the 16-weight `p4D` trampled flat = 27, where
the optimum pays 13. Difference: **14 = the entire 93 − 79 gap.**

## The mechanism: init-satisfied traps are invisible to the guidance

Why does the search buy goods1 up to L3? `p4D` (`not (stored goods1 level3)`)
is **satisfied at the initial state**, and the barrier-free guidance
(Stage C, 0.4.0) EXCLUDES init-satisfied preferences from the satisfaction
penalty — stripped out because penalizing their *transient* dips walled off
plateaus. The consequence: the guidance rewards storing goods1 to L3
(+4-weight `p2A` satisfied) and never sees the 16-weight trap waiting behind
it. Every restart-ladder profile reshuffles `g`/`h` but runs the SAME
satisfaction penalty underneath — every direction buries the L2-stopping
states the same way. That's the "direction-bound" plateau, caught in the act.

## Measured immediately: the old hatch flips the verdict

`FF_PREF_BARRIER=1` (keep init-satisfied preferences in the guidance) on the
0.5.0 binary:

| instance | default | barrier | SGPlan5 |
|---|---|---|---|
| tpp p05 | 93 | **89** | 79 |
| tpp p07 | 117 | **110** | 100 |
| tpp p08 | 147 | **129** | 105 |
| pathways p06 | 12.9 | **11** | 10 |
| **storage p07** | 124 | **60** | 160 |
| pathways p05 | 6 | 6.5 | 6.5 (win → tie) |
| rovers p04 / openstacks p01 | 418.7 / 19 | unchanged | — |

The Stage-C exclusion was the right call for the 0.4.0 engine. Under the 0.5
engine — anytime sweeps, restart ladder — it was leaving money on the table.

## The full-suite verdict: default flipped (0.5.1)

The 48-instance sweep with the barrier on: **storage p05–p08 collapse to
25/43/60/83** (from 31/121/124/148) — beats SGPlan5 on all eight instances,
**a clean domain sweep**, totals 234 vs 547; tpp tail 89/104/110/129;
pathways p06 at 11. Rovers, openstacks, trucks — bit-identical, untouched.
One casualty in the whole run: pathways p05, 6 → 6.5, an outright win
softening to an exact tie. Suite tally 19W/15T/14L. Init-satisfied
preferences stay KEPT in the guidance by default from 0.5.1 on
(`FF_PREF_NO_BARRIER=1` restores the exclusion if needed), verified
deterministic (storage p07 = 60 at 1 and 4 threads), all guards green.

The residual tpp gap after the barrier (89 vs 79) is the part guidance still
can't see at all — the goods5@L2-for-goods6's-sake move, *coordinated*
selection the local view has no eyes for.

## The 0.6 path, in order

1. **Barrier default, weight-aware.** Flip the init-satisfied exclusion to
   weight-aware inclusion (shield traps whose weight outweighs what
   trampling them buys; leave cheap prefs dip-free) — or go plain inclusion
   if the full sweep says the blunt version nets positive. Small, measured,
   immediate.
2. **Exact end-state selection.** Solve the selection problem head-on — per
   mutex group, choose the end value; supply/reachability caps pulled from
   static analysis; implication/equality couplings pulled from the
   preference structure — by exact search (the spaces are tiny: 5^7 here),
   then hand the chosen selection to the planner as HARD goals, tail
   forgoing the rest. This is what SGPlan's number IS on these domains;
   FF-style search is sharp at hitting a fixed target and blind at
   discovering a coordinated one. Generalizes to pathways (`or`-disjunct
   selection) and caps the whole class of gap the guidance can never see.
3. **Keep the eval-budget contract.** Selection runs before search and is
   combinatorial, not state-space — it doesn't touch determinism.

Items 1–2 replace the previous 0.6 sketch (per-stage λ pricing on the seed3
substrate): the forensics says the tails are a selection problem, not a
coordination-during-search problem.
