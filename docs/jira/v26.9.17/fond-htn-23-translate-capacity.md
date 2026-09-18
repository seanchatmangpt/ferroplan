---
id: fond-htn-23-translate-capacity
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: translate composite-BFS capacity — 4 IPC instances die on the internal 10 s wall"
standing: BLOCKED
created: 2026-09-17T23:55:00Z
source: fond-htn-05 + fond-htn-13 (translate-wall mismatches)
---

Read `_FOND-HTN-WAVE-CONTEXT.md`. Finding: `solve_hddl`'s pipeline hard-codes `TranslateLimits::default()` (10 s wall, internal state caps). Four small IPC-2023 instances (PCP_1, PO_Transport, Satellite-GTOHP, Transport) exceed it while the koala oracle solves each in < 1.3 s; a caller's 30 s `max_wall_ms` cannot lift the internal cap (measured in `crates/ferroplan/tests/fixtures/htn-oracle/RESULTS.md` + `htn-ipc2023/RESULTS.md`, 14k–47k composite states still queued at the wall).

Scope: (a) plumb `PlannerLimits::max_wall_ms` (and a public composite-state budget) through `solve_hddl` into `TranslateLimits` so callers can raise it; (b) profile the composite-BFS frontier canonicalization for the blowup (the wave already landed frontier canonicalization for blocksworld — e24f63e lineage — find the next hot spot); (c) turn the four LIMIT:translate-wall rows into SLOW-but-solved or documented capacity rows. Honest refusal stays lawful: never silently truncate.

Gates: `cargo test -p ferroplan --test htn_oracle --test htn_ipc2023` exit 0; RESULTS.md rows updated with new walls.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:55:00Z | BLOCKED | none | — | all |
| 2026-09-18T00:05:00Z | PARTIAL_ALIVE | fix/translate-capacity@6d14813 (worktree wt-a23) | orient: ticket+wave ctx read; worktree verified | (a) plumb max_wall_ms+budget into TranslateLimits; (b) profile composite-BFS; (c) RESULTS.md rows; gates |
