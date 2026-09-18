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
| 2026-09-18T01:12:00Z | PARTIAL_ALIVE | fuzz/api-panic-hunt @ 6d14813 (worktree wt-s32; base 90c2ae2 is ancestor — branch pre-cut at wave-4 main, docs-only delta) | call-target table built: 30 targets = 20 ferroplan-hddl pub fns + solve_hddl + solve_planning_type(×18) + solve_ppddl + validate_ppddl_policy + Eve::enter + capability_manifest + evaluate_readiness + route_planning_request + serde inputs; hunt test written (catch_unwind per call, ≤5 s asserted walls, seeded xorshift, garbage classes S/D/P/G/I/U/F/L/V/E/R/J); exploratory probes (throwaway file, since deleted) found ONE finding: grounder::evaluate_ground_goal / action_applicable recurse with no depth budget — depth 10k Not/And chain evaluates, depth 50k SIGABRTs (stack overflow, not catchable); text-reachable depth (300-deep pre) grounds fine, so parse threshold (~500, ticket 22) is hit first on the text path — the finding is reachable via the programmatic pub API | gate1, gate2, commit |
| 2026-09-18T01:15:00Z | PARTIAL_ALIVE | fuzz/api-panic-hunt @ 415b7d0 | gate1: `cargo test -p ferroplan --test api_panic_hunt` → exit 0 (15 passed, 0 failed, 2 ignored = the finding reproducers, committed #[ignore]d with measured 10k-ok/50k-abort boundary); gate2: `cargo test -p ferroplan-hddl` → exit 0 (138 unit + 9 doc, 0 failed); reproducer faithfully SIGABRTs when run `--ignored` (証); no src changes — fix is an explicit-stack rewrite, not a one-line conversion | report |
| 2026-09-18T01:15:30Z | ALIVE | fuzz/api-panic-hunt @ 415b7d0 | gates as above, both exit 0; deliverables: crates/ferroplan/tests/api_panic_hunt.rs (1318 lines: call-target table doc + 15 active tests + 2 #[ignore]d finding reproducers), crates/ferroplan/Cargo.toml (+serde_json dev-dep for JSON-garbage targets); finding feeds a follow-up hardening ticket in the ticket-22 recursion class (evaluate_ground_goal depth budget); operator wrote NOTHING on 産面 except the mandated test/manifest bytes | none for this ticket; follow-up ticket for FINDING-1 |
