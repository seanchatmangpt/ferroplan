---
id: fond-htn-55-wasip1-clippy
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: the three pre-existing wasip1 clippy diagnostics in wasi_abi"
standing: BLOCKED
branch: fix/wasip1-clippy
worktree: ~/ferroplan-worktrees/wt-d55
created: 2026-09-18T00:55:00Z
source: wave-3 ticket 17 out-of-scope note
---

Read `_WAVE4-CONTEXT.md`. Wave-3 ticket 17 left 3 pre-existing clippy diagnostics on the wasip1 target (host lane is clean because the module is cfg-gated out): `fp_dealloc`/`fp_alloc` raw-pointer API lints and the unknown-op `Ok(?..)` arm shape.

Scope:
1. Reproduce: `cargo clippy -p ferroplan-wasm --target wasm32-wasip1 --all-targets -- -D warnings` → exit 101 with the 3 diagnostics (record verbatim).
2. Fix each properly: raw-pointer FFI surface → keep `unsafe` but satisfy clippy with targeted `#[allow(clippy::...)]` plus a SAFETY comment block per item (the lint's own suggestion), or refactor where safe; the unknown-op arm → explicit binding instead of `?..` fragments. No behavior change — the wasm ABI wire format is frozen (byte-compat is proven by the existing wasip1 tests).
3. Gate: the same clippy invocation exits 0; `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib` still green (byte-compat witness).
4. If a 4th diagnostic appears from wave-4-merged code once integrated, leave it — record for the coordinator (your branch is pre-integration base; only fix the three named).

Gates: clippy exit 0 on target; wasip1 tests exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | fix/wasip1-clippy @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | fix/wasip1-clippy @ 75870de (wt-d55) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
