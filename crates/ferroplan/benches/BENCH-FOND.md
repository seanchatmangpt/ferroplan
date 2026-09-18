# BENCH-FOND — FOND solver + `solve_hddl` micro-domain baselines

Baseline numbers for the `fond` criterion suite (`benches/fond.rs`, ticket
`fond-htn-26`, wave v26.9.17). These are committed FACTS: the suite's seeded
generators replay the identical instances at the recorded seed, so any future
run on comparable hardware reproduces them modulo machine noise. No
cherry-picking: the full sweep is reported, including the slowest rows.

## Provenance

| field | value |
|---|---|
| numbers produced by | `bench/fond-criterion` @ **17b8cc7** (`bench(ferroplan): FOND criterion suite + CI threshold tripwires (fond-htn-26)`) |
| recorded | 2026-09-18T00:12Z |
| generator seed | `0x5EED_2026_0917` (`SEED` in `benches/fond.rs` and `tests/fond_threshold.rs`; same constant as `tests/fond_property.rs`) |
| bench command | `cargo bench -p ferroplan --bench fond -- --quick` — exit 0 |
| samples | criterion `--quick` mode: **10 samples** per benchmark (ticket floor >= 10); mean ± stddev from `target/criterion/**/new/estimates.json` |
| machine | Apple M3 Max, 16 cores, 48 GB RAM, macOS 26.2, rustc 1.97.1 (8bab26f4f 2026-07-14) |

Reproduce: `cargo bench -p ferroplan --bench fond` (default: 100 samples) or
with `-- --quick` to match this table exactly.

## Instance shapes (all inline, seeded, deterministic)

- **chain(n)** — deterministic forward chain, goal at the end; worst case for
  the strong fixpoint (one rung admitted per fixpoint round, `n - 1` rounds).
- **lattice(L x W, n = L*W)** — layered DAG, 2 deterministic actions per state
  (`stay`, seeded random `cross`); acyclic, one layer admitted per round, 2n
  action groups. Sizes: 10 = 5x2, 50 = 10x5, 200 = 20x10.
- **mixed_ladder(n)** — strong-cyclic retry ladder over exactly n states:
  `{self,next}` 50/50 rungs (strong fixpoint NoPlans -> dispatch falls
  through), **20% self-loop-only decoys** (`x*`, scanned by every phase
  sweep, never admitted), and **traps** (`p*` with a pure `wait` self-loop +
  a `venture` whose dead-sink outcome is never committable) that survive the
  Phase-2 witness prune and are pruned only by the Phase-3 committable
  goal-reachability sweep. Composition: n=10 -> 5 rungs/1 trap/2 decoys;
  n=50 -> 27/6/10; n=200 -> 107/26/40.
- **solve_hddl** — the 7 hand-authored `tests/fixtures/fond-htn-micro/`
  domains (provenance comments in-file; no koala files), end-to-end plus the
  separately-public `ferroplan_hddl` stages (parse / ground / translate).

## Baselines (mean ± stddev, 10 samples, criterion defaults otherwise)

### `fond_policy` — strong fixpoint (via `solve_planning_type(Fond)` dispatch)

| benchmark | mean | stddev |
|---|---|---|
| strong_fixpoint_chain_10 | 9.13 µs | 0.25 µs |
| strong_fixpoint_chain_50 | 396.6 µs | 4.9 µs |
| strong_fixpoint_chain_200 | 15.53 ms | 0.32 ms |
| strong_fixpoint_lattice_10 | 6.98 µs | 0.02 µs |
| strong_fixpoint_lattice_50 | 114.5 µs | 0.20 µs |
| strong_fixpoint_lattice_200 | 2.26 ms | 4.7 µs |

The chain is super-linear by construction (n rounds x O(n^2) group scans);
the lattice's extra width amortizes the same fixpoint over fewer rounds —
the 200-state gap (15.5 ms vs 2.3 ms) is the round-count difference, not an
anomaly.

### `fond_policy_strong_cyclic` — mixed retry-loop ladders

| benchmark | mean | stddev |
|---|---|---|
| strong_cyclic_mixed_ladder_10 | 22.2 µs | 0.04 µs |
| strong_cyclic_mixed_ladder_50 | 698.0 µs | 0.20 µs |
| strong_cyclic_mixed_ladder_200 | 22.40 ms | 64.0 µs |

NOTE: the wall includes the preceding strong-fixpoint NoPlan pass, because
`fond_policy`/`fond_policy_strong_cyclic` are private and the production
entry is the `solve_planning_type(Fond)` dispatch (strong first, strong-cyclic
on `NoPlan`). The tripwire pins that Phase 3 actually prunes every trap from
the returned policy (`tests/fond_threshold.rs`).

### `solve_planning_type` dispatch overhead

| benchmark | mean | stddev |
|---|---|---|
| solve_planning_type_dispatch_tiny_chain | 899 ns | 11 ns |

The floor every `PlanningType::Fond` request pays (validate + index + groups
+ one fixpoint round + policy build) on a 2-state deterministic chain.

### `solve_hddl` — micro domains, end-to-end + per-stage

`e2e` = parse + ground + translate + solve wall. The solve stage itself has
no public entry point (`solve_hddl_inner` is private), so solve-only cost is
e2e minus the three front stages (approximate; includes dispatch glue).

| fixture | e2e | parse | ground | translate |
|---|---|---|---|---|
| drop-retry | 70.9 µs ± 0.6 | 20.2 µs ± 0.2 | 7.9 µs ± 0.1 | 9.5 µs ± 0.1 |
| tray-dirty | 95.4 µs ± 0.3 | 31.5 µs ± 0.03 | 8.5 µs ± 0.3 | 22.3 µs ± 0.04 |
| sense-then-branch | 90.5 µs ± 0.3 | 25.6 µs ± 0.3 | 6.5 µs ± 0.2 | 18.2 µs ± 0.4 |
| grow-loop | 57.7 µs ± 3.9 | 14.8 µs ± 0.001 | 4.0 µs ± 0.02 | 7.7 µs ± 0.01 |
| supervisor-fail | 87.8 µs ± 0.4 | 25.9 µs ± 0.1 | 6.6 µs ± 0.001 | 15.9 µs ± 0.02 |
| plain-chain | 66.5 µs ± 0.8 | 17.8 µs ± 0.1 | 4.9 µs ± 0.02 | 11.7 µs ± 0.03 |
| both-branches-deadend (NoPlan) | 62.4 µs ± 1.1 | 13.2 µs ± 0.07 | 3.4 µs ± 0.03 | 6.3 µs ± 0.01 |

Every fixture is dominated by parse; solve-only cost (e2e minus stages) is
roughly 33-47 µs across the suite. `both-branches-deadend` solves to typed
`NoPlan` — its wall is the honest cost of PROVING unsolvability.

## Threshold tripwires (`tests/fond_threshold.rs`)

Plain `#[test]`s (so regressions fail CI, not just reports). Bounds are
release-shaped (the ticket's "200-state strong fixpoint < 50 ms") with a
>= 5x-measured generous multiple for the unoptimized dev profile, selected by
`cfg!(debug_assertions)` — dev walls were measured before fixing bounds:

| configuration | dev wall (measured, calibration run) | release baseline (criterion) | bound (dev / release) |
|---|---|---|---|
| chain_200 strong fixpoint | 237.7 ms | 15.5 ms | 1200 ms / 50 ms |
| lattice(20x10) strong fixpoint | 41.8 ms | 2.3 ms | 1200 ms / 50 ms |
| mixed_ladder_200 strong-cyclic | 219.3 ms | 22.4 ms | 1200 ms / 50 ms |
| solve_hddl micro fixtures (each) | 0.20-0.73 ms | ~0.06-0.10 ms | 1200 ms / 50 ms |

Gates (both observed green at 17b8cc7): `cargo test -p ferroplan --test
fond_threshold` (dev) and `cargo test -p ferroplan --release --test
fond_threshold` (release legs). The tripwire also asserts structural facts
per run: chain policy covers every non-goal state, the ladder routes through
the strong-cyclic fallback, and Phase 3 prunes every trap.
