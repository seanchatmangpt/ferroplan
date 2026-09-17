---
id: fond-htn-04-diff-fondhtn
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Differential: ferroplan solve_hddl vs oracle on FOND-HTN (goldens + agreement tests)"
standing: BLOCKED
branch: test/fond-htn-oracle
worktree: ~/ferroplan-worktrees/wt-oracle-fondhtn
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. KOALA POLICY binding: no koala files in the repo; koala domain files are read from `/tmp/fond-review/domains/` at TEST time via absolute paths, never copied.

## Scope
1. Compatibility matrix: read the worktree's `crates/ferroplan-hddl/src/parser.rs` grammar; classify every construct used by the 6 usable koala domains (`:ordered-subtasks`, `:tasks`, bare subtask lists, method `:precondition`, method `:constraints`, oneof top-level/empty-branch/overlap, `:goal` alongside `:htn`, typing) as SUPPORTED/UNSUPPORTED with evidence (code symbol).
2. Hand-authored in-repo fixtures (`crates/ferroplan/tests/fixtures/fond-htn/`, each with a one-line provenance comment "hand-authored, Transport-pattern"): micro Transport-like drop-retry (oneof success/empty-branch), micro Childsnack-like overlapping branches, micro Snake-like recursive method, micro Satellite-like sensing two-branch. Keep each ≤ 30 lines.
3. Oracle goldens: for each in-repo fixture AND each of the 6 koala domain pfile01 (run from /tmp): harvest via `/tmp/fond-oracle/oracle-run.sh` (self-sufficient protocol per T02 if harness missing). Commit verdicts ONLY as `crates/ferroplan/tests/fixtures/fond-htn/oracle-goldens.json`.
4. Agreement test `crates/ferroplan/tests/fond_htn_oracle.rs`: for in-repo fixtures — `solve_hddl` (read its signature in `crates/ferroplan/src/hddl.rs`) must agree with golden status (SOLVED↔solved-with-closed-policy; NOSOLUTION↔typed NoPlan); write an in-test outcome-closure checker. For the 6 koala domains: test is `#[ignore]`d "external corpus" cases reading absolute /tmp paths at runtime — assert only when files exist (skip-with-note otherwise), never panic on parse gaps: record gaps in the goldens as `{"ferroplan": "parse-gap", "reason": "<verbatim error>"}` and assert the gap classification stays true.
5. Commit findings: any semantic disagreement → dedicated `#[ignore]` test named `ORACLE_MISMATCH_<domain>` + History row with details.

## Gates
`cd ~/ferroplan-worktrees/wt-oracle-fondhtn && cargo test -p ferroplan --test fond_htn_oracle` exit 0 (ignored mismatch tests listed). `cargo test -p ferroplan-hddl` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/fond-htn-oracle @ base d2faf4d | — | all |
