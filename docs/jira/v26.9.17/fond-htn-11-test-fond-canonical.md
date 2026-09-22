---
id: fond-htn-11-test-fond-canonical
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Canonical flat-FOND solver suite (literature verdicts, explicit-state)"
standing: ALIVE
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
| 2026-09-17T22:43:48Z | PARTIAL_ALIVE | test/fond-canonical @ d2faf4d | oriented: `solve_planning_type` Fond arm (strong fixpoint → strong-cyclic fallback), `validate_problem` mass==1M confirmed enforced on this branch; `/tmp/fond-corpus/fond-flat/` present but NO expected-verdicts.json (T16 race) → self-sufficient encodings | scenarios 9 (7 canonical + flip pair), in-test checkers, gate run |
| 2026-09-17T22:48:15Z | PARTIAL_ALIVE | test/fond-canonical @ c8ebf06 | `cargo test -p ferroplan --test fond_canonical` exit 0 — 10 passed, 0 failed, 0 ignored (no literature-verdict failures ⇒ no #[ignore] findings, zero src/ edits); committed c8ebf06 | final row |
| 2026-09-17T22:48:15Z | ALIVE | test/fond-canonical @ c8ebf06 | gate exit 0 (see above); deliverable `crates/ferroplan/tests/fond_canonical.rs` (719 lines): 10 tests / 9 scenarios — flip self-loop (strong-cyclic + closed policy), flip dead-end (typed NoPlan), faults (strong DAG-to-goal), boolean (strong DAG-to-goal), coffee, wall, islands, tireworld-s (cyclic-only, cycle-in-policy asserted ⇒ fallback path proven), triangle-tireworld (cyclic-only + dead-end route avoidance asserted), tireworld-unsolvable (absorbing failure, typed NoPlan); in-test checkers: outcome-closed multiset cover + full 1M mass, policy-reachable coverage, goal reachability under policy, white/gray/black DFS cycle detector (exercised both directions = non-tautological); all fixtures hand-authored with per-scenario literature citations (Cimatti/Pistore/Roveri/Traverso AIJ 2003; Mattmüller et al. ICAPS 2008; Boutilier/Dean/Hanks AIJ 1999); two-outcome actions encoded 500000/500000 per branch mass==1M | none — remaining: merge wave integration only |
