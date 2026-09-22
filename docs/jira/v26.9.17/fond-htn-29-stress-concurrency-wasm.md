---
id: fond-htn-29-stress-concurrency-wasm
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Stress: 40-thread mixed-workload concurrency through the wasm ABI"
standing: ALIVE
branch: stress/concurrency-wasm
worktree: ~/ferroplan-worktrees/wt-s29
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Precedent: `solve_hddl_produces_correct_independent_results_under_concurrent_calls` (40 threads, library level) and the wasip1 test idiom (wave-3 ticket 17).

Scope:
1. Concurrency stress at the ABI level: N=40 worker threads each driving a SEPARATE wasmtime instance (or the established dispatch harness — read `crates/ferroplan-wasm/src/wasi_abi.rs` tests for the pattern; if one engine per thread is impossible under wasip1, stress the host-side dispatch in parallel and document why) with a MIXED workload: `hddl_solve` (micro Transport-pattern, drop-retry), `fond_policy` (retry-loop + dead-end variant expecting NoPlan), `htn_plan` (single-method chain), interleaved round-robin, 100 iterations per thread.
2. Invariants: every result matches the single-threaded golden for the same input (byte-stable policy JSON); no cross-contamination between threads; no deadlock (test wall ≤ 120 s); RSS ceiling noted.
3. Adversarial interleave: half the threads feed malformed inputs (typed errors) WHILE the other half solve — error traffic must not corrupt adjacent successes.
4. Findings (deadlock, contamination, flakiness) → `#[ignore]`d reproducer + History row; never a silent skip.

Gates: `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib` exit 0 run 3× (flakiness falsifier); host `cargo test -p ferroplan-wasm` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | stress/concurrency-wasm @ 90c2ae2 | — | all |
| 2026-09-17T23:54:23Z | PARTIAL_ALIVE | stress/concurrency-wasm @ 6d14813 | — | implement 40-thread mixed-workload dispatch stress + adversarial error interleave; wasm gate 3x + host gate; RSS note |
| 2026-09-18T00:07:33Z | PARTIAL_ALIVE | stress/concurrency-wasm @ aae68be | found wasm gate BUILD_BROKEN at base (956e545 added HddlError::RootTaskMismatch without updating wasi_abi::hddl_error_json); repaired with distinct FP_HDDL_ROOT_MISMATCH code | stress tests + host shim committed; gates next |
| 2026-09-18T00:07:33Z | ALIVE | stress/concurrency-wasm @ aae68be | wasip1 --lib x3 exit0 (9 passed each, RSS ~153MB wasmtime peak); host cargo test -p ferroplan-wasm exit0 (11 passed, RSS ~188MB); stress target x3 exit0, wall 0.13-0.15s/run, 40x100 mixed round-robin + 20/20 adversarial interleave all byte-match pre-spawn goldens; no deadlock/contamination/flake findings | none in scope; one-engine-per-thread impossible under wasip1 (no OS threads) — host-side dispatch stressed per ticket fallback, rationale in tests::concurrency_stress doc |
| 2026-09-18T01:36:00Z | ALIVE | stress/concurrency-wasm@aae68be | — | integration pending |
