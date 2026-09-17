# Wave-4 context — v26.9.17 (hardening, benchmarks, stress, docs)

Read `_FOND-HTN-WAVE-CONTEXT.md` FIRST — the KOALA POLICY, oracle runner contract, corpora map, ticket mechanics, and [1302] protocol all still bind, verbatim. This file adds wave-4 state only.

## State at wave-4 start

- main `90c2ae2`, clean, 695 tests / 0 failures (`cargo test -p ferroplan -p ferroplan-hddl`).
- Landed since wave 3: Phase-3 strong-cyclic goal-reachability prune (FOUND_BUG_1 fixed, reproducer always-on in `tests/fond_property.rs`); validation hardening; oneof grammar alignment; eve bridge; wasm op tests (wasip1+wasmtime idiom: `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run" cargo test -p ferroplan-wasm --target wasm32-wasip1 --lib`); full oracle goldens (`tests/fixtures/**/oracle-goldens.json`, `oracle-harvest-full.json`); docs/FOND-HTN.md.
- `fond_policy_strong_cyclic` was JUST hardened — do not restructure it; scoped edits only where your ticket says so.
- Open defects ticketed: 21 (`=` evaluation), 22 (parse depth), 23 (translate capacity). Known gaps: conditional-effect grounding (wave-3 T05: "nested 'when' out of scope", "unbound variable '?x'" on 2 PANDA pairs); `fond_policy`/`contingent_policy` loops still gated by `max_iterations` (T07 out-of-scope note); koala corpus budget divergences (Snake `max_ground_methods` 10000, 10 s translate walls).
- Oracle verdicts so far: IPC-2023 13/13 koala-solved; koala FOND-HTN harvest 18 SOLVED / 12 TIMEOUT (90 s walls); koala = strong-only decision procedure (cannot decide policy-cyclic domains — treat koala TIMEOUT on retry-style domains as expected).

## Wave-4 rules addenda

- Benchmarks: every number you publish must be reproducible — record command, seed, wall, and machine note (`sysctl -n machdep.cpu.brand_string`) in the RESULTS file you commit. No cherry-picking: report the full sweep, including refusals.
- Stress: bounded walls on everything (no test may exceed 120 s); RSS measured, not guessed (`/usr/bin/time -l` or `ps -o rss=`).
- Fuzz: seeded generators committed as tests; corpora of failing cases committed under `tests/fixtures/`; a finding is `#[ignore]`d + ticket History row, never a silent skip.
- Docs: cite only public literature and in-repo evidence (file paths); the KOALA POLICY applies to prose too — concepts, never koala source.
- Integration is the coordinator's (post-wave, serial). Never merge into main yourself. Tickets 21–23 live in this same day dir; work them where they are and append History there.
