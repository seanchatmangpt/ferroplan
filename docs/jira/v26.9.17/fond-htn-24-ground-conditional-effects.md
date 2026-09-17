---
id: fond-htn-24-ground-conditional-effects
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: ground conditional effects — 2 PANDA pairs die with nested-'when' / unbound-variable"
standing: BLOCKED
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
