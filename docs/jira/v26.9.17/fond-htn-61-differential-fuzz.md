---
id: fond-htn-61-differential-fuzz
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Differential fuzz: generator's VALID draws through BOTH engines, divergences ledgered"
standing: BLOCKED
branch: test/differential-fuzz
worktree: ~/ferroplan-worktrees/wt-h61
created: 2026-09-18T02:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Combines the two strongest walls: the committed fuzz generator (`crates/ferroplan/tests/hddl_fuzz_roundtrip.rs`, on main) and the koala oracle (concepts only; runner in /tmp).

Scope:
1. New `crates/ferroplan/tests/differential_fuzz.rs` + an `#[ignore]`d external driver: generate 100 VALID small HDDL domain+problem pairs by REUSING the committed generator's machinery (import/copy its public-ish helper — if it is test-private, extract the minimal generator into a shared `tests/common/` module in a separate commit; do not duplicate 1000 lines).
2. For each pair: ferroplan `solve_hddl` (10 s wall) records SOLVED / NOSOLUTION / typed-limit; koala oracle (`/tmp/fond-oracle/oracle-run.sh --mode flexible --timeout 30`) records its verdict (skip-with-note per pair if harness absent).
3. Classification: agreement / divergence-solvable-vs-not (the interesting class) / incomparable (either side TIMEOUT or typed-limit — expected: koala cannot decide cyclic-only domains). Commit the 100-pair ledger as `tests/fixtures/differential-fuzz/ledger.json` + a summary table in the ticket History.
4. Any divergence where BOTH engines consumed the input cleanly: minimized + `#[ignore]`d reproducer (finding), never papered over.

Gates: `cargo test -p ferroplan --test differential_fuzz` exit 0 (offline mode: ferroplan-only classification, oracle leg skipped-with-note when /tmp absent); the `-- --ignored` full run exit 0 on this machine with ≥80 oracle-consumable pairs.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:50:00Z | BLOCKED | test/differential-fuzz @ 75870de | — | all |
