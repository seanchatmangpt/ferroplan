---
id: fond-htn-12-test-fondhtn-regression
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Hand-authored FOND-HTN micro-domain regression suite for solve_hddl"
standing: BLOCKED
branch: test/fond-htn-koala
worktree: ~/ferroplan-worktrees/wt-test-fondhtn
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. ALL fixtures hand-authored with provenance comments ("hand-authored, <pattern>-pattern") — no koala files in the repo.

## Scope
1. Fixtures under `crates/ferroplan/tests/fixtures/fond-htn-micro/` — micro HDDL domain+problem pairs (each ≤ 30 lines): drop-retry (oneof success/empty-branch — Transport pattern), tray-dirty (overlapping branches — Childsnack pattern), sense-then-branch (two-branch sensing — Satellite pattern), grow-loop (recursive method — Snake pattern), supervisor-fail (fail-then-retry via second method — Depots pattern), plain-chain (deterministic baseline).
2. `crates/ferroplan/tests/fond_htn_micro.rs`: read `solve_hddl`'s signature from `crates/ferroplan/src/hddl.rs`; per fixture assert: solved with outcome-closed policy (in-test closure checker) OR typed NoPlan where the fixture is deliberately unsolvable (add one such: a oneof whose both branches dead-end). Also assert: policy outcomes count == branch count for the oneof action (no outcome collapsing), empty branch yields a no-change outcome, and recursion fixture terminates within budget.
3. No-panic/no-hang contract: every fixture wrapped with a wall budget from solve_hddl's API.
4. Report any semantic surprise in History (e.g., decomposition-order effects) — findings, not fixes.

## Gates
`cd ~/ferroplan-worktrees/wt-test-fondhtn && cargo test -p ferroplan --test fond_htn_micro` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/fond-htn-koala @ base d2faf4d | — | all |
