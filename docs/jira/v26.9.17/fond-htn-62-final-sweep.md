---
id: fond-htn-62-final-sweep
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Final sweep: every test target on main incl. externals, wasm, benches tripwire — report only"
standing: BLOCKED
branch: test/final-sweep
worktree: ~/ferroplan-worktrees/wt-h62
created: 2026-09-18T02:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Report-only snapshot of main @ 75870de lineage (your worktree base); findings, not fixes.

Scope:
1. Run EVERY test target: `cargo test -p ferroplan -p ferroplan-hddl -- --include-ignored` (externals skip-with-note where /tmp corpora absent — count them), `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib`, `cargo test -p ferroplan --test fond_threshold --release`, `cargo bench -p ferroplan --bench fond -- --quick` (sanity, not perf), `python3 scripts/verify_evidence_ids.py`, `cargo test -p ferroplan --doc`, plus every wave-added target you find in Cargo.toml [[test]] entries.
2. Tally table committed as `docs/jira/v26.9.17/_WAVE6-SWEEP.md`: per target — passed/failed/ignored(+reason classes), wall, exit code. Failures recorded verbatim with test names (they are findings for the coordinator — likely none if branches are green).
3. Note main's HEAD at your run time and any drift you observe mid-run (rider merges) — rerun affected targets once if drift detected.

Gates: your sweep completed with zero UNEXPECTED failures (expected: documented ignores, external skips); _WAVE6-SWEEP.md committed.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:50:00Z | BLOCKED | test/final-sweep @ 75870de | — | all |
