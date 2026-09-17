# fond-htn-oracle-summary — koala FOND-HTN oracle harvest (ticket fond-htn-02)

day: v26.9.17-fond-htn-hardening | oracle: koala at `/tmp/fond-review/Planner` via `/tmp/fond-oracle/oracle-run.sh` (flock-serialized)
run params: `--mode flexible --timeout 90 --heur ff` (coordinator-adjusted scope), 2026-09-17T23:11–23:29Z
raw verdicts: `/tmp/fond-oracle/verdicts/fond-htn.json` (31 lines) + `fond-htn.jsonl` (incremental) + `fond-htn-ah.jsonl` (AH case)
per-run logs + policies: `/tmp/fond-oracle/runs/<sha-of-args>/{log.txt,policy.txt}`

## Per-domain results (18 SOLVED / 12 TIMEOUT / 0 NOSOLUTION / 0 PARSE_ERROR)

| Domain | Problems used | SOLVED | TIMEOUT | NOSOLUTION | notes |
|---|---|---|---|---|---|
| Transport | pfile01–05 | 4 | 1 | 0 | p05 hits 90 s wall mid-search |
| Snake | pb01–05.snake (pfile01–05 mapped to snake pb series) | 3 | 2 | 0 | pb03, pb05 timeout |
| Childsnack | p01–05 | 1 | 4 | 0 | only p01 solves; p02–05 exhaust wall |
| Depots | p01–05 | 2 | 3 | 0 | p01–02 solve in <1 s |
| Rover | pfile01–05 | 3 | 2 | 0 | p03, p05 timeout |
| Satellite | 1obs-1sat-1mod, 1obs-2sat-1mod, 2obs-1sat-1mod, 2obs-1sat-2mod, 2obs-2sat-1mod (pfile01–05 mapped to first five sorted instances) | 5 | 0 | 0 | all solve in <1 s |
| AssemblyHierarchical | genericLinearProblem_depth01 (parse stage only) | — | — | — | **UNSUPPORTED** — see negative case below |

## Watchpoint outcomes

- **`:ordered-subtasks` (Childsnack/Depots/Rover/Snake)**: koala's own parser accepts its dialect — all four domains parse and reach the planner; verdicts are search outcomes (SOLVED/TIMEOUT), never PARSE_ERROR.
- **Satellite method `:constraints`**: parsed cleanly, zero "Parse error" occurrences in logs, all five instances SOLVED sub-second. Verbatim outcome: koala accepts `:constraints (and ...)` inside `(:method ...)` without diagnostic.
- **TIMEOUTs are genuine search exhaustion**: e.g. Transport p05 log ends `[AO*] explored=2240000 | nodes=6482436 | depth=61 | 89.6s` — AO* still expanding at the 90 s wall kill; every TIMEOUT has wall_s ≈ 90.0–90.5. These are "wall exhausted", NOT NOSOLUTION; do not cite them as unsolvability evidence.
- **NOSOLUTION cross-check (fixed-ld)**: not triggered — zero NOSOLUTION verdicts in the whole sweep, so per the adjusted scope no fixed-ld rerun ran.

## AssemblyHierarchical external negative case (parse stage only)

Ticket expected a PARSE_ERROR with a verbatim first error line. **Expectation falsified — koala does not reject this broken model; it silently mis-compiles it.**

- `pandaPIparser AssemblyHierarchical/domain.hddl genericLinearProblem_depth01.hddl` → exit 0, no error line (stderr is only the config banner).
- The domain declares **no `FaultyPort` type**, while the problem types objects `faultyCable-a - FaultyPort`, `faultyCable-b - FaultyPort` (2 refs in depth01; prior agent measured 14 refs on another depth instance) → parsed model contains **0** `FaultyPort` refs: objects silently dropped.
- `pandaPIgrounder` on the parsed model → exit 0.
- Recorded as status `UNSUPPORTED` in `fond-htn.json` (evidence files: `/tmp/fond-oracle/ah_parse.err`, `/tmp/fond-oracle/ah_parsed.out`, `/tmp/fond-oracle/ah_ground.err`). Implication for ferroplan: a parity test must assert **rejection** (undeclared type error at parse/validate), not mere non-crash — koala's silent drop is the anti-example.

## Harvest reliability notes

- All 30 runs + AH case serialized through the flock-protected runner (`/tmp/fond-oracle/.lock`); the runner's stale-shared-state pre/post-clean was active (patched 22:20Z after a prior TIMEOUT left stale `result.json`).
- One harvest-script defect during the run: initial launch omitted `.hddl` extensions → 0 runs, fixed, relaunched (no verdict impact; failure was at the runner's file-exists gate, before any oracle work).
- Problem-file naming is per-domain (`pfile0N` / `pb0N.snake` / `p0N` / named Satellite instances); mapping is recorded per row in the JSON `problem` field.

## Gate

≥ 28 recorded verdict lines: **31** (30 harvest + 1 AH) ✓ — summary file exists ✓
