---
id: fond-htn-65-translate-plumbing
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix (3rd attempt, SPLIT SCOPE): plumb caller walls into TranslateLimits only"
standing: BLOCKED
branch: fix/translate-plumbing
worktree: ~/ferroplan-worktrees/wt-a23 (reused, reset to 75870de)
created: 2026-09-18T02:50:00Z
supersedes: fond-htn-23 scope (plumbing half only; profiling dropped)
---

Read `_WAVE4-CONTEXT.md` + ticket 23's History (two prior agents died; the scope was too wide — this ticket is deliberately SMALL).

Scope (nothing else):
1. `crates/ferroplan/src/hddl.rs`: derive `TranslateLimits` from `PlannerLimits` in `solve_hddl_inner` — wall from `max_wall_ms` (0 → keep default 10 s? NO: 0 means unbounded per PlannerLimits docs — mirror ticket 43's mapping exactly: 0 → None/unbounded, else the caller's value; state-budget field from `max_states` via the same `MAX_STATES_PER_GROUND_INSTANCE`-style calibration ticket 43 used, so defaults reproduce today's envelope EXACTLY).
2. One test: default behavior byte-identical (existing suites); raised `max_wall_ms` lifts the translate wall end-to-end on a hand-authored domain that needs >10 s (or reuse a committed slow fixture; keep the test `#[ignore]`d if it needs >30 s wall).
3. Re-run the 9 wave-4 `LIMIT:translate-wall` IPC instances (read-only /tmp corpus) at 60 s caller walls; append outcomes to `tests/fixtures/ipc-sweep/RESULTS-wavec.md` (append-only addendum — file exists from ticket 43).

Explicitly OUT of scope: profiling, frontier canonicalization changes, capacity work (ticket 60 owns the leverage side).

Gates: `cargo test -p ferroplan -p ferroplan-hddl` exit 0; addendum rows added.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:50:00Z | BLOCKED | fix/translate-plumbing @ 75870de | — | all |
| 2026-09-18T23:30:00Z | ALIVE | merged 6757249 into wave6/land-v26917 (merge f980212, A7) | gates re-run on the FULL merged line: `cargo test -p ferroplan -p ferroplan-hddl` 85 suites, 776 passed / 0 failed / 47 documented ignores, exit 0 (includes translate_plumbing 1/1 + translate_wall_ipc_addendum runner green, ~9 min) | none (A7) |
