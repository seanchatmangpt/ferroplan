---
id: fond-htn-58-drop-retry-redecompose
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: micro-drop-retry oracle mismatch — ferroplan NoPlan where koala re-decomposes after a dead branch"
standing: BLOCKED
branch: fix/drop-retry-redecompose
worktree: ~/ferroplan-worktrees/wt-h58
created: 2026-09-18T02:50:00Z
source: fond-htn-41 heal-check (mismatch survived the oneof merge)
---

Read `_WAVE4-CONTEXT.md`. The `ORACLE_MISMATCH_micro_drop_retry` pin (crates/ferroplan/tests/fond_htn_oracle.rs, #[ignore]d): koala flexible SOLVES the micro Transport-pattern drop-retry (oneof success / EMPTY branch); ferroplan says NoPlan — and the mismatch SURVIVED the oneof-semantics merge, so it is a live translator-semantics divergence, not grammar. Hypothesis (from fond-htn-12's findings): koala re-offers method decomposition after a no-change outcome because its frontier keeps the abstract task pending; ferroplan's rank-canonicalized frontier may consume the decomposition offer once and dead-end on the empty-branch self-loop.

Scope:
1. Reproduce with the committed fixture; trace where the reachable region dies (which state loses its witness) — instrument locally, do not commit instrumentation.
2. Fix the translator/solver seam so the empty-branch outcome re-offers decomposition (the composite state after a no-change outcome must still carry the pending abstract task — frontier canonicalization must not collapse it into a state whose task network is already spent).
3. Un-ignore the mismatch pin → always-on agreement test; update oracle-goldens.json agreement ledger; if koala's own artifact is needed for contrast, cite the /tmp path in the golden note (no koala files in-repo).
4. Regression: the fond_htn_micro drop-retry test + fond_flat_oracle tireworld (cyclic-only, empty-branch-adjacent) must stay green.

Gates: `cargo test -p ferroplan --test fond_htn_oracle --test fond_htn_micro --test fond_flat_oracle` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:50:00Z | BLOCKED | fix/drop-retry-redecompose @ 75870de | — | all |
