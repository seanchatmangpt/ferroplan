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
| 2026-09-17T22:05:00Z | PARTIAL_ALIVE | test/hddl-adversarial @ d2faf4d | oriented: prior commits dfd907b/c7c2c4d/9a41178 read for coverage; corpus present (26 files at /tmp/fond-review/HDDL-Parser/tests/flawed_domains) | fixtures + hddl_adversarial.rs + corpus run + gates |
| 2026-09-17T22:56:35Z | PARTIAL_ALIVE | test/hddl-adversarial @ b3fd606 | suite written + probed; findings: (1) REAL PANIC — ferroplan-hddl translate.rs:641 total_order_ranks `.expect("order endpoint is a pending key")` on ordering edge to unknown subtask id → fixed via one-statement panic-to-error conversion (existing None path), commit b3fd606; (2) KNOWN DEFECT — 1000-deep `(and ...)` SIGABRTs process (stack overflow in parse_domain, unbounded read_one/parse_goal recursion; measured 250 ok / 500 abort) → #[ignore] + TODO(parse-depth-budget) per ticket, not a one-line fix; (3) recursion case falsifies ticket premise: canonicalization collapses loop→loop into a self-loop, translate terminates instantly, limits never fire, solver refuses cleanly; (4) audit gaps recorded as documented tolerance: domain-name mismatch unchecked, trailing extra paren discarded by read_top, validate_problem misses root arity + goal objects | corpus run + final gates |
| 2026-09-17T23:05:00Z | ALIVE | test/hddl-adversarial @ 25d064a | gate1 `cargo test -p ferroplan --test hddl_adversarial` exit 0: 25 passed / 0 failed / 2 ignored; gate2 `cargo test -p ferroplan-hddl` exit 0: 91 unit + 7 doc passed; corpus (#[ignore]d, run explicitly): 26 files → 3 REJECT(parse) + 6 REJECT(validate) + 4 REJECT(ground: 2× TypeCycle, 2× UnboundVariable) + 13 ACCEPT-in-shrink-only-allowlist + 0 panics; deliverables: crates/ferroplan/tests/hddl_adversarial.rs + 29 fixtures under crates/ferroplan/tests/fixtures/hddl-adversarial/ (all hand-authored, provenance comments; zero-length + non-UTF-8 .bin excepted, provenance in test file); src delta: 1 file, one statement (listed above) | none — remaining follow-ups filed as TODO(parse-depth-budget) + CORPUS_ACCEPTED_SCOPE_GAPS shrink ledger |
