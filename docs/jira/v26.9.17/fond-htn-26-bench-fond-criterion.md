---
id: fond-htn-26-bench-fond-criterion
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Bench: criterion suite for the FOND solvers and solve_hddl micro-domains"
standing: BLOCKED
branch: bench/fond-criterion
worktree: ~/ferroplan-worktrees/wt-b26
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. The repo has `benches/ppddl.rs` as the idiom precedent — mirror it.

Scope:
1. `crates/ferroplan/benches/fond.rs`: benchmark groups — (a) `fond_policy` strong fixpoint on generated chains/lattices (10/50/200 states); (b) `fond_policy_strong_cyclic` on retry-loop ladders (mixed: 20% self-loop-only decoy states present — exercises Phase 3); (c) `solve_planning_type(Fond)` dispatch overhead; (d) `solve_hddl` on the 7 hand-authored `tests/fixtures/fond-htn-micro/` domains (parse+ground+translate+solve wall, per-stage timing if the API exposes phases).
2. Seeded generators inline (deterministic), no external corpora in the bench path.
3. `benches/BENCH-FOND.md`: baseline table (mean ± stddev over ≥10 iterations, criterion defaults), machine note, exact commands. Commit the numbers as facts with the seed and commit SHA recorded.
4. Threshold tripwire: a plain `#[test]` that runs the small configurations with generous upper bounds (e.g., 200-state strong fixpoint < 50 ms) so regressions fail CI, not just reports.

Gates: `cargo bench -p ferroplan --bench fond -- --quick` exit 0; tripwire test exit 0; BENCH-FOND.md committed.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | bench/fond-criterion @ 90c2ae2 | — | all |
| 2026-09-17T23:49:41Z | PARTIAL_ALIVE | bench/fond-criterion @ 6d14813 | worktree verified clean, branch contains base 90c2ae2 | orient: ppddl idiom, fond API, fixtures |
| 2026-09-18T00:03:00Z | PARTIAL_ALIVE | bench/fond-criterion @ 17b8cc7 | fond.rs+tripwire committed; first bench run exit 101 (fixture path: manifest `..` pointed at crates/tests) -> fixed; bench --quick exit 0; tripwire dev=0 rel=0 (calibration: dev walls 5-6x release; bounds profile-aware) | BENCH-FOND.md |
| 2026-09-18T00:15:00Z | ALIVE | bench/fond-criterion @ 979ba49 | all gates rerun at final tree: bench --quick=0, tripwire-dev=0, tripwire-rel=0; BENCH-FOND.md committed (38 rows, seed 0x5EED_2026_0917, machine note M3 Max/48GB) | none (integration is coordinator) |
