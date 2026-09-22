---
id: fond-htn-36-docs-benchmarks
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs: BENCHMARKS.md — one canonical table of every committed result"
standing: ALIVE
branch: docs/benchmarks-consolidated
worktree: ~/ferroplan-worktrees/wt-d36
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`.

Scope:
1. New `docs/BENCHMARKS.md` consolidating ONLY already-committed results (do not run new measurements): wave-3 `tests/fixtures/htn-oracle/RESULTS.md` (21 differential instances), `tests/fixtures/htn-ipc2023/RESULTS.md` (13 instances), `tests/fixtures/fond-htn/oracle-harvest-full.json` + summary (31 koala runs), wave-3 flat-fond three-way verdicts (8 domains), plus whatever wave-4 bench RESULTS files exist on main at your start (check `tests/fixtures/ipc-sweep/`, `scaling-ladder/`, `benches/BENCH-FOND.md` — likely absent; note "pending wave-4" where so).
2. Format: one summary table per corpus (corpus | instances | solved | refused-by-limit | gaps | source file), then per-corpus reproduction command blocks (exact cargo invocation), then a "How to read a verdict" legend (SOLVED/NOSOLUTION/TIMEOUT/PARSE_ERROR/LIMIT:* / GAP:*).
3. Cross-links from `docs/FOND-HTN.md`'s oracle-methodology section and from `README.md`'s benchmarks area if one exists (add anchor if not).
4. Numbers must be traceable: every row cites its source file path; no recomputation, no smoothing.

Gates: `cargo test -p ferroplan --doc` exit 0; all cited paths exist (verify by script — a broken link fails your gate).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | docs/benchmarks-consolidated @ 90c2ae2 | — | all |
| 2026-09-18T00:54:08Z | ALIVE | docs/benchmarks-consolidated @ 6d14813 (worktree wt-d36, clean) | sources oriented: htn-oracle RESULTS 21, htn-ipc2023 RESULTS 13, oracle-harvest-full.json 31 (18S/12T/1U), fond-htn oracle-goldens 10, fond-flat 8 three-way; wave-4 ipc-sweep/ + scaling-ladder/ + benches/BENCH-FOND.md ABSENT → pending-wave-4 notes | BENCHMARKS.md, cross-links, gates, commit |
| 2026-09-18T00:58:42Z | ALIVE | docs/benchmarks-consolidated @ 6bf31c9 (worktree wt-d36, clean) | G1 link/path/traceability script exit 0 (14 links resolve, 19 cited paths exist, source counts 21/13/31/10/8 verified, 21 corpus-1 instances traceable); G2 `cargo test -p ferroplan --doc` exit 0 (1 passed/0 failed); committed 3 files, 264 ins: docs/BENCHMARKS.md + cross-links in docs/FOND-HTN.md §4 + README.md Benchmarks | none in scope (wave-4 bench rows append when RESULTS files land) |
| 2026-09-18T01:36:00Z | ALIVE | docs/benchmarks-consolidated@6bf31c9 | — | integration pending |
