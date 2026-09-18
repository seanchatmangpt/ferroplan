---
id: fond-htn-66-oracle-harvest-min
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Harvest (3rd attempt, MINIMAL): koala p06–p10 × 6 domains, 90 s, nothing else"
standing: BLOCKED
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
