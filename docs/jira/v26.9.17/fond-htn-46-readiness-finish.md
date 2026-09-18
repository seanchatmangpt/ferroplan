---
id: fond-htn-46-readiness-finish
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: readiness leftovers — unmapped guards, fmt drift, evidence-core ids"
standing: BLOCKED
branch: docs/readiness-finish
worktree: ~/ferroplan-worktrees/wt-d46
created: 2026-09-18T00:55:00Z
source: fond-htn-39 remaining-notes
---

Read `_WAVE4-CONTEXT.md`. Ticket 39 closed PARTIAL_ALIVE with four named leftovers; finish them.

Scope:
1. Map the unmapped guards ticket 39 listed: `fond_htn_oracle.rs` tests + fond_property's enumeration/reference tests → add as evidence ids under `fp.core.hddl`/`fp.core.fond` (existing real test names only); update the canonical count test in the same change.
2. `fortune5-admission.yml`'s static `evidence-core.txt` heredoc: add the fond./hddl. family ids so the CI evidence registry matches the manifest (read the workflow file first; keep its shape).
3. fmt drift: `cargo fmt` ONLY on files the wave-4 branches touched (test files + planning_runtime.rs + readiness.rs) — a fmt-only commit, no semantic changes; verify `git diff` is whitespace/layout only.
4. Do not fix the pre-existing clippy error at `ferroplan-hddl/src/translate.rs:647` — record it verbatim in your History row as pre-existing (the finish-wave's translate branches may move that line).

Gates: `cargo test -p ferroplan --lib readiness` exit 0; `scripts/verify_evidence_ids.py` exit 0; `cargo fmt --check` clean on the touched set.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | docs/readiness-finish @ 6d14813 | — | all |
