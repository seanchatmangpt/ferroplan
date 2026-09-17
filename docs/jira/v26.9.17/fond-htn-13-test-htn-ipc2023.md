---
id: fond-htn-13-test-htn-ipc2023
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "IPC-2023 deterministic HTN pipeline suite (solve_hddl end-to-end)"
standing: BLOCKED
branch: test/htn-ipc2023
worktree: ~/ferroplan-worktrees/wt-test-htn
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. IPC competition files MAY be copied into repo fixtures (canonical data, not koala code).

## Scope
1. Copy the 13 curated IPC-2023 instances from `/tmp/fond-corpus/htn-ipc2023/` into `crates/ferroplan/tests/fixtures/htn-ipc2023/<domain>/` (read that corpus README first — includes its compat-watch quirks: `:ordered-subtasks`, `:tasks` keyword, `.pddl` suffix on Lamps, empty `(and)` subtask lists, PCP_1 recursion).
2. `crates/ferroplan/tests/htn_ipc2023.rs`: per instance, `solve_hddl` (signature per `crates/ferroplan/src/hddl.rs`) with ≤ 30 s wall budget. Assert the contract: solved-with-closed-policy (in-test checker; for solved plans also replay-check if the API exposes steps/states) OR honest typed NoPlan/limit error — never panic/hang/garbage. PCP_1: bounded exit is the assertion (TIMEOUT-equivalent typed error or NoPlan acceptable).
3. `RESULTS.md` committed alongside: per instance parse/ground/translate status, outcome, wall, plan length, first-failure error verbatim; SLOW flag > 10 s (CHANGELOG already admits IPC-2020-style blowups — measure ours).
4. Parser gaps: record verbatim, do NOT fix the parser on this branch.

## Gates
`cd ~/ferroplan-worktrees/wt-test-htn && cargo test -p ferroplan --test htn_ipc2023` exit 0; RESULTS.md with 13 rows.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/htn-ipc2023 @ base d2faf4d | — | all |
| 2026-09-17T22:05:00Z | BLOCKED→work | test/htn-ipc2023 @ d2faf4d | worktree clean; corpus README read (13 domains, compat-watch noted) | fixtures copy, htn_ipc2023.rs, RESULTS.md, gate |
| 2026-09-17T22:47:55Z | ALIVE | test/htn-ipc2023 @ c78d9a4 | cargo test -p ferroplan --test htn_ipc2023 exit 0 (13 pass/0 fail; capture run 1-thread exit 0, 43.9s) | none in scope; follow-ups: translate-wall blowups PO_Transport/Satellite-GTOHP/Transport + internal 10s TranslateLimits not caller-raisable |
