# Wave-6 branch manifest — dispatch table (base 75870de)

Dispatch state recorded 2026-09-18T03:02:00Z by fond-htn-64 (`docs/wave6-ledger`)
from the wave-6 ticket frontmatter and verified against the repo: every worktree
below was checked at its branch (`git branch --show-current` + `rev-parse HEAD`
= `75870de`); the seven wave-5 integration merges and the readiness union pins
were read from main's history (sanity gate: every ticket file 41–56 parses,
≥2 History rows, final row carries a standing token — exit 0, see fond-htn-64
History). This is a DISPATCH manifest — tips do not exist yet; integration is
the coordinator's, post-wave, serial `git merge --no-ff` **by SHA** (wave-2
law: never merge a moving branch), main checkout sole writer.

## Wave-5 close (integration record, verified in git)

Integrated by the coordinator at the 75870de lineage before wave-6 dispatch —
each ticket's close row in its own History carries the same pairing:

| ticket | branch | integrated tip | merge |
|---|---|---|---|
| fond-htn-42 | fix/goal-eval-recursion | `85a2fe0` | `f0b8b9c` |
| fond-htn-43 | fix/ground-caps-plumbing | `3b8ee6e` (receipt commit atop code tip `5cb9041`) | `833994f` |
| fond-htn-44 | fix/duplicate-type-dedup | `8f587af` | `0f39d2d` |
| fond-htn-31 | fuzz/hddl-roundtrip | `0479c1e` | `8e54704` |
| fond-htn-33 | test/property-scaleup | `26ae36f` (receipt commit atop `8ddbddc`) | `2377802` |
| fond-htn-45 | docs/wave4-ledger | `121786f` | `874a438` |
| fond-htn-46 | docs/readiness-finish | `113d00e` (receipt commit atop `1fe7506`) | `1a6c393` |

Evidence pins on the integrated lineage: `543c25f` + `75870de` (readiness
count test pinned to the union **fond=17 / hddl=20**, `readiness.rs:1214-1215`).
fond-htn-41's integration receipt (main `9ae3f08`) is superseded by this
lineage. fond-htn-23 (`fix/translate-capacity`) and fond-htn-34 (oracle
extension, /tmp only) are superseded by wave-6 tickets **fond-htn-65** and
**fond-htn-66** respectively.

## Wave-6 dispatch table (19 agents)

Base for ALL worktrees: **75870de** (respawn worktrees wt-d47–wt-d56 and
wt-a23 verified fast-forwarded/reset to `75870de` at manifest time).

| # | ticket | branch | worktree | base | delivers |
|---|---|---|---|---|---|
| 1 | fond-htn-57 sc-choice-rewrite | fix/sc-choice-rewrite | ~/ferroplan-worktrees/wt-h57 | 75870de | HOTFIX FOUND_BUG_2: strong-cyclic witness-first goal-unreachable loop policies (from fond-htn-33; THE priority) |
| 2 | fond-htn-58 drop-retry re-decompose | fix/drop-retry-redecompose | ~/ferroplan-worktrees/wt-h58 | 75870de | micro-drop-retry oracle mismatch — re-decompose after dead branch |
| 3 | fond-htn-59 goal-drop budget | fix/goal-drop-budget | ~/ferroplan-worktrees/wt-h59 | 75870de | iterative Drop for deep GroundGoal chains (recursive-drop overflow) |
| 4 | fond-htn-60 grounding prune | fix/grounding-prune | ~/ferroplan-worktrees/wt-h60 | 75870de | reachability pruning for >1M-instance groundings |
| 5 | fond-htn-61 differential fuzz | test/differential-fuzz | ~/ferroplan-worktrees/wt-h61 | 75870de | generator VALID draws through BOTH engines, divergences ledgered |
| 6 | fond-htn-62 final sweep | test/final-sweep | ~/ferroplan-worktrees/wt-h62 | 75870de | every test target on main incl. externals, wasm, benches tripwire — report only |
| 7 | fond-htn-63 docs refresh | docs/waves45-refresh | ~/ferroplan-worktrees/wt-h63 | 75870de | waves-4/5 refresh — FOND-HTN, README limits, CHANGELOG |
| 8 | fond-htn-65 translate plumbing | fix/translate-plumbing | ~/ferroplan-worktrees/wt-a23 (reused, reset) | 75870de | 3rd attempt, SPLIT scope: plumb caller walls into TranslateLimits only (supersedes fond-htn-23) |
| 9 | fond-htn-66 oracle harvest-min | — (no branch) | /tmp only | — | 3rd attempt, MINIMAL: koala p06–p10 × 6 domains @ 90 s (supersedes fond-htn-34) |
| 10 | fond-htn-47 ignored inventory (2nd) | test/ignored-inventory | ~/ferroplan-worktrees/wt-d47 | 75870de | ignored-test inventory + stale-ignore sweep |
| 11 | fond-htn-48 workspace hygiene (2nd) | chore/workspace-hygiene | ~/ferroplan-worktrees/wt-d48 | 75870de | fmt, clippy on new code, intra-doc links |
| 12 | fond-htn-49 surface parity (2nd) | test/surface-parity | ~/ferroplan-worktrees/wt-d49 | 75870de | surface-parity suite — one domain, three entry points, identical policies |
| 13 | fond-htn-50 probabilistic iterations (2nd) | fix/probabilistic-iterations | ~/ferroplan-worktrees/wt-d50 | 75870de | value iteration convergence-gated, not iteration-capped |
| 14 | fond-htn-51 benchmarks drift-check (2nd) | docs/benchmarks-drift-check | ~/ferroplan-worktrees/wt-d51 | 75870de | docs numbers re-derived from source files, fail-closed |
| 15 | fond-htn-52 claim audit (2nd) | docs/claim-audit | ~/ferroplan-worktrees/wt-d52 | 75870de | every README/docs test-name citation verified, rename-tripped |
| 16 | fond-htn-53 ipc2020 profile (2nd) | bench/ipc2020-profile | ~/ferroplan-worktrees/wt-d53 | 75870de | profile the IPC-2020 blocksworld translate blowup — name the top term |
| 17 | fond-htn-54 fond-unsafe HDDL (2nd) | test/fond-unsafe-hddl | ~/ferroplan-worktrees/wt-d54 | 75870de | unsafe-states semantics through the HDDL path |
| 18 | fond-htn-55 wasip1 clippy (2nd) | fix/wasip1-clippy | ~/ferroplan-worktrees/wt-d55 | 75870de | the three pre-existing wasip1 clippy diagnostics in wasi_abi |
| 19 | fond-htn-56 fixtures dedupe (2nd) | chore/fixtures-dedupe | ~/ferroplan-worktrees/wt-d56 | 75870de | fixtures footprint audit — byte-identical IPC copies deduped |

Notes:

- Merge-order intent (coordinator's serial pass): src-fixing branches (57, 58,
  59, 60, 65) land before their consumers (61, 62); docs (63) last so conflict
  unions stay visible; second-attempt finishers (47–56) are independent lanes.
- fond-htn-57 unblocks fond-htn-33's FOUND_BUG_2 class (882/3676 sweep
  mismatches) — its `#[ignore]`d reproducer on main is the acceptance gate.
- fond-htn-64 (this ledger) and fond-htn-41 (wave-4 integration) are
  coordinator-adjacent bookkeeping tickets, not dispatch rows.
- Numbers cited in dispatch tickets come from COMMITTED results only
  (`fond_property_scaleup.rs` reproducer, oracle ledgers); never invented.
