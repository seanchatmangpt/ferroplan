---
id: fond-htn-11-test-fond-canonical
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Canonical flat-FOND solver suite (literature verdicts, explicit-state)"
standing: BLOCKED
branch: test/fond-canonical
worktree: ~/ferroplan-worktrees/wt-test-fond
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. NOTE: on this branch `validate_problem` still enforces mass==1M (the relax lives on `fix/fond-minors`, unmerged) — encode two-outcome actions as 500000/500000.

## Scope
`crates/ferroplan/tests/fond_canonical.rs` + fixtures encoding ≥ 7 canonical scenarios as `PlanningProblem` values (hand-authored, tiny, literature-faithful; cite source paper per scenario in a comment):
- tireworld-s (cyclic-only), triangle-tireworld (cyclic-only), islands (cyclic-only), wall (cyclic-only), faults (strong), boolean (strong), coffee (cyclic-only), plus tireworld-unsolvable (absorbing failure, no spare).
- Self-loop classic: `flip` s0→{s0,goal}: Fond dispatcher solves (strong-cyclic), policy outcome-closed; dead-end variant (flip may reach absorbing non-goal) ⇒ `Err(NoPlan)`.
- Strong cases: policy must ALSO solve with the acyclic property (no reachable cycle under policy — in-test checker: BFS under policy, assert DAG-to-goal).
- Cyclic-only cases: assert solved + closed; unsolvable: typed NoPlan, never a bogus policy.
- If `/tmp/fond-corpus/fond-flat/` exists (T16 may race), you MAY cross-check encodings against its expected-verdicts.json — optional, self-sufficiency required.

Any literature-verdict failure ⇒ do NOT edit src: `#[ignore]`d test + precise written diagnosis in History (finding, not papering over).

## Gates
`cd ~/ferroplan-worktrees/wt-test-fond && cargo test -p ferroplan --test fond_canonical` exit 0 (ignored findings listed).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/fond-canonical @ base d2faf4d | — | all |
