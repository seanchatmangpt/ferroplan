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
