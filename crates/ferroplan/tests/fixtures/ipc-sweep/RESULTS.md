# IPC-2023 full sweep — `solve_hddl` staged (43 domains)

Ticket `fond-htn-27-bench-ipc-full-sweep`, wave `v26.9.17-fond-htn-hardening`.

- machine:  (`sysctl -n machdep.cpu.brand_string`)
- date: 2026-09-28T20:29:06Z (UTC)
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
| assemblyhierarchical | 9 | 146 | 1273 | 0 | 4 | 60 | 569 | 633 | SOLVED | 0 | 99 |  |
| barman_bdi | 13 | 298 | 363 | 0 | 1 | 54 | 646 | 701 | SOLVED | 0 | 2447 |  |
| blocksworld_gtohp | 5 | 61 | 260 | 0 | 0 | 0 | 0 | 0 | SOLVED | 0 | 54 |  |
| blocksworld_hpddl | 5 | 90 | 156 | 0 | 0 | 1 | 7 | 8 | SOLVED | 0 | 136 |  |
| depots | 13 | 271 | 1885 | 0 | 6 | 10 | 60 | 76 | SOLVED | 0 | 81 |  |
| factories_simple | 9 | 243 | 372 | 0 | 1 | 3 | 3 | 7 | SOLVED | 0 | 66 |  |
| freecell_learned_ecai_16 | 30 | - | - | 3 | 63 | - | - | 66 | LIMIT:ground-actions | - | - |  |
| hiking | 19 | - | - | 0 | 152 | - | - | 152 | LIMIT:ground-methods | - | - |  |
| lamps | 1 | 4 | 38 | 0 | 0 | 0 | 0 | 0 | SOLVED | 0 | 14 |  |
| logistics_learned_ecai_16 | 15 | 428 | 1708 | 0 | 3 | 17 | 279 | 299 | SOLVED | 0 | 402 |  |
| minecraft_player | 87 | - | - | 4 | 79 | - | - | 83 | LIMIT:ground-methods | - | - |  |
| minecraft_regular | 87 | - | - | 0 | 72 | - | - | 72 | LIMIT:ground-methods | - | - |  |
| monroe_fo_1 | 86 | - | - | 1 | 47 | - | - | 48 | LIMIT:ground-actions | - | - |  |
| monroe_fo_19 | 78 | - | - | 1 | 38 | - | - | 39 | LIMIT:ground-actions | - | - |  |
| monroe_fo_2 | 78 | - | - | 2 | 29 | - | - | 31 | LIMIT:ground-actions | - | - |  |
| monroe_fo_20 | 78 | - | - | 1 | 29 | - | - | 30 | LIMIT:ground-actions | - | - |  |
| monroe_po_1 | 81 | - | - | 1 | 27 | - | - | 28 | LIMIT:ground-actions | - | - |  |
| monroe_po_19 | 80 | - | - | 1 | 27 | - | - | 28 | LIMIT:ground-actions | - | - |  |
| monroe_po_2 | 82 | - | - | 1 | 41 | - | - | 42 | LIMIT:ground-actions | - | - |  |
| monroe_po_20 | 78 | - | - | 1 | 47 | - | - | 48 | LIMIT:ground-actions | - | - |  |
| multiarm_blocksworld | 6 | 95 | 176 | 0 | 0 | 7 | 75 | 82 | SOLVED | 0 | 148 |  |
| pcp_1 | 0 | 11 | 12 | 0 | 0 | 11586 | - | 11586 | LIMIT:translate-wall | - | - | SLOW |
| pcp_16 | 0 | 11 | 12 | 0 | 0 | 11243 | - | 11243 | LIMIT:translate-states | - | - | SLOW |
| pcp_17 | 0 | 11 | 12 | 0 | 0 | 9539 | - | 9539 | LIMIT:translate-states | - | - |  |
| pcp_2 | 0 | 13 | 16 | 0 | 0 | 9214 | - | 9214 | LIMIT:translate-states | - | - |  |
| po_barman_bdi | 13 | 298 | 363 | 0 | 2 | 53 | 537 | 592 | SOLVED | 0 | 2447 |  |
| po_colouring | 7 | 3961 | 6339 | 0 | 26 | 11829 | - | 11855 | LIMIT:translate-states | - | - | SLOW |
| po_monroe_po_1 | 86 | - | - | 1 | 34 | - | - | 35 | LIMIT:ground-actions | - | - |  |
| po_monroe_po_2 | 81 | - | - | 1 | 23 | - | - | 24 | LIMIT:ground-actions | - | - |  |
| po_monroe_po_24 | 77 | - | - | 1 | 27 | - | - | 28 | LIMIT:ground-actions | - | - |  |
| po_monroe_po_25 | 78 | - | - | 1 | 34 | - | - | 35 | LIMIT:ground-actions | - | - |  |
| po_rover | 13 | 289 | 366 | 0 | 1 | 13455 | - | 13456 | LIMIT:translate-wall | - | - | SLOW |
| po_satellite | 6 | 14 | 22 | 0 | 0 | 0 | 0 | 0 | SOLVED | 0 | 8 |  |
| po_transport | 8 | 60 | 87 | 0 | 0 | 6921 | - | 6921 | LIMIT:translate-states | - | - |  |
| po_um_translog | 15 | 911 | 2092 | 1 | 8 | 181 | 5738 | 5928 | SOLVED | 0 | 47 |  |
| po_woodworking | 18 | - | - | 0 | 43 | - | - | 43 | LIMIT:ground-actions | - | - |  |
| robot | 4 | 12 | 21 | 0 | 0 | 0 | 0 | 0 | SOLVED | 0 | 9 |  |
| rover_gtohp | 14 | 354 | 446 | 0 | 1 | 6 | 11 | 18 | SOLVED | 0 | 345 |  |
| satellite_gtohp | 12 | 80 | 114 | 0 | 0 | 6884 | - | 6884 | LIMIT:translate-states | - | - |  |
| snake | 466 | - | - | 1 | 36 | - | - | 37 | LIMIT:ground-actions | - | - |  |
| towers | 4 | 144 | 495 | 0 | 1 | 0 | 0 | 1 | SOLVED | 0 | 6 |  |
| transport | 8 | 60 | 87 | 0 | 0 | 7657 | - | 7657 | LIMIT:translate-states | - | - |  |
| woodworking | 17 | - | - | 0 | 62 | - | - | 62 | LIMIT:ground-actions | - | - |  |
| TOTALS (43) | | | | 22 | 964 | 88720 | 7925 | 97631 | 15/43 SOLVED, 0 NOPLAN, 28 LIMIT:*, 0 GAP:* | | | |

## Coverage (honest)

End-to-end solved: **15/43**.

Refused by limits: **28** — by kind: LIMIT:ground-actions ×16, LIMIT:ground-methods ×3, LIMIT:translate-wall ×2, LIMIT:translate-states ×7

Pipeline gaps: **0** — verbatim first error each:


### Limit refusals, verbatim

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
- `pcp_1` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10001ms elapsed, limit 10000ms`
- `pcp_16` (LIMIT:translate-states): `translate memory limit exceeded: 200001 states interned, limit 200000`
- `pcp_17` (LIMIT:translate-states): `translate memory limit exceeded: 200011 states interned, limit 200000`
- `pcp_2` (LIMIT:translate-states): `translate memory limit exceeded: 200000 states interned, limit 200000`
- `po_colouring` (LIMIT:translate-states): `translate memory limit exceeded: 200002 states interned, limit 200000`
- `po_monroe_po_1` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_monroe_po_2` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_monroe_po_24` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_monroe_po_25` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `po_rover` (LIMIT:translate-wall): `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms`
- `po_transport` (LIMIT:translate-states): `translate memory limit exceeded: 200005 states interned, limit 200000`
- `po_woodworking` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `satellite_gtohp` (LIMIT:translate-states): `translate memory limit exceeded: 200006 states interned, limit 200000`
- `snake` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
- `transport` (LIMIT:translate-states): `translate memory limit exceeded: 200000 states interned, limit 200000`
- `woodworking` (LIMIT:ground-actions): `grounding limit exceeded: max_ground_actions (10000) exceeded`
