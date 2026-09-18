# Wave-4 branch manifest — 16 frozen SHAs (integration record)

Frozen 2026-09-18T01:36:00Z by fond-htn-45 (`docs/wave4-ledger`) from the wave-4
agent final reports; each ticket's final History receipt was verified to carry
the same tip (sanity gate: every ticket file 21–40 parses, ≥2 History rows,
final row carries a standing token — exit 0, see fond-htn-45 History).

Integration owner: **fond-htn-41** (main checkout, sole authorized writer this
wave). Serial `git merge --no-ff` **BY SHA** in the order below — order matters:
grounder/parser semantics land before their stress/fuzz/bench consumers; docs
land last so conflict unions stay visible. Never merge a moving branch — merge
the SHA only (wave-2 law); branches may advance after the freeze.

| # | branch | frozen SHA | delivers |
|---|---|---|---|
| 1 | fix/eq-goal-evaluation | `24c94f6` | grounder/translate `GroundGoal::Eq`, goal DNF folds, always-on micro-recurse oracle agreement test |
| 2 | fix/ground-conditional-effects | `818e9ea` | parser/grounder/validate/translate: `when` flattening, method `:effect`, `root_networks` — EXPECT CONFLICTS with #1 in grounder.rs/translate.rs: semantic union, keep BOTH the Eq variant and root_networks |
| 3 | fix/parse-depth-budget | `e272cbd` | parser reader depth budget + `ParseError::NestingTooDeep` (may conflict with #2's parser edits: keep both, depth budget wraps all) |
| 4 | fix/fond-loop-failsafes | `7fea6cc` | planning_runtime: `fond_policy` states+1 failsafe (max_iterations advisory) |
| 5 | stress/concurrency-wasm | `aae68be` | wasi_abi `FP_HDDL_ROOT_MISMATCH` arm + host dispatch stress test + `#[path]` shim (keep the match arm — it completes an exhaustive match) |
| 6 | fuzz/api-panic-hunt | `415b7d0` | api_panic_hunt suite + serde_json dev-dep (union Cargo.toml if conflict); FINDING-1 feeds fond-htn-42 |
| 7 | bench/fond-criterion | `979ba49` | benches/fond.rs + threshold tripwire + BENCH-FOND.md |
| 8 | bench/ipc-full-sweep | `d7659ea` | ipc_sweep runner + fixtures + `adapt_problem` pub |
| 9 | bench/scaling-ladder | `6a8ce08` | scaling_ladder runner + RESULTS.md |
| 10 | stress/memory-ceilings | `673370c` | memory_stress suite + Cargo.toml `[[test]] harness=false` (union with #6's Cargo.toml edit) |
| 11 | docs/fond-htn-wave4 | `f0034fb` | docs/FOND-HTN.md wave-4 refresh (conflicts with #1/#2 doc sections: keep all sections, union); `<!-- WAVE4-NUMBERS -->` left empty |
| 12 | docs/benchmarks-consolidated | `6bf31c9` | docs/BENCHMARKS.md + crosslinks (incl. README pointer) |
| 13 | docs/readme-capability | `d1fbfb5` | README capability section + current-limits list (conflicts with #12's pointer: keep both pointers) |
| 14 | docs/book-fond-htn | `e67a37d` | book/src/fond-htn.md + SUMMARY.md registration |
| 15 | docs/readiness-refresh | `1576abc` | readiness.rs evidence ids (+21) + verify script — PARTIAL_ALIVE: leftovers ticketed as fond-htn-46 |
| 16 | docs/changelog-0.28-draft | `8ead69a` | CHANGELOG Unreleased consolidation (no version cut) |

Per-merge gate (src-touching #1,2,3,4,5,8,10,15): scoped `cargo test` exit 0
before proceeding, per the fond-htn-41 manifest; full gates at closeout.

Excluded from this integration — **respawn-in-flight**, finish-wave owns them;
they land via the coordinator's final serial pass after the finish wave:

- fix/translate-capacity (ticket 23, wt-a23)
- fuzz/hddl-roundtrip (ticket 31, wt-s31)
- test/property-scaleup (ticket 33, wt-p33)
- oracle extension (ticket 34, /tmp only, no branch)

Post-merge numbers fill (fond-htn-41 step 2): `<!-- WAVE4-NUMBERS -->` in
docs/FOND-HTN.md from COMMITTED results only — benches/BENCH-FOND.md (#7),
tests/fixtures/ipc-sweep/RESULTS.md (#8), tests/fixtures/scaling-ladder/RESULTS.md
(#9). Never invent numbers.
