# _WAVE6-SWEEP.md — fond-htn-62 final sweep (deviation: run on the A7 landing branch, not main)

Run by A7 (ticket b4p-f5-04, deviation (a): the wave-6 landing branch
`wave6/land-v26917` in worktree `~/ferroplan-worktrees/wt-f5-04-land`
stands in for main; the coordinator lands it to main + pushes after receipt
verification).

- HEAD at run time: `809fce5` (merges: 57 `d00dcf8`, 58 `b06b907`, 59
  `b854f70`, 60 wip `b03ea19`, 61 `f979c53`+wip `848201b`, 65 `6757249`,
  63 `cd4a7a9`, 64 `78e220f`).
- Drift during the run: docs-only edits (FOND-HTN.md/README/CHANGELOG
  "in flight" → landed refresh) were staged mid-run and committed AFTER the
  sweep completed; no Cargo/test file changed, so no affected target was
  invalidated. Sweep HEAD `809fce5` remains the tested tree for every
  `#[test]` in the workspace.
- Machine: Apple M3 Max class (same box as the wave-4 capacity rows).
- Oracle note: `/tmp` was wiped between the wave-6 harvest runs and this
  sweep — `/tmp/fond-oracle`, `/tmp/fond-review` (koala checkout + HDDL-Parser
  IPC corpus), `/tmp/fond-corpus` are ALL ABSENT. Every failure below is
  that single class (`EXTERNAL_ABSENT`); there are ZERO unexpected failures.

## Tally

| target | command | passed | failed | ignored | exit | class/notes |
|---|---|---|---|---|---|---|
| ferroplan + ferroplan-hddl, ALL tests incl. ignored | `cargo test --no-fail-fast -p ferroplan -p ferroplan-hddl -- --include-ignored` | 817 | 6 | 0 | 101 | 6× `EXTERNAL_ABSENT` (below); wall ≈ 27 min |
| — differential_fuzz (3 fails) | | 1 | 3 | 0 | | `differential_full_with_oracle`, `ORACLE_MISMATCH_fuzz_koala_strong_only`, `ORACLE_MISMATCH_fuzz_redecomposition`: spawn `/tmp/fond-oracle/oracle-run.sh` → ENOENT |
| — fond_flat_oracle (1 fail) | | 9 | 1 | 0 | | `oracle_live_reruns_match_the_golden_signature`: same oracle spawn ENOENT (fond_flat_oracle.rs:516) |
| — grounding_prune_ipc_rerun (1 fail) | | 0 | 1 | 0 | | `default_caps_rerun_of_the_16_stuck_domains_under_relevance_pruning`: corpus `/tmp/fond-review/HDDL-Parser/tests/ipc/...` ENOENT (contract-honest fail, its own header documents the requirement) |
| — ground_caps_ipc_addendum (1 fail) | | 0 | 1 | 0 | | `raised_caps_rerun_of_the_17_ground_cap_refused_domains`: same corpus ENOENT |
| ferroplan-wasm lib (wasip1) | `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib` | 9 | 0 | 0 | 0 | wasmtime runner |
| fond_threshold (release) | `cargo test -p ferroplan --test fond_threshold --release` | 3 | 0 | 0 | 0 | |
| ferroplan doc-tests | `cargo test -p ferroplan --doc` | 1 | 0 | 0 | 0 | |
| evidence ids | `python3 scripts/verify_evidence_ids.py` | — | — | — | 0 | all ids verifiable |
| criterion bench sanity | `cargo bench -p ferroplan --bench fond -- --quick` | 76 benches | 0 | — | 0 | sanity only, not perf |
| default-profile regression (65's gate, pre-sweep) | `cargo test -p ferroplan -p ferroplan-hddl` | 776 | 0 | 47 | 0 | 85 suites; the 47 documented ignores are the external-corpus + long-run classes |

Externals skipped-with-note by absence (counted): the koala oracle harness
(`/tmp/fond-oracle`), the koala FOND-HTN domains
(`/tmp/fond-review/domains`), the IPC-2023 HDDL-Parser corpus
(`/tmp/fond-review/HDDL-Parser/tests/ipc`). Tests in those classes that ran
anyway under `--include-ignored` failed honestly with ENOENT (6, above);
the `#[ignore]`d ones that guard on presence stayed ignored in the
default-profile run (47).

## Findings

1. Zero unexpected failures. Every red test is `EXTERNAL_ABSENT` and names
   its missing path in the panic — the contract-honest behavior the wave-4
   rules require.
2. The `fond-htn-61` offline tripwire-vs-ledger verdict flips are documented
   in `tests/fixtures/differential-fuzz/ledger.pre-wave6-hotfix-harvest.json`
   provenance + ticket 61 History; re-harvest requires the oracle harness
   rebuild (coordinator note).
3. `translate_wall_ipc_addendum` (9 instances × 60 s caller walls) is the
   long pole of the sweep (~9 min) — expected per ticket 65's History.

## Gate (ticket 62)

Sweep completed with zero UNEXPECTED failures; this file committed on the
landing branch for the coordinator to carry into main.
