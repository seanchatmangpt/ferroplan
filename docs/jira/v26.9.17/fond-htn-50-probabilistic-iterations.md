---
id: fond-htn-50-probabilistic-iterations
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: probabilistic_policy value iteration — convergence-gated, not iteration-capped"
standing: BLOCKED
branch: fix/probabilistic-iterations
worktree: ~/ferroplan-worktrees/wt-d50
created: 2026-09-18T00:55:00Z
source: fond-htn-25 out-of-scope observation
---

Read `_WAVE4-CONTEXT.md`. `probabilistic_policy`'s value iteration is still capped by `max_iterations` (default 512); truncation returns a silently SUBOPTIMAL policy (value degradation, not unsoundness) — the last mass-blind/iteration-gated loop.

Scope:
1. Convert the loop to convergence-gated: iterate while any value changes by more than the existing epsilon semantics (read the current loop — it has a `changed` flag on exact inequality); bound it by a mathematical failsafe (iterations × states upper bound for exact VI convergence on finite MDPs, or `states × actions + 1` sweeps with the standard contraction argument under discount < 1; for undiscounted reachability use the states+1 argument) — breach ⇒ `Err(Timeout)`, never silent success; `max_iterations` becomes advisory (document on the field, mirroring ticket 25's pattern).
2. Tests: (a) a chain MDP needing >512 sweeps under `max_iterations: 1` still converges to the optimal policy; (b) value-equality test against a hand-computed 3-state MDP (exact expected values); (c) undiscounted goal-probability MDP converges to 1.0 for a solvable retry loop.
3. Keep the strong/strong-cyclic solvers untouched; diffs confined to `probabilistic_policy` + doc comment + tests.

Gates: `cargo test -p ferroplan --lib planning_runtime && cargo test -p ferroplan --test planning_runtime` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | fix/probabilistic-iterations @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | fix/probabilistic-iterations @ 75870de (wt-d50) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
