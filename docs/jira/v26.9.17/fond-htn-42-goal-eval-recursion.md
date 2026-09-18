---
id: fond-htn-42-goal-eval-recursion
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: depth budget for evaluate_ground_goal recursion (panic-hunt finding)"
standing: BLOCKED
branch: fix/goal-eval-recursion
worktree: ~/ferroplan-worktrees/wt-a42
created: 2026-09-18T00:55:00Z
source: fond-htn-32 finding (two #[ignore]d reproducers on fuzz/api-panic-hunt)
---

Read `_WAVE4-CONTEXT.md`. Finding: `ferroplan_hddl::grounder::evaluate_ground_goal` (and `action_applicable` via it) recurses over `GroundGoal::Not/And/Or` with no depth budget — depth 10k evaluates, depth 50k SIGABRTs the process. The text path is protected by the parser budget (ticket 22); the PROGRAMMATIC pub API is not.

Scope:
1. Depth-budget the recursion: thread a depth counter (budget = a `pub const DEFAULT_MAX_GOAL_DEPTH` aligned with the parser's 256, overridable via a `*_with_budget` entry or a field on GroundingLimits) returning a typed `GroundError::GoalTooDeep`-shaped refusal; apply the same to `relaxed_satisfiable` if it shares the recursion shape.
2. Tests: programmatic depth-50k goal → typed error, exit 0, no signal (mirror the panic-hunt reproducer); depth-100 legal goal evaluates fine. Un-ignore the two reproducers on that branch's copy if present (they live on the fuzz branch — instead add your own equivalents here).
3. Keep diffs inside the evaluation paths; do not touch parser/translate beyond the wiring.

Gates: `cargo test -p ferroplan-hddl` exit 0; the 50k test passes without signal.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | fix/goal-eval-recursion @ 6d14813 | — | all |
| 2026-09-18T01:40:00Z | ALIVE | fix/goal-eval-recursion @ 6fc8f01 (worktree wt-a42; base 6d14813 is parent, delta = ticket-docs commit only) | oriented: read wave+wave4 context, ticket-32 finding (eval recursion 10k-ok/50k-SIGABRT), ticket-22 landed shape (DEFAULT_MAX_PARSE_DEPTH=256 on fix/parse-depth-budget, not yet in this base → align const at 256); enumerating callers of evaluate_ground_goal/action_applicable/relaxed_satisfiable before threading the depth counter | depth budget + GoalTooDeep + _with_budget; 50k/100 tests; gates |
| 2026-09-18T02:20:00Z | ALIVE (done) | fix/goal-eval-recursion @ 85a2fe0 | gate1 `cargo test -p ferroplan-hddl` exit 0 (144 unit incl. 6 new depth-budget tests + 9 doc-tests); gate2 `cargo test -p ferroplan` exit 0 (156+ passed, pre-existing ignores only); falsifier witnessed: 50k-deep Not chain → typed `GroundError::GoalTooDeep{depth:257, budget:256}` on the plain harness thread, exit 0, no signal (dev-loop evidence: WITHOUT the fix the same shape SIGABRTs — hit and fixed a real drop-path overflow in the 10k probe during development, now on a 64 MiB probe thread); relaxed_satisfiable (And/Or arms) same refusal, propagated through compute_reachability; short-circuit parity preserved; translate wiring only (TranslateError::Ground carries GroundError verbatim; 2 call sites) | remaining: none for this ticket. Notes for coordinator: (1) recursive Drop of adversarial-depth GroundGoal chains is an adjacent drop-path hazard OUT of this ticket's scope — 50k fixtures are mem::forget-ed after assertion with in-test justification; a drop-budget/iterative-Drop ticket would close it (crate forbids unsafe, so custom Drop with worklist); (2) pre-existing warning `unused import: GroundError` in crates/ferroplan/tests/hddl_adversarial.rs:28 — verified present on base (stashed run), left alone per scope discipline; (3) not pushed; integration is coordinator's |
