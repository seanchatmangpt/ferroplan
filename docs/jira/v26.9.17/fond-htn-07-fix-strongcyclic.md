---
id: fond-htn-07-fix-strongcyclic
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: strong-cyclic truncation dead-sink (outcome-blind post-check)"
standing: ALIVE
branch: fix/fond-sc-closure
worktree: ~/ferroplan-worktrees/wt-fondsc
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. All work in your worktree; never touch main; never push. Line numbers may drift — locate by symbol.

## Defect
`crates/ferroplan/src/planning_runtime.rs`, `fond_policy_strong_cyclic` (~L811): Phase-2 prune loop capped by `limits.max_iterations` (serde-visible). A state validated early in a round can keep a witness pointing at a state pruned later the same round; truncation then returns a policy with a dead sink (choice → pruned choice-less non-goal state). Post-loop checks (~L888) verify only initials-in-surviving + has-some-choice — never outcome closure. Reproducer: states s0(init),x,z,g(goal); a:s0→x; b:x→g+x→z; `max_iterations:1` ⇒ bogus policy {s0→a}.

## Fix (all three)
1. Replace the `max_iterations` gate with the mathematical bound: loop `while changed` with failsafe cap `problem.states.len()+1`; exceeding failsafe ⇒ `Err(Timeout)` (never silent success). `max_iterations` no longer gates this fixpoint. Preserve wall-clock deadline semantics (Timeout must propagate, not be swallowed).
2. Strengthen post-loop check to outcome closure: every surviving non-goal state's choice outcomes ⊆ (goal states ∪ surviving states with a choice); violation ⇒ `Err(NoPlan)`.
3. Convert the tautological post-check in `fond_policy` (~L765) to `debug_assert!` with comment, or delete — state which in the commit message (dead code, no behavior change).

## Tests
(a) The 4-state reproducer: post-fix must return `Err(NoPlan)` (s0 pruned in round 2). (b) N-state chain proves convergence ≤ states+1 rounds even with `max_iterations:1`. (c) Existing `fond_policy_strong_cyclic_*` tests unchanged and green.

## Gates
`cd ~/ferroplan-worktrees/wt-fondsc && cargo test -p ferroplan --lib planning_runtime && cargo test -p ferroplan --test planning_runtime` exit 0. Atomic commits.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | fix/fond-sc-closure @ base d2faf4d | — | all |
| 2026-09-17T21:43:07Z | PARTIAL_ALIVE | fix/fond-sc-closure @ d2faf4d (worktree clean) | orient: defect+fix+tests read, helpers/state_index/validate_problem confirmed | implement fix 1-3 + tests (a)(b), gates |
| 2026-09-17T22:17:12Z | ALIVE | fix/fond-sc-closure @ 6df15f3 (fix 1+2+tests) then 9d1ed32 (fix 3 debug_assert!) | intermediate tree: gate1 gate2 exit 0; final tree: `cargo test -p ferroplan --lib planning_runtime` exit 0 (8 passed, incl. 2 new + 3 existing strong_cyclic unchanged); `cargo test -p ferroplan --test planning_runtime` exit 0 (12 passed); compound `gate1 && gate2` exit 0; full `cargo test -p ferroplan` exit 0 (62 result lines all ok) | none — both commits landed, tree clean, not pushed |
| 2026-09-17T22:17:12Z | ALIVE | fix/fond-sc-closure @ 9d1ed32 | falsification note: at max_iterations:1 the pre-fix code returned NoPlan on the 4-state fixture via the initials check (Phase 1 itself truncated); the bogus {s0→a} dead sink manifests when truncation lands between rounds 1-2 — test comments written to state this truthfully; closure check rejects the shape unconditionally regardless | none |
