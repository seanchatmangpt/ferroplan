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
