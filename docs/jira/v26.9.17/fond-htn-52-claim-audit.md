---
id: fond-htn-52-claim-audit
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: claim-audit — every README/docs test-name citation verified, rename-tripped"
standing: BLOCKED
branch: docs/claim-audit
worktree: ~/ferroplan-worktrees/wt-d52
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. Generalize ticket 37/46's one-off verifications into a permanent gate.

Scope:
1. `scripts/verify_doc_claims.py` (executable, fail-closed): scan README.md, docs/FOND-HTN.md, docs/BENCHMARKS.md, CHANGELOG.md (Unreleased section only) for (a) backticked identifiers that look like test names (regex `fn NAME` or existing patterns — conservative list), verifying each exists as a test fn in `crates/**`; (b) repo-relative paths in backticks/links, verifying existence; (c) `tests/fixtures/**` citations verifying file presence. Exit 1 listing each broken claim with file:line.
2. A `#[test]` home for it (same skip-if-absent convention as ticket 51's tripwire).
3. Fix any base-tree broken claims you find (they are docs bugs); record counts (claims checked / broken / fixed).
4. Keep the conservative regex — a false-positive-prone gate gets deleted; better 10 unchecked claims than 1 false failure. Document that choice in the script header.

Gates: `python3 scripts/verify_doc_claims.py` exit 0; your test exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | docs/claim-audit @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | docs/claim-audit @ 75870de (wt-d52) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
