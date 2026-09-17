---
id: fond-htn-36-docs-benchmarks
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs: BENCHMARKS.md — one canonical table of every committed result"
standing: BLOCKED
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
