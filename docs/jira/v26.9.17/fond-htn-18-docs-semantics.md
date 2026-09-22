---
id: fond-htn-18-docs-semantics
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "docs/FOND-HTN.md — semantics, architecture, oracle methodology, migration deviations"
standing: ALIVE
branch: docs/fond-htn-semantics
worktree: ~/ferroplan-worktrees/wt-docs
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. Documentation of CONCEPTS with citations to the public literature (Chen & Bercher flexible FOND HTN; Cimatti et al. strong/strong-cyclic; Höller et al. HDDL) — never koala source.

## Scope
Write `docs/FOND-HTN.md` (in the worktree; match the repo's docs voice):
1. Language surface: the HDDL subset + `oneof` rules as implemented (top-level-only, k≥1 with k==1 deterministic, empty branch allowed, overlap allowed, nesting/`when`-in-branch rejected — reflect the frozen `fix/oneof-koala-semantics` branch as "landing"; note deviations from the reference dialect explicitly, e.g. koala parses-but-drops `when` in branches, we reject).
2. Semantics: strong vs strong-cyclic definitions (winning-set characterizations), fairness assumption, outcome-closed policies, (TN, state) policy concept credited to the formalism's authors.
3. Architecture map: ferroplan-hddl parse→ground→translate; `solve_hddl`; `planning_runtime` dispatch; wasm ops; Eve stage (bridge in flight via T10).
4. Oracle methodology: differential testing against the external oracle, golden verdict files (facts-only), flock serialization rationale, /tmp lifecycle, known oracle quirks.
5. Migration deviations table: koala concept → ferroplan equivalent → status (e.g., TN-emptiness-only goal → goal facts + TN empty; method preconditions compiled away → our representation; fixed vs flexible method commitment).
6. Compliance box: the KOALA POLICY verbatim summary + pointer to the wave context doc.

## Gates
`cd ~/ferroplan-worktrees/wt-docs && cargo test -p ferroplan --doc` exit 0 (docs compile; no code changes expected). One atomic commit.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | docs/fond-htn-semantics @ base d2faf4d | — | all |
| 2026-09-17T22:05:00Z | PARTIAL_ALIVE | docs/fond-htn-semantics @ d2faf4d (worktree clean) | worktree verified | doc authoring; gate pending |
| 2026-09-17T22:25:00Z | PARTIAL_ALIVE | docs/fond-htn-semantics @ d2faf4d + docs/FOND-HTN.md (uncommitted) | cargo test -p ferroplan --doc exit 0 (1 passed, 0 failed) | atomic commit; final row |
| 2026-09-17T22:32:00Z | ALIVE | docs/fond-htn-semantics @ 3a001c3 | cargo test -p ferroplan --doc exit 0 | none — all 6 scope sections delivered; awaiting coordinator merge; never pushed |
