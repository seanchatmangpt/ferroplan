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
