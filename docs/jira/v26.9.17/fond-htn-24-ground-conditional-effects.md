---
id: fond-htn-24-ground-conditional-effects
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: ground conditional effects — 2 PANDA pairs die with nested-'when' / unbound-variable"
standing: ALIVE
branch: fix/ground-conditional-effects
worktree: ~/ferroplan-worktrees/wt-a24
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md` (and wave-3 context). Finding (wave-3 T05, `tests/fixtures/htn-oracle/RESULTS.md`): the PANDA `method-effect` and `conditional-effect` pairs — which the koala oracle SOLVES — die in ferroplan's grounder with `GROUND_ERROR: nested 'when' out of scope` and `unbound variable '?x'`.

Scope:
1. Reproduce both failures from the committed fixtures (they are in-repo under `tests/fixtures/htn-oracle/`).
2. Implement conditional-effect grounding to the level the corpus needs: `when` under action `:effect` (condition = ground goal over the same binding, effect = conjunctive add/del), including inside `oneof` branches if the parser's aligned grammar permits (it refuses `when`-in-branch — then only action-level `when` matters); reject deeper nesting with the existing typed error, documented.
3. Translate side: conditional effects become guarded outcomes — evaluate the guard against the source state per outcome (follow `ppddl`'s conditional-effect handling as the in-repo precedent, `crates/ferroplan/src/ppddl/compile/part06.rs`).
4. Flip the two htn_oracle `KNOWN_MISMATCHES` rows to agreement; un-ignore any `#[ignore]`d tripwire they own.

Gates: `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test htn_oracle` exit 0, both PANDA pairs SOLVED with closed policies.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | fix/ground-conditional-effects @ 90c2ae2 | — | all |
| 2026-09-17T16:49:26Z (file mtime of this row's append; wave-4 agent a24 start) | ALIVE | fix/ground-conditional-effects @ 6d14813 | — | reproduce 2 PANDA failures → implement conditional-effect grounding (action-level `when`, conj add/del; deeper nesting → typed error) → translate guarded outcomes (ppddl part06 precedent) → flip 2 KNOWN_MISMATCHES + un-ignore owned tripwires → gates |
| 2026-09-18T00:41:32Z | ALIVE | fix/ground-conditional-effects @ 818e9ea | REPRODUCTION: both PANDA fixtures re-run via their #[ignore]d demos → both GROUND_ERROR — actual live error through solve_hddl is validation `NonGroundRootSubtaskArg` on root subtask 'task0' (wave-3's recorded `nested 'when'`/`unbound variable '?x'` fire on direct grounder calls; root cause for both: parser silently DROPPED `(:htn :parameters ...)` and method `:effect`, so `?x` could never bind and conditional effects never existed). FIX (818e9ea, 8 files +1027/−131): parser parses `:htn :parameters` + method `:effect` (EffectCtx::Nested — when/and/literals, oneof refused); validate checks declared htn-parameter variables (subtype rule as method vars) + method-effect predicates/vars; grounder flattens when-in-when to conjunctive guards (depth≥3 keeps `nested 'when' is out of scope`), grounds method effects onto GroundMethod.effect, grounds root network per admissible htn-parameter binding (GroundedIR.root_networks replaces root_subtasks/root_order), folds method-effect adds into compute_reachability (prune_unreachable stays sound); translate applies guarded outcomes via one shared apply_effect_branch (guards vs SOURCE state, ppddl part06 precedent) for action outcomes AND decomposition, one initial state per root network. Doc: docs/FOND-HTN.md new conditional-effect rules section; RESULTS.md addendum with fresh sweep | gates: final |
| 2026-09-18T00:41:32Z | ALIVE | fix/ground-conditional-effects @ 818e9ea | GATES: `cargo test -p ferroplan-hddl` exit 0 (146 lib + 9 doc tests, 0 failed; includes 8 new unit tests: flattening, depth-3 typed refusal, method-effect grounding, per-binding root networks, zero-binding → empty, undeclared/ill-typed var refusals); `cargo test -p ferroplan --test htn_oracle` exit 0 (3 passed/6 ignored, 85.7s): agreement gate green with both PANDA pairs SOLVED **and outcome-closure-checked** (they left KNOWN_MISMATCHES); their old #[ignore]d GROUND_ERROR demos replaced by active SOLVED tripwires; KNOWN_MISMATCHES now 4/21 (translate-capacity only, other tickets). Full `cargo test -p ferroplan` suite: exit 0, 0 failures (no collateral damage). clippy -p ferroplan-hddl: only pre-existing warnings (verified outside diff hunks). Measure sweep re-run serial (Apple M3 Max): panda-conditional-effect SOLVED translation_ms=1, panda-method-effect SOLVED translation_ms=0; 4 translate-capacity instances unchanged | none — ticket scope complete; commit 818e9ea local (never pushed); operator did not write: all of the above |
| 2026-09-18T01:36:00Z | ALIVE | fix/ground-conditional-effects@818e9ea | — | integration pending |
