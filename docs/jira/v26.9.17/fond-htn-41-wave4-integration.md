---
id: fond-htn-41-wave4-integration
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: serial integration of the 16 frozen wave-4 branch SHAs + numbers fill"
standing: BLOCKED
worktree: MAIN CHECKOUT /Users/sac/ferroplan (sole authorized writer this wave)
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. You are the ONLY agent authorized to write /Users/sac/ferroplan (merges, docs-number fills, ticket History appends). Rider protocol: merge only when `git status` is clean apart from pre-existing untracked `plugins/*` and sibling ticket-file History rows (uncommitted rows in docs/jira/** are expected — do NOT commit them, carry them through merges; commit only your merge/number-fill commits). Never push. Red gate → `git merge --abort` + History + BLOCKED, continue with independent SHAs.

## Merge manifest (exact SHAs, frozen; order matters) — merge BY SHA with `--no-ff`

1. `24c94f6` fix/eq-goal-evaluation (grounder/translate: `GroundGoal::Eq`, goal DNF folds)
2. `818e9ea` fix/ground-conditional-effects (grounder/parser/validate/translate: `when` flattening, method `:effect`, `root_networks` replacing root_subtasks/root_order — EXPECT CONFLICTS with 1 in grounder.rs/translate.rs: semantic union, keep BOTH the Eq variant and root_networks)
3. `e272cbd` fix/parse-depth-budget (parser.rs reader depth budget — may conflict with 2's parser edits: keep both, depth budget wraps all)
4. `7fea6cc` fix/fond-loop-failsafes (planning_runtime: fond_policy failsafe)
5. `aae68be` stress/concurrency-wasm (wasi_abi: adds `FP_HDDL_ROOT_MISMATCH` arm to an exhaustive match — keep; plus host stress test + `#[path]` shim)
6. `415b7d0` fuzz/api-panic-hunt (tests + Cargo.toml dev-dep — union Cargo.toml if conflicts)
7. `979ba49` bench/fond-criterion (benches/fond.rs + fond_threshold + BENCH-FOND.md)
8. `d7659ea` bench/ipc-full-sweep (makes `adapt_problem` pub in hddl.rs + ipc_sweep + fixtures)
9. `6a8ce08` bench/scaling-ladder
10. `673370c` stress/memory-ceilings (Cargo.toml `[[test]] harness=false` — union with 6's edit)
11. `f0034fb` docs/fond-htn-wave4 (FOND-HTN.md — conflicts with 1/2's doc sections: keep all sections, union)
12. `6bf31c9` docs/benchmarks-consolidated (BENCHMARKS.md + crosslinks incl. README pointer)
13. `d1fbfb5` docs/readme-capability (README — conflicts with 12's pointer: keep both pointers)
14. `e67a37d` docs/book-fond-htn
15. `1576abc` docs/readiness-refresh (readiness.rs evidence ids + verify script)
16. `8ead69a` docs/changelog-0.28-draft (CHANGELOG)

After EACH merge of a src-touching branch (1,2,3,4,5,8,10,15): `cargo test -p ferroplan --lib planning_runtime` or `-p ferroplan-hddl` as relevant, exit 0 before proceeding (full gate at the end replaces per-merge breadth).

## Post-merge closeout (on main, committed)

1. Full gates: `cargo test -p ferroplan -p ferroplan-hddl` (expect ≥ wave-3's 695 + wave-4 additions); `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib`; `cargo test -p ferroplan --test fond_htn_oracle -- --include-ignored` — record whether the wave-3 `ORACLE_MISMATCH_*` cases healed now that `=` evaluation + conditional effects landed.
2. Fill `<!-- WAVE4-NUMBERS -->` in docs/FOND-HTN.md from COMMITTED results only: chain_200/ladder_200/lattice_200 means + dispatch floor (BENCH-FOND.md), IPC sweep 12/43 + refusal histogram (ipc-sweep/RESULTS.md), ladder envelope + knee (scaling-ladder/RESULTS.md). One commit.
3. Append wave-4 rows to docs/BENCHMARKS.md (ipc-sweep, scaling-ladder, BENCH-FOND) in its established format, citing the source files.
4. Ticket History rows per merge batch + final; report `git log --oneline -20` shape.

Do NOT merge anything else (respawn branches 23/31/33/34 and finish-wave branches 42–56 land with the coordinator after this wave).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | main @ 6d14813 | — | 16 merges + closeout |
