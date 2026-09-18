---
id: fond-htn-31-fuzz-hddl-roundtrip
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fuzz: seeded random HDDL generator — 2000-case parse→validate→ground→translate round-trip"
standing: ALIVE
branch: fuzz/hddl-roundtrip
worktree: ~/ferroplan-worktrees/wt-s31
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`.

Scope:
1. Seeded grammar-faithful random HDDL generator (in `crates/ferroplan/tests/` as a module): draws from the SUPPORTED surface only — typed domains, predicates, actions (conjunctive effects incl. `oneof` top-level with empty/overlapping branches), compound tasks, methods with `<` orderings and `:precondition`, problem `:htn`/`:init`/`:goal` — parameterized by size (2–6 tasks, 2–8 actions, 2–10 objects) and a mutation switch (grammatically-valid-but-semantically-odd: shadowing names, unused params, singleton types, duplicate-ish method bodies).
2. 2000 deterministic cases (fixed seed; mixture 70% valid / 30% mutated): each runs parse → validate → ground → translate under small caps and asserts: no panic (catch_unwind), any Err is a typed error of the crate, any Ok has an outcome-closed policy or honest NoPlan downstream of `solve_hddl` on a bounded budget (sample 10% through solve_hddl for end-to-end depth).
3. Minimizer: on any panic/failure, shrink by halving the draw until minimal, commit the shrunken case under `tests/fixtures/fuzz-found/` with the seed; finding = `#[ignore]`d test + History row.
4. Re-run stability: same seed twice ⇒ identical outcomes (generator determinism proof).

Gates: `cargo test -p ferroplan --test hddl_fuzz_roundtrip` exit 0 (≤ 120 s wall: shrink the default to 500 cases if needed, keep 2000 behind `-- --ignored`).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | fuzz/hddl-roundtrip @ 90c2ae2 | — | all |
| 2026-09-18T00:20:00Z | PARTIAL_ALIVE | fuzz/hddl-roundtrip @ 6d14813 | oriented: worktree wt-s31 clean on branch; SUPPORTED surface read (parser/ast/validate/grounder/translate/lib); writing hddl_fuzz_roundtrip.rs | generator+sweep, gates, findings, commit |
| 2026-09-18T01:05:00Z | PARTIAL_ALIVE | fuzz/hddl-roundtrip @ 6d14813 | (respawn after prior agent cancel; worktree verified clean at base 6d14813) wrote `crates/ferroplan/tests/hddl_fuzz_roundtrip.rs`: SplitMix64 seeded generator (SUPPORTED surface only; sizes 2–6 tasks / 2–8 actions / 2–10 objects; ~30% mutated draws: shadowing/duplicate names, unused params, duplicate param names, singleton types, duplicate-ish method bodies; oneof top-level with empty + overlapping branches; negative pre/goal literals; ordered + partially-ordered `:subtasks`/`:ordering` networks), 500-case default sweep run twice (determinism proof), 2000-case sweep behind `-- --ignored`, catch_unwind no-panic + typed-Err + VALID-draw-must-parse-and-validate generator-drift tripwires + validate-passed⇒ground-not-Validation consistency assert, ~10% solve_hddl sample (bounded: 1 s watchdog, small ground/translate caps) asserting solved⇒non-empty OUTCOME-CLOSED policy checked against a re-run translate, WorkerPanicked=finding, halving minimizer + `tests/fixtures/fuzz-found/` fixture writer + KNOWN_FINDINGS regression-tripwire table (empty — no findings) | gates |
| 2026-09-18T01:50:00Z | PARTIAL_ALIVE | fuzz/hddl-roundtrip @ 6d14813 | first gate run caught a generator renderer bug (action/method s-expr closed early) via the generator-drift tripwire itself — minimized + fixtures auto-written by the finding path, generator fixed, temp fixtures discarded (generator bug, not a pipeline finding); gate `cargo test -p ferroplan --test hddl_fuzz_roundtrip` exit 0 in ~46 s (≤120 s): 410/500 draws reached translate-ok, solve sample 40/40 typed (5 solved — every policy outcome-closed — 35 honest NoPlan), 0 panics, 0 warnings; gate re-run x2 back-to-back stable; determinism compare made timeout-pair tolerant (wall-clock watchdog refusals are load-dependent, not seed-dependent — documented in the test) | ignored 2000 sweep, commit |
| 2026-09-18T02:15:00Z | ALIVE | fuzz/hddl-roundtrip @ 6d14813 (this commit) | ignored sweep `cargo test -p ferroplan --test hddl_fuzz_roundtrip -- --ignored` exit 0 in 156 s (2000 cases x2, determinism holds); final default gate re-run exit 0 in 46 s, 0 warnings; no findings across all runs (zero panics; every Ok plan outcome-closed or honest NoPlan; every Err typed) — KNOWN_FINDINGS stays empty, fixtures dir not created | commit + final receipt || 2026-09-18T01:36:00Z | respawn-in-flight | fuzz/hddl-roundtrip@6d14813 (wt-s31, uncommitted) | — | standing stays respawn-in-flight — finish-wave owns it |
| 2026-09-18T03:02:00Z | ALIVE | fuzz/hddl-roundtrip @ 0479c1e | — | coordinator-close: respawn-ALIVE — final agent receipt stands (02:15Z: ignored 2000-case sweep exit 0, zero findings, KNOWN_FINDINGS empty); integrated by coordinator at 75870de lineage (merge 8e54704); readiness union fond=17/hddl=20 | none — closed |
