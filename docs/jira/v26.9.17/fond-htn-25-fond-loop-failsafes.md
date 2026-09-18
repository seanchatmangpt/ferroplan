---
id: fond-htn-25-fond-loop-failsafes
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: apply the states+1 failsafe pattern to fond_policy and contingent loops"
standing: BLOCKED
branch: fix/fond-loop-failsafes
worktree: ~/ferroplan-worktrees/wt-a25
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Out-of-scope note from ticket 07: `fond_policy`'s fixpoint and `conformant_plan`'s BFS are still gated by `limits.max_iterations` (serde-visible, caller-settable). For `fond_policy` the μ-fixpoint needs at most `states + 1` changing rounds (each admits ≥1 state), so a caller-set `max_iterations` below that rejects solvable domains with a bogus `NoPlan` — same defect class 07 fixed for the strong-cyclic loops.

Scope:
1. `fond_policy`: replace the `max_iterations` gate with the `while changed` + `states + 1` failsafe (`Err(Timeout)` on breach — never silent success), preserving wall-clock checks. Test: 12-hop chain (mirror ticket 07's) solves under `max_iterations: 1`; the wave-1 4-state reproducer unchanged.
2. `conformant_plan`: audit its bounds — `max_depth`/`max_states` are structural (belief BFS), keep them; but if any loop is also iteration-gated, apply the same failsafe reasoning or document why the gate is sound there.
3. Do NOT touch `fond_policy_strong_cyclic` (just hardened) or `contingent_policy` (depth-memo fixed on 52c3501) beyond what items 1–2 require.
4. Doc: one paragraph in the module docs stating the bound policy: structural fixpoints bounded by their own math; caller caps advisory only for those.

Gates: `cargo test -p ferroplan --lib planning_runtime && cargo test -p ferroplan --test planning_runtime --test fond_canonical` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | fix/fond-loop-failsafes @ 90c2ae2 | — | all |
| 2026-09-17T23:56:54Z | PARTIAL_ALIVE | fix/fond-loop-failsafes @ 7fea6cc | gate1 `cargo test -p ferroplan --lib planning_runtime` exit 0 (14 passed; incl. new `fond_policy_converges_past_max_iterations_on_a_chain`, `fond_policy_still_rejects_the_dead_sink_reproducer`); gate2 `cargo test -p ferroplan --test planning_runtime --test fond_canonical` exit 0 (20 + 10 passed) | none in scope; note: `probabilistic_policy`'s value iteration is still `max_iterations`-gated (out of scope, documented on the field) |
| 2026-09-17T23:56:54Z | ALIVE | fix/fond-loop-failsafes @ 7fea6cc | both gates exit 0 (recorded with pipefail) | none — awaiting coordinator integration |
| 2026-09-18T01:36:00Z | ALIVE | fix/fond-loop-failsafes@7fea6cc | — | integration pending |
