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
| 2026-09-18T00:05:00Z | PARTIAL_ALIVE | fix/eq-goal-evaluation@6d14813 (wt-a21) | — | orient: code |
| 2026-09-18T00:35:00Z | PARTIAL_ALIVE | fix/eq-goal-evaluation@6d14813 (wt-a21, uncommitted) | cargo test -p ferroplan-hddl --lib: 141 pass (grounder Eq/when-fold tests in) | translate tests, oracle test flip + goldens, docs parity note, full gates |
| 2026-09-18T01:05:00Z | PARTIAL_ALIVE | fix/eq-goal-evaluation@24c94f6 (wt-a21) | gate A: cargo test -p ferroplan-hddl exit 0 (144 lib + 9 doc, 0 fail); gate B: cargo test -p ferroplan --test fond_htn_oracle exit 0 (micro_recurse_agrees_with_oracle un-ignored + passing, SOLVED + outcome-closed); extra boundary: cargo test -p ferroplan exit 0 (all binaries, 0 fail) | ticket closed if receipt accepted |
| 2026-09-18T01:10:00Z | ALIVE | fix/eq-goal-evaluation@24c94f6 (wt-a21; not merged, not pushed) | gates A+B exit 0 (see prior row); falsifier PASSED: micro-recurse fixture solves (ferroplan SOLVED + outcome-closed, agrees with koala SOLVED@0.212s) | none — ready for coordinator integration (serial, post-wave) |
| 2026-09-18T01:36:00Z | ALIVE | fix/eq-goal-evaluation@24c94f6 | — | integration pending |

Receipt: repo/ferroplan base 90c2ae2, branch tip 24c94f6, 5 files changed (+558/-22): grounder.rs (GroundGoal::Eq lowering + evaluate/relaxed arms + static when-condition fold with never-present marker), translate.rs (to_dnf = fold + MalformedTermEquality), oracle-goldens.json (micro-recurse healed, oracle verdict untouched), fond_htn_oracle.rs (mismatch pin promoted to always-on agreement test; matrix row corrected), docs/FOND-HTN.md (= semantics + FaultyPort parity deviation row). New tests: 8 (Eq state-independence incl. relaxed, method-grounding shapes + action not(=), method-precondition decomposition gating both directions, when-fold grounding, when never-fire e2e + marker-leak tripwire, :goal folds). Falsifiers attempted: micro-recurse must solve (passed, live run in gate B); pre-existing fixture-g when(=) domain must keep grounding (passed, 144/144); goldens ledger consistency test passed. 比: manufactured_lines/total_delivered_lines = 0/558 — all hand-written; no admitted pack/generator expresses planner-internal HDDL semantics (no UNSUPPORTED ledger row or HANDWRITTEN.md exists in this repo; wave receipts are ticket History rows per _FOND-HTN-WAVE-CONTEXT.md ticket mechanics — gap recorded here rather than papered over). Operator wrote nothing (execution, gates, commits all agent-run). Did NOT do: merge to main (coordinator owns integration), push (forbidden), touch koala sources (KOALA POLICY), reformat pre-existing rustfmt drift in untouched regions (left as-is, verified pre-existing on base via stash check). [1302]: none encountered.
