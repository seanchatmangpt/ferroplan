---
id: fond-htn-49-surface-parity
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: surface-parity suite — same domain, three entry points, identical policies"
standing: CLOSED
branch: test/surface-parity
worktree: ~/ferroplan-worktrees/wt-d49
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. NOTE (base caveat): your branch base (6d14813) predates the wave-4 merges — `solve_hddl_from_eve` and the wasm `RootTaskMismatch` code exist at base from wave 3. Write the suite against BASE APIs; it must still compile post-integration (avoid asserting error-code enums that wave-4 branches extend; assert shapes, not exhaustive matches).

Scope:
1. `crates/ferroplan/tests/surface_parity.rs`: for THREE hand-authored micro domains (deterministic chain; oneof drop-retry; unsolvable both-branches-deadend — reuse wave-3 micro patterns, new files with provenance comments under `tests/fixtures/surface-parity/`): run each through (a) library `solve_hddl`, (b) `solve_hddl_from_eve` (construct EveHandoff via `Eve::enter`), (c) the wasm ABI `hddl_solve` op (host-side dispatch idiom from wasi_abi's tests; if the host shim from wave-4 ticket 29 is absent at base, use the established `dispatch_json` pattern in-module). For the unsolvable domain add (d) `solve_planning_type(PlanningType::Fond)` over an equivalent explicit PlanningProblem.
2. Invariants: solved-flag agreement across all surfaces; policy entry sets identical (state+action pairs, order-insensitive compare); outcome multisets identical; unsolvable → typed NoPlan on every surface (surface-specific error envelopes may differ — assert the inner Planner/NoPlan signal).
3. Failure → `#[ignore]`d + History finding (a parity break is a real defect, not a test problem).

Gates: `cargo test -p ferroplan --test surface_parity` exit 0; wasm leg via `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib` if you host the parity check in wasi_abi tests instead — pick ONE home and document it.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | test/surface-parity @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | test/surface-parity @ 75870de (wt-d49) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
| 2026-09-22T00:00:00Z | CLOSED | fer-07-respawns (merged to release/v26.9.22) | RETIRED, recorded as topology not silent pruning: the work was never started twice (wave-5 cut mid-flight, wave-6 respawn produced zero commits — worktree sat at 75870de); the v26.9.22 lane retires it rather than re-respawning a third time. Scope above stays the record of what a future cycle would owe. Worktree wt-d49 pruned (branch had 0 commits outside main). | none (re-open as a fresh ticket if the capability is still wanted) |
