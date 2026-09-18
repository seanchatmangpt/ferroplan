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
| 2026-09-18T01:03:00Z | PARTIAL_ALIVE | none (main checkout read-only, /tmp work) | oriented: ledger has 31 recs (18 SOLVED/12 TIMEOUT@90s/1 UNSUPPORTED); TIMEOUT dozen identified; oracle binaries verified on disk | harvest 6x5 @120s; timeout-dozen rerun @300s; ferroplan solve_hddl cross-diff; extension summary |
| 2026-09-18T01:35:00Z | PARTIAL_ALIVE | none (/tmp work) | RESPAWN oriented: prior agent left harvest-fondhtn-ext.sh (scope-1, 14/~30 lines done in fond-htn-ext.jsonl), harvest-fondhtn-ext2.sh (scope-2 koala @300s, not run), ferroplan-timeout12.sh + /tmp/fond-ext-driver driver (scope-2 ferroplan, not run), merge-ext-verdicts.py (ledger merge, not run). Verifying driver vs repo solve_hddl API, then resuming scope-1 | finish scope-1 harvest; run ext2 @300s; run ferroplan timeout12 @60s walls; merge ledger; write /tmp/fond-corpus/fond-htn-oracle-extension.md |
| 2026-09-18T01:53:12Z | PARTIAL_ALIVE | none (/tmp work) | GATE-1 scope-1 harvest DONE: fond-htn-ext.jsonl 30 recs (8 SOLVED / 22 TIMEOUT @120s / 0 NOSOLUTION -> no fixed-ld reruns). Driver rebuilt vs repo HEAD ferroplan v0.27.1 (cargo build rc=0, 22.97s) — API match compile-proven; smoke: koala SOLVED Transport pfile01 0.165s, ferroplan ERROR translate memory cap 200k states (known T23 defect, honest verdict, will be reported). Exit 0 | ext2 koala @300s; ferroplan timeout12 @60s; merge; extension summary || 2026-09-18T01:36:00Z | respawn-in-flight | none (/tmp only) | — | standing stays respawn-in-flight — finish-wave owns it |
| 2026-09-18T03:02:00Z | BLOCKED | none (/tmp only) | — | coordinator-close: superseded by fond-htn-66 (oracle harvest, 3rd attempt, MINIMAL: p06-p10 x 6 domains @90 s, wave 6) | fond-htn-66 owns |
