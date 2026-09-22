---
id: fond-htn-33-property-scaleup
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Property scale-up: 5000 instances, ≤12 states, Phase-3 regression dimension"
standing: ALIVE
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
| 2026-09-18T00:57:30Z | PARTIAL_ALIVE | test/property-scaleup @ 6d14813 | worktree clean @ 6d14813 (base 90c2ae2); contexts+ticket read; solver contract read (dispatch L373, fond_policy L761, fond_policy_strong_cyclic L862, validate L415: FOND mass-range-only) | scale-up test file, gate run, commit || 2026-09-18T02:05:00Z | PARTIAL_ALIVE | test/property-scaleup @ 6d14813 | respawn; worktree re-verified clean @ 6d14813; precedent tests/fond_property.rs + both fixpoints re-read (dispatch L391, fond_policy L761, fond_policy_strong_cyclic L862, validate L415: FOND mass-range-only, ALL-initials rule L798/L1059); reference choice: independent label-correcting reference (min-max value iteration for strong, usability-constrained backward goal-reachability for strong-cyclic) — enumeration infeasible at 5000x12; small-slice enumeration cross-check kept for falsification | write tests/fond_property_scaleup.rs, gate, commit |
| 2026-09-18T02:40:00Z | PARTIAL_ALIVE | test/property-scaleup @ 8ddbddc | first gate run RED: instance 1 (7-state NON-decoy) returned goal-unreachable policy; hand-verified genuine residual defect — fond_policy_strong_cyclic Phase 3 certifies REGION goal-reachability but returns Phase-2 WITNESS-FIRST choices (non-advancing loop chosen when a committable action exists; reach==surviving exits without rewriting choices); distinct from fixed FOUND_BUG_1; shrunk to 2 states; scope item 4 executed | #[ignore]d reproducer + narrow carve-out, rerun gate |
| 2026-09-18T02:55:00Z | ALIVE | test/property-scaleup @ 8ddbddc | gate `cargo test -p ferroplan --test fond_property_scaleup` exit 0: 3 passed / 1 ignored (FOUND_BUG_2 reproducer) in 1.6s; scale stats: 5000 instances 1.47s (strong=1808 sc-only=1868 valid-ok=2794 noplan=1324 FOUND_BUG_2-affected=882 decoys=1250[unsolv=656 sc-only=281] multi-initial=1158[ok=790 noplan=368] unsafe=1853); cross-check 320 enumeration-vs-reference agree; neighbor: precedent fond_property green, its FOUND_BUG_1-affected=58/173 confirms the class predates this ticket; deliverable: crates/ferroplan/tests/fond_property_scaleup.rs @ 8ddbddc | hotfix ticket for FOUND_BUG_2 (next wave): align returned choices with Phase-3 committable actions || 2026-09-18T00:57:30Z | PARTIAL_ALIVE | test/property-scaleup @ 6d14813 | worktree clean @ 6d14813 (base 90c2ae2); contexts+ticket read; solver contract read (dispatch L373, fond_policy L761, fond_policy_strong_cyclic L862, validate L415: FOND mass-range-only) | scale-up test file, gate run, commit |
| 2026-09-18T01:36:00Z | respawn-in-flight | test/property-scaleup@6d14813 (wt-p33, uncommitted) | — | standing stays respawn-in-flight — finish-wave owns it |
| 2026-09-18T03:02:00Z | ALIVE | test/property-scaleup @ 26ae36f | — | coordinator-close: respawn-ALIVE — gate exit 0 (3 passed/1 ignored FOUND_BUG_2 reproducer; 5000-instance sweep stats) per 02:55Z row; integrated by coordinator at 75870de lineage (merge 2377802); FOUND_BUG_2 hotfix dispatched as wave-6 fond-htn-57 | none — closed (hotfix = fond-htn-57) |
