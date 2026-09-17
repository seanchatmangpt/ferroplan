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
| 2026-09-17T21:45:00Z | PARTIAL_ALIVE | test/fond-htn-oracle @ d2faf4d | compat matrix done (parser.rs evidence, see fond_htn_oracle.rs docs); 4 hand-authored fixture pairs committed-to-worktree (micro-drop-retry, micro-overlap, micro-recurse, micro-sense); /tmp/fond-oracle arrived via T01 (build.sh exit 0 "oracle ready") after absent-at-21:33Z poll — consumed, not self-delivered | harvest 4+6 oracle verdicts, oracle-goldens.json, fond_htn_oracle.rs, gates |
| 2026-09-17T22:12:00Z | PARTIAL_ALIVE | test/fond-htn-oracle @ d2faf4d | harvest 10/10 via oracle-run.sh --mode flexible --heur ff (4 fixtures + 6 koala first-problem files; micro-sense NOSOLUTION cross-checked --mode fixed-ld; first micro-sense variant with bare (not …) precondition crashed koala grounder — task_defs.rs:30 panic — fixture dialect updated to (and (not …)), recorded in goldens); probe cargo run observed live ferroplan outcomes → goldens finalized | warning cleanup, final gates, commit |
| 2026-09-17T22:20:00Z | ALIVE | test/fond-htn-oracle @ c27d0ce | gates: cargo test -p ferroplan --test fond_htn_oracle exit 0 (2 passed, 9 ignored = 6 external-corpus + 3 ORACLE_MISMATCH listed); same -- --ignored exit 0 (9 passed, 10s); cargo test -p ferroplan-hddl exit 0 (91 unit + 7 doc, 0 fail); 0 warnings. FINDINGS (semantic, 3): micro-drop-retry+Transport shape — koala flexible re-decomposition vs ferroplan dead-terminal NoPlan (owner fix/oneof-koala-semantics 4d40f99, not in base); micro-sense — koala strong(acyclic) NOSOLUTION vs ferroplan strong-cyclic SOLVED, both sound in own semantics; micro-recurse — '=' parse/validate-ok (d2faf4d) but evaluate_ground_goal never evaluates positive (= …) true → NoPlan on solvable problem (ferroplan-hddl gap, new). Resource-limit divergences (goldens "error" + verbatim): Transport/Childsnack 10s max_wall_ms translate timeout, Snake max_ground_methods 10000. Depots/Rover/Satellite/Childsnack-independent agreement: Depots, Rover, Satellite agree | none — ticket scope complete |
