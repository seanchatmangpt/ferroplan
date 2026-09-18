# ferroplan-hddl validation notes — fond-htn-08 (v26.9.17)

What `src/validate.rs` checks, what was added by this ticket, and the external
validation run over the koala FOND-HTN corpus. Both entry points
(`validate_domain`/`validate_problem`, plus the `*_with_warnings` variants) run
inside `grounder::ground` before any grounding work, so every check below is
enforced on the solve path.

## 1. Inventory: checks that already existed (base d2faf4d)

- Method/subtask/root task references resolve to a declared task or action
  (`ValidationError::UnknownTaskOrAction`).
- Method head arity (`ValidationError::ArityMismatch`).
- Predicates referenced in action preconditions/effects were declared
  (`UndefinedPredicate`, built-in `=` exempt).
- Types referenced in typed parameter lists (predicate/numeric-fluent/task/
  action/method params, action-precondition and `:goal` quantifier binders)
  were declared (`UndefinedType`).
- Duplicate task/predicate/type declarations (`DuplicateDefinition`).

## 2. Gap list (ticket item 1) and what was added

| Reference check | Status at base | Now |
|---|---|---|
| cyclic type hierarchy | missing | `CyclicTypeHierarchy` (parent-chain walk incl. self-parents) |
| undeclared types | params + some binders only | + `:constants`/`:objects` type names, method-precondition quantifier binders (`UndefinedType`) |
| duplicate predicate/task/method/action/object declarations | task/predicate/type only | + `DuplicateKind::Action`, `DuplicateKind::Method`, `DuplicateKind::Object` (domain `:constants` + problem `:objects`) |
| predicate arity at use | missing | `PredicateArityMismatch` over action preconditions/effects (incl. `when` conditions), method preconditions, `:init`, `:goal` |
| method-head = declared compound task + arity | declared *name* + arity, but an action-named head gave a misleading error | `MethodHeadNotCompoundTask` for action heads; `ArityMismatch` for arity |
| subtask exists + arity + argument typing (subtyping) | existence only | `ArityMismatch` per subtask call; `UnknownMethodVariable` for undeclared variables; `ArgumentTypeMismatch` when the argument's type is not a subtype of the callee's parameter type (`is_subtype` walk over `:types`, `object` = top) |
| method-precondition variables declared | missing | `UnknownMethodVariable` (scoped walk; `forall`/`exists` binders introduce scope) |
| ordering constraints acyclic (methods AND problem `:htn`) | missing | `CyclicOrdering` (Kahn + concrete cycle path) per network; endpoints must be real subtask ids (`UndefinedOrderRef`) |
| init atoms declared/typed | missing | `UndefinedPredicate` / `PredicateArityMismatch` / `NonGroundInitAtom` / `UnknownConstant` / `ArgumentTypeMismatch` per `:init` atom |
| problem goal atoms | only quantifier types checked | + declared predicate, arity, ground-constant existence/typing per `:goal` atom |
| unrefinable compound task (TDG reachability + nullability, ignoring preconditions) | missing | WARNING `ValidationWarning::UnrefinableCompoundTask` via `validate_domain_with_warnings`/`validate_problem_with_warnings`; least-fixpoint: a compound task is refinable once some method for it has a network whose compound subtasks are all refinable (empty network = nullability base case); warns only for *referenced* tasks (heads, method-network subtasks, root `:htn` subtasks) |

## 3. Koala gaps beaten (ticket item 2)

- **Dangling ordering-constraint ids**: koala silently ignores ordering edges
  naming non-existent subtask ids; `UndefinedOrderRef` is a typed ERROR
  (methods and problem `:htn` alike).
- **Problem with no usable task network**: koala accepts problems whose
  `:htn` carries no subtasks (and root networks with bare variables);
  `MissingTaskNetwork` (empty root network) and `NonGroundRootSubtaskArg`
  (variable root-subtask argument) are typed ERRORs — `solve_hddl`'s
  decomposition model has no input in either case. See the Satellite
  `1obs-2sat-1mod` finding below for the latter firing on real corpus data.

## 4. Deliberate boundaries (not checked, recorded per the silent-pruning ban)

- Task-vs-action name collisions across the shared task namespace (only
  within-kind duplicates are checked).
- Schema-level constant resolution inside method networks: real-world HDDL
  method networks reference bare constants without a `:constants` block (our
  own `translate.rs` shortcut test does), and the ticket's scope puts
  ground-term checking at the problem level (`:init`, `:goal`, root `:htn`).
- Free (unbound) variables in `:goal` atoms — only `:init` and root `:htn`
  groundness is enforced; goal quantifier typing is checked, goal atom
  variables are not scope-analyzed.
- Predicate arity inside `increase`/`decrease` numeric-fluent references
  (separate namespace; refused at grounding by
  `GroundError::UnsupportedNumericFluent`).
- A `:types` parent name that is never declared as a child still counts as
  declared (pre-existing `declared_type_names` rule; unchanged).

## 5. External validation run (2026-09-17)

Harness: `/tmp/hddl-validate-run` (files stay in /tmp; path-dependency on the
worktree crate; prints one JSON line per domain/problem pair). Corpus:
`/tmp/fond-review/domains/` (koala FOND-HTN domains; read as an external test
oracle, nothing vendored).

Required run — first problem of each of the 6 usable domains, plus the
upstream-broken domain:

| Domain | Problem | Result |
|---|---|---|
| Transport | pfile01.hddl | OK, 0 warnings |
| Childsnack | p01.hddl | OK, 0 warnings |
| Depots | p01.hddl | OK, 0 warnings |
| Rover | pfile01.hddl | OK, 0 warnings |
| Satellite | 1obs-1sat-1mod.hddl | OK, 0 warnings |
| Snake | pb01.snake.hddl | OK, 0 warnings |
| AssemblyHierarchical | genericLinearProblem_depth01.hddl | REJECTED: `type 'FaultyPort' is used but never declared in :types` — expected/correct |

Full sweep (all 15 problems per domain), run for extra evidence:

| Domain | Problems | OK (0 warnings) | Rejected |
|---|---|---|---|
| Transport | 15 | 15 | — |
| Childsnack | 15 | 14 | p03: `'bread6' is used but is neither a declared problem object nor a domain constant` (`:objects` declares bread1–bread5 only; `bread6` appears in `:init` — upstream corpus defect, true positive) |
| Depots | 15 | 15 | — |
| Rover | 15 | 15 | — |
| Satellite | 15 | 14 | 1obs-2sat-1mod: root task-network subtask 'task0' has a variable argument (the koala dialect declares `:htn :parameters (...)` and never grounds them — unusable as solve_hddl input, true positive for this pipeline) |
| Snake | 15 | 15 | — |
| AssemblyHierarchical | 15 | 0 | all 15 rejected on undeclared `FaultyPort` (upstream-broken domain; parse-rejection case per wave context) |

Reading: zero false positives on the 6 usable domains' 90 problems minus the
two upstream defects above; the unrefinable-task fixpoint produced zero
warnings on all usable domains (their compound tasks are all refinable).

## 6. Tests added / changed (gate: `cargo test -p ferroplan-hddl`, exit 0)

- Accept: `fixtures/g` — hand-authored transport-style FOND-HTN domain+problem
  (provenance comment in-file; NOT copied from any planner repo) validates
  clean with zero warnings; a wrong-cargo-type root call is rejected with
  `ArgumentTypeMismatch`.
- Reject: dangling order id (method + root), cyclic ordering (method + root),
  unrefinable-task warning (with nullability positive case via fixture f's
  `setdone`), duplicate method/action/object, undeclared type on
  constant/object, missing task network (empty `:htn` and absent `:htn`),
  predicate arity at use (precondition/init/goal), undeclared predicate in
  method precondition, subtask arity, argument typing (positive subtype +
  negative supertype), unknown method variable (precondition + subtask arg),
  cyclic/self-parent type hierarchy, unknown constant in init/root, non-ground
  init/root subtask, method head naming an action, method head arity.
- Existing tests grounding empty-root-network problems were given real root
  subtasks (their subject was grounding internals, not empty networks);
  `MissingTaskNetwork` is enforced inside `ground()` via `validate_problem`.

## Addendum — duplicate type declarations decriminalized (ticket fond-htn-44, 2026-09-17)

The sweep finding this addendum answers: IPC-2023 PO_UM-Translog (competition
input, koala HDDL-Parser corpus) was rejected with `type 'Regular_Truck' is
declared more than once`, blocking the whole domain. The ticket premise ("the
same type line twice identically") is falsified by the file itself: it declares
the same type under DIFFERENT parents (`Regular_Truck - Regular_Vehicle` plus
`Regular_Truck - Truck`, ~40 such names) — multiple inheritance, and legal
competition input. Identical-only acceptance would have left the domain
blocked; the recorded decision is therefore:

- Duplicate TYPE names are no longer an error. `DuplicateKind::Type` is removed
  (an unproducible error variant is fabricated vocabulary). Duplicate
  task/predicate/action/method/object declarations remain typed errors
  (corpus check: no such duplicates in PO_UM-Translog).
- `TypeDef.parent: Map<Name, Name>` is now `TypeDef.parents: Map<Name,
  Set<Name>>`: the parser keeps the UNION of declared `child - parent` edges
  (previously last-wins, which silently dropped declared edges and hid the
  conflict from validation). `is_subtype`, `check_cyclic_type_hierarchy`, and
  `grounder`'s `ancestors_of`/`build_type_closure` walk all parents.
- New warning `ValidationWarning::DuplicateTypeDeclaration { name }` (domain
  level, `validate_domain_with_warnings`): one per redeclared type name,
  mirroring the `UnrefinableCompoundTask` warnings channel.
- Cycle detection is now diamond-aware (white/grey/black DFS): shared
  ancestors reached through two branches (`Regular_Truck -> {Regular_Vehicle,
  Truck} -> Vehicle`) are NOT cycles; a type reappearing on the current path
  still is (`CyclicTypeHierarchy` / `GroundError::TypeCycle`).
- Evidence: `cargo test -p ferroplan-hddl` green (141 lib + 9 doc tests), the
  real PO_UM-Translog domain+problem parse, validate, warn, and ground via the
  `#[ignore]`d external test (`external_po_um_translog_validates_and_grounds`,
  run with `cargo test -p ferroplan-hddl -- --ignored`; files stay in /tmp per
  KOALA POLICY, nothing vendored).
