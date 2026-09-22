---
id: fond-htn-54-fond-unsafe-hddl
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Test: unsafe-states semantics through the HDDL path (library-level only today)"
standing: ALIVE
branch: test/fond-unsafe-hddl
worktree: ~/ferroplan-worktrees/wt-d54
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. `unsafe_states` is exercised in library-level FOND tests only — nothing proves the HDDL → translate → solve path honors a never-enter contract (there is no HDDL syntax for "unsafe"; the explicit PlanningProblem side is where it lives — verify the seams).

Scope:
1. Hand-authored micro fixtures under `tests/fixtures/fond-unsafe/` (provenance comments): (a) drop-into-abyss HDDL domain — an oneof action whose second branch leads to an unavoidable dead-end fact pattern (no actions applicable) — solvable only by avoiding that action; (b) the same domain where BOTH branches dead-end (unsolvable); (c) an explicit `PlanningProblem` twin of (a) carrying a true `unsafe_states` entry, proving library-level refusal.
2. `crates/ferroplan/tests/fond_unsafe_hddl.rs`: (a) solves avoiding the abyss (policy never contains the abyss action); (b) typed NoPlan; (c) NoPlan via unsafe_states; PLUS a structural invariant checker: in every solved policy, no outcome lands in a state with zero outgoing actions and non-goal facts (dead-sink freedom at the HDDL level — generalizes the abyss pattern).
3. If (a) or (b) misbehaves (e.g., a policy that gambles into the abyss), that is a semantic FINDING: `#[ignore]`d reproducer + History (feasibility gating of oneof branches is the suspected seam).

Gates: `cargo test -p ferroplan --test fond_unsafe_hddl` exit 0 (findings ignored-listed).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | test/fond-unsafe-hddl @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | test/fond-unsafe-hddl @ 75870de (wt-d54) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
| 2026-09-22T00:00:00Z | ALIVE | fer-07-respawns (merged to release/v26.9.22) | FINISHED by FERROPLAN-26922-07: tests/fond_unsafe_hddl.rs (4 tests, all green first run — no finding needed) — (a) abyss-avoid: policy solves and never contains a gamble* action; (b) abyss-both: typed NoPlan; (c) explicit PlanningProblem twin whose only route runs through an unsafe_states entry refuses NoPlan, and solves once the entry is removed, pinning the seam that solve_hddl hardcodes unsafe_states to default; plus a structural dead-sink-freedom walk over the solved policy. Fixtures hand-authored in-repo (provenance in file headers). Gate: cargo test -p ferroplan --test fond_unsafe_hddl == 0 (4 passed). Feeds the fond-htn-67 ruling neighborhood (goal-set/unsafe HDDL semantics). | none |
