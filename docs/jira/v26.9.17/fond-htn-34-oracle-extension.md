---
id: fond-htn-34-oracle-extension
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Oracle extension: koala p06–p10 harvest + ferroplan cross-diff on the TIMEOUT dozen"
standing: BLOCKED
worktree: none (/tmp only)
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. /tmp only; never copy koala files into any repo.

Scope:
1. Harvest: koala oracle (`/tmp/fond-oracle/oracle-run.sh`) over the 6 usable domains × pfile06–pfile10 from `/tmp/fond-review/domains/`, mode flexible, `--timeout 120`. Append to `/tmp/fond-oracle/verdicts/fond-htn.json` (new records, never mutate old).
2. Cross-diff the wave-3 TIMEOUT dozen (12 instances at 90 s): rerun each at `--timeout 300`; still-TIMEOUT is an honest verdict (koala = strong-only). THEN run ferroplan `solve_hddl` on the same files (read its API from the repo; generous walls 60 s) and record where ferroplan solves what koala could not decide — the migration's differentiator evidence.
3. `/tmp/fond-corpus/fond-htn-oracle-extension.md`: per-instance table both engines + a summary paragraph ("ferroplan decided N of the 12 koala TIMEOUTs: S solved / R refused").

Gates: ≥ 30 new verdict records + extension summary exists; every record has artifact + wall.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | none | — | all |
