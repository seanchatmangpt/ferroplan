---
id: fond-htn-59-goal-drop-budget
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: iterative Drop for deep GroundGoal chains (recursive-drop overflow hazard)"
standing: ALIVE
branch: fix/goal-drop-budget
worktree: ~/ferroplan-worktrees/wt-h59
created: 2026-09-18T02:50:00Z
source: fond-htn-42 out-of-scope note (mem::forget workaround in its tests)
---

Read `_WAVE4-CONTEXT.md`. The evaluator recursion is depth-budgeted (GoalTooDeep at 256), but building a deep-but-legal (≤256) chain is possible AND adversarial programmatic input at depth ~50k can still be CONSTRUCTED (the budget refuses evaluation, then the value DROPS — and recursive Drop overflows at a similar depth). Ticket 42's tests `mem::forget` their deep fixtures to dodge this.

Scope:
1. `crates/ferroplan-hddl/src/grounder.rs`: implement an iterative/manual drop for `GroundGoal` (a `Drop` impl that repeatedly takes and drains children via `std::mem::take`-style tail destruction with an explicit worklist — the standard recursion-free Drop pattern), so any depth DROPS in O(1) stack.
2. Replace ticket 42's `mem::forget` workarounds with real drops (remove the justification comments).
3. Tests: construct depth-50k chains (as ticket 42's tests do), refuse evaluation with GoalTooDeep, then DROP them normally — no signal (pre-fix this aborts); depth-250 chain drops after normal evaluation.

Gates: `cargo test -p ferroplan-hddl` exit 0; the 50k drop test passes without signal.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:50:00Z | BLOCKED | fix/goal-drop-budget @ 75870de | — | all |
| 2026-09-18T02:59:31Z | ALIVE | fix/goal-drop-budget @ 304fa66 | work begun on iterative GroundGoal Drop; worktree clean at base tip 55e1997 | impl+tests+gates |
| 2026-09-18T03:00:22Z | ALIVE | fix/goal-drop-budget @ 304fa66 | `cargo test -p ferroplan-hddl` exit 0 (169 lib + 9 doc, 0 failed; 50k fixtures drop at scope end, no signal); `cargo test -p ferroplan-hddl --lib grounder::tests` exit 0 (depth-250 + all ex-forget tests green); `cargo clippy -p ferroplan-hddl --all-targets` exit 0 (16 pre-existing warnings, none in grounder.rs); `cargo check -p ferroplan --all-targets` exit 0 (consumer crate compiles against new Drop). Falsifier: gate run with the mem::forget dodge REMOVED — pre-fix this exact run aborts the harness on recursive drop; observed 0 failed, no signal | none in scope |
