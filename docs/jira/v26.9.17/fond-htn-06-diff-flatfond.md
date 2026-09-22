---
id: fond-htn-06-diff-flatfond
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Differential: canonical flat-FOND domains (tireworld family) — embedding vs both engines"
standing: ALIVE
branch: test/fond-flat-oracle
worktree: ~/ferroplan-worktrees/wt-oracle-flat
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Everything hand-authored by you (canonical domain semantics from the literature — cite the originating paper in each file header). No koala files.

## Scope
1. Author 8 canonical flat-FOND domains, each in two encodings under `crates/ferroplan/tests/fixtures/fond-flat/<domain>/`:
   a. `problem.json` — explicit `PlanningProblem` (tiny, 4–12 states): tireworld (strong-cyclic), triangle-tireworld-3cities (strong-cyclic), islands-2 (strong-cyclic), wall-2rows (strong-cyclic), faults-1bit (strong), boolean-not (strong), coffee (strong-cyclic), river-unsafe (unsolvable variant). Use two-outcome actions; probability masses 500000/500000 (this branch still enforces mass==1M — see context doc repo-state note).
   b. `domain.hddl` + `problem.hddl` — HDDL embedding: nondet action effects via oneof; a `finish` primitive action whose precondition is the goal facts; root task = ordered (drive…, finish) via methods that leave step choice to the planner where the domain needs it.
2. Literature classes (recorded in each `verdict.json` alongside): {tireworld: cyclic-only, triangle: cyclic-only, islands: cyclic-only, wall: cyclic-only, faults: strong, boolean: strong, coffee: cyclic-only, river: unsolvable}.
3. Run BOTH engines: oracle on the HDDL encoding (`oracle-run.sh --mode flexible`, self-sufficient protocol per T02); ferroplan on BOTH encodings — `solve_planning_type(PlanningType::Fond)` for `problem.json`, `solve_hddl` for the HDDL pair.
4. Test `crates/ferroplan/tests/fond_flat_oracle.rs`: for each domain assert three-way class agreement (ferroplan-explicit, ferroplan-hddl, oracle) with the literature class; cyclic-only cases must solve via the Fond dispatcher AND produce outcome-closed policies; unsolvable must be typed NoPlan on both ferroplan paths. Mismatch → `#[ignore]` test + History row.
5. Commit `oracle-goldens.json` (verdicts only).

## Gates
`cargo test -p ferroplan --test fond_flat_oracle` exit 0; goldens + 8 fixture dirs committed.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/fond-flat-oracle @ base d2faf4d | — | all |
| 2026-09-17T21:50:00Z | ALIVE | test/fond-flat-oracle @ d2faf4d (clean) | worktree clean at base; API orient (fond_policy ~L721, fond_policy_strong_cyclic ~L811, dispatch L380, solve_hddl, translate decompose/oneof semantics) | 8x(problem.json+domain.hddl+problem.hddl+verdict.json), test, goldens |
| 2026-09-17T22:16:00Z | ALIVE | test/fond-flat-oracle @ d2faf4d (8 fixture dirs authored) | oracle sweep via /tmp/fond-oracle/oracle-run.sh (flock T01), flexible 8/8 + fixed-ld cross-check 5/5, exit 0: strong(faults,boolean)=SOLVED ~0.1s, unsolvable(river)=NOSOLUTION 0.16s, cyclic-only(tireworld,triangle,islands,wall,coffee)=TIMEOUT both modes — signature: no finite strong plan tree; harness health-proven on Transport pfile01 (SOLVED 0.2s) | test+gate |
| 2026-09-17T22:29:00Z | ALIVE | test/fond-flat-oracle @ 61fa3a7 | `cargo test -p ferroplan --test fond_flat_oracle` exit 0 (9 passed, 1 ignored, 0 failed, 0.04s); `-- --ignored` live-oracle rerun exit 0 (31.3s); `cargo test -p ferroplan --lib hddl` exit 0 (3 passed) | history+report |
| 2026-09-17T22:32:00Z | ALIVE | test/fond-flat-oracle @ 61fa3a7 (parent d2faf4d, not pushed) | goldens + 8 fixture dirs + fond_flat_oracle.rs committed (34 files, 1280 ins); three-way class agreement green on all 8 domains; outcome-closed policies verified on both ferroplan paths for all solvable classes; unsolvable typed NoPlan on both paths | none — oracle strong-cyclic support gap recorded in goldens semantics_note (koala diverges where ferroplan solves; candidate fact for dfcm-agent-pack ontology) |
