---
id: fond-htn-33-property-scaleup
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Property scale-up: 5000 instances, ≤12 states, Phase-3 regression dimension"
standing: BLOCKED
branch: test/property-scaleup
worktree: ~/ferroplan-worktrees/wt-p33
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Precedent: `tests/fond_property.rs` (320 instances, 3–8 states, policy-enumeration reference, FOUND_BUG_1).

Scope:
1. Extend the generator: 5000 instances, 3–12 states, 2–4 actions, 1–4 outcomes per (state,action), optional unsafe states, optional multiple initial states (dispatcher requires ALL initials winning — reference must match), and a dedicated 25% "decoy" mixture rich in closed-but-goal-unreachable loops (the Phase-3 defect class — regression dimension).
2. Policy enumeration at 12 states = 4^12 ≈ 16M worst case — too big: cap enumeration at ≤ 3^10 by construction (limit action-choices per state to 3 for states>8, documented), or implement the reference as an independent label-correcting SCC computation instead of enumeration — your choice, but it must remain algorithmically independent of the solver's fixpoints (no copying the solver's code into the reference).
3. Assertions as before: solvability equivalence (strong ∪ strong-cyclic), outcome closure, determinism; wall ≤ 60 s total.
4. Any mismatch: shrink to ≤ 5 states, commit `#[ignore]`d + History (feeds the next hotfix, not this ticket).

Gates: `cargo test -p ferroplan --test fond_property_scaleup` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | test/property-scaleup @ 90c2ae2 | — | all |
| 2026-09-18T00:57:30Z | PARTIAL_ALIVE | test/property-scaleup @ 6d14813 | worktree clean @ 6d14813 (base 90c2ae2); contexts+ticket read; solver contract read (dispatch L373, fond_policy L761, fond_policy_strong_cyclic L862, validate L415: FOND mass-range-only) | scale-up test file, gate run, commit |
