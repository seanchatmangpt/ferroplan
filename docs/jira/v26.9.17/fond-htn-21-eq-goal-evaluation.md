---
id: fond-htn-21-eq-goal-evaluation
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: positive (= …) passes parse/validate but never evaluates true in translate"
standing: BLOCKED
created: 2026-09-17T23:55:00Z
source: fond-htn-04 differential wave (ORACLE_MISMATCH micro-recurse fixture)
---

Read `_FOND-HTN-WAVE-CONTEXT.md`. Finding: d2faf4d taught the parser to recognize the built-in `=` term-equality predicate, and validation accepts it, but `translate`'s `evaluate_ground_goal` has no positive-`(= …)` arm — the atom never evaluates true, so deterministically solvable micro-recurse fixtures return `NoPlan`. Evidence: wt-oracle-fondhtn branch `test/fond-htn-oracle` `ORACLE_MISMATCH_*` tests + goldens (`crates/ferroplan/tests/fixtures/fond-htn/oracle-goldens.json`).

Scope: add `=` evaluation in the ground goal/effect paths (term equality against ground objects, `=` and `not =` in preconditions/method conditions), tests: `(= a a)` true, `(= a b)` false, `(not (= a b))` true, inside a method `:precondition` gating decomposition, plus flip the corresponding ORACLE_MISMATCH_#[ignore] to an always-on agreement test. Falsifier: the micro-recurse fixture must solve. Also record the koala parity fact (T02): koala's parser silently DROPS undeclared `FaultyPort` objects (exit 0) where ferroplan correctly rejects — keep ferroplan's behavior, add the parity note to docs/FOND-HTN.md's deviations table.

Gates: `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test fond_htn_oracle` exit 0 with the mismatch test un-ignored.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:55:00Z | BLOCKED | none | — | all |
