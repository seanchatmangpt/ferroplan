---
id: fond-htn-29-stress-concurrency-wasm
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Stress: 40-thread mixed-workload concurrency through the wasm ABI"
standing: BLOCKED
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
