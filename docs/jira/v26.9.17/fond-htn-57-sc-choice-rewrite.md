---
id: fond-htn-57-sc-choice-rewrite
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "HOTFIX: FOUND_BUG_2 — strong-cyclic returns witness-first goal-unreachable loop policies"
standing: BLOCKED
branch: fix/sc-choice-rewrite
worktree: ~/ferroplan-worktrees/wt-h57
created: 2026-09-18T02:50:00Z
source: fond-htn-33 scale-up (882/3676 = 24% of reference-solvable instances)
---

Read `_WAVE4-CONTEXT.md`. THE priority ticket of this wave. Finding (fond_property_scaleup FOUND_BUG_2, #[ignore]d reproducer on main with the full diagnosis in its doc comment): `fond_policy_strong_cyclic`'s Phase 3 certifies goal-reachability of the surviving REGION but returns Phase-2 witness-FIRST choices — when a state's first closed action is a pure loop and a different committable action exists, the fixpoint closes with `reach == surviving` and the loop exits without rewriting `choices`. Distinct from the fixed FOUND_BUG_1 (there an advancing action did not exist; here it does and is not chosen).

Scope:
1. Read the reproducer `fond_property_scaleup_FOUND_BUG_2_*` in `crates/ferroplan/tests/fond_property_scaleup.rs` — its doc comment documents the fix shape.
2. Fix in `crates/ferroplan/src/planning_runtime.rs` `fond_policy_strong_cyclic`: record each state's reach-discovery RANK during the Phase-3 BFS; when the fixpoint closes, rewrite every surviving non-goal state's choice to a committable action (all outcomes ⊆ surviving) with at least one outcome of strictly smaller rank (advancing). If a state has no such action it cannot be in reach — prune instead (defense in depth). Choices must be recomputed after any prune cascade (or fold the rewrite into the alternating loop).
3. Tests: (a) un-ignore the FOUND_BUG_2 reproducer (must pass); (b) remove the scale-up gate's carve-out for this class so the 5000-instance sweep FAILS on any recurrence — run it, assert 0 mismatches; (c) the wave-3 tests (retry loop, two retry points, dead-sink reproducer, chain, self-loop rejections, prefers-advancing-action) must stay green unchanged; (d) new unit test: state with loop-first + advancing-second action (BTreeMap order puts the loop first) must return the advancing action.
4. Update docs/FOND-HTN.md Phase-3 paragraph: the region AND the choices are reach-certified (two-line edit).

Gates: `cargo test -p ferroplan --lib planning_runtime && cargo test -p ferroplan --test fond_property_scaleup && cargo test -p ferroplan --test fond_property --test fond_canonical` exit 0 (no ignores remaining in scaleup beyond its documented load-tolerance).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:50:00Z | BLOCKED | fix/sc-choice-rewrite @ 75870de | — | all |
| 2026-09-18T23:30:00Z | ALIVE | merged d00dcf8 into wave6/land-v26917 (merge 8f1a875, A7 landing branch wt-f5-04-land) | gates re-run in LANDING worktree post-merge: planning_runtime 15/15 exit 0; fond_property_scaleup 4/4 exit 0, 0 ignored, carve-out removed | none (A7) |
