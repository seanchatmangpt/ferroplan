# RUNBOOK — v26.9.17 FOND-HTN hardening wave (20 tickets, 20 agents, one message)

Canonical dispatch form (agents receive ONLY ticket path + worktree; the ticket +
`_FOND-HTN-WAVE-CONTEXT.md` carry everything else):

> You are working ticket `<docs/jira/v26.9.17/<file>>`. Read that ticket and
> `docs/jira/v26.9.17/_FOND-HTN-WAVE-CONTEXT.md` FIRST. Work in `<worktree | /tmp>`.
> Append a History row at start / after each gate / at end (append-only, ts UTC).
> Gates must actually run (commands + exit codes into History). Never push, never
> rebase shared branches, never touch `/Users/sac/ferroplan` unless your ticket
> says so. If you hit `[1302]` rate limits: wait ~60 s, retry ×3, else BLOCKED with
> the error id. Final message: standing | branch+SHA | gates+exits | deliverables |
> remaining.

| ticket | agent work surface | branch |
|---|---|---|
| fond-htn-01-oracle-runner | /tmp/fond-oracle | — |
| fond-htn-02-oracle-harvest-fondhtn | /tmp | — |
| fond-htn-03-oracle-harvest-htn | /tmp | — |
| fond-htn-04-diff-fondhtn | ~/ferroplan-worktrees/wt-oracle-fondhtn | test/fond-htn-oracle |
| fond-htn-05-diff-htn | ~/ferroplan-worktrees/wt-oracle-htn | test/htn-ipc-oracle |
| fond-htn-06-diff-flatfond | ~/ferroplan-worktrees/wt-oracle-flat | test/fond-flat-oracle |
| fond-htn-07-fix-strongcyclic | ~/ferroplan-worktrees/wt-fondsc | fix/fond-sc-closure |
| fond-htn-08-fix-validation | ~/ferroplan-worktrees/wt-validate | fix/hddl-validation |
| fond-htn-09-fix-minors | ~/ferroplan-worktrees/wt-minor | fix/fond-minors (tip 00fe98d) |
| fond-htn-10-feat-eve-bridge | ~/ferroplan-worktrees/wt-bridge | feat/eve-hddl-bridge |
| fond-htn-11-test-fond-canonical | ~/ferroplan-worktrees/wt-test-fond | test/fond-canonical |
| fond-htn-12-test-fondhtn-regression | ~/ferroplan-worktrees/wt-test-fondhtn | test/fond-htn-koala |
| fond-htn-13-test-htn-ipc2023 | ~/ferroplan-worktrees/wt-test-htn | test/htn-ipc2023 |
| fond-htn-14-test-adversarial | ~/ferroplan-worktrees/wt-test-robust | test/hddl-adversarial |
| fond-htn-15-test-property | ~/ferroplan-worktrees/wt-test-cross | test/fond-property |
| fond-htn-16-corpus-flatfond | /tmp/fond-corpus/fond-flat | — |
| fond-htn-17-test-wasm | ~/ferroplan-worktrees/wt-test-wasm | test/wasm-fond |
| fond-htn-18-docs-semantics | ~/ferroplan-worktrees/wt-docs | docs/fond-htn-semantics |
| fond-htn-19-compliance | read-only + /tmp/fond-oracle | — |
| fond-htn-20-integration | /Users/sac/ferroplan (main, sole writer) | main |

Concurrency note (並): 20 heavyweight in flight = operator-ordered (caution band
17–25; bulk-25 precedent 23/25 with explained [1302] losses). Expect 1–3 losses;
coordinator respawns per ticket after drain. T20 merges SHAs only — race-free vs
advancing branches. T09 owns contingent_policy; T07 owns both fond solvers'
regions — their only overlap (dead post-check in fond_policy) is explicitly
delegated to T07 by T09's ticket.

Coordinator post-wave: respawn losses → integrate new branches serially (same
protocol as T20, using each branch's final tip) → update tickets to ALIVE/BLOCKED
→ receipt with 比 and ledger deltas.

---

# Wave-4 — harden / benchmark / stress / document (20 agents, one message)

Same canonical dispatch form as wave 3. Tickets 21–23 (wave-3 follow-ups) are worked as-is.
New tickets 24–40. Integration remains the coordinator's, post-wave, serial.

| ticket | work surface | branch |
|---|---|---|
| fond-htn-21-eq-goal-evaluation | ~/ferroplan-worktrees/wt-a21 | fix/eq-goal-evaluation |
| fond-htn-22-parse-depth-budget | ~/ferroplan-worktrees/wt-a22 | fix/parse-depth-budget |
| fond-htn-23-translate-capacity | ~/ferroplan-worktrees/wt-a23 | fix/translate-capacity |
| fond-htn-24-ground-conditional-effects | ~/ferroplan-worktrees/wt-a24 | fix/ground-conditional-effects |
| fond-htn-25-fond-loop-failsafes | ~/ferroplan-worktrees/wt-a25 | fix/fond-loop-failsafes |
| fond-htn-26-bench-fond-criterion | ~/ferroplan-worktrees/wt-b26 | bench/fond-criterion |
| fond-htn-27-bench-ipc-full-sweep | ~/ferroplan-worktrees/wt-b27 | bench/ipc-full-sweep |
| fond-htn-28-bench-scaling-ladder | ~/ferroplan-worktrees/wt-b28 | bench/scaling-ladder |
| fond-htn-29-stress-concurrency-wasm | ~/ferroplan-worktrees/wt-s29 | stress/concurrency-wasm |
| fond-htn-30-stress-memory-ceilings | ~/ferroplan-worktrees/wt-s30 | stress/memory-ceilings |
| fond-htn-31-fuzz-hddl-roundtrip | ~/ferroplan-worktrees/wt-s31 | fuzz/hddl-roundtrip |
| fond-htn-32-fuzz-api-panic-hunt | ~/ferroplan-worktrees/wt-s32 | fuzz/api-panic-hunt |
| fond-htn-33-property-scaleup | ~/ferroplan-worktrees/wt-p33 | test/property-scaleup |
| fond-htn-34-oracle-extension | /tmp only | — |
| fond-htn-35-docs-fond-htn-update | ~/ferroplan-worktrees/wt-d35 | docs/fond-htn-wave4 |
| fond-htn-36-docs-benchmarks | ~/ferroplan-worktrees/wt-d36 | docs/benchmarks-consolidated |
| fond-htn-37-docs-readme | ~/ferroplan-worktrees/wt-d37 | docs/readme-capability |
| fond-htn-38-docs-book-chapter | ~/ferroplan-worktrees/wt-d38 | docs/book-fond-htn |
| fond-htn-39-docs-readiness-refresh | ~/ferroplan-worktrees/wt-d39 | docs/readiness-refresh |
| fond-htn-40-docs-changelog-draft | ~/ferroplan-worktrees/wt-d40 | docs/changelog-0.28-draft |

File-ownership lanes to minimize merge seams: 21/24 share ferroplan-hddl eval paths
(21 = goal `=`; 24 = effect `when` — disjoint functions, keep diffs inside them);
25 owns planning_runtime loop bounds; 23 owns translate limits plumbing;
26–28 own new files only; 29–32 own new test files (+29 may touch wasi_abi tests module);
35–40 own docs (+39 touches readiness.rs and its test only).
