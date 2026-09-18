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

---

# RESULTS-wavec addendum 2 — raised-caller-WALLS re-run of the 9 translate-wall instances (append-only)

Ticket `fond-htn-65-translate-plumbing` (wave `v26.9.17-fond-htn-hardening`). **Append-only
addendum to the addendum above** (ticket fond-htn-43's rows are untouched): one re-run of the 9
instances the wave-4 sweep classified `LIMIT:translate-wall` (pcp_1, pcp_16, pcp_17, pcp_2,
po_colouring, po_rover, po_transport, satellite_gtohp, transport), under the raised caller walls
the ticket-65 translate plumbing makes expressible through the public `solve_hddl` API.

## Corpus (no substitution needed)

Run from the **committed** sweep fixtures (`tests/fixtures/ipc-sweep/`, ticket fond-htn-27) —
which HAVE landed on this branch's base (`fix/translate-plumbing` @ `75870de`), unlike ticket
fond-htn-43's base, which needed the `/tmp` substitution recorded above. All 18 files (9 domains
+ 9 first-listed problems) were verified byte-identical (`diff -q`) with the external read-only
corpus `/tmp/fond-review/HDDL-Parser/tests/ipc/` (HDDL-Parser checkout `1f2977e`, IPC-2023
competition data), so these rows are instance-for-instance comparable with BOTH the wave-4 rows
in `RESULTS.md` and the ticket-43 addendum rows above. Runner committed as
`crates/ferroplan/tests/translate_wall_ipc_addendum.rs` (`#[ignore]`d long-run).

## Configuration under test

- `PlannerLimits { max_wall_ms: 60_000, max_states: 10_000_000, ..default }` through the public
  `solve_hddl` — the SAME one-line declaration ticket fond-htn-43's addendum used, which its rows
  could not lift translate with and this ticket's plumbing can: `translate_limits_from` derives
  translate's internal wall = 60 s and composite-state ceiling = `10_000_000 / 2 = 5_000_000`
  (calibration ratio 2; default `max_states = 100_000` reproduces the historical 200 000 exactly).
  Ground caps 1 000 000 / 1 000 000, ground wall 60 s, solver wall 60 s — none binding here.
- Structural note, verified by the rows below: `solve_hddl`'s cumulative watchdog and translate's
  internal wall are derived from the SAME `max_wall_ms`, so for any instance whose translate phase
  alone needs >= 60 s the watchdog (started earlier) always fires first — a caller wanting
  translate's 60 s wall to be the binding cap must declare `max_wall_ms` above translate's
  expected need. That is a caller-side declaration choice, by design ("callers that want a bound
  must say so, in one place").
- Debug build (`cargo test` default), same as the wave-4 sweep and the addendum above, for
  comparability.

- machine: Apple M3 Max (`sysctl -n machdep.cpu.brand_string`)
- date: 2026-09-18T03:59Z (UTC); full run wall 540 s
- command: `cargo test -p ferroplan --test translate_wall_ipc_addendum -- --ignored --nocapture` (exit 0)
- row shape: `domain | wave-4 outcome (translate wall 10 s) | raised-caps outcome (translate wall 60 s) | total ms | verbatim detail` — walls are cumulative `solve_hddl` wall (parse+ground+translate+solve), watchdog-capped.

## Re-run rows (9/9, honest typed outcomes, 0 panic)

| domain | wave-4 outcome | raised-caps outcome (60 s caller walls) | total ms | verbatim detail |
|---|---|---|---|---|
| pcp_1 | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60002 | `Timeout { elapsed_ms: 60000, limit_ms: 60000 }` |
| pcp_16 | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60004 | `Timeout { elapsed_ms: 60004, limit_ms: 60000 }` |
| pcp_17 | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60000 | `Timeout { elapsed_ms: 60000, limit_ms: 60000 }` |
| pcp_2 | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60009 | `Timeout { elapsed_ms: 60005, limit_ms: 60000 }` |
| po_colouring | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60005 | `Timeout { elapsed_ms: 60005, limit_ms: 60000 }` |
| po_rover | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60005 | `Timeout { elapsed_ms: 60005, limit_ms: 60000 }` |
| po_transport | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60003 | `Timeout { elapsed_ms: 60003, limit_ms: 60000 }` |
| satellite_gtohp | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60005 | `Timeout { elapsed_ms: 60005, limit_ms: 60000 }` |
| transport | LIMIT:translate-wall | **LIMIT:pipeline-watchdog** | 60000 | `Timeout { elapsed_ms: 60000, limit_ms: 60000 }` |

## Translate progress at the wall (orphaned-worker diagnostics, honest attribution)

`translate`'s diagnostic `eprintln` fires from the ORPHANED worker thread slightly after the
watchdog has already returned to the caller, and stderr interleaves across the sequential
instances — so per-instance attribution of the diagnostics below is NOT claimable. Aggregate over
the run's 9 diagnostics (one per worker, all at translate's 60 s internal wall): **116,317 to
405,692 states interned, 256,274 to 627,291 transitions built, 30,473 to 243,691 states still
queued** at the moment each worker died. Even the smallest far exceeds the 200 000 composite-state
ceiling the old `TranslateLimits::default()` enforced — i.e. the raised 5 M ceiling is what let
translate run the full minute at all.

## Honest coverage statement

- **0/9 solved** at the raised configuration; **0/9 still `LIMIT:translate-wall`** at the old
  10 s wall — all 9 now progress through translate for the full minute and die on the cumulative
  `solve_hddl` watchdog (`LIMIT:pipeline-watchdog`, limit 60000 ms), which is the structural
  consequence of declaring one wall for the whole pipeline (see the configuration note above).
- The plumbing is therefore verified end-to-end on real IPC instances (translate's internal wall
  now follows the caller's declaration; defaults unchanged — the wave-4 rows above and
  `RESULTS.md` still hold), but capacity, not the wall, is these 9 instances' binding constraint:
  composite-state growth is roughly linear in wall at this frontier (116k-406k interned in 60 s,
  30k-244k queued), consistent with ticket fond-htn-23's 110 s manual re-measure (524k-813k
  interned at 110 s, all 4 of its instances still non-convergent). Extrapolation says no sane
  wall alone converges any of the 9; the leverage is the composite-BFS frontier canonicalization
  (ticket fond-htn-23's dropped profiling half — out of THIS ticket's scope by design).
- No instance panicked, no garbage `Ok(solved=false)`, no hang past the 60 s cumulative watchdog
  — the runner exits 0 iff all 9 outcomes are honest typed answers.
