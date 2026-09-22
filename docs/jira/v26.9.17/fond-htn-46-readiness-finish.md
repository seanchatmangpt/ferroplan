---
id: fond-htn-46-readiness-finish
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: readiness leftovers — unmapped guards, fmt drift, evidence-core ids"
standing: ALIVE
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
| 2026-09-18T01:12:30Z | PARTIAL_ALIVE | docs/readiness-finish @ wt-d46 (base observed 6fc8f01 = 6d14813 + ticket-cut docs commit; ticket file only exists at 6fc8f01+, so base left as-created, not moved) | oriented: ticket + both wave contexts + ticket-39 branch diff (90c2ae2..1576abc) read; ticket 39's 21 ids enumerated for disjointness: fond_canonical.* ×10, fond_property.fond_property_FOUND_BUG_1_strong_cyclic_accepts_goal_unreachable_self_loop, fond_htn_micro.* ×9, htn_oracle.solve_hddl_agrees_with_oracle_goldens_on_deterministic_htn_corpus | map fond_htn_oracle.rs + fond_property enumeration/reference ids; workflow heredoc; fmt touched set; gates || 2026-09-18T01:12:30Z | PARTIAL_ALIVE | docs/readiness-finish @ wt-d46 (base observed 6fc8f01 = 6d14813 + ticket-cut docs commit; ticket file only exists at 6fc8f01+, so base left as-created, not moved) | oriented: ticket + both wave contexts + ticket-39 branch diff (90c2ae2..1576abc) read; ticket 39's 21 ids enumerated for disjointness: fond_canonical.* x10, fond_property.fond_property_FOUND_BUG_1_strong_cyclic_accepts_goal_unreachable_self_loop, fond_htn_micro.* x9, htn_oracle.solve_hddl_agrees_with_oracle_goldens_on_deterministic_htn_corpus | map fond_htn_oracle.rs + fond_property enumeration/reference ids; workflow heredoc; fmt touched set; gates |
| 2026-09-18T01:31:00Z | ALIVE | docs/readiness-finish @ 1fe7506 (e740db0 semantic + 1fe7506 fmt-only) | gates at 1fe7506: `cargo test -p ferroplan --lib readiness` exit 0 (4 passed); `python3 scripts/verify_evidence_ids.py` exit 0 (85 ids: 16 test-name-bound, 69 thematic; falsified with fictional id -> exit 1 fail-closed, restored -> exit 0); `rustfmt --check` on touched set exit 0; compile of all 9 fmt-touched test targets exit 0; heredoc-vs-manifest coverage 85/85 ids attested across CI evidence files, 0 stale. Delivered: fp.core.fond += fond_property.{fond_solver_matches_independent_policy_enumeration, reference_oracle_agrees_with_hand_computed_semantics}; fp.core.hddl += fond_htn_oracle.{in_repo_fixtures_match_recorded_ferroplan_outcomes, goldens_agreement_ledger_is_consistent} (all disjoint from ticket 39's 21); canonical count test pins fond=6/hddl=10; workflow evidence-core.txt heredoc += 12 fond./hddl. family ids + those 4 test-name ids; scripts/verify_evidence_ids.py vendored byte-identical from docs/readiness-refresh@1576abc (clean add/add at integration); fmt-only commit over 9 drifted test files + src/planning_runtime.rs (readiness.rs fmt-clean at base), proven by re-running rustfmt on pristine /tmp HEAD copies -> byte-identical (10/10). Pre-existing clippy at ferroplan-hddl/src/translate.rs:647:9 recorded verbatim, NOT fixed: `warning: this \`match\` expression can be replaced with \`?\`` / clippy::question_mark / rust-clippy 1.97.0 (`match indeg.get_mut(after) { Some(d) => *d += 1, None => return None }`) — error under CI `-D warnings`; finish-wave translate branches may move that line. Operator did not write: gates, verification, and commit mechanics manufactured; ~153 of ~215 added lines are vendored script bytes, manifest/heredoc/test edits ~60 lines hand-directed | integration union (ticket 41/45): (1) merge readiness.rs — mine appended at array ends, expect trivial overlap with 1576abc; (2) count pins must bump to fond=15/hddl=18 after union; (3) heredoc must ALSO gain ticket 39's 21 ids (fond_canonical.* x10, fond_property.FOUND_BUG_1, fond_htn_micro.* x9, htn_oracle.* x1) or the CI admit-job `overall_state == 'admitted'` assert trips by design; (4) keep ONE verify_evidence_ids.py. Deliberately unmapped: 9 `#[ignore]`d fond_htn_oracle.rs tests (6 external-corpus /tmp readers + 3 ORACLE_MISMATCH repros owned by tickets 21/22/23 + strong-vs-strong-cyclic semantics note) — not runnable CI evidence; re-map when their ignores come off |
| 2026-09-18T03:02:00Z | ALIVE | docs/readiness-finish @ 113d00e | — | coordinator-close: integrated by coordinator at 75870de lineage (merge 1a6c393); tip 113d00e = receipt commit atop 1fe7506 (final row predated it); union pinned fond=17/hddl=20 (readiness.rs count test @ 75870de) | none — closed |
