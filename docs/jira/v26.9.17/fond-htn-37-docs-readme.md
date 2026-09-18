---
id: fond-htn-37-docs-readme
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs: README capability statement — FOND/HTN/HDDL, honestly, for the first time"
standing: BLOCKED
branch: docs/readme-capability
worktree: ~/ferroplan-worktrees/wt-d37
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. The README currently says nothing about FOND/HTN/HDDL (wave-3 audit: zero grep hits). The stack now exists and is tested — say so, precisely.

Scope:
1. README section "Hierarchical & non-deterministic planning" (match the README's existing voice and structure): what's supported — HDDL subset + `oneof` rules one line each; strong + strong-cyclic FOND policies over explicit state graphs; `solve_hddl` entry point; wasm ops `hddl_solve`/`htn_plan`/`fond_policy`; Eve bridge one-liner; pointer to `docs/FOND-HTN.md` and `docs/BENCHMARKS.md`.
2. Honesty rules: every claim maps to a test name or RESULTS file (cite inline like the rest of the repo does); state the known limits verbatim from the deviation tickets (translate 10 s internal cap, conditional-effect grounding gap, parse depth) as a short "current limits" list — the README must under-promise.
3. Do not touch version numbers, badges, or unrelated sections; minimal diff.

Gates: `cargo test -p ferroplan --doc` exit 0; every test name cited exists (script-verify).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | docs/readme-capability @ 90c2ae2 | — | all |
| 2026-09-18T00:56:08Z | PARTIAL_ALIVE | docs/readme-capability @ 6d14813 (wt-d37; base 90c2ae2 is ancestor, delta = wave-4 ticket-docs commit only) | oriented: README + docs/FOND-HTN.md + deviation tickets 21-24 read; section drafted after "Library" (H2 "Hierarchical & non-deterministic planning"): HDDL subset one bullet, `oneof` rules one line each, strong+strong-cyclic over explicit state graphs, FOND-HTN (TN,state) policies, `solve_hddl` entry point, wasm ops `hddl_solve`/`htn_plan`/`fond_policy`, Eve `solve_hddl_from_eve` one-liner, pointers to docs/FOND-HTN.md + docs/BENCHMARKS.md; "Current limits" list states the three deviation-ticket limits (translate 10 s internal cap w/ 4 IPC-2023 instances; conditional-effect grounding `nested 'when'`/`unbound variable '?x'` on 2 PANDA pairs; parse depth 1000-deep abort, pinned `#[ignore]`d `deep_nesting_1000_and_does_not_overflow_or_hang`), each citing its RESULTS file + ticket path. Diff +109 on README.md only; version numbers/badges/other sections untouched | gates |
| 2026-09-18T00:57:30Z | ALIVE | docs/readme-capability @ d1fbfb5 (wt-d37; local only, never pushed) | Gate A: `cargo test -p ferroplan --doc` exit 0 (1 passed, 0 failed). Gate B (script-verify): extracted all 29 test-like identifiers cited in the new section, grep-verified each as `fn <name>` or `ipc_test!(<name>,` under crates/ — 29/29 OK, exit 0. Extra boundary Gate C: `cargo test -p ferroplan --test fond_canonical --test hddl_adversarial` exit 0 (10 passed; 25 passed + 2 ignored — the ignored pair is exactly the documented parse-depth defect case + the /tmp external corpus). Cited file paths verified on-branch: htn-ipc2023/RESULTS.md, htn-oracle/RESULTS.md, fond-flat/oracle-goldens.json, docs/FOND-HTN.md, tickets 22-24. Honest note: `docs/BENCHMARKS.md` pointer cites ticket fond-htn-36's deliverable (wt-d36) — absent on this branch until the coordinator's serial post-wave integration lands both; all other citations resolve on this branch now. [1302]: none encountered | none — ready for coordinator integration (serial, post-wave; merge order must keep the docs/BENCHMARKS.md pointer either with ticket 36's file or strike it) |
