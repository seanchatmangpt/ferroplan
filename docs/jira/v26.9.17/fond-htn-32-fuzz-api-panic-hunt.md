---
id: fond-htn-32-fuzz-api-panic-hunt
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fuzz: public-API panic hunt — structured garbage at every pub fn"
standing: BLOCKED
branch: fuzz/api-panic-hunt
worktree: ~/ferroplan-worktrees/wt-s32
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`.

Scope:
1. Enumerate the public surface: every `pub fn` in `ferroplan-hddl` (parse/validate/ground/translate/probabilistic), `ferroplan::solve_hddl`, `ferroplan::solve_planning_type` (all 18 PlanningTypes), `ferroplan::solve_ppddl`/`validate_ppddl_policy`, `eve::Eve::enter`, `readiness::capability_manifest`/`evaluate_readiness`, `route_planning_request`. Build a call-target table in the test file.
2. Structured garbage per signature type: empty strings, whitespace-only, unicode, 1 MB strings, deeply-nested JSON values (serde inputs), NaN/∞ where floats, zero/negative ints in Options/Limits, `PlannerLimits { max_states: 0, max_depth: 0, max_iterations: 0, max_wall_ms: 0 }`, empty `PlanningProblem` fields, unicode task ids colliding with keywords. Each wrapped in catch_unwind — a panic is a FINDING.
3. Assertions: Result-returning fns return Err (typed) or Ok — never panic; budget-bounded walls on every call (≤ 5 s each; total ≤ 120 s).
4. Findings: minimized + committed as `#[ignore]`d reproducers + History rows (these feed tickets like 22); fix NOTHING in src unless it is a one-line obvious panic→typed conversion, listed explicitly.

Gates: `cargo test -p ferroplan --test api_panic_hunt` exit 0 (findings ignored-listed); `cargo test -p ferroplan-hddl` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | fuzz/api-panic-hunt @ 90c2ae2 | — | all |
