---
id: fond-htn-16-corpus-flatfond
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Canonical flat-FOND explicit-state corpus + literature verdicts (/tmp)"
standing: BLOCKED
worktree: none (/tmp only)
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. /tmp only; no repo writes.

## Scope
Deliver `/tmp/fond-corpus/fond-flat/`:
1. 8+ canonical domains (tireworld, triangle-tireworld, islands, wall, faults, boolean, coffee, river, blocksworld-FOND, earth-observation — do as many as time allows, minimum 8): small PDDL-style domain+problem files under `pddl/` (hand-authored faithful to the papers — cite source per file; use WebSearch to pull canonical definitions where memory is uncertain, e.g. IPPC/FIP/PRP papers).
2. `explicit/` — the same problems as explicit-state JSON (ferroplan `PlanningProblem` shape: states[{id,facts}], initial_states, goal.facts, transitions[{action,from,to,probability_ppm}] with 500000/500000 masses). Keep ≤ 40 states per instance; downsize where needed.
3. `expected-verdicts.json`: {instance: "strong"|"cyclic-only"|"unsolvable"} from the literature, with a `source` citation per row.
4. `README.md`: per domain — what the non-determinism models, citation, verdict, encoding notes (what you downsized).

Gates: JSON parses, ≥ 8 domains, every verdict row carries a source.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | none | — | all |
