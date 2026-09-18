---
id: fond-htn-27-bench-ipc-full-sweep
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Bench: full IPC-2023 43-domain sweep through solve_hddl (60 s staged walls)"
standing: BLOCKED
branch: bench/ipc-full-sweep
worktree: ~/ferroplan-worktrees/wt-b27
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Corpus: all 43 IPC-2023 domains vendored at `/tmp/fond-review/HDDL-Parser/tests/ipc/` (domain.hddl + first-listed problem each; IPC competition data — copying instances into repo fixtures is allowed, but keep the committed footprint ≤ 2 MB: prefer domain + ONE smallest problem per domain).

Scope:
1. Runner (test-shaped, `#[ignore]`d long-run + a sampled always-on subset of 6): for each of the 43 domains, run `solve_hddl` staged — parse, ground, translate, solve — with per-stage walls (10 s parse+ground, 60 s translate, 30 s solve; note ticket 23 may not have landed — where the internal 10 s translate cap binds, record it verbatim as the result).
2. `crates/ferroplan/tests/fixtures/ipc-sweep/RESULTS.md`: per domain — objects, methods, parse/ground/translate/solve wall each, outcome (SOLVED/NoPlan/LIMIT:*/GAP:*), plan length; SLOW flags; totals row. Machine note + commands.
3. Honest coverage statement: X/43 solved end-to-end, Y refused by limits (named), Z parser gaps (verbatim first error each).
4. Sampled always-on subset: 6 fastest domains as a normal test (< 30 s total) so the sweep has a CI heartbeat.

Gates: `cargo test -p ferroplan --test ipc_sweep` exit 0 (sampled); `-- --ignored` full sweep exit 0 with RESULTS.md written; no panic anywhere in the 43.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | bench/ipc-full-sweep @ 90c2ae2 | — | all |
| 2026-09-17T23:58:41Z | ALIVE | bench/ipc-full-sweep @ 6d14813 (wt-b27) | started: corpus surveyed (43 domains, 1.2MB footprint ok), runner design staged | runner+fixtures+gates |
| 2026-09-18T00:18:24Z | ALIVE | bench/ipc-full-sweep @ 6d14813 (wt-b27) | sweep run1: exit 0, 43/43 honest outcomes, 0 panic; sampled gate1 placeholder caught transport at translate 10s cap -> sampled re-derived; gates re-running on final code | commit + final rows |
| 2026-09-18T00:23:19Z | ALIVE | bench/ipc-full-sweep @ d7659ea (wt-b27) | gate1 sampled `cargo test -p ferroplan --test ipc_sweep` exit 0 (heartbeat 6 fastest SOLVED, 0.1 s); gate2 full `-- --ignored` exit 0, RESULTS.md written, 43/43 honest outcomes, 0 panic, 2 runs identical classifications | none on ticket; follow-ups: t23 translate cap binds 9 domains, ground caps bind 17, 3 ground-validation gaps (verbatim in RESULTS.md) |
