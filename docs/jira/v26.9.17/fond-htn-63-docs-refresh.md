---
id: fond-htn-63-docs-refresh
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs: waves-4/5 refresh — FOND-HTN, README limits, CHANGELOG additions"
standing: ALIVE
branch: docs/waves45-refresh
worktree: ~/ferroplan-worktrees/wt-h63
created: 2026-09-18T02:50:00Z
---

Read `_WAVE4-CONTEXT.md`. All facts are on main at your base (75870de lineage) — no speculation about in-flight branches; where a hotfix ticket (57/58/60) will change a stated limit, keep the CURRENT behavior and note the ticket id inline.

Scope (docs only):
1. `docs/FOND-HTN.md`: budget-knobs table gains the grounded-caps mapping (ticket 43's `grounding_limits_from`, default-identical calibration); deviations/limits refreshed: multi-inheritance types now supported (ticket 44 union-of-parents), goal-eval depth budget 256 + GoalTooDeep (ticket 42), parser depth budget (ticket 22), fuzz wall exists (ticket 31: 2000-case, zero findings), property scale-up 5000-instance wall exists + its FOUND_BUG_2 status "fix in flight (ticket 57)".
2. `README.md` current-limits list: strike the two fixed ones (conditional-effect grounding landed via ticket 24; parse-depth via ticket 22), keep translate-cap and add the >1M-grounding note (ticket 60 in flight).
3. `CHANGELOG.md` [Unreleased]: append a "### Finished hardening" block covering: multi-parent type declarations; goal-evaluation + parser depth budgets; grounding caps plumbing; the 2000-case fuzz wall + 5000-instance property wall (FOUND_BUG_2 found — fix noted in flight); surface-facts only, every claim commit-backed.
4. Cross-check consistency: every number cited matches its committed RESULTS/test file (run scripts/verify_doc_claims.py if present at base, else manual).

Gates: `cargo test -p ferroplan --doc` exit 0; one atomic commit.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|

| 2026-09-18T02:50:00Z | BLOCKED | docs/waves45-refresh @ 75870de | — | all |
| 2026-09-18T03:05:00Z | ALIVE | docs/waves45-refresh @ 55e1997 | worktree clean, branch exclusive to wt-h63; ticket+contexts read | scope 1-4 |
| 2026-09-18T03:40:00Z | ALIVE | docs/waves45-refresh @ cd4a7a9 | `cargo test -p ferroplan --doc` exit 0 (1 passed/0 failed); `scripts/verify_evidence_ids.py` exit 0 (106 ids verifiable); manual claim cross-check 10/10 vs committed constants/RESULTS-wavec/test files (no verify_doc_claims.py at base); one atomic commit cd4a7a9, 3 files, +165/-34 | none — final: FOND-HTN.md (knobs table +caps mapping, 22/42/44 refreshes, walls subsection, Known gaps re-enumerated, 57/60 noted in flight), README (2 fixed limits struck, >1M note added), CHANGELOG (Finished-hardening block). Observed outside scope: CHANGELOG Known-limits rows for tickets 21/24 now describe pre-fix behavior (supersession note added; full rewrite left to a future docs ticket). Ticket-file History rows carried through finish-wave per convention. |
| 2026-09-18T23:30:00Z | ALIVE | merged cd4a7a9 into wave6/land-v26917 (merge f097293, A7); auto-merge clean | post-landing refresh committed 3d07d02: FOND-HTN/README/CHANGELOG 'in flight' notes updated to LANDED for 57/60/65 with one-line outcomes (60's default-envelope claim kept honestly partial) + wave-6 changelog bullets | none (A7) |