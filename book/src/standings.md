# Competition standings

Three fields, three years, one ledger. ferroplan runs against
International Planning Competitions **IPC-5 (2006)**, **IPC-6
(2008)**, and **IPC-7 (2011)** — every deterministic satisficing
track, swept at standard budgets (60 s classical / 30 s temporal,
three concurrent jobs), every plan checked against
[VAL](https://github.com/KCL-Planning/VAL) before it counts. The
tables below are GENERATED (`python3 benchmarks/standings.py`)
straight from the per-instance sweep logs, re-run against the final
binary at every release cut. No hand edits. The numbers stand on
their own.

Two honesty markers run through the boards:

- **Reference-scored**: checked against the official competition
  field. The IPC-5 preference boards pull from the vendored official
  results archive (`benchmarks/IPC5-results.tgz` — provenance in
  `benchmarks/ATTRIBUTION.md`): per-instance `MetricValue`s for
  SGPlan5, HPlan-P, MIPS-XXL, MIPS-BDD, the rest of the 2006 field.
  The headline: **ferroplan beats SGPlan5 — the track winner —
  24W/4T/10L on the qualitative suite**, taking rovers, storage, and
  tpp outright, splitting openstacks (first pass of the graft, scored
  against a stale 0.8-era ledger, read 12/3/23 — the correction sits
  on the board where it happened). The IPC-5 propositional track is
  scored on plan length against the archive field.
- **Coverage-only**: no aligned reference yet. Either no official
  per-instance archive is vendored (IPC-6/7), or the runner doesn't
  capture the track's quality currency (makespan, for the 2006 time
  tracks — a named debt, not an oversight).

Failure classes, counted per unsolved instance: `timeout` (budget ran
out), `mem-cap` (hit the address-space ceiling — environmental,
logged apart from engine verdicts), `engine-reject/error` (rejected on
sight — feature gaps like the four timed modal operators show up here
by name), `search` (died mid-flight, budget still had room).

Optimal tracks stay out of scope — ferroplan is a satisficing
planner, and the tables say so in plain text rather than leaving a
gap. The IPC-7 sequential multi-core track runs under its own
competition rule (wall-clock, all cores; per-thread-count determinism
holds) on the sweep box's 4 cores. The t8 row is flagged
oversubscribed.

{{#include ../../benchmarks/ipc-standings.md}}

## Reading the boards

- [`benchmarks/ipc5-scoreboard.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc5-scoreboard.md)
  — IPC-5 simple preferences, ferroplan vs the official field.
- [`benchmarks/ipc5-qualitative-scoreboard.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc5-qualitative-scoreboard.md)
  — IPC-5 qualitative preferences, reference-grafted with the full
  W/T/L accounting.
- [`benchmarks/ipc67-results.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc67-results.md)
  / [`benchmarks/ipc67-temporal.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc67-temporal.md)
  / [`benchmarks/ipc67-netben.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc67-netben.md)
  — the standing IPC-6/7 scoreboards (seq-sat, tempo-sat,
  net-benefit), per-variant.
- [`benchmarks/ipc5-prop.md`](https://github.com/seanchatmangpt/ferroplan/blob/main/benchmarks/ipc5-prop.md)
  and siblings (`ipc5-time`, `ipc5-metric-time`, `ipc5-constraints`)
  — the 2006 deterministic-track sweeps, first entered in 0.16.
- `benchmarks/ipc7-mco-t{2,4,8}.md` — the multi-core rows.

Per-cycle history — what moved, why, cut by cut — lives in the
`docs/roadmap-0.*.md` records. This chapter reads the CURRENT
standing, nothing older.
