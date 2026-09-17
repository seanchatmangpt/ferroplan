---
id: fond-htn-10-feat-eve-bridge
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Wire Eve's DecomposeHddl stage to solve_hddl + capability manifest coverage"
standing: BLOCKED
branch: feat/eve-hddl-bridge
worktree: ~/ferroplan-worktrees/wt-bridge
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only; no push.

## Scope
1. Bridge: public fn in `crates/ferroplan/src/hddl.rs` (e.g. `solve_hddl_from_eve(&EveHandoff, ...)`) consuming `handoff.hddl.{domain,problem,root_task}` → `solve_hddl` (read its real signature + tests first; adapt root-task format to what it expects). Deterministic AND Probabilistic regimes both solve the HDDL half (PPDDL half explicitly out of scope — doc it). Test: minimal EveHandoff via `Eve::enter` (construction idiom in `crates/ferroplan/tests/eve_genesis.rs`) carrying a micro HDDL domain (1 compound task → 1 method → 1 primitive) ⇒ solved.
2. Manifest: add `fp.core.hddl` and `fp.core.fond` entries to `capability_manifest()` in `readiness.rs`, evidence ids named after REAL test names (hddl.*, fond.*). Update the canonical count/non-fictional test in the same change. Update the Fond row in `docs/planning-types.md` to mention strong + strong-cyclic dispatch.
3. Do not touch planning_runtime.rs (owned by T07/T09 branches).

## Gates
`cd ~/ferroplan-worktrees/wt-bridge && cargo test -p ferroplan --lib readiness && cargo test -p ferroplan --lib hddl && cargo test -p ferroplan --test eve_genesis` exit 0, plus any suite you touched.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | feat/eve-hddl-bridge @ base d2faf4d | — | all |
