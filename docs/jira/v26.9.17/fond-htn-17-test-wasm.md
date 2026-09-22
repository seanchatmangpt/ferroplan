---
id: fond-htn-17-test-wasm
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "WASM ABI: hddl_solve / fond_policy / htn_plan ops coverage"
standing: ALIVE
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
| 2026-09-17T22:05:00Z | ALIVE | test/wasm-fond @ d2faf4d | oriented: wasi_abi.rs dispatch+tests read; browser infra = wasm_bindgen_test (commit 1ac9520, on origin/main only, NOT in this branch) run via `wasm-pack test --node`; branch gate = native dispatch_json tests | write tests, run gates |
| 2026-09-17T22:20:00Z | PARTIAL_ALIVE | test/wasm-fond @ 658ea7b | cargo check -p ferroplan-wasm --target wasm32-wasip1: was BUILD_BROKEN (E0004 hddl_error_json non-exhaustive, pre-existing; wasi_abi is cfg(wasm32-wasip1)-gated so host never compiles it) | cherry-picked remote fixes d493c19+29134d7 (linear pair rooted at our ancestor 3e1d27a) to unblock the crate's own test target; now writing tests |
| 2026-09-17T22:52:34Z | ALIVE | test/wasm-fond @ 172464e | gates: `cargo test -p ferroplan-wasm` (host) exit 0; `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib` exit 0, 9 passed/0 failed (4 pre-existing + 5 new, real execution in wasip1 guest); fmt --check exit 0; clippy(wasip1) exit 101 but all 3 diagnostics pre-existing production code (fp_dealloc/fp_call raw-ptr, unknown-op Ok(?) arm), none in new tests, CI clippy lane is host-only where module is cfg'd out | none for scope; note: wasip1 build was pre-broken (E0004) — unblocked via cherry-picks 47a898c+658ea7b mirroring remote fixes d493c19/29134d7 |
