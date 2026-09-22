---
id: fond-htn-51-benchmarks-drift-check
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: benchmarks drift-check — docs numbers re-derived from source files, fail-closed"
standing: CLOSED
branch: docs/benchmarks-drift-check
worktree: ~/ferroplan-worktrees/wt-d51
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. 延 anchor: BENCHMARKS.md numbers must be re-derivable, not trusted prose.

Scope:
1. `scripts/verify_benchmarks.py` (executable, fail-closed): parse `docs/BENCHMARKS.md` at base (wave-3 corpora rows) — for each corpus row, re-derive instance counts + solved/refused tallies from the cited source file (RESULTS.md tables, JSON arrays) and compare to the doc's numbers; exit 1 with a precise diff on any mismatch; exit 0 with a one-line summary per corpus.
2. Wire it as a lightweight gate: a `#[test]` in ferroplan (name `benchmarks_doc_numbers_match_sources`, invoking the script via `env!("CARGO_MANIFEST_DIR")`-relative path, skipped-with-note only if the script is absent) — the drift tripwire.
3. Tolerate wave-4 rows appearing later: unknown corpus sections that cite not-yet-existing files are reported as "pending", not failures (match the "pending wave-4" convention already in the doc).
4. Fix any drift you find at base in BENCHMARKS.md itself (append-only corrections with a note), unless the doc is already accurate — report either way.

Gates: `python3 scripts/verify_benchmarks.py` exit 0; `cargo test -p ferroplan --test benchmarks_drift` exit 0 (or your chosen test home).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | docs/benchmarks-drift-check @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | docs/benchmarks-drift-check @ 75870de (wt-d51) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
| 2026-09-22T00:00:00Z | CLOSED | fer-07-respawns (merged to release/v26.9.22) | RETIRED, recorded as topology not silent pruning: the work was never started twice (wave-5 cut mid-flight, wave-6 respawn produced zero commits — worktree sat at 75870de); the v26.9.22 lane retires it rather than re-respawning a third time. Scope above stays the record of what a future cycle would owe. Worktree wt-d51 pruned (branch had 0 commits outside main). | none (re-open as a fresh ticket if the capability is still wanted) |
