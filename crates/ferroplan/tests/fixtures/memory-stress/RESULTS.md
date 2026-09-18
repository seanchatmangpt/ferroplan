# Memory-ceiling stress results — ticket `fond-htn-30-stress-memory-ceilings`

Adversarial grounding/translation blowups under measured ceilings: every case
ends in a **typed refusal** (`ResourceBound`/`Timeout`-shaped family) or a
**clean solve within the ceiling** — never a signal, never an unbounded RSS
climb. All schemas are hand-authored, seeded, and generated in-test
(`crates/ferroplan/tests/memory_stress.rs`); no corpus files are involved
(KOALA POLICY: concepts only).

- Date (UTC): 2026-09-18
- Machine: Apple M3 Max, macOS 26.2 (`sysctl -n machdep.cpu.brand_string`)
- Build: debug (`cargo test -p ferroplan --test memory_stress` builds the
  dev profile; all numbers below are debug-build numbers)
- Worktree/branch: `~/ferroplan-worktrees/wt-s30` @ `stress/memory-ceilings`
- Seeds (per-case constants in the test file): `0xA11CE` (binding-product-8p),
  `0xB0B` (binding-ladder-2to8), `0xFAA7` (method-fanout-50x10), `0xDA7A`
  (fanout-dedup-50x10), `0xC0FFEE` (predicates-500), `0x7A1C`
  (typing-chain-300). The seed shuffles object/name insertion order via a
  SplitMix64 Fisher-Yates — outcomes are order-invariant by construction.

## Reproduce

```sh
# Gate 1 — always-on sampled subset (2 fastest measured cases):
cargo test -p ferroplan --test memory_stress
# Gate 2 — full 7-case suite:
cargo test -p ferroplan --test memory_stress -- --ignored
```

Both exits 0 on the committing machine (full-suite wall ~1.4 s against the
120 s wave cap; three consecutive full runs stable: 1478/1460/1458/1441 ms).

## Measurement method (documented per ticket)

- Each case runs as a **child process**: the test binary re-execs itself
  (`harness = false` own `main`) with `FERROPLAN_MEMORY_STRESS_CHILD=<case>`
  under a `sh -c 'ulimit -v 524288; exec "$1"'` wrapper — `RLIMIT_AS` 512 MiB
  as a hard backstop. macOS treats `RLIMIT_AS` as best-effort (not every
  allocation path is subject to it), so the rlimit is NOT the evidence.
- **Peak RSS is measured**, not guessed: the parent samples the child every
  ~10 ms via `ps -o rss=` (KiB units) and records the maximum. Resolution is
  one sampling interval; sub-interval spikes can be missed, but the
  committed count-based ceilings (`max_ground_actions`, `max_states`,
  `max_task_network_depth`) make sub-10 ms unbounded growth impossible —
  and the parent hard-fails any case whose measured peak exceeds the 512 MiB
  budget. (`mach_task_info` would need unsafe FFI; `ps` sampling is the
  ticket's documented portable fallback.)
- **Refusal/solve latency** is measured inside the child around the exact
  pipeline call (`Instant`), so it is the phase's own latency, excluding
  process startup. Parent wall (includes spawn + sampling) shown alongside.
- Ceilings configured per case (the ticket's "configure the caps" mandate):
  `GroundingLimits::max_ground_actions = 10_000` (binding cases),
  `TranslateLimits::max_states = Some(5_000)` (fan-out case),
  `TranslateLimits::max_task_network_depth = 5` (dedup probe),
  `PlannerLimits::max_wall_ms = 100` (planner case); everything else at
  crate defaults (ground 10k actions/methods + 10 s wall, translate 200k
  states + 10 s wall, planner 512 iterations / 100k states / 10 s wall).

## Peak-RSS table

Canonical run: gate 2 above (2026-09-18, commit recorded in the ticket
History row carrying these gates).

| case | family | outcome | phase | naive product | elapsed (ms) | peak RSS (KiB) | peak RSS (MiB) |
|---|---|---|---|---|---|---|---|
| binding-product-8p | binding-product explosion | refused | ground | 16777216 | 184 | 12256 | 12.0 |
| binding-ladder-2to8 | binding-product explosion | refused | ground | 19173952 | 56 | 11328 | 11.1 |
| method-fanout-50x10 | wide method fan-out | refused | translate | 97656250000000000 | 804 | 35744 | 34.9 |
| fanout-dedup-50x10 | wide method fan-out (dedup probe) | refused | translate | 97656250000000000 | 34 | 4400 | 4.3 |
| predicates-500 | predicate-heavy | solved | solve | 500 | 18 | 6256 | 6.1 |
| typing-chain-300 | deep-but-legal typing | solved | solve | 300 | 122 | 6848 | 6.7 |
| planner-wall-bound | planner ceiling | refused | planner | 1000 | 114 | 4128 | 4.0 |

## Refusal-latency table

| case | phase | typed outcome | latency (ms) |
|---|---|---|---|
| binding-product-8p | ground | grounding limit exceeded: max_ground_actions (10000) exceeded (end-to-end: HDDL grounding error: grounding limit exceeded: max_ground_actions (10000) exceeded) | 184 (parent wall 195 ms) |
| binding-ladder-2to8 | ground | grounding limit exceeded: max_ground_actions (10000) exceeded | 56 (parent wall 66 ms) |
| method-fanout-50x10 | translate | translate memory limit exceeded: 5026 states interned, limit 5000 | 804 (parent wall 816 ms) |
| fanout-dedup-50x10 | translate | task-network address 'r.r.s0.s0.s0.s0.s0' exceeded max_task_network_depth (5) | 34 (parent wall 42 ms) |
| predicates-500 | solve | solved | 18 (parent wall 27 ms) |
| typing-chain-300 | solve | solved | 122 (parent wall 130 ms) |
| planner-wall-bound | planner | Timeout { elapsed_ms: 113, limit_ms: 100 } | 114 (parent wall 123 ms) |

## Reading the numbers (no cherry-picking: full sweep, refusals included)

- **Naive product vs. actual work is the whole story.** The binding cases
  carry a *naive* binding product of 16.7M/19.2M, but the lazy
  `BindingIter` + per-binding cap check answers after ~10,001 *enumerated*
  bindings: 56–184 ms, ~11–12 MiB peak. An eager grounder would have had to
  materialize the full product (GBs) before refusing.
- **Distinguishable states are the fan-out's memory carrier.** 50 methods x
  10 subtasks has a naive decomposition product of 50^10 (~9.8e16). With a
  per-method `mark-i` effect (states content-distinguishable), the
  translator interns 5,026 states before `max_states = 5_000` refuses —
  0.8 s, ~35 MiB peak. With *identical* methods (probe), content dedup
  collapses the state count to ~1 per level (43 interned states in the
  first observation); the frontier *pending list* (x10 per level) is then
  the exponential carrier, and a tight `max_task_network_depth = 5` is the
  correct ceiling — 34 ms, ~4.3 MiB. Default depth (64) is unreachable
  before the 10 s wall fires; the ceiling must match the shape.
- **Both positive-arm cases solve cleanly** within purely default ceilings:
  500 predicates (18 ms, ~6 MiB) and a 300-deep legal typing chain
  (122 ms, ~6.7 MiB — the deepest object resolving at the root type proves
  the closure walks the full chain).
- **Planner ceiling:** a flat 1,000-state FOND chain (value iteration needs
  ~1,000 rounds) under `max_wall_ms = 100` refuses with typed
  `PlannerError::Timeout` at 113–115 ms — round cost (~10^6 group visits)
  is far above timer resolution, so the refusal is deterministic.
- **Sampled subset choice:** the 2 fastest measured cases are
  `predicates-500` (18 ms) and `fanout-dedup-50x10` (34 ms) — they are
  `SAMPLED_CASES` in the test file (always-on gate cost ~90 ms including
  process spawn). Re-measure before changing the set.
- Zero signals, zero panics, zero harness kills across every run performed
  in this session (including the failed first attempts at authoring time,
  whose failures were generator bugs — caught by these gates, fixed in the
  generator, never in the pipeline under test).
