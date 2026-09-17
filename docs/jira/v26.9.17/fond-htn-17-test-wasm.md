---
id: fond-htn-17-test-wasm
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "WASM ABI: hddl_solve / fond_policy / htn_plan ops coverage"
standing: BLOCKED
branch: test/wasm-fond
worktree: ~/ferroplan-worktrees/wt-test-wasm
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. NOTE: this branch's `hierarchical_plan` is still first-method (backtracking lands via frozen branch post-wave) — use single-method HTN fixtures only.

## Scope
1. Read `crates/ferroplan-wasm/src/wasi_abi.rs` (ops `hddl_solve`, `htn_plan`, `fond_policy`) + the crate's existing wasm test infra (recent commit "real wasm_bindgen_test browser coverage" — find the invocation the repo's CI uses; check `.github/workflows/` and the crate README) and mirror its calling idiom.
2. Add tests (in the established style/location): (a) `hddl_solve` on a hand-authored micro FOND-HTN domain (one 2-branch oneof with empty branch — Transport pattern, provenance comment) ⇒ solved + policy shape survives the ABI (outcomes count preserved); (b) `fond_policy` op on a retry-loop problem (self-loop + goal outcome) ⇒ solved, closed; on a dead-end variant ⇒ NoPlan-shaped result per the ABI's error convention; (c) `htn_plan` op on a single-method 2-level hierarchy ⇒ decomposition order matches; (d) ABI negative: malformed HDDL string ⇒ typed error, not trap/panic.
3. If the browser runtime is unavailable in this environment, run whatever headless variant the repo already uses; if none can run here, write the tests + document exact run command in History and mark standing PARTIAL_ALIVE with the reason.

## Gates
`cd ~/ferroplan-worktrees/wt-test-wasm && cargo test -p ferroplan-wasm` exit 0 (or documented PARTIAL_ALIVE with the runnable command + why it can't run here).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/wasm-fond @ base d2faf4d | — | all |
