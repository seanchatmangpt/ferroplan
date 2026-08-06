# Derived predicates (axioms) — reachability over a map

Field test of ferroplan's `:derived` support. `reachable` isn't set by any action
effect — it's defined by a rule, the transitive closure of `link`:

```pddl
(:derived (reachable ?a ?b - poi)
  (or (link ?a ?b)
      (exists (?c - poi) (and (link ?a ?c) (reachable ?c ?b)))))
```

Because the map (`link`) holds **static**, ferroplan computes the full `reachable`
closure once at grounding, folds it straight into the problem's init — no
per-state axiom evaluation, no hand-written reachable pairs. This is the "explore
the map, build the graph, can I get there" primitive any game world needs.

```sh
ff -o examples/reachability/domain.pddl -f examples/reachability/problem.pddl
# travel camp -> cave directly (reachable camp cave is derived)
```

**Scope:** static derived predicates — body over facts no action ever touches —
are supported, recursion included. A derived predicate whose body leans on facts
an action *does* change (a genuine per-state axiom) returns a clear error instead
of mis-planning.
