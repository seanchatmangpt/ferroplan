---
id: fond-htn-31-fuzz-hddl-roundtrip
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fuzz: seeded random HDDL generator — 2000-case parse→validate→ground→translate round-trip"
standing: BLOCKED
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
| 2026-09-18T01:36:00Z | respawn-in-flight | fuzz/hddl-roundtrip@6d14813 (wt-s31, uncommitted) | — | standing stays respawn-in-flight — finish-wave owns it |
