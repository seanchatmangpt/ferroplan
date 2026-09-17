# IPC-2023 deterministic HTN pipeline suite — RESULTS

Ticket: `fond-htn-13-test-htn-ipc2023` · branch `test/htn-ipc2023` · 2026-09-17.

Provenance: fixtures are verbatim copies of the 13 curated instances from
`/tmp/fond-corpus/htn-ipc2023/` (IPC 2023 hierarchical-track competition data —
canonical inputs, not koala code; copying admitted by the wave context).
Runner: `ferroplan::hddl::solve_hddl` end-to-end (parse → ground → translate →
FOND `fond_policy`/`fond_policy_strong_cyclic`), 30 s wall budget per instance.
For every SOLVED row the in-test checker verified policy closure against the
independently re-derived ground IR (every reachable non-goal state has an
entry; entry outcomes exactly match the IR transition relation) and replayed
any reported steps as real IR edges.

Command (gate): `cargo test -p ferroplan --test htn_ipc2023` → exit 0,
13 passed / 0 failed. Measured run: `-- --test-threads=1 --nocapture`,
13 passed in 43.9 s (walls below are from that run; stable across both runs).

| instance | parse | ground | translate | outcome | wall (ms) | plan (policy entries / steps) | SLOW (>10 s) |
|---|---|---|---|---|---|---|---|
| Blocksworld-GTOHP | ok | ok | ok | SOLVED | 18 | 54 / 0 | |
| Blocksworld-HPDDL | ok | ok | ok | SOLVED | 97 | 136 / 0 | |
| Depots | ok | ok | ok | SOLVED | 905 | 81 / 0 | |
| Factories-simple | ok | ok | ok | SOLVED | 58 | 66 / 0 | |
| Lamps | ok | ok | ok | SOLVED | 2 | 24 / 0 | |
| Multiarm-Blocksworld | ok | ok | ok | SOLVED | 1081 | 148 / 0 | |
| PCP_1 | ok | ok | TIMEOUT @ 10 s | LIMIT:translate-wall | 10287 | — | SLOW |
| PO_Satellite | ok | ok | ok | SOLVED | 4 | 8 / 0 | |
| PO_Transport | ok | ok | TIMEOUT @ 10 s | LIMIT:translate-wall | 10412 | — | SLOW |
| Robot | ok | ok | ok | SOLVED | 3 | 9 / 0 | |
| Satellite-GTOHP | ok | ok | TIMEOUT @ 10 s | LIMIT:translate-wall | 10365 | — | SLOW |
| Towers | ok | ok | ok | SOLVED | 9 | 6 / 0 | |
| Transport | ok | ok | TIMEOUT @ 10 s | LIMIT:translate-wall | 10468 | — | SLOW |

Notes on the table: "plan" is a policy (`UniversalPlan.policy`); the FOND
policy path reports no linear `steps` (0 for every solved instance), so
policy-entry count is the plan-size measure. Solved plans' policies were all
closed per the checker — zero contract violations anywhere.

## First-failure errors, verbatim (the 4 LIMIT rows)

All four are the same typed, honest limit — the translate phase's own internal
wall clock (`TranslateLimits::default()` = 10 000 ms), not the suite's 30 s
budget:

```
HDDL translation error: translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms
```

- PCP_1 — expected class (compat-watch #8: recursive Post-Correspondence
  method cycles, unbounded decomposition depth; the ticket pre-admits a bounded
  exit as the assertion).
- PO_Transport, Satellite-GTOHP, Transport — NOT obviously expected: small
  instances (9–13 objects, 6–10 methods) that nonetheless exceed 10 s of
  translate-phase work. Suspected translate-phase blowup on method-
  precondition/partial-order-heavy decompositions; measured here per the
  ticket ("CHANGELOG already admits IPC-2020-style blowups — measure ours").
  The binding constraint is `solve_hddl`'s hardcoded `TranslateLimits::default()`
  (10 s), which a caller's 30 s budget cannot raise.

## Parser gaps

None. All 13 domains and 13 problems parsed cleanly — every corpus
compat-watch quirk was accepted by `ferroplan-hddl` as-is: `:ordered-subtasks`
(10 domains), `:tasks` inside problem `(:htn …)` (Lamps), `.pddl` problem
suffix (Lamps), empty `(and)` subtask lists, hyphenated predicate names
(Factories-simple), `:constraints` incl. `not(= ..)` (PO_Satellite), undeclared
objects `Y`/`N` with negated-init goal (Lamps), zero-object problem (PCP_1).
No parser source was modified on this branch.

## Outcome classes

9 SOLVED (closed-policy verified) · 4 LIMIT (typed translate-wall) ·
0 NoPlan · 0 parse/ground gaps · 0 panics · 0 hangs · 0 garbage results.
