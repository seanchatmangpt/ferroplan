# IPC-2023 full sweep — `solve_hddl` staged (43 domains)

Ticket `fond-htn-27-bench-ipc-full-sweep`, wave `v26.9.17-fond-htn-hardening`.

- machine: Apple M3 Max (`sysctl -n machdep.cpu.brand_string`)
- date: 2026-09-18T00:21:12Z (UTC)
- commands:
  - `cargo test -p ferroplan --test ipc_sweep` (sampled 6-domain heartbeat, exit 0)
  - `cargo test -p ferroplan --test ipc_sweep -- --ignored` (this full sweep, exit 0)
- corpus: all 43 IPC-2023 hierarchical-track domains, verbatim competition data (`domain.hddl` + first-listed problem each), vendored under `tests/fixtures/ipc-sweep/` (≈1.2 MB, under the 2 MB ticket cap).
- stage budgets (per-stage, the ticket's staged spec — NOT solve_hddl's cumulative watchdog):
  - parse: 10 s outer wall (the parser has no internal cap);
  - ground: `GroundingLimits::default()` — exactly what `solve_hddl_inner` passes (10 s wall per grounding phase; 10k ground actions/methods) — under a 60 s anti-hang watchdog; where an internal cap binds, its verbatim refusal is the recorded result;
  - translate: `TranslateLimits::default()` — exactly what `solve_hddl_inner` passes (10 s wall / 200k states / depth 64) — under a 60 s anti-hang watchdog. Ticket 23 was NOT landed on this branch, so the internal 10 s translate wall is the binding translate budget and `LIMIT:translate-wall` rows carry its verbatim `limit 10000ms` refusal, per the ticket;
  - solve: `adapt_problem` + `solve_planning_type(PlanningType::Fond)` (the exact `solve_hddl_inner` tail), `PlannerLimits::max_wall_ms = 30000`, under a 120 s anti-hang watchdog.
- `objects` = problem `:objects` entries; `ground actions`/`ground methods` = grounded IR instance counts (`GroundingLimits` caps apply to these); walls in milliseconds; `-` = stage skipped after an earlier refusal, or (in the count columns) a grounding that refused at a cap/validation before counts existed.
- Contract: no domain panicked, no garbage `Ok(solved=false)`, no hang past an anti-hang wall — the sweep test exits 0 iff all 43 outcomes are honest typed answers (SOLVED / NOPLAN / LIMIT / GAP).

| domain | objects | ground actions | ground methods | parse ms | ground ms | translate ms | solve ms | total ms | outcome | plan len | policy entries | flags |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| assemblyhierarchical | 9 | 146 | 1273 | 10 | 17 | 119 | 7613 | 7759 | SOLVED | 0 | 99 |  |
| barman_bdi | 13 | 298 | 363 | 1 | 10 | 1753 | 34823 | 36587 | LIMIT:solve-wall | - | - | SLOW |
| blocksworld_gtohp | 5 | 61 | 260 | 0 | 2 | 3 | 10 | 15 | SOLVED | 0 | 54 |  |
| blocksworld_hpddl | 5 | 90 | 156 | 0 | 1 | 11 | 83 | 95 | SOLVED | 0 | 136 |  |
| depots | 13 | 271 | 1885 | 0 | 26 | 82 | 788 | 896 | SOLVED | 0 | 81 |  |
| factories_simple | 9 | 243 | 372 | 0 | 5 | 12 | 39 | 56 | SOLVED | 0 | 66 |  |
| freecell_learned_ecai_16 | 30 | - | - | 12 | 149 | - | - | 161 | LIMIT:ground-actions | - | - |  |
| hiking | 19 | - | - | 1 | 519 | - | - | 520 | LIMIT:ground-methods | - | - |  |
| lamps | 1 | 4 | 38 | 0 | 0 | 0 | 0 | 0 | SOLVED | 0 | 24 |  |
| logistics_learned_ecai_16 | 15 | 428 | 1708 | 2 | 17 | 71 | 3788 | 3878 | SOLVED | 0 | 402 |  |
| minecraft_player | 87 | - | - | 15 | 303 | - | - | 318 | LIMIT:ground-methods | - | - |  |
| minecraft_regular | 87 | - | - | 1 | 253 | - | - | 254 | LIMIT:ground-methods | - | - |  |
| monroe_fo_1 | 86 | - | - | 4 | 119 | - | - | 123 | LIMIT:ground-actions | - | - |  |
| monroe_fo_19 | 78 | - | - | 5 | 121 | - | - | 126 | LIMIT:ground-actions | - | - |  |
| monroe_fo_2 | 78 | - | - | 5 | 120 | - | - | 125 | LIMIT:ground-actions | - | - |  |
| monroe_fo_20 | 78 | - | - | 5 | 120 | - | - | 125 | LIMIT:ground-actions | - | - |  |
| monroe_po_1 | 81 | - | - | 5 | 119 | - | - | 124 | LIMIT:ground-actions | - | - |  |
| monroe_po_19 | 80 | - | - | 5 | 118 | - | - | 123 | LIMIT:ground-actions | - | - |  |
| monroe_po_2 | 82 | - | - | 5 | 118 | - | - | 123 | LIMIT:ground-actions | - | - |  |
| monroe_po_20 | 78 | - | - | 5 | 117 | - | - | 122 | LIMIT:ground-actions | - | - |  |
| multiarm_blocksworld | 6 | 95 | 176 | 0 | 2 | 34 | 1044 | 1080 | SOLVED | 0 | 148 |  |
| pcp_1 | 0 | 11 | 12 | 0 | 0 | 10282 | - | 10282 | LIMIT:translate-wall | - | - | SLOW |
| pcp_16 | 0 | 11 | 12 | 1 | 0 | 10328 | - | 10329 | LIMIT:translate-wall | - | - | SLOW |
| pcp_17 | 0 | 11 | 12 | 1 | 1 | 10272 | - | 10274 | LIMIT:translate-wall | - | - | SLOW |
| pcp_2 | 0 | 13 | 16 | 1 | 0 | 10310 | - | 10311 | LIMIT:translate-wall | - | - | SLOW |
| po_barman_bdi | 13 | 298 | 363 | 2 | 10 | 1737 | 33678 | 35427 | LIMIT:solve-wall | - | - | SLOW |
| po_colouring | 7 | 3961 | 6339 | 0 | 134 | 10253 | - | 10387 | LIMIT:translate-wall | - | - | SLOW |
| po_monroe_po_1 | 86 | - | - | 5 | 118 | - | - | 123 | LIMIT:ground-actions | - | - |  |
| po_monroe_po_2 | 81 | - | - | 5 | 117 | - | - | 122 | LIMIT:ground-actions | - | - |  |
| po_monroe_po_24 | 77 | - | - | 5 | 116 | - | - | 121 | LIMIT:ground-actions | - | - |  |
| po_monroe_po_25 | 78 | - | - | 5 | 116 | - | - | 121 | LIMIT:ground-actions | - | - |  |
| po_rover | 13 | 289 | 366 | 1 | 8 | 10398 | - | 10407 | LIMIT:translate-wall | - | - | SLOW |
| po_satellite | 6 | 14 | 22 | 1 | 1 | 1 | 1 | 4 | SOLVED | 0 | 8 |  |
| po_transport | 8 | 60 | 87 | 0 | 1 | 10384 | - | 10385 | LIMIT:translate-wall | - | - | SLOW |
| po_um_translog | 15 | - | - | 5 | 0 | - | - | 5 | GAP:ground | - | - |  |
| po_woodworking | 18 | - | - | 3 | 0 | - | - | 3 | GAP:ground | - | - |  |
| robot | 4 | 12 | 21 | 0 | 0 | 1 | 0 | 1 | SOLVED | 0 | 9 |  |
| rover_gtohp | 14 | 354 | 446 | 1 | 9 | 39 | 97 | 146 | SOLVED | 0 | 345 |  |
| satellite_gtohp | 12 | 80 | 114 | 0 | 1 | 10367 | - | 10368 | LIMIT:translate-wall | - | - | SLOW |
| snake | 466 | - | - | 5 | 117 | - | - | 122 | LIMIT:ground-actions | - | - |  |
| towers | 4 | 144 | 495 | 0 | 6 | 1 | 0 | 7 | SOLVED | 0 | 6 |  |
| transport | 8 | 60 | 87 | 0 | 1 | 10514 | - | 10515 | LIMIT:translate-wall | - | - | SLOW |
| woodworking | 17 | - | - | 4 | 0 | - | - | 4 | GAP:ground | - | - |  |
| TOTALS (43) | | | | 126 | 3012 | 96972 | 81964 | 182074 | 12/43 SOLVED, 0 NOPLAN, 28 LIMIT:*, 3 GAP:* | | | |

## Coverage (honest)

End-to-end solved: **12/43**.

Refused by limits: **28** — by kind: LIMIT:solve-wall ×2, LIMIT:ground-actions ×14, LIMIT:ground-methods ×3, LIMIT:translate-wall ×9

Pipeline gaps: **3** — verbatim first error each:

- `po_um_translog` (GAP:ground): `validation error: type 'Regular_Truck' is declared more than once`
- `po_woodworking` (GAP:ground): `validation error: root task-network subtask 'task0' has a variable argument; the initial task network must be ground`
- `woodworking` (GAP:ground): `validation error: root task-network subtask 'task0' has a variable argument; the initial task network must be ground`

### Limit refusals, verbatim

- `barman_bdi` (LIMIT:solve-wall): `Timeout { elapsed_ms: 34749, limit_ms: 30000 }`
- `freecell_learned_ecai_16` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `hiking` (LIMIT:ground-methods): `grounding limit exceeded: max_ground_methods (10000) exceeded`
- `minecraft_player` (LIMIT:ground-methods): `grounding limit exceeded: max_ground_methods (10000) exceeded`
- `minecraft_regular` (LIMIT:ground-methods): `grounding limit exceeded: max_ground_methods (10000) exceeded`
- `monroe_fo_1` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `monroe_fo_19` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `monroe_fo_2` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `monroe_fo_20` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `monroe_po_1` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `monroe_po_19` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `monroe_po_2` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `monroe_po_20` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `pcp_1` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `pcp_16` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `pcp_17` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `pcp_2` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `po_barman_bdi` (LIMIT:solve-wall): `Timeout { elapsed_ms: 33636, limit_ms: 30000 }`
- `po_colouring` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `po_monroe_po_1` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_monroe_po_2` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_monroe_po_24` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_monroe_po_25` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_rover` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `po_transport` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `satellite_gtohp` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `snake` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `transport` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
