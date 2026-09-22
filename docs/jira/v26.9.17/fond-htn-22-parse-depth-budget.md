---
id: fond-htn-22-parse-depth-budget
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: unbounded parser recursion — 1000-deep (and …) SIGABRTs the process"
standing: ALIVE
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
| 2026-09-17T23:53:00Z | ALIVE | fix/parse-depth-budget @ 6d14813 (worktree wt-a22; base 90c2ae2 is ancestor, delta = ticket-docs commit only) | oriented: abort site is `read_one` (Sexp reader) — single choke point bounds goal/effect/sexp_to_string/Drop recursion transitively; `preprocess` already iterative; `HddlError::Parse(String)` mapping needs no change | depth budget + NestingTooDeep{line,column,budget} + _with_budget overrides; un-ignore adversarial 1000-deep; 10k-deep falsifier; gates |
| 2026-09-18T00:01:45Z | ALIVE | fix/parse-depth-budget @ e272cbd | `cargo test -p ferroplan-hddl` exit 0 (145 passed incl. 7 new depth-budget unit tests, 9 doc-tests); `cargo test -p ferroplan --test hddl_adversarial` exit 0 (27 passed: un-ignored `deep_nesting_1000_and_returns_typed_nesting_error_never_aborts` + falsifier `ten_thousand_deep_and_returns_typed_error_exit_zero_no_signal`; 1 ignored = external corpus runner, unrelated); `cargo test -p ferroplan --no-run` exit 0 (all sibling test targets compile) | none — integration is coordinator's |
| 2026-09-18T00:01:45Z | ALIVE (done) | fix/parse-depth-budget @ e272cbd | falsifier run as specified: 10 000-deep input → typed `ParseError::NestingTooDeep` (budget payload asserted = 256), exit 0, no signal; 1000-deep pipeline surfaces `HddlError::Parse("nesting too deep ...")`; `DEFAULT_MAX_PARSE_DEPTH=256`, override via `parse_domain_with_budget`/`parse_problem_with_budget` | remaining: none (coordinator integrates; never pushed) |
| 2026-09-18T01:36:00Z | ALIVE | fix/parse-depth-budget@e272cbd | — | integration pending |
