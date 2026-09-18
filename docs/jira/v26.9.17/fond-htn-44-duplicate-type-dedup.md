---
id: fond-htn-44-duplicate-type-dedup
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: tolerate duplicate identical type declarations (PO_UM-Translog IPC gap)"
standing: BLOCKED
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
