# ferroplan 0.11 roadmap — the guidance cycle

Scope locked 2026-07-19. 0.10 shut every non-guidance wall class down
cold (memory, grounding, semantics, scheduling, validation); what's
still standing fails for ONE reason, no matter how you classify it —
the heuristic. transport11 — h^FF is the delete-relaxation floor and
throughput is off the hook; storage11-t — 3 M nodes, live heap,
`helpful → 0`; model-train — `avg_helpful 0.0`, every pass runs itself
dry; TMS — interleaving scale with no gradient to climb; floor-tile/
visit-all/sokoban seq-sat tails — the classic h^FF-blind families. 0.11
goes after guidance head-on, plus the game-embedding think API
(shovel-ready per the design answers already on file, independent of
the guidance work, and better guidance shrinks think budgets anyway).

Baselines (0.10.0 binary): tempo-sat **387/630** at 30 s / 3 jobs, all
VAL-validated — storage11-t 0/20, TMS 0/20, match-cellar 6/20,
parc-printer-t 18/30 + 7/20, sokoban-t 10/30 + 2/20, floor-tile-t
3/20, turn-and-open 1/20 at 60 s. Seq-sat: transport11 0/20,
floor-tile11 5/20, visit-all11 8/20, sokoban 21/30 + 11/20.

## Phase 1 — the temporal LAMA rung

The one transfer with real evidence behind it: landmark counting +
preferred-operator boosting was the 0.9 breakthrough on exactly this
plateau profile (barman 0/4 → 4/4), and the decision-epoch search
carries NO fact-landmark guidance at all — its phase-1 key runs on FF
h plus only the numeric-threshold term.

- `TNode` gains a landmark-accepted bitset (the `lama.rs` shape);
  `landmarks_for` (already generalized over (start, goal) in 0.9)
  seeds the pass; the phase-1 key adds an unaccepted-landmark term.
- Preferred-operator boosting via a second open list (the node's
  `helpful` set is already computed in the prune pass) with the
  lama-style mixed deterministic batch.
- Phase-2 complete passes stay untouched — completeness is theirs, so
  the rung can only add solves, never cost them. `FF_NO_TLAMA=1`
  restores the 0.10 search bit-for-bit.
- Acceptance: measured coverage on storage11-t / TMS / match-cellar /
  parc-printer-t / sokoban-t moves, or carries a recorded diagnosis;
  sentinels (crew, pegsol, elevator, openstacks-t) hold still; every
  new solve VAL-validates; t1 ≡ t8.

## Recorded — Phase 1 (2026-07-20): MEASURED NEGATIVE, ships opt-in

Three variants, each run at the 30 s baseline. None came back positive:

1. **Key term in the pruned pass**: the landmark gradient FIGHTS h
   where the two disagree — crew-planning 50/50 → 36/50, the 0.10
   sokoban-t/floor-tile-t gains lost 6 (turn-and-open did gain 1→2/20,
   the only positive number any variant showed).
2. **Unbounded dedicated rung**: crew/floor-tile restored,
   parc-printer08-t +1 — but the failed rung burned a full node-cap
   slice and sokoban-t stayed −3 at the wall.
3. **Bounded rung (50k nodes)**: zero new coverage anywhere; the
   parc-printer +1 needed more than the cap allowed; sokoban-t still
   −3 (borderline solves this close to the 30 s wall can't spare even
   a seconds-scale failed bet).

Diagnosis, on the record: snap tasks' fact landmarks are dominated by
RUNNING-token chains that accept in path order no matter what the
search picks — the unaccepted count carries almost no branching signal
on these walls, unlike barman's classical landmarks (deep
resource-chain ordering), which is what made the 0.9 rung win. The
machinery ships opt-in (`FF_TLAMA=1`; default is 0.10 behavior
bit-for-bit) with the landmark supply counts sitting in the debug dump
(`[tsearch] tlama: N`).

## Phase 2 — helpful-action drift repair

storage/model-train show helpful sets thinning to ZERO the farther you
get from init: FF's strict filter (relaxed-plan ops at layer 0, really
applicable) runs dry, and the prune pass degrades into a full scan.
Fast Downward's laxer preferred-operator definition keeps a set alive
where FF's would already have starved.

- ONLY when the strict set is empty, fall back to: applicable ops
  whose add intersects the relaxed plan's selected facts (any layer).
  Strict-nonempty nodes stay bit-identical by construction, fencing the
  classical sentinels off from the change.
- `FF_STRICT_HELPFUL=1` restores. Measured on storage / model-train /
  turn-and-open (does the prune pass re-arm?) AND the classical
  sentinels (gripper/blocks/barman eval counts must not move — their
  strict sets are already nonempty).
- Acceptance: measured wins or a recorded negative; no sentinel drift.

## Recorded — Phase 2 (2026-07-20): MEASURED NEGATIVE, ships opt-in

The mechanism was real, just sharper than the roadmap guessed:
`relaxed_helpful` already carries a last-resort fallback, so the RAW
set is rarely empty — the drift turned out to be the temporal
`eval_node`'s Start|Classical FILTER emptying an already-nonempty set
when relaxed plans route through agenda-fired END ops (storage's
stored helpful averaged 0.0). The repair (`helpful_needed_adders`:
applicable ops adding a fact the relaxed plan still needs) re-armed
the sets (storage holds ~1.0–1.3 deep into the search) but RESTRICTS
block (a) on exactly the nodes where the empty set used to mean a full
scan — zero new solves anywhere. Ships opt-in (`FF_LAX_HELPFUL=1`);
default stays the 0.10 pruned pass.

**Measurement-conditions caveat, on the record**: today's sweeps read
sokoban-t at 8/30 + 1/20 against the scoreboard's 10/30 + 2/20 — an A/B
of the 0.10 binary against today's box proved it ENVIRONMENTAL (i3
solo: 35.5 s on the 0.10 binary today against its recorded 24.7 s
under 3-job contention on the scoreboard day; the current binary sits
within ~5% of 0.10). Wall-clock scoreboards inherit box variance; the
borderline band (solves within ~5 s of the wall) flips right along
with it. Verdicts above stand unaffected — no variant showed gains
anywhere; crew's 50/50 → 36/50 under the key-term was margin-scale,
not borderline.

## Phase 3 — one bounded swing at a richer classical h

transport11 needs a different gradient, not a faster one. The bounded,
honest version: an unaccepted-landmark-count TERM in the classical
ladder's best-first ordering (not just the separate LAMA rung),
`SearchCfg`-weighted, default off unless the numbers say otherwise.

- Measured against transport11 / floor-tile11 / visit-all11 and the
  full costs subset for regressions. House rules: measured-win-or-
  recorded-dead-end; this phase is explicitly free to land NEGATIVE
  without apology — the record is the deliverable either way.

## Recorded — Phase 3 (2026-07-20): MEASURED NEGATIVE, the record is the deliverable

`FF_CLM=3` against default at 60 s: transport08 15/30 IDENTICAL solve
sets (and transport11 0/20 on both — the landmark count adds no
gradient where h^FF already sits at the floor); visit-all 7/20
identical (EHC solves these; the term lives on the best-first
fallback and never once fires — plumbing confirmed); floor-tile WORSE
with the term (2/20 against 5/20; the builds contended with that half
of the sweep so the magnitude is suspect, but the direction isn't).
Third guidance transfer, third clean negative — the cycle's real
finding is the pattern itself: **transport/floor-tile-class walls
need a genuinely different heuristic (red-black / semantic landmarks
over numeric structure), not reweightings of what already exists.**
`FF_CLM` stays as the experiment hatch; defaults bit-identical.

## Phase 4 — the budgeted-think API (game track)

The design answers already on file (real-time, episodic
stop-and-think, plan-on-demand): a think is a BOUNDED call — eval
budget + memory target — on a long-lived `Session` (ground once,
replan many).

- `Options` grows an explicit think-budget surface (evals cap exists;
  add a node-memory target that plumbs to the existing deterministic
  caps: `node_cap_for` / `temporal_node_cap`).
- `Session::replan` honors the budget on every path (resolve ladder,
  portfolio, temporal); a capped think returns its incumbent or an
  honest budget-exhausted verdict, never wall-clock nondeterminism.
- An `examples/` episodic-replan walkthrough (think → follow → world
  drifts → rethink) as the living doc for the game embedding.
- Acceptance: a think with a tiny budget returns fast and
  deterministically at any thread count; the example runs in the
  suite (`--examples` build) and the budget knobs are documented.

## Phase 5 — 0.11.0 release mechanics

CHANGELOG `[0.11.0]`, workspace bump 0.10.0 → 0.11.0, README refresh,
`rustup update stable` first, full gate (fmt / clippy `-D warnings` /
suite), scoreboards refreshed with the release binary where phases
moved them, main fast-forwarded and publish.sh-ready (the user runs
publish.sh).

## Recorded (cycle close, 2026-07-20)

- **Phase 4 SHIPPED**: `Session::replan_budgeted(max_evaluated,
  memory_mb)` + `SearchCfg.node_bytes_target` through the per-node
  byte model; the determinism test caught EHC's op-scaled cap ignoring
  `max_eval` (a 1-eval think solved anyway) — the caller's budget now
  bounds EHC too. `examples/game_think.rs` is the episodic
  walkthrough. Suite 144/0.
- **Phase 5**: versions 0.10.0 → 0.11.0, CHANGELOG/README refreshed,
  latest stable confirmed (1.97.1), full gate green. Default-path
  behavior unchanged from 0.10.0 (every experiment hatched off), so
  the 0.10.0 scoreboards stand — no refresh needed.
- **The cycle's finding**: three principled transfers, three clean
  negatives with recorded diagnoses. The next guidance lever has to be
  a genuinely different h — red-black planning or semantic landmarks
  over numeric structure — not reweightings of existing signals.
