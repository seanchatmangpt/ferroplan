---
id: fond-htn-43-ground-caps-plumbing
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: expose grounding caps through solve_hddl; re-run the 17 cap-refused IPC domains"
standing: BLOCKED
branch: fix/ground-caps-plumbing
worktree: ~/ferroplan-worktrees/wt-a43
created: 2026-09-18T00:55:00Z
source: fond-htn-27 sweep (ground-actions ×14, ground-methods ×3)
---

Read `_WAVE4-CONTEXT.md`. Ticket 23's plumbing pattern (translate side) is the precedent; do the grounding side. NOTE: parallel branches (tickets 23/24) also touch hddl.rs plumbing — confine your diff to the grounding-limit plumbing line(s) and the new test; expect the coordinator to union.

Scope:
1. Plumb caller limits into grounding: `solve_hddl` (and `solve_hddl_from_eve` if it shares the inner path) should derive `GroundingLimits` from `PlannerLimits` (wall from `max_wall_ms`; action/method caps scaled from `max_states` or new optional fields — choose the least-API-noise option and document it).
2. Re-run the 17 cap-refused IPC-2023 domains from `tests/fixtures/ipc-sweep/` with the raised caps (ground caps high, walls 60 s) — record which now solve/translate/refuse differently in `tests/fixtures/ipc-sweep/RESULTS-wavec.md` (append-only addendum; do not rewrite wave-4 rows).
3. Tests: a domain whose default ground cap refuses but solves under raised caps via the public API (hand-authored, small); default behavior unchanged (existing suites green).

Gates: `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test htn_ipc2023` exit 0; addendum committed.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | fix/ground-caps-plumbing @ 6d14813 | — | all |
