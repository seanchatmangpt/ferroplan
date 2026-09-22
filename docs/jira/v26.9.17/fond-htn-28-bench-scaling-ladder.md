---
id: fond-htn-28-bench-scaling-ladder
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Bench: scaling ladder — find the capacity knee per pipeline stage"
standing: ALIVE
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
| 2026-09-17T23:58:21Z | ALIVE | bench/scaling-ladder @ 6d14813 | start: ticket+context read, worktree clean | pipeline API mapped, test authoring next |
| 2026-09-18T00:13:12Z | ALIVE | bench/scaling-ladder @ worktree | gate1: cargo test -p ferroplan --test scaling_ladder exit 0 (2 passed: heartbeat n=8 both families + generator determinism; 1 ignored long-run) | full ladder run, RESULTS.md, gate2 |
| 2026-09-18T00:17:40Z | ALIVE | bench/scaling-ladder @ 212d1b9 | full ladder: cargo test -p ferroplan --test scaling_ladder -- --ignored exit 0; 12/12 rungs SOLVED, zero refusals (chain-world + transport-drop, n∈{4,8,16,32,64,128}, 60s/stage); solve dominates (14.7s @ n=128 transport); RSS 97.6MB measured | RESULTS.md commit + gate2 re-run |
| 2026-09-18T00:17:40Z | ALIVE | bench/scaling-ladder @ 6a8ce08 | gate2: -- --ignored re-run exit 0, structural counts byte-identical across runs; RESULTS.md committed (machine note: Apple M3 Max, seed 20260917, RSS 95344KB); sampled gate re-verified exit 0 | none — ticket complete; not merged (integration is coordinator's) |
| 2026-09-18T01:36:00Z | ALIVE | bench/scaling-ladder@6a8ce08 | — | integration pending |
