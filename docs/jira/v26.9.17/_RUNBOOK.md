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

---

# Finish-wave — close out wave-4 (20 agents: 4 respawns + integrator + 15 finishers)

| ticket | work surface | branch |
|---|---|---|
| 23 (respawn) | ~/ferroplan-worktrees/wt-a23 | fix/translate-capacity |
| 31 (respawn) | ~/ferroplan-worktrees/wt-s31 | fuzz/hddl-roundtrip |
| 33 (respawn) | ~/ferroplan-worktrees/wt-p33 | test/property-scaleup |
| 34 (respawn) | /tmp only | — |
| 41 integration | /Users/sac/ferroplan (main, sole writer) | main |
| 42 goal-eval recursion | ~/ferroplan-worktrees/wt-a42 | fix/goal-eval-recursion |
| 43 ground caps plumbing | ~/ferroplan-worktrees/wt-a43 | fix/ground-caps-plumbing |
| 44 duplicate-type dedup | ~/ferroplan-worktrees/wt-a44 | fix/duplicate-type-dedup |
| 45 wave-4 ledger | ~/ferroplan-worktrees/wt-d45 | docs/wave4-ledger |
| 46 readiness finish | ~/ferroplan-worktrees/wt-d46 | docs/readiness-finish |
| 47 ignored inventory | ~/ferroplan-worktrees/wt-d47 | test/ignored-inventory |
| 48 workspace hygiene | ~/ferroplan-worktrees/wt-d48 | chore/workspace-hygiene |
| 49 surface parity | ~/ferroplan-worktrees/wt-d49 | test/surface-parity |
| 50 probabilistic iterations | ~/ferroplan-worktrees/wt-d50 | fix/probabilistic-iterations |
| 51 benchmarks drift-check | ~/ferroplan-worktrees/wt-d51 | docs/benchmarks-drift-check |
| 52 claim audit | ~/ferroplan-worktrees/wt-d52 | docs/claim-audit |
| 53 ipc2020 profile | ~/ferroplan-worktrees/wt-d53 | bench/ipc2020-profile |
| 54 unsafe HDDL wall | ~/ferroplan-worktrees/wt-d54 | test/fond-unsafe-hddl |
| 55 wasip1 clippy | ~/ferroplan-worktrees/wt-d55 | fix/wasip1-clippy |
| 56 fixtures dedupe | ~/ferroplan-worktrees/wt-d56 | chore/fixtures-dedupe |

Ticket 41 merges ONLY the 16 frozen wave-4 SHAs; respawn + finisher branches are the
coordinator's final serial integration after this wave.

---

# Wave-6 — audit + finish (20 agents)

Coordinator pre-wave: integrated the 7 landed finish-wave branches into main (75870de
lineage; readiness union fond=17/hddl=20; fuzz digest tags extended). Respawns 47-56
worktrees fast-forwarded to 75870de; wt-a23 carries the SPLIT-scope ticket 65.

| ticket | work surface | branch |
|---|---|---|
| 57 HOTFIX sc-choice-rewrite | ~/ferroplan-worktrees/wt-h57 | fix/sc-choice-rewrite |
| 58 drop-retry re-decompose | ~/ferroplan-worktrees/wt-h58 | fix/drop-retry-redecompose |
| 59 goal-drop budget | ~/ferroplan-worktrees/wt-h59 | fix/goal-drop-budget |
| 60 grounding prune | ~/ferroplan-worktrees/wt-h60 | fix/grounding-prune |
| 61 differential fuzz | ~/ferroplan-worktrees/wt-h61 | test/differential-fuzz |
| 62 final sweep (report) | ~/ferroplan-worktrees/wt-h62 | test/final-sweep |
| 63 docs waves-4/5 refresh | ~/ferroplan-worktrees/wt-h63 | docs/waves45-refresh |
| 65 translate plumbing (3rd, split) | ~/ferroplan-worktrees/wt-a23 | fix/translate-plumbing |
| 66 oracle harvest (3rd, minimal) | /tmp only | — |
| 47 ignored inventory (2nd) | ~/ferroplan-worktrees/wt-d47 | test/ignored-inventory |
| 48 workspace hygiene (2nd) | ~/ferroplan-worktrees/wt-d48 | chore/workspace-hygiene |
| 49 surface parity (2nd) | ~/ferroplan-worktrees/wt-d49 | test/surface-parity |
| 50 probabilistic iterations (2nd) | ~/ferroplan-worktrees/wt-d50 | fix/probabilistic-iterations |
| 51 benchmarks drift (2nd) | ~/ferroplan-worktrees/wt-d51 | docs/benchmarks-drift-check |
| 52 claim audit (2nd) | ~/ferroplan-worktrees/wt-d52 | docs/claim-audit |
| 53 ipc2020 profile (2nd) | ~/ferroplan-worktrees/wt-d53 | bench/ipc2020-profile |
| 54 unsafe HDDL (2nd) | ~/ferroplan-worktrees/wt-d54 | test/fond-unsafe-hddl |
| 55 wasip1 clippy (2nd) | ~/ferroplan-worktrees/wt-d55 | fix/wasip1-clippy |
| 56 fixtures dedupe (2nd) | ~/ferroplan-worktrees/wt-d56 | chore/fixtures-dedupe |

Base for ALL worktrees: 75870de (tickets 57-63 reference it in frontmatter).
