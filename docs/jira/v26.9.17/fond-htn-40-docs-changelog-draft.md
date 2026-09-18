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
| 2026-09-18T00:20:00Z | PARTIAL_ALIVE | docs/changelog-0.28-draft @ 6d14813 (worktree wt-d40) | evidence pass done: git log d2faf4d..HEAD read; 2× RESULTS.md, oracle-harvest-full.json (31 runs), fond_property.rs (320-instance wall), hddl_adversarial.rs (27 cases), commits 28e525d/0e28182/956e545 verified | draft Unreleased block; gate; commit |
| 2026-09-18T00:35:00Z | ALIVE | docs/changelog-0.28-draft @ 8ead69a (worktree wt-d40) | cargo test -p ferroplan --doc → exit 0 (1 passed / 0 failed); md lint by eye; atomic commit 8ead69a (CHANGELOG.md +190/−24); Unreleased consolidated (front-end, solve_hddl+wasm ops, strong/strong-cyclic incl. Phase-3 FOUND_BUG_1 fix, validation, Eve bridge+manifest, 4 test walls, limits 21/22/23/24); hddl-hardening bullet integrated into Hardened, not duplicated; no version cut, no dates, every claim behind a commit/RESULTS/golden | none — ready for coordinator integration (never merge into main myself) |
