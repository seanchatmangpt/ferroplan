---
id: fond-htn-08-fix-validation
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Harden ferroplan-hddl validation to reference level and beyond"
standing: BLOCKED
branch: fix/hddl-validation
worktree: ~/ferroplan-worktrees/wt-validate
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only; no push. Concept parity only — no koala code (the reference checks below are semantic concepts from the HDDL literature, fine to implement).

## Scope
1. Inventory existing checks in `crates/ferroplan-hddl/src/validate.rs` (+ parser-deferred checks). List gaps vs: cyclic type hierarchy; undeclared types; duplicate predicate/task/method/action/object declarations; predicate arity at use; method-head = declared compound task + arity; subtask exists + arity + argument typing (subtyping); method-precondition variables declared; ordering constraints acyclic (methods AND problem `:htn`); init atoms declared/typed; unrefinable compound task (TDG reachability + nullability, ignoring preconditions) as a WARNING.
2. Koala's gaps you must beat: ordering-constraint IDs referencing non-existent subtask IDs → typed ERROR; problem with neither `:htn` nor usable task network for solve_hddl's model → typed ERROR.
3. Implement missing checks with typed errors (extend the crate's error vocabulary); warnings channel for unrefinable tasks matching the existing API style.
4. External validation run (files stay in /tmp): run your validator over the 6 usable koala domains + pfile01 each, and AssemblyHierarchical pfile01 (must FAIL on undeclared `FaultyPort` — correct behavior). Record per-domain results in the ticket History and a committed `crates/ferroplan-hddl/VALIDATION-NOTES.md`.
5. Tests: accept (Transport domain+pfile01), reject per new check (bad ordering ID, cyclic ordering, unrefinable-warning, duplicate method, undeclared type, missing task network).

## Gates
`cd ~/ferroplan-worktrees/wt-validate && cargo test -p ferroplan-hddl` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | fix/hddl-validation @ base d2faf4d | — | all |
| 2026-09-17T22:05:00Z | PARTIAL_ALIVE | fix/hddl-validation (worktree, uncommitted) | inventory done; new checks + fixture g + tests written, gate pending | gate, external run, notes, commit |
| 2026-09-17T22:25:00Z | PARTIAL_ALIVE | fix/hddl-validation (worktree, uncommitted) | `cargo test -p ferroplan-hddl` exit 0 (126 unit + 9 doctests, 0 failed); clippy: no new warnings vs base; `cargo test -p ferroplan --lib hddl` 3/3, wasm crate compiles | external run, notes, commit |
| 2026-09-17T22:40:00Z | PARTIAL_ALIVE | fix/hddl-validation (worktree, uncommitted) | external run (harness /tmp/hddl-validate-run, files stay in /tmp): 6 usable domains + first problem all OK with 0 warnings; AssemblyHierarchical depth01 REJECTED `UndefinedType(FaultyPort)` as required; full 15-problem sweep: Transport 15/15, Depots 15/15, Rover 15/15, Snake 15/15, Childsnack 14/15 (p03: undeclared object bread6 in :init — upstream defect, true positive), Satellite 14/15 (1obs-2sat-1mod: non-ground root :htn variables — true positive, koala accepts we reject) | notes, commit |
| 2026-09-17T22:50:00Z | ALIVE | fix/hddl-validation @ 0e28182 | gate re-run on committed head: `cargo test -p ferroplan-hddl` exit 0 (126+9, 0 failed); worktree clean; VALIDATION-NOTES.md committed with inventory + external results; no push, main untouched | none |
