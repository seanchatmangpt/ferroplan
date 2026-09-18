---
id: fond-htn-58-drop-retry-redecompose
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: micro-drop-retry oracle mismatch — ferroplan NoPlan where koala re-decomposes after a dead branch"
standing: ALIVE
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
| 2026-09-18T02:52:00Z | PARTIAL_ALIVE | fix/drop-retry-redecompose @ 55e1997 | pin `ORACLE_MISMATCH_micro_drop_retry --ignored` rerun: ok (ferroplan live = NoPlan, matches golden); local translate dump (scratch example, not committed): empty-branch drop outcome → task-network-spent terminal s4, 0 outgoing edges → region dead | instrument done; fix |
| 2026-09-18T03:05:00Z | PARTIAL_ALIVE | fix/drop-retry-redecompose @ 4964b74 | scoped gates `cargo test -p ferroplan --test fond_htn_oracle --test fond_htn_micro --test fond_flat_oracle` exit 0 (22 passed / 8 legitimately ignored); pin tripped its own "mismatch healed" guard pre-promotion | extra-evidence sweep, ledger, close |
| 2026-09-18T03:10:59Z | ALIVE | fix/drop-retry-redecompose @ 4964b74 | all scoped gates exit 0; extra evidence: ferroplan-hddl unit 169/0 (new `deterministic_no_change_execution_still_discharges_the_task` + updated empty-branch test), fond_property 3/0 incl. FOUND_BUG_1 reproducer, fond_threshold 3/0, htn_oracle 3/0, htn_ipc2023 13/0, hddl_adversarial 27/0, hddl_fuzz_roundtrip 1/0, eve_genesis 15/0, api_panic_hunt 16/0 | none |

Fix: `translate` execution-move loop keeps the task frontier un-advanced when a
**multi-outcome** action's outcome leaves the fact set unchanged — the no-change
outcome projects onto the exact source composite state (self-loop) and the
pending task is re-offered; the strong fixpoint still refuses the self-loop and
the strong-cyclic fallback closes the fair retry. Deterministic single-outcome
no-change executions still discharge (unit tripwire pins the boundary).
Instrumentation was a scratch `examples/dump_micro.rs`, deleted before commit.
Deliverables: `crates/ferroplan-hddl/src/translate.rs` (seam fix + 2 unit tests),
`crates/ferroplan/tests/fond_htn_oracle.rs` (pin → always-on
`micro_drop_retry_agrees_with_oracle`), `crates/ferroplan/tests/fond_htn_micro.rs`
(no-change outcome self-loops on the executing state),
`crates/ferroplan/tests/fixtures/fond-htn/oracle-goldens.json` (agreement ledger
healed: SOLVED/SOLVED, oracle artifact /tmp/fond-oracle/runs/20260917T214753Z-domain-problem-27522).
| 2026-09-18T23:30:00Z | ALIVE | merged b06b907 into wave6/land-v26917 (merge 0e156c3, A7) | gates re-run in landing worktree: fond_htn_oracle 9/9 (1 documented external ignore), fond_htn_micro 9/9, fond_flat_oracle 4/4, exit 0 | none (A7) |
