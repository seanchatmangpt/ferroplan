---
id: fond-htn-01-oracle-runner
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Formalize the koala oracle runner (flock-serialized, idempotent)"
standing: BLOCKED
worktree: none (/tmp only)
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. You touch ONLY `/tmp/fond-oracle/` — no repo, no worktree.

## Scope
Deliver the runner contract from the context doc:
1. `/tmp/fond-oracle/build.sh` — flock `/tmp/fond-oracle/.lock`; idempotent; verifies the three binaries exist and are executable; if missing, rebuilds per the macOS notes in the context doc (brew bison/flex/gengetopt; `make CXX=g++ CC=gcc`; bliss unzip→patch→sed→`make CC=g++`). Exit 0 when ready; print a one-line READY manifest (binary → sha256).
2. `/tmp/fond-oracle/oracle-run.sh <domain> <problem> [--mode M] [--timeout SEC] [--heur H]` — takes the same flock (serializes `solve.py`'s shared temp files); runs `python3 solve.py ...` with `--output` into `/tmp/fond-oracle/runs/<sha-of-args>/policy.txt`; enforces timeout (solve.py takes minutes — convert; also wrap with an outer wall kill as backstop); classifies and prints exactly ONE JSON line: status ∈ SOLVED (policy artifact non-empty / planner success output), NOSOLUTION, PARSE_ERROR (parser stage failed), ERROR, TIMEOUT. Capture full logs to `runs/<id>/log.txt`. A NOSOLUTION verdict exits 0.
3. Smoke gates: Transport pfile01 `--mode flexible` → SOLVED; a deliberately broken problem file (truncate pfile01 to 3 lines) → PARSE_ERROR; `--mode fixed-ld --timeout 60` on Transport pfile01 → some verdict with exit 0. Run each smoke twice to prove flock serialization works (launch two in parallel with `&`, both must return consistent verdicts).
4. `README.md` in /tmp/fond-oracle: usage, JSON schema, artifact layout, known quirks.

## Gates
Three smokes green + double-run serialization check. No repo changes (verify `git -C /Users/sac/ferroplan status --porcelain` unchanged apart from pre-existing untracked).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | none | — | all |
