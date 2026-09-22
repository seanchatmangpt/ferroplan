---
id: fond-htn-65-translate-plumbing
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix (3rd attempt, SPLIT SCOPE): plumb caller walls into TranslateLimits only"
standing: ALIVE
branch: fix/translate-plumbing
worktree: ~/ferroplan-worktrees/wt-a23 (reused, reset to 75870de)
created: 2026-09-18T02:50:00Z
supersedes: fond-htn-23 scope (plumbing half only; profiling dropped)
---

Read `_WAVE4-CONTEXT.md` + ticket 23's History (two prior agents died; the scope was too wide — this ticket is deliberately SMALL).

Scope (nothing else):
1. `crates/ferroplan/src/hddl.rs`: derive `TranslateLimits` from `PlannerLimits` in `solve_hddl_inner` — wall from `max_wall_ms` (0 → keep default 10 s? NO: 0 means unbounded per PlannerLimits docs — mirror ticket 43's mapping exactly: 0 → None/unbounded, else the caller's value; state-budget field from `max_states` via the same `MAX_STATES_PER_GROUND_INSTANCE`-style calibration ticket 43 used, so defaults reproduce today's envelope EXACTLY).
2. One test: default behavior byte-identical (existing suites); raised `max_wall_ms` lifts the translate wall end-to-end on a hand-authored domain that needs >10 s (or reuse a committed slow fixture; keep the test `#[ignore]`d if it needs >30 s wall).
3. Re-run the 9 wave-4 `LIMIT:translate-wall` IPC instances (read-only /tmp corpus) at 60 s caller walls; append outcomes to `tests/fixtures/ipc-sweep/RESULTS-wavec.md` (append-only addendum — file exists from ticket 43).

Explicitly OUT of scope: profiling, frontier canonicalization changes, capacity work (ticket 60 owns the leverage side).

Gates: `cargo test -p ferroplan -p ferroplan-hddl` exit 0; addendum rows added.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|

| 2026-09-18T02:50:00Z | BLOCKED | fix/translate-plumbing @ 75870de | — | all |
| 2026-09-18T03:05:00Z | PARTIAL_ALIVE | fix/translate-plumbing @ 75870de (worktree wt-a23, clean) | orient: ticket 65 + ticket 23 History + both wave contexts read; seam confirmed (`solve_hddl_inner` passes `TranslateLimits::default()`, hddl.rs ~L284); ticket-43 precedent in tree (`grounding_limits_from` @ L243, calibration-constant shape); 9 wave-4 translate-wall rows identified (pcp_1/16/17/2, po_colouring, po_rover, po_transport, satellite_gtohp, transport — `fixtures/ipc-sweep/RESULTS.md`); committed sweep fixtures PRESENT on this base (ticket 41 landed), so the re-run reads ticket-27's exact committed pairings (same competition bytes as the /tmp corpus) instead of ticket-43's /tmp substitution; `TranslateLimits`: depth 64 / wall Some(10s) / states Some(200k) → calibration ratio 2 off `max_states=100_000` reproduces the default envelope exactly | plumb `translate_limits_from`; calibrate slow fixture (measure, not guess); one test; re-run 9; addendum; gates; commit |
| 2026-09-18T04:10:00Z | ALIVE | fix/translate-plumbing @ 9bbe2c3 (worktree wt-a23) | (1) plumbing landed: `translate_limits_from` (hddl.rs, mirrors `grounding_limits_from`: wall `max_wall_ms` verbatim 0→None, states `max_states/2` — default 100k→200k reproduces the envelope exactly, depth left at 64) wired into `solve_hddl_inner`; 3 stale "pending ticket-23" doc comments updated; (2) fixture calibration MEASURED not guessed: naive toggle-domain probe was solver-bound (translate 0.94s vs solve 13.5s at 16k states) — replaced with method-choice chain stamping 3000-literal effect blocks (K=8/B=3000, ticket-24 construct), decoupling translate's intern-clone cost from solver diameter; measured translate ALONE 14-16s (default wall fires at ~10.1s: 465 states/464 transitions/233 queued), solve 56ms; (3) one test `translate_plumbing.rs` green end-to-end (solved at raised wall in ~15s; debug-only wall>10s assert proves the lift; cfg!(debug_assertions) guard per ground_wall.rs convention); clippy clean on new files, rustfmt clean on new files (pre-existing drift in untouched files left alone) | gates on final tree; addendum run+rows; commit 2 |
| 2026-09-18T04:15:00Z | ALIVE | fix/translate-plumbing @ 6757249 (worktree wt-a23) | (3) re-run exit 0 (540s): 9/9 honest rows — 0/9 solved, 0/9 still at the old 10s wall; ALL 9 now `LIMIT:pipeline-watchdog` (cumulative 60s = same `max_wall_ms` declaration, structurally fires before translate's own 60s wall; documented as a caller-declaration choice); orphaned translate workers interned 116,317-405,692 states (30,473-243,691 queued) at death — past the old 200k ceiling, so the raised 5M ceiling is what let translate run the full minute; capacity (not the wall) binds these 9, consistent with ticket-23's 110s re-measure; addendum 2 appended to RESULTS-wavec.md (append-only; ticket-43 rows untouched); corpus note: committed fixtures verified byte-identical (diff -q, 18/18 files) with /tmp/fond-review/HDDL-Parser/tests/ipc/ — no substitution needed | FINAL GATES |
| 2026-09-18T04:25:00Z | ALIVE | fix/translate-plumbing @ 6757249 (worktree wt-a23, clean; commits 9bbe2c3 plumbing+test, 6757249 addendum runner+RESULTS) | FINAL GATES exactly as ticketed: `cargo test -p ferroplan -p ferroplan-hddl` exit 0 (761 passed / 0 failed / 45 ignored across 83 suites), including the default-behavior pins: htn_ipc2023 13/13 (four verbatim TIMEOUT@10s LIMIT:translate-wall assertions), ipc_sweep sampled heartbeat 1/1, ground_caps_plumbing 3/3 (ticket-43 grounding side unchanged), fond_htn_micro 9/9; new translate_plumbing 1/1 (14.55s); both long-run runners correctly #[ignore]d. 比: 100% manufactured on 産面 this session (2 source-file edits + 2 new test files, zero hand-written bypasses; operator wrote nothing). No [1302] incidents. Ticket-file History rows appended in main checkout per day-dir convention (not committed on branch — coordinator unions) | none on ticket |
| 2026-09-18T23:30:00Z | ALIVE | merged 6757249 into wave6/land-v26917 (merge f980212, A7) | gates re-run on the FULL merged line: `cargo test -p ferroplan -p ferroplan-hddl` 85 suites, 776 passed / 0 failed / 47 documented ignores, exit 0 (includes translate_plumbing 1/1 + translate_wall_ipc_addendum runner green, ~9 min) | none (A7) |