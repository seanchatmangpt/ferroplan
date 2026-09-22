---
id: fond-htn-64-wave6-ledger
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: wave-5/6 ledger close (final rows + branch manifest + runbook completion)"
standing: ALIVE
branch: docs/wave6-ledger
worktree: ~/ferroplan-worktrees/wt-h64
created: 2026-09-18T02:55:00Z
---

Read `_WAVE4-CONTEXT.md`. Same job ticket 45 did for wave 4 — ledger only, append-only, on your branch.

Scope:
1. For every wave-5 ticket (41–56): read its History; append a coordinator-close row ONLY where the final row is not already a real receipt — known outcomes: 41 ALIVE (main 9ae3f08 lineage, now superseded by coordinator integration at 75870de — say so); 42/43/44/45/46 ALIVE (tips 85a2fe0/3b8ee6e/8f587af/121786f/113d00e — note "integrated by coordinator at 75870de lineage, readiness union fond=17/hddl=20"); 31/33 respawn-ALIVE (tips 0479c1e/26ae36f — same integration note); 23/34 superseded by tickets 65/66 (say exactly that); 47–56 second-attempt in flight (wave 6).
2. `_WAVE6-BRANCH-MANIFEST.md`: the wave-6 dispatch table (tickets 57–66 + respawns 47–56) with branch + base SHA 75870de, mirroring ticket 45's manifest format.
3. Runbook wave-6 table: verify it matches the actual tickets (committed already); fix drift if any.

Gates: same sanity script style as ticket 45 — every ticket file 41–56 parses, ≥2 History rows, final row has a standing token. Exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:55:00Z | BLOCKED | docs/wave6-ledger @ 75870de | — | all |
| 2026-09-18T02:57:00Z | PARTIAL_ALIVE | docs/wave6-ledger @ 6cacbda (wt-h64) | orient: ticket + _FOND-HTN-WAVE-CONTEXT.md + _WAVE4-CONTEXT.md read; wave-5 Histories (23/31/33/34, 41-56) read read-only from main (committed main state == branch state for the whole edit set; live uncommitted main rows confined to 57/60/63/65/66 — zero overlap); known outcomes verified in git BEFORE writing: 7 integration merges f0b8b9c/833994f/0f39d2d/8e54704/2377802/874a438/1a6c393 -> tips 85a2fe0/3b8ee6e/8f587af/0479c1e/26ae36f/121786f/113d00e; readiness pins fond=17/hddl=20 verified at readiness.rs:1214-1215 @ 75870de; respawn worktrees wt-d47..wt-d56 verified on-branch @ 75870de; 45 precedent checked (close rows land even where agent receipts exist — ticket 22) | close rows 20 files; _WAVE6-BRANCH-MANIFEST.md; runbook check; gate; commits |
| 2026-09-18T03:06:00Z | ALIVE | docs/wave6-ledger @ f96ef6f (+ this commit) | gate: `python3 /tmp/verify_wave6_ledger.py` exit 0 (24 ticket files parse — 41-56 required + 23/31/33/34 touched; >=2 History rows each; final rows carry standing tokens incl. respawn-in-flight per the 45 precedent; runbook wave-6 table 19/19 vs ticket frontmatter — zero drift, no edit per never-duplicate-a-ledger-table; manifest carries base 75870de + 19 dispatch lanes + 7 integrated tips + readiness union fond=17/hddl=20); falsifier: last-row deletion on ticket 47 -> gate exit 1, restored -> exit 0; deliverables @ f96ef6f: coordinator-close rows appended to 23 (BLOCKED — superseded by fond-htn-65, said exactly), 34 (BLOCKED — superseded by fond-htn-66), 31/33 (ALIVE respawn tips 0479c1e/26ae36f, integrated at 75870de lineage, 33's FOUND_BUG_2 hotfix linked to fond-htn-57), 41 (ALIVE — 9ae3f08 receipt stands, superseded by coordinator integration at 75870de), 42-46 (ALIVE integrated tips 85a2fe0/3b8ee6e/8f587af/121786f/113d00e — 121786f resolves 45's "@ <this commit>" placeholder; readiness union fond=17/hddl=20), 47-56 (respawn-in-flight — second attempt in flight, wave 6, worktrees verified @ 75870de); new docs/jira/v26.9.17/_WAVE6-BRANCH-MANIFEST.md (wave-5 integration record + wave-6 dispatch table, mirrors the wave-4 manifest) | none — wave-5/6 ledger closed; coordinator owns wave-6 serial integration |
| 2026-09-18T23:30:00Z | ALIVE | merged 78e220f into wave6/land-v26917 (merge 809fce5, A7) | ledger rows carried; A7 landing rows appended in worktree copies of tickets (coordinator unions with main checkout) | none (A7) |
