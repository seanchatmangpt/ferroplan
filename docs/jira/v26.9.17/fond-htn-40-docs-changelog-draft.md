---
id: fond-htn-40-docs-changelog-draft
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs: aggregate the FOND-HTN line into CHANGELOG Unreleased"
standing: BLOCKED
branch: docs/changelog-0.28-draft
worktree: ~/ferroplan-worktrees/wt-d40
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`.

Scope:
1. Read `git log --oneline d2faf4d..HEAD` and the existing Unreleased section; write a consolidated Unreleased block (house voice — see 0.27.0/0.27.1 entries for register) covering: native HDDL front-end (parse/ground/translate), solve_hddl + wasm ops, strong + strong-cyclic solvers incl. the Phase-3 reachability fix, validation hardening, eve bridge + manifest admission, the test walls (canonical suites, property equivalence, adversarial, differential goldens), and honest current limits (translate cap, conditional-effect grounding, parse depth).
2. No version cut — Unreleased only; no dates; no claims without a commit or RESULTS file behind them.
3. Keep the existing Unreleased hddl-hardening bullet — integrate, don't duplicate.

Gates: `cargo test -p ferroplan --doc` exit 0; one atomic commit; markdown lint by eye (repo has no md linter).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | docs/changelog-0.28-draft @ 90c2ae2 | — | all |
