---
id: fond-htn-28-bench-scaling-ladder
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Bench: scaling ladder — find the capacity knee per pipeline stage"
standing: BLOCKED
branch: bench/scaling-ladder
worktree: ~/ferroplan-worktrees/wt-b28
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`.

Scope:
1. Hand-author (provenance comments) two parametric HDDL domain families with instance generators (seeded, inline in the test): total-order blocksworld-style (n blocks) and Transport-style with a failing drop oneof (n packages, m locations) — the wave's own micro-domain patterns scaled up.
2. Ladder: n ∈ {4, 8, 16, 32, 64, 128}; per rung record per-stage walls (parse/ground/translate/solve) and the translated composite-state count, wall-bounded at 60 s per stage; stop the ladder at the first refusal and say which stage knelt.
3. Deliver `crates/ferroplan/tests/fixtures/scaling-ladder/RESULTS.md` + a plotted-free summary table (rung | walls | states | outcome) + the capacity envelope statement ("ground is linear in X, translate superlinear past Y, solve dominated by Z") backed by the numbers.
4. `#[ignore]`d long-run test + sampled always-on rung (n=8) as CI heartbeat.

Gates: `cargo test -p ferroplan --test scaling_ladder` exit 0 (sampled); `-- --ignored` exit 0; RESULTS.md committed with machine note.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | bench/scaling-ladder @ 90c2ae2 | — | all |
