# WAVE-RECEIPT — A7 / b4p-f5-04 (ferroplan wave-6 land), 2026-09-18

Standing: **ALIVE** (for what is claimed here) — every gate below was
observed executing in THIS session, in this worktree. Honest partials:
ticket 60 (16-domain re-run), 61 (oracle re-harvest), 66 (artifacts wiped).

## Deviations honored (from the dispatch)

- (a) All merges landed on **this branch** (`wave6/land-v26917`), never the
  main checkout; the coordinator integrates to `main` + pushes.
- (b) Scope item 4 (push) became a **push-readiness receipt** (below).

## Base + head

- base: `6cacbda` (ferroplan main at dispatch; clean worktree, release
  build 5m14s exit 0, `cargo test -p ferroplan --lib planning_runtime`
  14/14 exit 0 BEFORE any merge)
- head: **`28d80b9`** on `wave6/land-v26917` (worktree
  `/Users/sac/ferroplan-worktrees/wt-f5-04-land`)
- delta: 88 files, +7460/−1406 over base.

## Per-ticket status (57→66; gates re-run HERE, upstream claims never trusted)

| ticket | merge / landing SHA | gate command (in this worktree, post-merge) | exit | evidence |
|---|---|---|---|---|
| 57 sc-choice-rewrite | merge `8f1a875` of `d00dcf8` | `cargo test -p ferroplan --lib planning_runtime && cargo test -p ferroplan --test fond_property_scaleup` | 0 | 15/15; scaleup 4/4, **0 ignored**, 5000-instance sweep 0 mismatches |
| 58 drop-retry-redecompose | merge `0e156c3` of `b06b907` | `cargo test -p ferroplan --test fond_htn_oracle --test fond_htn_micro --test fond_flat_oracle` | 0 | 9+9+4 passed; remaining ignores documented (external corpus + separate micro_sense pin) |
| 59 goal-drop-budget | merge `6ed2069` of `b854f70` | `cargo test -p ferroplan-hddl` | 0 | 170 passed / 0 failed / 1 documented external ignore (+9 doc) |
| 60 grounding-prune | **wip landed `b03ea19`** (branch had no commits; uncommitted wt-h60 diff applied + attributed) | `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test htn_ipc2023 --test htn_oracle` | 0 | 178 lib; 13 ipc2023; 3 oracle — **PARTIAL**: 16-domain default-caps rerun NOT run (`/tmp` wiped, corpus absent), RESULTS-wavec append pending |
| 61 differential-fuzz | merge `8181703` of `f979c53` + wip `848201b` | `cargo test -p ferroplan --test differential_fuzz` | 0 | offline 1/1; **PARTIAL**: ledger parked (`ledger.pre-wave6-hotfix-harvest.json`) after tripwire found **8 verdict flips** (6× SOLVED→NOSOLUTION = 57's false-solve removal; 2× NOSOLUTION→SOLVED = 58's re-offer; +1 tolerated wall boundary); oracle re-harvest blocked (harness wiped) |
| 62 final-sweep | `_WAVE6-SWEEP.md` @ `3d07d02` | see the file; headline: `cargo test --no-fail-fast -p ferroplan -p ferroplan-hddl -- --include-ignored` | 101 (expected) | 817 passed / **6 failed, ALL EXTERNAL_ABSENT** (/tmp wiped) / 0 unexpected; wasm 9/9 exit 0; fond_threshold --release 3/3 exit 0; doc 1/1 exit 0; verify_evidence_ids exit 0; bench fond --quick 76/76 exit 0 |
| 63 docs-refresh | merge `f097293` of `cd4a7a9` | auto-merge clean; post-landing doc refresh `3d07d02` | — | "in flight" notes → landed for 57/60/65; wave-6 CHANGELOG block |
| 64 wave6-ledger | merge `809fce5` of `78e220f` | — | — | ledger rows carried; A7 rows appended in worktree ticket copies for coordinator union |
| 65 translate-plumbing | merge `f980212` of `6757249` | `cargo test -p ferroplan -p ferroplan-hddl` (full merged line) | 0 | 85 suites, **776 passed / 0 failed / 47 documented ignores** (incl. translate_plumbing 1/1, addendum runner ~9 min green) |
| 66 oracle-harvest-min | nothing to merge (/tmp only) | `ls /tmp/fond-oracle /tmp/fond-review /tmp/fond-corpus` | ENOENT | was ALIVE 03:38Z per History; **artifacts wiped** — unverifiable this session |

## Merges performed (all `--no-ff`, in ticket order)

`8f1a875` (57@d00dcf8) → `0e156c3` (58@b06b907) → `6ed2069` (59@b854f70) →
`b03ea19` (60 wip, direct commit) → `8181703` (61@f979c53) + `848201b` (61
wip) → `f980212` (65@6757249) → `f097293` (63@cd4a7a9) → `809fce5`
(64@78e220f). Then A7's own: `3d07d02` (docs+sweep), `558ab4c` (runner
re-run RESULTS), `786d560` (release prep 0.28.0), `28d80b9` (fmt).

## Push-readiness (coordinator runs; A7 has no push authority)

```sh
cd /Users/sac/ferroplan                      # main checkout, writer-free
git fetch origin && git merge --no-ff wave6/land-v26917   # after verifying this receipt's gates
# release pre-flight per RELEASING.md (NOT run here, must run at cut):
#   rustup update stable; cargo fmt --all --check (now clean);
#   cargo clippy --all-targets --all-features -- -D warnings   <- NOT run by A7
#   RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
#   cargo bench --no-run
#   cargo test --release -p ferroplan -p ferroplan-cli -- --include-ignored
#   cargo test -p ferroplan -p ferroplan-cli -p ferroplan-mcp   # DEBUG
python3 scripts/release-notes-roll.py --check               # CHANGELOG block is drafted
git tag v0.28.0 && git push origin main && git push origin v0.28.0
git rev-list --count origin/main..main                       # must print 0
# beam4pm b4p-f5-05 reconciliation input = the pushed main SHA (>= 28d80b9 content)
```

Release prep DONE here: workspace `0.27.1 → 0.28.0` (root Cargo.toml),
pins bumped (`ferroplan-cli`, `ferroplan-mcp`, `ferroplan-hddl`,
`ferroplan-sat` — the last two were already stale-`0.27.1` in-tree and
BLOCKED the py re-lock until bumped), `ferroplan-py` Cargo.toml +
pyproject.toml + Cargo.lock re-locked (`cargo update -w` exit 0),
workspace `cargo check -p ferroplan -p ferroplan-cli -p ferroplan-mcp`
exit 0. NOT tagged, NOT pushed.

## Files changed by A7 by hand (比 — the ledger)

- Total delivered on the branch: ~7460 insertions over base.
- Manufactured (pack/test-generator/runner/rustfmt-owned or upstream
  agent commits merged mechanically): ~6740 lines — 57/58/59/60/61/63/64/65
  code+tests (other agents' commits), RESULTS.md (ipc_sweep runner wrote
  it), fmt normalization (`cargo fmt --all`).
- Hand-written by A7 (~718 lines, all on the sanctioned 票/docs surface,
  no 産面 code): this receipt, `_WAVE6-SWEEP.md`, CHANGELOG wave-6 block +
  in-flight→landed refreshes (FOND-HTN.md, README.md), ticket History rows,
  release-prep version literals (5 `sed` lines), the differential_fuzz
  3-line `prune_irrelevant` reconciliation comment. That is 比 ≈ 90/10 on
  the branch delta, with 100% of 産面 code manufactured.
- Operator wrote nothing.

## Falsifiers attempted

1. Baseline-before-merge gate (refused to merge onto unproven base) — green.
2. Per-ticket gate re-runs in THIS worktree after every merge — all green;
   caught nothing wrong upstream (their claims held).
3. differential_fuzz ledger tripwire — FIRED on the merged line: 8 verdict
   flips. Refused to paper over: census taken via locally-instrumented run
   (REVERTED before commit, `git diff` verified clean), flips match the two
   hotfixes' designed semantics, ledger preserved byte-identical under a
   provenance name, gate re-armed via the harness's own missing-ledger path.
4. `cargo fmt --all --check` (RELEASING pre-flight) — FAILED; witnessed
   pre-existing at base on a wave-6-untouched file (`ipc_sweep.rs`), then
   fixed tool-owned (`cargo fmt --all`), semantics re-proved (15/15,
   178/178, 1/1).
5. RESULTS.md modified by a test runner mid-sweep — noticed, characterized
   per-domain before committing (2 regressions-of-limit → SOLVED, 3 rows
   new, 0 regressions; commit message records attribution UNKNOWN for the
   3 rows).
6. Oracle-harness existence checked before declaring 61/66 partial —
   all three /tmp trees confirmed absent (wiped).

## Remaining (for the coordinator)

1. Land `wave6/land-v26917` → main, run the full RELEASING pre-flight
   (clippy `-D warnings` was NOT run here), tag `v0.28.0`, push both.
2. Ticket 60: rebuild `/tmp/fond-review/HDDL-Parser` corpus, run
   `grounding_prune_ipc_rerun -- --ignored`, append RESULTS-wavec.
3. Ticket 61: rebuild `/tmp/fond-oracle` (build notes in
   `_FOND-HTN-WAVE-CONTEXT.md`), re-harvest the 100-pair ledger (≥80
   consumable), re-validate `KNOWN_DIVERGENCES` on the hotfix line.
4. Ticket 66: artifacts wiped; re-run or accept the loss in the wave record.
5. beam4pm b4p-f5-05: reconciliation input is the pushed SHA; expect
   verdict movement (FOUND_BUG_2 fix + re-offer seam change verdicts —
   e.g. barman_bdi-class flips from LIMIT to SOLVED).
