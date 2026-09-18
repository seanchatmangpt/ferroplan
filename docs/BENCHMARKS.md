# FOND-HTN benchmarks — canonical consolidation

One canonical table of **every committed FOND-HTN result** as of
2026-09-18 (wave 3 + wave 4 integrated). This file consolidates only
already-committed numbers: every row cites its source file, nothing is
recomputed, re-measured, or smoothed. No new measurements were run to
produce this page.

- Oracle = an external FOND-HTN planner used strictly as a test oracle
  (koala/pandaPI, run from `/tmp` via a flock-serialized runner); see
  [FOND-HTN.md § 4 Oracle methodology](FOND-HTN.md#4-oracle-methodology)
  for the runner contract and KOALA policy.
- Measurement conditions per source file govern; committed runs were made
  on an Apple M3 Max (macOS, debug-profile `cargo test` runs, serial
  `--test-threads=1` where stated). Walls vary run to run — the verdict
  classes are the stable fact, the walls are context.

## Summary — one row per corpus

| corpus | instances | solved (ferroplan) | solved (oracle) | refused-by-limit | gaps | source file |
|---|---|---|---|---|---|---|
| HTN differential (IPC-2023 + PANDA + SHOP3) | 21 | 14 | 20 | 4 (`TRANSLATE_ERROR`, 10 s translate wall) | 2 grounder refusals (`GAP:*`) + 1 open verdict | [`crates/ferroplan/tests/fixtures/htn-oracle/RESULTS.md`](../crates/ferroplan/tests/fixtures/htn-oracle/RESULTS.md) |
| IPC-2023 HTN pipeline suite | 13 | 9 | — | 4 (`LIMIT:translate-wall`) | 0 | [`crates/ferroplan/tests/fixtures/htn-ipc2023/RESULTS.md`](../crates/ferroplan/tests/fixtures/htn-ipc2023/RESULTS.md) |
| FOND-HTN oracle harvest (7 koala domains × 5 + 1 negative case) | 31 | — | 18 | 12 (`TIMEOUT`, 90 s wall) | 1 `UNSUPPORTED` (parse-stage negative case) | [`crates/ferroplan/tests/fixtures/fond-htn/oracle-harvest-full.json`](../crates/ferroplan/tests/fixtures/fond-htn/oracle-harvest-full.json) + [summary](../crates/ferroplan/tests/fixtures/fond-htn/oracle-harvest-full-summary.md) |
| FOND-HTN golden agreement ledger (micro + external cases) | 10 | 5 | 9 | 3 (ferroplan `error`, resource-limit) | 3 admitted semantic divergences (pinned by tests) | [`crates/ferroplan/tests/fixtures/fond-htn/oracle-goldens.json`](../crates/ferroplan/tests/fixtures/fond-htn/oracle-goldens.json) |
| flat-FOND three-way (8 hand-authored domains) | 8 | 7 | 2 | 5 (oracle `TIMEOUT` on cyclic-only — koala-side signature) | 0 (8/8 three-way class agreement) | [`crates/ferroplan/tests/fixtures/fond-flat/oracle-goldens.json`](../crates/ferroplan/tests/fixtures/fond-flat/oracle-goldens.json) + 8 × `verdict.json` |
| IPC full sweep (wave-4, 43 domains × 1 problem) | 43 | 12 | — | 28 (`LIMIT:*`: 14 ground-actions, 9 translate-wall, 3 ground-methods, 2 solve-wall) | 3 `GAP:ground` (typed validation refusals) | [`crates/ferroplan/tests/fixtures/ipc-sweep/RESULTS.md`](../crates/ferroplan/tests/fixtures/ipc-sweep/RESULTS.md) |
| Scaling ladder (wave-4, 2 families × 6 rungs) | 12 | 12 | — | 0 — no refusal through n=128 | 0 | [`crates/ferroplan/tests/fixtures/scaling-ladder/RESULTS.md`](../crates/ferroplan/tests/fixtures/scaling-ladder/RESULTS.md) |
| FOND criterion bench (wave-4, seeded micro-domains) | 10 benchmarks | — | — | — | — | [`crates/ferroplan/benches/BENCH-FOND.md`](../crates/ferroplan/benches/BENCH-FOND.md) |

"solved (oracle)" is blank where the corpus records no oracle column
(rows 2, 6, 7: ferroplan-only suites; the oracle side of the shared
HTN instances is row 1), where the corpus is an oracle-side harvest
with no ferroplan column (row 3), or where the entry is a wall-clock
benchmark rather than a verdict corpus (row 8 — the criterion bench's
facts are means ± stddev, see corpus 8).

## Corpus 1 — HTN differential: ferroplan vs oracle, 21 instances

Source: [`crates/ferroplan/tests/fixtures/htn-oracle/RESULTS.md`](../crates/ferroplan/tests/fixtures/htn-oracle/RESULTS.md)
(ticket `fond-htn-05-diff-htn`). 13 IPC-2023 + 4 PANDA feature tests +
4 SHOP3 hand-ports. Verdict rule: oracle SOLVED if any mode solves,
NOSOLUTION only if both modes agree; strict agreement 14/21, 1/21
open-verdict class, 6/21 admitted mismatches (each pinned by an
`#[ignore]`d demonstration test in `htn_oracle.rs`).

| outcome | count | instances |
|---|---|---|
| SOLVED (both engines) | 14 | Blocksworld-GTOHP, Blocksworld-HPDDL, Depots, Factories-simple, Lamps, Multiarm-Blocksworld, PO_Satellite, Robot, Towers, panda-empty, panda-interleaving, shop3-port-ab-ordering, shop3-port-loan-credit, shop3-port-swap |
| `TRANSLATE_ERROR` (oracle SOLVED) — `LIMIT:translate` | 4 | PCP_1, PO_Transport, Satellite-GTOHP, Transport |
| `GROUND_ERROR` (oracle SOLVED) — `GAP:*` | 2 | panda-conditional-effect (`nested 'when' is out of scope`), panda-method-effect (`unbound variable '?x'`) |
| open verdict (oracle harness ERROR, ferroplan clean `NoPlan`) | 1 | shop3-port-loan-noplan |

Every strict SOLVED agreement was additionally outcome-closure-checked
against the independently re-derived ground IR. The 4 translate-limit
rows are the internal `TranslateLimits::default()` 10 000 ms wall inside
`solve_hddl`'s pipeline (ticketed as capacity work); the 2 `GAP:*` rows
are loud, typed grounder refusals — no panic, no garbage.

Reproduce:

```sh
# gate (deterministic, no oracle):
cargo test -p ferroplan --test htn_oracle
# measurement run that produced the committed walls:
cargo test -p ferroplan --test htn_oracle measure_translation_walls_for_results_md -- --ignored --test-threads=1 --nocapture
# oracle side (external, /tmp only):
bash /tmp/fond-oracle/build.sh
bash /tmp/fond-oracle/oracle-run.sh <domain.hddl> <problem.hddl> --mode flexible
```

## Corpus 2 — IPC-2023 HTN pipeline suite, 13 instances

Source: [`crates/ferroplan/tests/fixtures/htn-ipc2023/RESULTS.md`](../crates/ferroplan/tests/fixtures/htn-ipc2023/RESULTS.md)
(ticket `fond-htn-13-test-htn-ipc2023`). `solve_hddl` end-to-end
(parse → ground → translate → FOND policy), 30 s wall budget; every
SOLVED row policy-closure-checked against the re-derived ground IR.

| outcome | count | instances |
|---|---|---|
| SOLVED (closure-verified) | 9 | Blocksworld-GTOHP, Blocksworld-HPDDL, Depots, Factories-simple, Lamps, Multiarm-Blocksworld, PO_Satellite, Robot, Towers |
| `LIMIT:translate-wall` @ 10 s | 4 | PCP_1, PO_Transport, Satellite-GTOHP, Transport |

0 parse gaps · 0 ground gaps · 0 NoPlan · 0 panics · 0 hangs. All four
limit rows are the typed translate wall clock
(`translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`),
not the suite's 30 s budget.

Reproduce:

```sh
cargo test -p ferroplan --test htn_ipc2023
# measured run (walls in the source table):
cargo test -p ferroplan --test htn_ipc2023 -- --test-threads=1 --nocapture
```

## Corpus 3 — FOND-HTN oracle harvest, 31 recorded verdicts

Sources: [`oracle-harvest-full.json`](../crates/ferroplan/tests/fixtures/fond-htn/oracle-harvest-full.json)
(31 verdict records) and
[`oracle-harvest-full-summary.md`](../crates/ferroplan/tests/fixtures/fond-htn/oracle-harvest-full-summary.md)
(ticket `fond-htn-02-oracle-harvest-fondhtn`). Oracle-side harvest:
`--mode flexible --timeout 90 --heur ff`, flock-serialized, 2026-09-17.
This file is a committed fact record; no in-repo test consumes it.

| domain | instances | SOLVED | TIMEOUT | notes |
|---|---|---|---|---|
| Transport | 5 | 4 | 1 | p05 hits the 90 s wall mid-search |
| Snake | 5 | 3 | 2 | |
| Childsnack | 5 | 1 | 4 | only p01 solves |
| Depots | 5 | 2 | 3 | |
| Rover | 5 | 3 | 2 | |
| Satellite | 5 | 5 | 0 | all sub-second |
| AssemblyHierarchical | 1 | — | — | `UNSUPPORTED` — parse-stage negative case: the oracle silently mis-compiles the undeclared `FaultyPort` type (objects dropped, exit 0); the anti-example behind ferroplan's parse-rejection requirement |

Total: **18 SOLVED / 12 TIMEOUT / 0 NOSOLUTION / 0 PARSE_ERROR /
1 UNSUPPORTED**. TIMEOUT rows are genuine search exhaustion at the 90 s
wall (every wall_s ≈ 90.0–90.5) — the oracle is a strong-only decision
procedure, so a TIMEOUT on a retry-style (cyclic-policy) domain is
expected and is **not** unsolvability evidence.

Reproduce:

```sh
bash /tmp/fond-oracle/build.sh
# per instance (serialized through the flock-protected runner):
bash /tmp/fond-oracle/oracle-run.sh <domain.hddl> <problem.hddl> --mode flexible --timeout 90 --heur ff
# per-run logs + policies land under /tmp/fond-oracle/runs/<id>/
```

## Corpus 4 — FOND-HTN golden agreement ledger, 10 runs

Source: [`crates/ferroplan/tests/fixtures/fond-htn/oracle-goldens.json`](../crates/ferroplan/tests/fixtures/fond-htn/oracle-goldens.json)
(tickets `fond-htn-11`/`fond-htn-12`; consumed by
`crates/ferroplan/tests/fond_htn_oracle.rs`). Oracle protocol:
`--mode flexible --heur ff --timeout 180`, every NOSOLUTION
cross-checked with `--mode fixed-ld`; ferroplan outcomes observed live
via `solve_hddl(PlannerLimits::default())`.

| run | oracle | ferroplan | agree | divergence |
|---|---|---|---|---|
| micro-drop-retry | SOLVED | NOSOLUTION | no | semantic |
| micro-overlap | SOLVED | SOLVED | yes | — |
| micro-recurse | SOLVED | NOSOLUTION | no | semantic |
| micro-sense | NOSOLUTION | SOLVED | no | semantic |
| koala-Transport | SOLVED | error | no | resource-limit |
| koala-Childsnack | SOLVED | error | no | resource-limit |
| koala-Snake | SOLVED | error | no | resource-limit |
| koala-Satellite | SOLVED | SOLVED | yes | — |
| koala-Depots | SOLVED | SOLVED | yes | — |
| koala-Rover | SOLVED | SOLVED | yes | — |

4/10 agreement; 3 semantic divergences and 3 resource-limit divergences,
each recorded with a verbatim reason in the goldens file and pinned by a
named test — divergences are ledger rows, not silent skips.

Reproduce:

```sh
cargo test -p ferroplan --test fond_htn_oracle
```

## Corpus 5 — flat-FOND three-way, 8 hand-authored domains

Sources: [`crates/ferroplan/tests/fixtures/fond-flat/oracle-goldens.json`](../crates/ferroplan/tests/fixtures/fond-flat/oracle-goldens.json)
plus one `verdict.json` per domain (tickets `fond-htn-06-diff-flatfond`
+ `fond-htn-16-corpus-flatfond`), test
`crates/ferroplan/tests/fond_flat_oracle.rs`. Three-way = literature
class / oracle verdict / ferroplan verdict (explicit + HDDL encodings
both).

| domain | literature class | oracle (flexible / fixed-ld) | ferroplan (both encodings) |
|---|---|---|---|
| tireworld | cyclic-only | TIMEOUT / TIMEOUT | SOLVED (strong-cyclic), closure-verified |
| triangle-tireworld-3cities | cyclic-only | TIMEOUT / TIMEOUT | SOLVED (strong-cyclic), closure-verified |
| islands-2 | cyclic-only | TIMEOUT / TIMEOUT | SOLVED (strong-cyclic), closure-verified |
| wall-2rows | cyclic-only | TIMEOUT / TIMEOUT | SOLVED (strong-cyclic), closure-verified |
| coffee | cyclic-only | TIMEOUT / TIMEOUT | SOLVED (strong-cyclic), closure-verified |
| faults-1bit | strong | SOLVED (~0.1 s) | SOLVED (strong) |
| boolean-not | strong | SOLVED (~0.1 s) | SOLVED (strong) |
| river-unsafe | unsolvable | NOSOLUTION (~0.15 s) | typed `NoPlan` |

8/8 three-way class agreement. The oracle TIMEOUTs on the five
cyclic-only domains are the *signature* of a strong-only search facing
cyclic-policy-only problems (≤ 12-state instances, walls of 60–123 s,
identical under `fixed-ld`) — not a capability comparison; ferroplan's
strong-cyclic dispatcher solves all five and produces outcome-closed
policies.

Reproduce:

```sh
cargo test -p ferroplan --test fond_flat_oracle
# live oracle cross-check (external, slow):
cargo test -p ferroplan --test fond_flat_oracle -- --ignored
```

## Corpus 6 — IPC-2023 full sweep: `solve_hddl` staged, 43 domains

Source: [`crates/ferroplan/tests/fixtures/ipc-sweep/RESULTS.md`](../crates/ferroplan/tests/fixtures/ipc-sweep/RESULTS.md)
(ticket `fond-htn-27`). All 43 IPC-2023 hierarchical-track domains,
verbatim competition data (`domain.hddl` + first-listed problem each),
staged per-stage budgets (parse 10 s outer; ground/translate =
`GroundingLimits`/`TranslateLimits::default()` — the exact budgets
`solve_hddl_inner` passes; solve `max_wall_ms = 30000`), each stage under
an anti-hang watchdog. Recorded 2026-09-18T00:21:12Z, Apple M3 Max.

| outcome | count |
|---|---|
| SOLVED (typed, policy produced) | 12 |
| `LIMIT:ground-actions` (max 10 000 exceeded) | 14 |
| `LIMIT:translate-wall` (internal 10 000 ms wall — pre-ticket-23 budget) | 9 |
| `LIMIT:ground-methods` (max 10 000 exceeded) | 3 |
| `LIMIT:solve-wall` (30 s planner budget) | 2 |
| `GAP:ground` (typed validation refusal: duplicate type decl ×1; variable-arg root task-network subtask ×2) | 3 |

0 NOPLAN · 0 panics · 0 hangs past a watchdog — the sweep test exits 0
iff all 43 outcomes are honest typed answers. Full per-domain table and
verbatim refusal strings in the source file.

Reproduce:

```sh
# gate (sampled 6-domain heartbeat):
cargo test -p ferroplan --test ipc_sweep
# full 43-domain sweep (this table):
cargo test -p ferroplan --test ipc_sweep -- --ignored
```

## Corpus 7 — scaling ladder: 2 families × 6 rungs, n up to 128

Source: [`crates/ferroplan/tests/fixtures/scaling-ladder/RESULTS.md`](../crates/ferroplan/tests/fixtures/scaling-ladder/RESULTS.md)
(ticket `fond-htn-28`; machine-written by `crates/ferroplan/tests/scaling_ladder.rs`,
seed 20260917, max RSS ~93 MiB, two runs byte-identical in structural
counts, walls within ~±3%). Stage bounds: 60 s per stage, 2 M states.

| family | rungs | refused | solve wall at n=128 | solve share of rung wall |
|---|---|---|---|---|
| chain-world | 4→128 (6) | 0 | 1626 ms | 76% |
| transport-drop | 4→128 (6) | 0 | 14 668 ms | 93% |

**Zero refusals through n=128 — the ladder did not stop.** Measured
shape: ground linear in n (transport; quadratic for chain's n²+n
schemas), translate linear in composite states with walls ≈×5 per
doubling past n≈32, solve ≈×7.7–7.9 per doubling (≈ cubic). Projected
capacity knee = the solve fixpoint: the 60 s solve bound first bites at
≈n 200–210 (transport-drop) / ≈n 430 (chain-world) — projections from
measured doubling factors, not observed refusals.

Reproduce:

```sh
cargo test -p ferroplan --test scaling_ladder -- --ignored --nocapture
```

## Corpus 8 — FOND criterion bench: seeded micro-domain baselines

Source: [`crates/ferroplan/benches/BENCH-FOND.md`](../crates/ferroplan/benches/BENCH-FOND.md)
(ticket `fond-htn-26`; `cargo bench -p ferroplan --bench fond -- --quick`,
10 samples per benchmark, seed `0x5EED_2026_0917` — identical instances
replay from the seed; Apple M3 Max, 2026-09-18T00:12Z). Means ± stddev,
release profile:

| benchmark | mean |
|---|---|
| strong_fixpoint_chain_10 / _50 / _200 | 9.13 µs / 396.6 µs / 15.53 ms |
| strong_fixpoint_lattice_10 / _50 / _200 | 6.98 µs / 114.5 µs / 2.26 ms |
| strong_cyclic_mixed_ladder_10 / _50 / _200 | 22.2 µs / 698.0 µs / 22.40 ms |
| solve_planning_type_dispatch_tiny_chain (dispatch floor) | 899 ns |

Strong-cyclic walls include the preceding strong-fixpoint NoPlan pass
(production dispatch is strong first, strong-cyclic on `NoPlan`).
Structural + wall tripwires pin these shapes in
[`crates/ferroplan/tests/fond_threshold.rs`](../crates/ferroplan/tests/fond_threshold.rs)
(release bound 50 ms / dev 1200 ms, both green at the recording commit
`17b8cc7`). Per-fixture `solve_hddl` micro walls (7 fixtures, parse-
dominated, solve-only ≈ 33–47 µs) are in the source file.

Reproduce:

```sh
cargo bench -p ferroplan --bench fond -- --quick   # matches the table
cargo test -p ferroplan --test fond_threshold      # CI tripwires
```

## How to read a verdict

Verdict classes appearing in the committed sources, and what each one
means (and does not mean):

- **SOLVED** — the engine returned a plan/policy. For FOND rows marked
  "closure-verified", the policy was additionally checked outcome-closed
  against the ground IR (every reachable non-goal state has a total
  entry; entry outcomes match the IR transition relation; every
  policy-following execution reaches a goal). A SOLVED from the oracle
  in "flexible" mode is a *strong* plan (its strong-cyclic support is
  preliminary).
- **NOSOLUTION** — search exhausted, no plan exists. A successful,
  typed verdict (ferroplan: `PlannerError::NoPlan`) — never an error.
  Under the oracle, only trusted when both `flexible` and `fixed-ld`
  agree.
- **TIMEOUT** — wall clock exhausted while still searching. **Not**
  unsolvability evidence; the run was killed mid-expansion.
- **PARSE_ERROR** — the input was rejected at the parse stage (loud,
  typed refusal, no panic). Recorded as `UNSUPPORTED` in the harvest
  where the case is a negative-corpus entry rather than a run.
- **ERROR** — oracle harness/pipeline failure before or outside search
  (e.g. serializer crash). Neither solved nor unsolvable: an *open
  verdict*; ferroplan-side rows are judged only as "bounded, typed
  exit".
- **LIMIT:\*** — a typed internal limit refused before a verdict:
  `LIMIT:translate-wall` = the translate phase's own 10 000 ms wall
  (`TranslateLimits::default()`), `LIMIT:translate` (corpus 1's
  `TRANSLATE_ERROR` rows) = the same limit seen as a typed
  `TRANSLATE_ERROR`. Honest, loud refusals — not wrong answers.
- **GAP:\*** — a capability gap, honestly refused:
  `GAP:conditional-effects` (action-level nested `when` out of scope),
  `GAP:method-effect-grounding` (zero-action method-`:effect` domains,
  unbound variable). Loud, typed, ticketed.
- **UNSUPPORTED** — a recorded negative case kept as an anti-example
  (AssemblyHierarchical silent mis-compile), not a corpus instance
  verdict.

Rule of thumb used across every corpus: **verdicts are classes, walls
are context.** Wall clocks vary run to run; the stable, asserted facts
are the outcome classes and (for SOLVED rows) policy closure.

## Provenance and policy

- Every number above is copied from the cited committed file; no
  recomputation, no smoothing, no cherry-picking (refusals and
  divergences are reported in full).
- The external oracle's code is never vendored; only its verdicts (JSON
  fact records) are committed, per the KOALA policy in
  [FOND-HTN.md § 4](FOND-HTN.md#4-oracle-methodology).
- IPC/competition fixture files are canonical competition data;
  hand-authored fixtures carry provenance comments in their
  `verdict.json`/headers.
- Wave-4 corpus rows 6–8 (`ipc-sweep/`, `scaling-ladder/`,
  `benches/BENCH-FOND.md`) were appended at wave-4 integration
  (2026-09-18) from their committed RESULTS files, with their own repro
  commands. Earlier rows were consolidated at wave-3 close.
