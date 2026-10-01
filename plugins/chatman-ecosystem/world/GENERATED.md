# GENERATED

`ferroplan-self-host-domain.pddl` and `../scripts/world_tables.py` (the `PREDICATES` and `GOALS`
dicts imported by `../scripts/project-world.py`) are projections of
`/ontology/ferroplan-self-host.ttl`. Never hand-edit them; change the ontology or the templates in
`queries/` and `templates/` and re-run ggen (rules `self-host-pddl`, `self-host-tables`).

`ferroplan-self-host-problem.pddl` stays hand-written (default problem instance).

## Ledger

| Item | Standing | Reason |
|---|---|---|
| PDDL text emitter | UNSUPPORTED(generator-capability: pddl-text) | no ggen-marketplace pack emits PDDL text; local templates used, wf:Action shape of pddl-embedded-workflow-pack is the model |
| `ferroplan-domain.ttl` reuse | NOT APPLICABLE | that graph models ferroplan's API/tool shapes, not a planning domain |

## Falsifier

Rename one `shw:Action` or one `shw:dimensionKey` in the ontology and regenerate: the `.pddl`
action name and the `PREDICATES` key both change, and `ff` plans with the mutated domain.
