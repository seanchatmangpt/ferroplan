---
id: fond-htn-45-wave4-ledger
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: wave-4 ticket ledger hygiene (final rows, SHA manifest, runbook update)"
standing: BLOCKED
branch: docs/wave4-ledger
worktree: ~/ferroplan-worktrees/wt-d45
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. Ledger only — append-only edits inside docs/jira/v26.9.17/** on your branch.

Scope:
1. Read every wave-4 ticket's History table (21–40). For each: verify the final row is a real receipt (has gates+exits and a standing); append a coordinator-close row (ts | ALIVE/PARTIAL_ALIVE/BLOCKED-respawned | branch+tip-SHA from the agent reports below | "—" | "integration pending") for any ticket lacking one. Known from agent final reports: 21→fix/eq-goal-evaluation@24c94f6; 22→fix/parse-depth-budget@e272cbd; 24→fix/ground-conditional-effects@818e9ea; 25→fix/fond-loop-failsafes@7fea6cc; 26→bench/fond-criterion@979ba49; 27→bench/ipc-full-sweep@d7659ea; 28→bench/scaling-ladder@6a8ce08; 29→stress/concurrency-wasm@aae68be; 30→stress/memory-ceilings@673370c; 32→fuzz/api-panic-hunt@415b7d0; 35→docs/fond-htn-wave4@f0034fb; 36→docs/benchmarks-consolidated@6bf31c9; 37→docs/readme-capability@d1fbfb5; 38→docs/book-fond-htn@e67a37d; 39→docs/readiness-refresh@1576abc (PARTIAL_ALIVE); 40→docs/changelog-0.28-draft@8ead69a. Tickets 23/31/33/34: standing stays respawn-in-flight (finish-wave owns them) — append a row saying exactly that.
2. `_RUNBOOK.md`: append the finish-wave dispatch table (tickets 41–56 + respawns) in the established format, with the branch/worktree map you find in the finish-wave tickets.
3. `docs/jira/v26.9.17/_WAVE4-BRANCH-MANIFEST.md`: the 16 frozen SHAs (list above) + merge-order note, as the integration record.

Gates: python sanity script (your own, run once): every ticket file 21–40 parses, has ≥2 History rows, final row has a standing token. Exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | docs/wave4-ledger @ 6d14813 | — | all |
| 2026-09-18T01:34:15Z | PARTIAL_ALIVE | docs/wave4-ledger @ 6fc8f01 (wt-d45) | orient: ticket + both wave contexts read; 20 wave-4 ticket Histories read; runbook finish-wave table verified row-by-row vs tickets 41–56 frontmatter — 20/20 match, table pre-exists at 6fc8f01 (cut commit), no duplicate append; main-checkout sibling rows read-only (row counts identical to committed state; live RESPAWN rows on 23/34 uncommitted on main) | coordinator-close rows 21–40; _WAVE4-BRANCH-MANIFEST.md; gate; commit |
| 2026-09-18T01:36:00Z | ALIVE | docs/wave4-ledger @ <this commit> | gate: `python3 /tmp/verify_wave4_ledger.py` exit 0 (20/20 ticket files parse, ≥2 History rows each, final row standing tokens; 16 close rows carry frozen branch@SHA + "integration pending"; 4 respawn-in-flight rows verified); deliverables: coordinator-close rows appended to tickets 21–40 (16 ALIVE — 39 PARTIAL_ALIVE — with frozen tips 24c94f6/e272cbd/818e9ea/7fea6cc/979ba49/d7659ea/6a8ce08/aae68be/673370c/415b7d0/f0034fb/6bf31c9/d1fbfb5/e67a37d/1576abc/8ead69a; 23/31/33/34 respawn-in-flight rows as ordered); new `docs/jira/v26.9.17/_WAVE4-BRANCH-MANIFEST.md` (16 frozen SHAs, fond-htn-41 merge order + conflict notes, respawn exclusions, numbers-fill sources); `_RUNBOOK.md` finish-wave table verified 20/20 vs tickets 41–56 frontmatter, pre-existing at 6fc8f01 — no edit (never duplicate a ledger table) | none — ledger closed for integration (fond-htn-41 merges the 16; coordinator lands respawns post-finish-wave) |
| 2026-09-18T03:02:00Z | ALIVE | docs/wave4-ledger @ 121786f | — | coordinator-close: integrated by coordinator at 75870de lineage (merge 874a438); tip 121786f resolves this ticket's final-row "@ <this commit>" placeholder; readiness union fond=17/hddl=20 | none — closed |
