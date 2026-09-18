---
id: fond-htn-38-docs-book-chapter
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs: book chapter — fond-htn.md for the mdbook build"
standing: BLOCKED
branch: docs/book-fond-htn
worktree: ~/ferroplan-worktrees/wt-d38
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. The book lives in `book/src/` (see `book/src/eve-genesis.md` for voice/structure and the book's SUMMARY.md for the TOC).

Scope:
1. New `book/src/fond-htn.md`: narrative chapter — the problem (non-determinism + hierarchy), the two solution concepts (strong vs strong-cyclic with the fairness intuition, credited to the public literature), the pipeline story (parse → ground → translate → fixpoints), the retry-loop worked example with a tiny policy diagram (ASCII/mermaid per book idiom), the oracle methodology as a war story (differential testing, the FOUND_BUG_1 hunt), and "where it runs" (library, wasm ops, eve).
2. Register in `book/src/SUMMARY.md` after the eve chapter.
3. Consistency: same facts as `docs/FOND-HTN.md` (read it first); no new claims; no koala source, concepts only; every number cited from committed RESULTS files.
4. Build gate: if the book has a build (`mdbook build` config or CI lane — check `.github/workflows/`), run it; if mdbook isn't installed, `cargo test -p ferroplan --doc` as the doc gate and record the mdbook command in History for CI.

Gates: book builds (or documented unavailability + doc gate exit 0); one atomic commit.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | docs/book-fond-htn @ 90c2ae2 | — | all |
| 2026-09-18T00:49:15Z | IN_PROGRESS | docs/book-fond-htn @ 6d14813 (wt-d38, clean) | orient done: FOND-HTN.md + wave contexts + book idiom read; mdbook 0.4x present, CI lane `mdbook build book` (pages.yml) | chapter, SUMMARY row, gate, commit |
| 2026-09-18T00:52:05Z | ALIVE | docs/book-fond-htn @ e67a37d (wt-d38) | `mdbook build book` exit 0 (book/book/fond-htn.html rendered, 27 KB, eve-genesis link resolves); falsifier sweep: 0 "koala" in src+html, numbers anchored (79/320 planning_runtime.rs, 320/seed fond_property.rs, 18/12/0+1 oracle-harvest-full-summary.md, 14/6/1 htn-oracle RESULTS.md, 695/0 wave receipt); 1 atomic commit (2 files, +246) | none — integration is coordinator's (post-wave, serial) |
