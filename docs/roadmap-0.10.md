# ferroplan 0.10 roadmap

Scope locked 2026-07-19: three items left on the STATUS.md board (5, 7,
8), shipped as 0.10.0. Risk goes first, so early work can vouch for what
comes after — the runner's temporal VAL lands first so any coverage
Phase 2 claws back gets checked by an outside witness, the deep
temporal dig runs second, the bounded-quality feature closes the set.

Baselines (all measured this cycle, 30 s / 1 thread / 3 jobs unless
noted): turn-and-open 0/20, temporal-machine-shop 0/20, storage11 0/20,
sokoban11-t 0/20, floor-tile11-t 0/20, match-cellar 6/20 — confirmed
search/semantics walls (identical coverage before and after the cycle's
three grounding fixes). Seq-sat quality domains: floor-tile11 5/20,
visit-all11 8/20 (60 s). Tempo-sat total 326/630 plus the elevator and
openstacks-ADL recoveries.

## Phase 1 — runner polish (agenda item 8)

- **Temporal VAL**: `ipc67.py` validates tempo-sat plans with VAL —
  `TimedPlan::to_ipc()` already emits the `time: (action) [duration]`
  format VAL parses natively; the runner just never passed temporal
  plans through the door. Build VAL via `benchmarks/get-val.sh` when
  it's not on hand.
- **Per-job memory cap**: `RLIMIT_AS` per spawned `ff` (default:
  total RAM / jobs, overridable) so a parallel sweep loses ONE job to
  a memory spike instead of the OOM killer taking the whole box down
  with it — elevator-11's grounding transients killed sibling jobs
  this cycle.
- Acceptance: a tempo-sat sweep reports `val n/n` (not `-`) for every
  solved instance; a deliberately-tiny cap kills only its own job.

## Phase 2 — temporal required-concurrency (agenda item 7)

The decision-epoch scheme starts actions only at event times — a
node's clock or a pending end. The classic hole in that fence: an
action forced to start strictly INSIDE another's interval, at a
non-event time. Whether the four 0/20 walls sit on that hole, on
ordinary search scale, or on something else entirely — that's what
this phase measures before it touches a line of fix code.

- Step 1 (recon): minimal reproductions pulled from turn-and-open,
  temporal-machine-shop, and storage11 instance-1. Sort each into a
  bin: (a) NO applicable start at any reachable epoch → a hole in the
  semantics; (b) the goal is reachable but the search runs out of
  budget chasing it → guidance/scale; (c) grounding/duration debris →
  plumbing.
- Step 2 (fix to the diagnosis, not around it): a semantics hole gets
  the smallest sound epoch extension that clears the repro (candidate:
  same-epoch chained starts already work; the missing move is "start
  X so its END lands at an event," i.e. end-aligned starts); a
  guidance wall gets logged straight and folded into the guidance
  agenda — no hacking at it out of turn.
- Acceptance: each wall variant either books measured coverage or
  carries a one-paragraph diagnosis of why it can't, repro checked
  into the suite either way. Every new solve VAL-validates (Phase 1).

## Phase 3 — length-anytime within one search (agenda item 5)

The 0.9-era idea, still on file: after the first incumbent lands, the
SAME search keeps draining its open list under a tightening g-bound
instead of handing back the result immediately — the machinery
(in-sweep tightening, `g_bound` pruning) already lives on the bounded
metric paths and is measured; this phase aims it at plain length on
the default path.

- Opt-in flag ladder per house rules: default ON only if measured
  never-worse on coverage AND wall time at the scoreboard budget;
  otherwise it ships opt-in with the measurement on the record.
- Acceptance: shorter plans on floor-tile/visit-all/sokoban solved
  instances within the SAME eval budget, identical coverage, t1 ≡ t8
  determinism held (eval-count accounting, never the wall clock).

## Phase 4 — 0.10.0 release mechanics

- CHANGELOG `[0.10.0]`: the full cycle — transport11 attribution,
  fact-space compaction + temporal node cap + stratified grounding
  (elevator recoveries), `?duration` + state-dependent durations,
  the DNF static-resolution collapse (openstacks-ADL +71), the
  budget-aware portfolio, Phases 1–3 above.
- Workspace version bump 0.9.0 → 0.10.0; README coverage refresh;
  `rustup update stable` FIRST (the 0.9.0 clippy-skew lesson,
  RELEASING.md); full gate (fmt, clippy `-D warnings`, suite); main
  fast-forwarded and publish.sh-ready (the user runs publish.sh).

## Recorded (cycle close, 2026-07-19)

- **Phase 1 SHIPPED**: temporal VAL (crew 20/20 val'd on first run) +
  `--mem-gb` RLIMIT_AS per job (elevator-11 spikes now die alone,
  logged with a `mem-cap` note). Acceptance beaten: the validation
  caught a live bug on its first outing — same-instant numeric
  write-write slipped past the fact-only mutex — fixed in
  `epsilon_separate` (numeric footprints; cap 600 → 2000 happenings).
  elevator-numeric val 1/3 → 3/3.
- **Phase 2 ANSWERED + SHIPPED**: no required-concurrency hole in the
  semantics (minimal turn-and-open repro sits in the suite now); the
  real amplifier was absolute-time visited keys — shift-invariant
  deltas on TIL-free tasks shipped (`FF_TEMPORAL_ABS_KEY`). sokoban08-t
  7→10/30, sokoban11-t 0→2/20, floor-tile11-t 0→3/20 (30 s),
  turn-and-open 0→1/20 (60 s; i1 solo in ~25 s), all VAL-green.
  storage11 turned out to be guidance (3 M nodes, live heap,
  helpful→0); TMS is interleaving scale (~47 pending ends/node). Both
  logged into the guidance agenda.
- **Phase 3 MEASURED NEGATIVE, ships opt-in** (`FF_LEN_ANYTIME=1`):
  at 60 s the drain cost 9 instances of coverage (sokoban −7) against
  4 shorter sokoban plans (−234 steps) and zero gains on the
  motivating domains — their plans already come from EHC/LAMA first
  incumbents. Default OFF per the roadmap's never-worse rule.
- **Phase 4**: versions 0.9.0 → 0.10.0, CHANGELOG/README refreshed,
  latest stable confirmed (1.97.1), full gate green; tempo-sat
  scoreboard refreshed with the 0.10.0 binary (see
  `benchmarks/ipc67-temporal.md`).
