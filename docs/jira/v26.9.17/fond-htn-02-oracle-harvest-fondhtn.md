---
id: fond-htn-02-oracle-harvest-fondhtn
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Oracle harvest: koala FOND-HTN domains (6 usable × p01–05)"
standing: BLOCKED
worktree: none (/tmp only)
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. /tmp only; never copy koala files into any repo.

## Scope
1. Ensure harness: `bash /tmp/fond-oracle/build.sh`; if the runner is missing, wait-and-retry every 120 s for up to 20 min, then self-deliver a minimal flock wrapper yourself (context doc contract) — record which path you took.
2. Harvest: for each of Transport, Snake, Childsnack, Depots, Rover, Satellite in `/tmp/fond-review/domains/`, problems pfile01–pfile05: run `oracle-run.sh --mode flexible --timeout 180 --heur ff`. If verdict is NOSOLUTION, rerun `--mode fixed-ld` (classification evidence). AssemblyHierarchical: run domain+pfile01 through PARSE stage only — expect failure; record verbatim first error line (external negative case).
3. Expected watchpoints: Childsnack/Depots/Rover/Snake use `:ordered-subtasks` (does koala's own parser accept? it should — it's koala's dialect); Satellite uses method `:constraints` — record outcome verbatim.
4. Output `/tmp/fond-oracle/verdicts/fond-htn.json`: array of {domain, problem, mode, status, wall_s, artifact, note}. Plus `/tmp/fond-corpus/fond-htn-oracle-summary.md`: per-domain table + any NOSOLUTION cross-check result.

## Gates
≥ 28 recorded verdict lines (6×5 − expected parse failures still recorded as PARSE_ERROR) + summary file exists.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | none | — | all |
| 2026-09-17T21:52:00Z | BLOCKED | none | start: /tmp/fond-oracle absent, binaries present in /tmp/fond-review/Planner; probing build.sh per contract | harness, harvest 6×5 + AH parse case, verdicts+summary |
| 2026-09-17T21:39:43Z | PARTIAL_ALIVE | none | build.sh exit 0 — T01 runner delivered 21:39:28Z, fallback not needed; harvest launched (flexible/ff/180s, NOSOLUTION->fixed-ld) | 30 runs + AH parse case, verdicts+summary | 
| 2026-09-17T21:42:02Z | PARTIAL_ALIVE | none | AH negative case: parser exit 0 (no error line — expectation falsified); parser SILENTLY DROPS FaultyPort objects (14 refs in problem -> 0 in parsed model); grounder also exit 0. Verbatim outcome recorded, not fabricated error line | harvest in flight | 
