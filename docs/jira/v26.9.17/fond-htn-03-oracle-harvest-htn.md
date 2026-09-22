---
id: fond-htn-03-oracle-harvest-htn
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Oracle harvest: deterministic HTN (IPC-2023 ×13, IPC-2020 ×4, PANDA ×4, SHOP3 ×4)"
standing: ALIVE
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
| 2026-09-17T21:33:00Z | PARTIAL_ALIVE | none (/tmp only) | ticket+context read | harness check pending |
| 2026-09-17T21:49:22Z | PARTIAL_ALIVE | none (/tmp only) | harness: T01 landed, build.sh exit 0; smokes: Transport fixed-ld+add TIMEOUT@120s exit0, flexible SOLVED 0.3s, truncated->ERROR (reclassified PARSE_ERROR via verbatim "Parse error" log line; T01 script untouched) | harvest 25 running |
| 2026-09-17T22:18:20Z | PARTIAL_ALIVE | none (/tmp only) | harvest complete: 26 verdict lines (25 instances, loan-noplan flexible rerun); 24 SOLVED / 2 TIMEOUT; all receipts verified on disk | assembly + gate |
| 2026-09-17T22:18:20Z | ALIVE | none (/tmp only) | gates: 26>=25 verdict lines exit0; htn.json valid T02-schema array exit0; summary exists exit0; repo untouched except ticket appends | none — findings: loan-noplan NOSOLUTION UNCONFIRMED (TIMEOUT both modes, 120s); PANDA PARSE_ERROR expectations falsified (koala accepts+ solves all 4); PCP_1 SOLVED under fixed-LD bounding (artifact prob 1.0000); T01 runner updated mid-wave, receipts per-run consistent |
