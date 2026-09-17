# Wave receipt — v26.9.17 FOND-HTN hardening (証)

Operator cut: migrate koala → ferroplan; koala = external test oracle ONLY; zero koala code in any repo we own (compliance sweep verdict: CLEAN, 0 violations — byte-hash of 742 koala files vs 18,301 repo files, 1 collision = canonical IPC data, /tmp/fond-oracle/COMPLIANCE.md).

## Standings (20 tickets + coordinator work)

| ticket | standing | landed as |
|---|---|---|
| 01 oracle runner | ALIVE | /tmp/fond-oracle (build.sh + flock oracle-run.sh + README); stale-result.json hazard found+fixed |
| 02 harvest fond-htn | ALIVE (respawn; first agent inactive-timeout) | 31 verdicts: 18 SOLVED / 12 TIMEOUT / 0 NOSOLUTION; committed as golden facts |
| 03 harvest htn | ALIVE | 26 verdicts: IPC-2023 13/13, PANDA 4/4, SHOP3 3/4 + loan-noplan TIMEOUT (koala can't prove unsolvability in budget) |
| 04 diff fond-htn | ALIVE | test/fond-htn-oracle; 3 semantic findings (one → ticket 21) |
| 05 diff htn | ALIVE | test/htn-ipc-oracle; 4 translate-wall + 2 ground-conditional gaps (→ ticket 23) |
| 06 diff flat-fond | ALIVE | test/fond-flat-oracle; 8 domains three-way agreement; koala = strong-only decision procedure (frozen in goldens) |
| 07 strong-cyclic truncation | ALIVE | fix/fond-sc-closure |
| 08 validation | ALIVE | fix/hddl-validation; beats reference on ordering-ID + empty-:htn gaps; 2 more upstream corpus defects found |
| 09 minors | ALIVE | verified contingent-memo (52c3501) + mass fix (00fe98d) already on branch; discriminating falsifier run |
| 10 eve bridge + manifest | ALIVE | feat/eve-hddl-bridge; fp.core.fond + fp.core.hddl admitted (19→21) |
| 11 canonical fond | ALIVE | test/fond-canonical; 10/10 literature verdicts hold |
| 12 fond-htn micro | ALIVE | test/fond-htn-koala; 7 hand-authored fixtures, 9 tests |
| 13 htn ipc2023 | ALIVE | test/htn-ipc2023; 9 solved / 4 translate-wall (ticket 23) |
| 14 adversarial | ALIVE | test/hddl-adversarial; 1 panic→typed fix landed; parse-depth → ticket 22 |
| 15 property | ALIVE (found a real bug) | test/fond-property; FOUND_BUG_1 fixed by coordinator (fix/fond-sc-goalreach), reproducer promoted to always-on guard |
| 16 corpus flat | ALIVE | /tmp/fond-corpus/fond-flat; 8 source-verified domains, independent-fixpoint falsifier |
| 17 wasm | ALIVE | test/wasm-fond; 9 wasip1 tests incl. 5 new; pre-broken wasip1 build repaired via cherry-picks of remote fixes |
| 18 docs | ALIVE | docs/fond-htn-semantics → docs/FOND-HTN.md |
| 19 compliance | ALIVE | CLEAN (both sweeps) |
| 20 integration | ALIVE | 3 frozen SHAs merged serially; then coordinator unioned the diverged main (upstream 0.28 line + rider HDDL line, merge 726ce37) and integrated all 14 branch tips + hotfix |

## Coordinator (non-delegated) work

Union merge of diverged lineages (57 vs 25 commits; CHANGELOG/Cargo.toml conflicts resolved);
T07 test-splice conflict resolution; 2 cross-branch match-arm fixes; 6 adversarial expectations
pinned to the landed strict semantics; Phase-3 goal-reachability hotfix (FOUND_BUG_1, 79/320
property instances) + 3 regression tests + reproducer promotion; T02 respawn; follow-up tickets
21–23; this receipt.

## Gates (commands + exits)

- cargo test -p ferroplan -p ferroplan-hddl (post-integration, pre-hotfix): 691 passed / 0 failed
- hotfix worktree: planning_runtime 12/12; fond_canonical 10/10; fond_property 3/3 (incl. un-ignored FOUND_BUG_1) — exits 0
- final full gate: see History row appended below on completion
- oracle smokes (T01): flexible SOLVED 0.2 s; PARSE_ERROR classification; flock double-run — exits 0

## 比 / ledger

All production lines this wave are agent- or coordinator-manufactured (17 branches + hotfix + docs); the operator wrote zero bytes. No HANDWRITTEN.md entries required (no UNSUPPORTED capability use). Koala-derived content in repos: none (sweep CLEAN).

## Falsifiers that survived → permanent guards

- FOUND_BUG_1 reproducer (always-on) — guards: strong-cyclic goal-reachability
- truncation dead-sink + chain-convergence tests — guard: fixpoint round bounds
- ORACLE_MISMATCH_* ignored tripwires — guard: koala/ferroplan divergence ledger
- oracle-goldens.json agreement tests — guard: differential drift
