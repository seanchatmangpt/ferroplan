---
id: fond-htn-43-ground-caps-plumbing
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: expose grounding caps through solve_hddl; re-run the 17 cap-refused IPC domains"
standing: BLOCKED
branch: fix/ground-caps-plumbing
worktree: ~/ferroplan-worktrees/wt-a43
created: 2026-09-18T00:55:00Z
source: fond-htn-27 sweep (ground-actions ×14, ground-methods ×3)
---

Read `_WAVE4-CONTEXT.md`. Ticket 23's plumbing pattern (translate side) is the precedent; do the grounding side. NOTE: parallel branches (tickets 23/24) also touch hddl.rs plumbing — confine your diff to the grounding-limit plumbing line(s) and the new test; expect the coordinator to union.

Scope:
1. Plumb caller limits into grounding: `solve_hddl` (and `solve_hddl_from_eve` if it shares the inner path) should derive `GroundingLimits` from `PlannerLimits` (wall from `max_wall_ms`; action/method caps scaled from `max_states` or new optional fields — choose the least-API-noise option and document it).
2. Re-run the 17 cap-refused IPC-2023 domains from `tests/fixtures/ipc-sweep/` with the raised caps (ground caps high, walls 60 s) — record which now solve/translate/refuse differently in `tests/fixtures/ipc-sweep/RESULTS-wavec.md` (append-only addendum; do not rewrite wave-4 rows).
3. Tests: a domain whose default ground cap refuses but solves under raised caps via the public API (hand-authored, small); default behavior unchanged (existing suites green).

Gates: `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test htn_ipc2023` exit 0; addendum committed.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | fix/ground-caps-plumbing @ 6d14813 | — | all |
| 2026-09-18T01:03:30Z | ALIVE | fix/ground-caps-plumbing @ 6fc8f01 (worktree wt-a43) | orient: ticket + both wave contexts read; seam found (`solve_hddl_inner` passes `Default::default()` to `ground`, line 212); ticket-23 precedent read (its in-flight pattern: default-calibrated field, `TranslateLimits` untouched); 17 cap-refused domains taken from ticket-27 committed sweep RESULTS (ground-actions ×14, ground-methods ×3) | design derivation; plumb; test; re-run; gates |
| 2026-09-18T01:20:00Z | ALIVE | fix/ground-caps-plumbing @ 6fc8f01 (worktree wt-a43) | derivation landed (least-API-noise: zero new fields; caps = max_states/10 calibrated so `PlannerLimits::default()` reproduces the historical 10k/10k/10s envelope exactly; wall from `max_wall_ms`, 0→None; `solve_hddl_from_eve` shares the inner path by construction); new `ground_caps_plumbing` tests 3/3 exit 0 (default refuses >10k verbatim `max_ground_actions`; sub-cap domain still solves at defaults; raised `max_states` solves via public API); 1ms-budget micro test widened to tolerate grounding's own wall-clock refusal (by-design consequence, same class ticket 23's translate plumbing will introduce) | addendum re-run; gates; commit |
| 2026-09-18T01:48:00Z | ALIVE | fix/ground-caps-plumbing @ 6fc8f01 (worktree wt-a43) | addendum re-run exit 0 (238 s): SUBSTITUTION per coordinator order — committed `tests/fixtures/ipc-sweep/` fixtures absent at this base (land via ticket 41), measurements read the external read-only `/tmp/fond-review/HDDL-Parser/tests/ipc/` corpus (IPC competition data @ HDDL-Parser `1f2977e`, run from /tmp per KOALA POLICY, never vendored; pairings identical to ticket-27's committed runner). Outcomes: 0/17 solved; hiking changed refusal class ground-methods→translate-wall (grounds fully at 1M methods, dies on translate's un-plumbed internal 10 s wall, 48689 states interned/7550 queued — ticket 23's seam); 16/17 still cap-bound, now known to exceed 1,000,000 ground instances (cap refusals at 8.1–29.2 s, far inside the 60 s ground wall); `RESULTS-wavec.md` addendum written (append-only, wave-4 rows untouched), runner committed `#[ignore]`d | gates on final tree; commits |
| 2026-09-18T02:05:00Z | ALIVE | fix/ground-caps-plumbing @ 5d219b5 (worktree wt-a43) | FINAL GATES on final tree, exactly as ticketed: `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test htn_ipc2023` exit 0 (138+9 doc-tests, 13/13 ipc2023); additional evidence: full `cargo test -p ferroplan -p ferroplan-hddl` exit 0 (0 failures incl. fond_htn_micro 9/9, htn_oracle, fond_htn_oracle, hddl_adversarial 25/25); clippy clean on new files, rustfmt clean on new files (pre-existing fmt drift in untouched files left alone per confinement) | ticket commit |
| 2026-09-18T02:08:03Z | ALIVE | fix/ground-caps-plumbing @ 5cb9041 (worktree wt-a43; ticket commit follows this row) | commits: 5d219b5 (plumbing+tests), 5cb9041 (addendum runner+RESULTS-wavec.md) | none on ticket; note for ticket 41 integrator: `tests/fixtures/ipc-sweep/` now exists with RESULTS-wavec.md only — merge ticket 27's fixtures+RESULTS.md alongside it; note for ticket 23: hiking + the 9 wave-4 translate-wall rows are your seam's evidence |
