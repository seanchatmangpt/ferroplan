---
id: fond-htn-14-test-adversarial
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Adversarial robustness: malformed HDDL never panics or hangs"
standing: BLOCKED
branch: test/hddl-adversarial
worktree: ~/ferroplan-worktrees/wt-test-robust
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. Sleath–Bercher flawed files are koala-repo data → run them from `/tmp` ONLY as an external negative corpus; in-repo cases are hand-authored by you.

## Scope
1. Read prior hardening commits for coverage: `dfd907b` (adversarial input), `c7c2c4d` (memory ceiling), `9a41178` (typed refusals) — extend, don't duplicate.
2. `crates/ferroplan/tests/fixtures/hddl-adversarial/` + `crates/ferroplan/tests/hddl_adversarial.rs`: ≥ 25 hand-authored malformed cases across: unbalanced parens; empty domain; keyword typos; `:method` without `:task`; ordering ID → unknown subtask; cyclic ordering; wrong-arity `:htn` task; `(oneof)`; nested oneof; oneof in `:precondition`; `when` inside oneof branch; undeclared predicate in effect; domain-name mismatch; infinite decomposition recursion (must hit translate limits — API wall/memory knobs, never hang; if no knob fits, `#[ignore]` + TODO note); deep nesting (1000-deep and); zero-length file; binary garbage; unicode identifiers.
3. Assert per case: typed error variant (specific where predictable) or clean reject; wrap every call so a PANIC fails the test.
4. External negative corpus: run ferroplan-hddl parse (+validate) over the 26 Sleath–Bercher files at `/tmp/fond-review/HDDL-Parser/tests/flawed_domains/` from an `#[ignore]`d test reading absolute paths (skip-with-note if dir absent): each must reject with a diagnostic, never panic. Record outcomes in History.

## Gates
`cd ~/ferroplan-worktrees/wt-test-robust && cargo test -p ferroplan --test hddl_adversarial && cargo test -p ferroplan-hddl` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/hddl-adversarial @ base d2faf4d | — | all |
