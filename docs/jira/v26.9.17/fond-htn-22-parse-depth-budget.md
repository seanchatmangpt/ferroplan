---
id: fond-htn-22-parse-depth-budget
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: unbounded parser recursion — 1000-deep (and …) SIGABRTs the process"
standing: BLOCKED
created: 2026-09-17T23:55:00Z
source: fond-htn-14 adversarial wave (TODO(parse-depth-budget))
---

Read `_FOND-HTN-WAVE-CONTEXT.md`. Finding: `parse_domain` recurses per nesting level with no depth budget; measured: 250-deep `(and …)` parses, 500 aborts, 1000 SIGABRTs the process (stack overflow kills the test harness — worse than a panic). The adversarial suite carries it as `#[ignore]`d `deeply_nested_effect_is_bounded_never_aborts` (test/hddl-adversarial branch, now on main).

Scope: thread a depth counter through `parse_domain`/`parse_problem`/goal/effect recursion (or convert the hot path to an explicit worklist); budget from a constant with a typed `ParseError::NestingTooDeep` (line/position payload); make the budget overridable for tests. Un-ignore the adversarial case asserting the typed error at 1000-deep. Falsifier: 10 000-deep input returns the typed error, exit 0, no signal.

Gates: `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test hddl_adversarial` exit 0 with the case enabled.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:55:00Z | BLOCKED | none | — | all |
