---
id: fond-htn-44-duplicate-type-dedup
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: tolerate duplicate identical type declarations (PO_UM-Translog IPC gap)"
standing: ALIVE
branch: fix/duplicate-type-dedup
worktree: ~/ferroplan-worktrees/wt-a44
created: 2026-09-18T00:55:00Z
source: fond-htn-27 sweep (PO_UM-Translog: "type 'Regular_Truck' is declared more than once")
---

Read `_WAVE4-CONTEXT.md`. The IPC-2023 PO_UM-Translog domain declares the same type line twice identically — competition-legal input our validation rejects, blocking the whole domain.

Scope:
1. Validation: duplicate type declarations that are IDENTICAL (same name, same parent) → accept (optionally a warning); genuinely conflicting duplicates (same name, different parent) stay a typed error. Mirror whatever warnings channel exists (UnrefinableCompoundTask precedent).
2. Same policy for duplicate identical `:predicates`/`:objects` entries IF they are also rejected today — check, and align only if blocked by the corpus (do not widen scope beyond evidence).
3. Tests: PO_UM-Translog-shaped minimal fixture (two identical type lines) passes validation + grounds; conflicting-parent duplicate still rejected; add a `#[ignore]`d external case running the real PO_UM-Translog files from `/tmp/fond-review/HDDL-Parser/tests/ipc/PO_UM-Translog/` (skip-with-note if absent).
4. Do not touch parser structure beyond what validation needs.

Gates: `cargo test -p ferroplan-hddl` exit 0; new tests green.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | fix/duplicate-type-dedup @ 6d14813 | — | all |
| 2026-09-17T18:40:00Z | PARTIAL_ALIVE | fix/duplicate-type-dedup @ 6fc8f01 | evidence check: real PO_UM-Translog `:types` FALSIFIES ticket premise — not "same line twice identically" but same type under DIFFERENT parents (multi-parent, e.g. `Regular_Truck - Regular_Vehicle` + `Regular_Truck - Truck`, lines 4-5); identical-only acceptance would NOT unblock the corpus. Decision (recorded failed-edge against premise, scope widened only this far): union-of-parents (multiple inheritance) semantics + warnings; conflicting-different-parent stays legal (corpus-blocked otherwise). Duplicate predicates/objects: corpus check found NONE → those duplicate checks stay errors per scope item 2. Parser must preserve parent SETS (today last-wins collapse hides conflicts from validation) — minimal `TypeDef.parent` -> `parents: Map<Name, Set<Name>>`. | implement, tests, gates |
| 2026-09-17T19:05:00Z | ALIVE | fix/duplicate-type-dedup @ 8f587af | `cargo test -p ferroplan-hddl` exit 0 (141 lib + 9 doc, 0 failed, 1 ignored); `cargo test -p ferroplan -p ferroplan-hddl` exit 0 (224 passed, 0 failed — day-standard gate, dependents unaffected); `cargo test -p ferroplan-hddl --lib -- --ignored` exit 0 — REAL PO_UM-Translog domain (1723 ln, 51 actions, 52 methods) + 18-A problem parse, validate, warn, ground. Adversarial result: 2 first-draft fixtures self-falsified (`(:types d b - d …)` groups bare `d` as child of itself — validator correctly rejected the self-parent under the new syntax; fixtures rewritten to legal shape, detector kept). `translate.rs` fmt-only churn reverted (untouched file); residual fmt drift in touched files is pre-existing at base. Deliverables: ast.rs `TypeDef.parents`, parser.rs `parse_types` union, validate.rs (`DuplicateTypeDeclaration` warning, `DuplicateKind::Type` removed, diamond-safe cycle DFS, tests), grounder.rs (multi-parent `ancestors_of`/`build_type_closure`, `TypeCycle` preserved), VALIDATION-NOTES.md addendum, `#[ignore]`d external test (corpus stays in /tmp, nothing vendored). 比: 100% hand-written on 産面 — no pack/generator expresses HDDL type-system semantics in this repo; no HANDWRITTEN.md ledger instantiated in this repo (History rows here are the operative ledger). | none — ticket complete |
| 2026-09-18T03:02:00Z | ALIVE | fix/duplicate-type-dedup @ 8f587af | — | coordinator-close: integrated by coordinator at 75870de lineage (merge 0f39d2d); tip matches final-row receipt 8f587af; readiness union fond=17/hddl=20 | none — closed |
