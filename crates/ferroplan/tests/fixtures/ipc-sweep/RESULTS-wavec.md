# RESULTS-wavec — raised-caps re-run of the 17 ground-cap-refused domains (append-only addendum)

Ticket `fond-htn-43-ground-caps-plumbing` (wave `v26.9.17-fond-htn-hardening`). **Append-only
addendum to `RESULTS.md`** (ticket fond-htn-27's wave-4 sweep): the wave-4 rows above/below are
NOT rewritten; this file records one re-run of the 17 domains that sweep classified
`LIMIT:ground-actions` (14) / `LIMIT:ground-methods` (3), under the raised grounding caps the
ticket-43 plumbing makes expressible through the public `solve_hddl` API.

## Instance substitution (recorded per coordinator order)

The committed sweep fixtures (`tests/fixtures/ipc-sweep/`) land via ticket fond-htn-41 and were
NOT present on this branch's base (`fix/ground-caps-plumbing` @ `6d14813`). Per the coordinator's
substitution order, the 17 measurements below read the **external, read-only** corpus
`/tmp/fond-review/HDDL-Parser/tests/ipc/` (IPC-2023 hierarchical-track competition data at
HDDL-Parser checkout `1f2977eb512f46c69a82fac7a8e9bdcc112c41be`; run from `/tmp` per the KOALA
POLICY, never vendored). The (domain, first-listed problem) pairings are identical to ticket
fond-htn-27's committed runner (`crates/ferroplan/tests/ipc_sweep.rs` on
`bench/ipc-full-sweep`), so rows are instance-for-instance comparable with the wave-4 sweep.
The runner is committed as
`crates/ferroplan/tests/ground_caps_ipc_addendum.rs` (`#[ignore]`d long-run; panics honestly if
the external corpus is absent).

## Configuration under test

- `PlannerLimits { max_wall_ms: 60_000, max_states: 10_000_000, ..default }` through the public
  `solve_hddl` — via the ticket-43 plumbing (`grounding_limits_from`): ground caps
  `10_000_000 / 10 = 1_000_000` ground actions AND methods, internal grounding wall 60 s,
  solver wall 60 s, cumulative `solve_hddl` watchdog 60 s. One caller-side declaration; no
  hard-coded internals touched.
- The translate stage's internal `TranslateLimits::default()` (10 s wall / 200 000 states /
  depth 64) is deliberately **not** lifted — that is ticket fond-htn-23's seam; where it binds
  the verbatim refusal is recorded.
- Debug build (`cargo test` default), same as the wave-4 sweep, for comparability.

- machine: Apple M3 Max (`sysctl -n machdep.cpu.brand_string`)
- date: 2026-09-18T01:44Z (UTC); full run wall 238 s
- command: `cargo test -p ferroplan --test ground_caps_ipc_addendum -- --ignored --nocapture` (exit 0)
- row shape: `domain | outcome | total wall ms | verbatim detail` — walls are cumulative
  `solve_hddl` wall (parse+ground+translate+solve), not per-stage.

## Re-run rows (17/17, honest typed outcomes, 0 panic)

| domain | wave-4 outcome (caps 10k) | raised-caps outcome (caps 1M) | total ms | verbatim detail |
|---|---|---|---|---|
| freecell_learned_ecai_16 | LIMIT:ground-actions | LIMIT:ground-actions | 19221 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| hiking | LIMIT:ground-methods | **LIMIT:translate-wall** (progressed past grounding) | 12809 | `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms` — translate's own eprintln: `translate: wall-clock limit hit -- 48689 states interned, 53347 transitions built, 7550 states still queued` |
| minecraft_player | LIMIT:ground-methods | LIMIT:ground-methods | 26921 | `grounding limit exceeded: max_ground_methods (1000000) exceeded` |
| minecraft_regular | LIMIT:ground-methods | LIMIT:ground-methods | 29213 | `grounding limit exceeded: max_ground_methods (1000000) exceeded` |
| monroe_fo_1 | LIMIT:ground-actions | LIMIT:ground-actions | 9697 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| monroe_fo_19 | LIMIT:ground-actions | LIMIT:ground-actions | 10104 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| monroe_fo_2 | LIMIT:ground-actions | LIMIT:ground-actions | 14901 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| monroe_fo_20 | LIMIT:ground-actions | LIMIT:ground-actions | 10422 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| monroe_po_1 | LIMIT:ground-actions | LIMIT:ground-actions | 9509 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| monroe_po_19 | LIMIT:ground-actions | LIMIT:ground-actions | 8755 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| monroe_po_2 | LIMIT:ground-actions | LIMIT:ground-actions | 8943 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| monroe_po_20 | LIMIT:ground-actions | LIMIT:ground-actions | 10335 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| po_monroe_po_1 | LIMIT:ground-actions | LIMIT:ground-actions | 12033 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| po_monroe_po_2 | LIMIT:ground-actions | LIMIT:ground-actions | 14037 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| po_monroe_po_24 | LIMIT:ground-actions | LIMIT:ground-actions | 8504 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| po_monroe_po_25 | LIMIT:ground-actions | LIMIT:ground-actions | 8128 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |
| snake | LIMIT:ground-actions | LIMIT:ground-actions | 24555 | `grounding limit exceeded: max_ground_actions (1000000) exceeded` |

## Honest coverage statement

- **0/17 solved end-to-end** at the raised configuration.
- **1/17 changed refusal class**: `hiking` (ground-methods 10k-refused in wave 4) grounds fully
  under the 1M method cap and now dies on the **translate** 10 s internal wall with 48 689
  composite states interned (200 000 cap) and 7 550 still queued — i.e. hiking's bottleneck
  moved from the grounding seam (ticket 43, fixed) to the translate seam (ticket 23's scope).
- **16/17 refuse in the same class, at the raised bound**: every refusal above fired at
  `max_ground_actions`/`max_ground_methods (1000000)` — each wall is well under the 60 s
  grounding wall (8.1–29.2 s), so the caps bind, not the clock: the **true ground-instance
  counts of these 16 domains exceed 1 000 000** (up from "> 10 000" known after wave 4).
- Consequence for follow-ups: lifting these 16 by caps alone requires bounds two orders of
  magnitude higher (≥ 100M instances) whose grounding wall would then bind first at any
  caller-sane wall; the leverage is in `prune_unreachable` reachability pre-pass
  (`GroundingLimits::prune_unreachable`, currently `false` on this path — a semantics choice,
  out of ticket-43 scope) and/or problem-instance decomposition, not in ever-higher caps.
  `hiking` is the one domain ticket 23's translate plumbing would immediately help.
- No domain panicked, no garbage `Ok(solved=false)`, no hang past the 60 s cumulative watchdog
  — the runner exits 0 iff all 17 outcomes are honest typed answers.
