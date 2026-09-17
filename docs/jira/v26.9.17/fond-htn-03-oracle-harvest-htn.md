---
id: fond-htn-03-oracle-harvest-htn
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Oracle harvest: deterministic HTN (IPC-2023 ×13, IPC-2020 ×4, PANDA ×4, SHOP3 ×4)"
standing: BLOCKED
worktree: none (/tmp only)
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. /tmp only.

## Scope
1. Ensure harness per T02's protocol.
2. Harvest over: (a) the 13 instances in `/tmp/fond-corpus/htn-ipc2023/` (see its README for exact domain/problem paths); (b) the 4 IPC-2020 curated pairs in `/tmp/fond-corpus/htn-other/curated/` (Childsnack p01, Elevator-Learned s01-0, Blocksworld-GTOHP p01, Entertainment pfile01); (c) the 4 PANDA feature tests (empty, interleaving, method-effect, conditional-effect — note: methods-with-effects and `when` effects are outside koala's own HDDL dialect; record PARSE_ERROR verbatim if so); (d) the 4 SHOP3 hand-ports (swap, ab-ordering, loan-credit, loan-noplan — loan-noplan is expected NOSOLUTION).
3. Modes: `--mode fixed-ld --timeout 120 --heur add`, then `--mode flexible --timeout 120` only where fixed-ld did not solve.
4. Output `/tmp/fond-oracle/verdicts/htn.json` (same schema as T02) + `/tmp/fond-corpus/htn-oracle-summary.md`. Watch PCP_1 (recursive) — TIMEOUT is an honest verdict, record it.

## Gates
≥ 25 recorded verdict lines + summary exists.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | none | — | all |
