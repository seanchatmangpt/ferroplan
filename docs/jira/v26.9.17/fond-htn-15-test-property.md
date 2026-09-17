---
id: fond-htn-15-test-property
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Property test: FOND solvers vs independent brute-force policy enumeration"
standing: BLOCKED
branch: test/fond-property
worktree: ~/ferroplan-worktrees/wt-test-cross
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. Mass note as in T11: generate mass-valid problems only (partitions of 1M per (from,action) group).

## Scope
`crates/ferroplan/tests/fond_property.rs` — seeded deterministic generator + INDEPENDENT reference (policy enumeration, not a fixpoint):
1. Generate 300+ instances: 3..=8 states, 1 goal, 2..=3 actions, nondet outcome edges (1..=3 per (state,action)), some deterministic, single initial, optional 0..=1 unsafe absorbing non-goal state; fixed seed; serialize each generated instance into the failure message (PlanningProblem JSON via serde) so any failure is a reproducer.
2. Reference semantics (implement literally): enumerate policies (≤ 3^8 assignments). STRONG ⟺ some π whose reachable graph from s0 is acyclic with all leaves goal. STRONG-CYCLIC ⟺ some π whose outcome-closed reachable set R satisfies: every state in R can reach a goal within R (Cimatti characterization). UNSAFE states are forbidden targets.
3. Assertions: (a) equivalence — `solve_planning_type(PlanningType::Fond)` Ok ⟺ strong ∪ strong-cyclic solvable; Err(NoPlan) ⟺ neither; (b) closure — every returned policy outcome-closed (goal or policy key); (c) determinism — same input twice ⇒ identical policy output.
4. Total test wall < 30 s; loop budget asserted.
5. Any mismatch: shrink to ≤ 5 states, commit as `#[ignore]`d `fond_property_FOUND_BUG_*` with expected-vs-actual — a finding for the coordinator; do not edit src.

## Gates
`cd ~/ferroplan-worktrees/wt-test-cross && cargo test -p ferroplan --test fond_property` exit 0 (ignored reproducers listed).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/fond-property @ base d2faf4d | — | all |
