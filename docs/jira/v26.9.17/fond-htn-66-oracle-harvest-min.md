---
id: fond-htn-66-oracle-harvest-min
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Harvest (3rd attempt, MINIMAL): koala p06–p10 × 6 domains, 90 s, nothing else"
standing: PARTIAL_ALIVE
worktree: none (/tmp only)
created: 2026-09-18T02:50:00Z
supersedes: fond-htn-34 scope (cross-diff leg dropped — coordinator does it from the verdicts)
---

Read `_WAVE4-CONTEXT.md`. Two prior agents died on the wider scope. MINIMAL scope, strictly:

1. `bash /tmp/fond-oracle/build.sh` (exit 0 — harness exists).
2. For each of Transport, Snake, Childsnack, Depots, Rover, Satellite in /tmp/fond-review/domains/: pfile06–pfile10 via `/tmp/fond-oracle/oracle-run.sh --mode flexible --timeout 90`. 30 runs, serialized (the runner flocks).
3. Append each verdict to `/tmp/fond-oracle/verdicts/fond-htn.json` (new records only, schema as T02's) and write `/tmp/fond-corpus/fond-htn-oracle-extension.md` — one table (domain/problem/status/wall) + a 3-sentence summary. NO ferroplan runs, NO re-runs of p01–p05, NO fixed-ld cross-checks.

Done. If a single run hangs beyond its 90 s + overhead, record TIMEOUT and move on.

Gates: 30 new verdict records + summary file exists.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|

| 2026-09-18T02:50:00Z | BLOCKED | none | — | all |
| 2026-09-18T03:05:00Z | PARTIAL_ALIVE | none (/tmp only) | attempt-3 oriented: fond-htn.json 31 recs (T02, p01-p05, unmerged by f34); no oracle procs running; f34 left 120s/300s ext jsonl (different timeout -> different run dirs, no collision). Launching MINIMAL: build.sh gate then 30 fresh flexible/ff/90s runs | build gate, 30 runs @90s, merge 30 new recs into fond-htn.json (T02 schema), /tmp/fond-corpus/fond-htn-oracle-extension.md |
| 2026-09-18T03:33:00Z | PARTIAL_ALIVE | none (/tmp only) | GATE-2 harvest DONE: 30/30 flexible/ff/90s exit 0 (8 SOLVED / 22 TIMEOUT, zero NOSOLUTION/PARSE_ERROR/ERROR, no run hung past wall+overhead; worst wall 110.8s incl. serialize tail). GATE-3 merge DONE: fond-htn.json 31->61 recs, first 31 verified content-identical (append-only), new 30 in exact T02 schema; raw lines fond-htn-min.jsonl; script harvest-fondhtn-min.sh | summary file /tmp/fond-corpus/fond-htn-oracle-extension.md |
| 2026-09-18T03:38:00Z | ALIVE | none (/tmp only) | GATE-4 summary DONE: /tmp/fond-corpus/fond-htn-oracle-extension.md written (30-row table domain/problem/status/wall + totals + 3-sentence summary; TIMEOUT wall range 90.0-110.8s verified vs raw). All ticket gates green: 30 new verdict recs + summary exists. Deliverables: fond-htn.json (61 recs, append-only verified), fond-htn-min.jsonl, harvest-fondhtn-min.sh, fond-htn-oracle-extension.md. No repo files touched besides this ticket; no koala files copied to any repo | none (awaiting coordinator) |
| 2026-09-18T23:30:00Z | PARTIAL_ALIVE (downgraded by observation) | none (/tmp only) | was ALIVE 03:38Z per History (30/30 runs, 61 recs, summary file). THIS SESSION: /tmp wiped — /tmp/fond-oracle, /tmp/fond-review, /tmp/fond-corpus ALL ABSENT; deliverables unverifiable, no repo files to merge | coordinator: rebuild oracle harness (context build notes) if verdicts must survive; otherwise record the wipe in the wave receipt |