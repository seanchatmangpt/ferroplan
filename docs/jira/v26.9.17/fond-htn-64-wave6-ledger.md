---
id: fond-htn-64-wave6-ledger
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: wave-5/6 ledger close (final rows + branch manifest + runbook completion)"
standing: BLOCKED
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
