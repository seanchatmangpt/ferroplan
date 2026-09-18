---
id: fond-htn-39-docs-readiness-refresh
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs+test: readiness evidence refresh for fp.core.fond / fp.core.hddl"
standing: BLOCKED
branch: docs/readiness-refresh
worktree: ~/ferroplan-worktrees/wt-d39
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. The manifest gained `fp.core.fond` (4 evidence ids) and `fp.core.hddl` (8) in wave 3 — the wave-4 suites add stronger evidence; refresh and harden the admission story.

Scope:
1. Audit `readiness.rs` `capability_manifest()` + the canonical test: do the current evidence ids still name REAL tests (rename-sensitive)? Update any that drifted; add ids for the new always-on guards (`fond_property_FOUND_BUG_1_*`, `fond_canonical`, `fond_htn_micro`, `htn_oracle` agreement) keeping the non-fictional rule: every id must be an existing test name (script-verify).
2. `evaluate_readiness`: check its guidance text for Fond/Hddl mentions; refresh to point at the current entry points (`solve_hddl`, wasm ops) if stale.
3. `docs/FORTUNE5-CAPABILITY-ADMISSION.md`: add the two capabilities to its surface list with one-line evidence pointers (wave-3 audit found it listed HDDL as a surface while the manifest didn't — close that inversion fully).
4. Do not change admission semantics (bounds/candidate-only rules) — documentation + evidence-mapping only.

Gates: `cargo test -p ferroplan --lib readiness` exit 0; evidence-id existence script exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | docs/readiness-refresh @ 90c2ae2 | — | all |
| 2026-09-18T00:47:20Z | PARTIAL_ALIVE | docs/readiness-refresh @ wt-d39 (base 90c2ae2) | oriented: readiness.rs + manifest read; evidence-id audit starting | audit ids, add new guards, guidance check, admission doc, script gate, readiness test gate |
| 2026-09-18T00:57:18Z | PARTIAL_ALIVE | docs/readiness-refresh @ wt-d39 (uncommitted) | audit: all 12 wave-3 ids alive (no rename drift); +21 new ids (fond_canonical ×10 + fond_property FOUND_BUG_1 guard → fp.core.fond; fond_htn_micro ×9 + htn_oracle golden agreement → fp.core.hddl); `python3 scripts/verify_evidence_ids.py` exit 0 (102 ids: 33 test-name-bound, 69 thematic; falsified — fictional id → exit 1 fail-closed); `cargo test -p ferroplan --lib readiness` exit 0 (4 passed); guidance sweep: evaluate_readiness emits no guidance text, nothing stale — item 2 no-op; fmt/clippy fail exit 1/101 BUT pre-existing at base 6d14813 (stash-verified; drift in planning_runtime.rs + wave-4 test files + ferroplan-hddl/translate.rs — none are my files; readiness.rs itself fmt-clean); admission doc surface list + verifier note added | commit, re-run both gates at committed SHA, final row |
| 2026-09-18T00:57:49Z | PARTIAL_ALIVE | docs/readiness-refresh @ 1576abc | both gates re-run at committed SHA 1576abcb9f0ccd36fdc697e6d3a51a223a492446: `python3 scripts/verify_evidence_ids.py` exit 0 (102 ids, 33 test-name-bound); `cargo test -p ferroplan --lib readiness` exit 0 (4 passed, 0 failed); tree clean; never pushed | scope complete. Remaining (noted, out of ticket scope): pre-existing fmt drift in wave-4 test files + planning_runtime.rs, pre-existing clippy error ferroplan-hddl/src/translate.rs:647, `.github/workflows/fortune5-admission.yml` evidence-core.txt does not yet carry fond./hddl. family ids (pre-existing since wave 3); fond_htn_oracle.rs + fond_property.rs enumeration/reference tests exist but were not named by the ticket's guard enumeration — left unmapped, recorded here |
