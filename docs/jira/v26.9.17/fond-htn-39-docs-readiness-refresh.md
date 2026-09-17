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
