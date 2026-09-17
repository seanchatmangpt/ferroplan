---
id: fond-htn-30-stress-memory-ceilings
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Stress: memory ceilings — adversarial grounding blowups refuse, never OOM"
standing: BLOCKED
branch: stress/memory-ceilings
worktree: ~/ferroplan-worktrees/wt-s30
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Precedent: c7c2c4d "hard memory ceiling to translate BFS"; T14's finding that the BINDING odometer closed an unbounded-memory DoS.

Scope:
1. Hand-author adversarial schema generators (seeded, in-test): actions with 2–8 typed parameters over object sets sized to make the naive binding product explode (10^7+ combinations); wide method fan-out (task with 50 methods × 10 subtasks); predicate-heavy domains (500 predicates); deep-but-legal typing chains.
2. Run each under measured ceilings: configure `GroundingLimits`/`TranslateLimits`/`PlannerLimits` caps, run the child with `RLIMIT_AS` (e.g. 512 MB — spawn via a helper binary or `std::process::Command` with a shell ulimit wrapper; if the test harness forbids that, run in-process with the crate's own ceilings and measure RSS via `mach_task_info`/`ps` sampling, documenting the method).
3. Contract per case: typed refusal (`ResourceBound`/`Timeout`-shaped) OR clean solve within ceiling — never SIGKILL, never an unbounded RSS climb (record peak RSS per case).
4. Peak-RSS table + refusal-latency table committed in `crates/ferroplan/tests/fixtures/memory-stress/RESULTS.md`; sampled always-on subset (2 fastest cases) + `#[ignore]`d full set.

Gates: `cargo test -p ferroplan --test memory_stress` exit 0 (sampled); `-- --ignored` exit 0; every case's outcome typed-or-solved (zero signals).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | stress/memory-ceilings @ 90c2ae2 | — | all |
