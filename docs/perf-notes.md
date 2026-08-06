# Performance notes

The hot paths got cut open under load. This is the field log: what was found,
what got patched, what's still bleeding cycles. Trust `cargo bench -p ferroplan
--bench planning` (criterion — the only instrument that doesn't lie on a loaded
machine) and the deterministic `benchmarks/perf.py` (evaluated-states; a
*constant-factor* win leaves these bit-identical, a *search-strategy* win moves
them and demands a re-baseline).

## Methodology caveats (scars from the last run)

- **Wall-clock lies** below ~15% deltas — the same binary clocked 11.5–14s under
  background load, same code, different shadows. Trust criterion, or the `min`
  across many runs, nothing else.
- **`atos` symbolication misdirects on optimized builds** — inlined hot code
  gets pinned on the wrong symbol. A profile flagging "22% `core::fmt::Display`"
  / "12% clap" inside the *search* was a ghost: no `format!`/`Display` lives in
  `search.rs` or `heuristic.rs`. Read the de-noised shape — heuristic,
  successor-gen — not the raw top symbols staring back at you.

## Wins on the board

| fix | instance | before | after | guarantee |
|---|---|---|---|---|
| **Grounding: static-precondition param-domain restriction** (`ground.rs`) | gripper p02 | 658 µs | 247 µs (2.65×) | identical ground ops |
| | 150-ball untyped, 1-step | 1.56 s | ~0 | |
| | gripper-250 (partition) | 11.9 s | 3.96 s (3×) | |
| **EHC: op-count-scaled work cap** (`search.rs`) | gripper-250 `--mode ff` | 2.16M evals / 33 s | 32k evals / 0.86 s (38×) | plan-valid; h untouched |

Trace the wound to its source. (1) Untyped domains were enumerating the full
cartesian product of every parameter (gripper `pick`: 154³ ≈ 3.6M bindings/action)
and string-matching 99.98% of it into the trash — fixed by restricting each
param's domain by its static unary preconditions first. (2) EHC's fixed
`TOTAL_CAP=30_000` was letting large-but-easy instances fall through into the
*unpruned* best-first (2.16M evals); the cap now scales as `(200·n_ops).max(30k)`,
and EHC's near-greedy arm closes them out clean. Small/typed instances never
felt either fix — they were never bleeding.

## Ranked backlog (pulled from the ultracode analysis workflow)

Each cut here is correctness-preserving; the "preserves" column is how you check the wound closed clean.

1. **Generation-counter `Scratch::reset`** (h-identity) — swap the per-eval
   `op_layer`/`selected`/`need_fact` `.fill()`s (`2·n_ops + n_facts` writes) for a
   `gen` bump and a per-access stamp check. ~4% back on heuristic-bound instances;
   ~10 fragile gate sites waiting to bite (notably `select`'s `op_layer == 0`).
   Verify: gripper-250 holds exactly 32,123 evals, 40 tests green.
2. **Preferred-operator (helpful-action) best-first**, behind a flag (plan-valid) —
   the FF-parity fix for instances that *genuinely* stall out (deep plateaus the
   cap fix can't touch). Variant A (heap-key bonus, stays complete) is the safe
   entry. Higher ceiling; needs a flag and an evaluated-count re-baseline.
3. **`apply_into` clone-on-survival** (h-identity) — `apply` is cloning a full
   `State` per applicable op *before* the cost-bound and visited dedup throw most
   of them away; route into a reusable buffer instead, materialize only the
   survivors.
4. **Pre-size `visited` / static `op_has_relevant_neff`** (h-identity) — small,
   low-risk allocator and scan trims.

**Don't touch** an applicable-action index or a scattered `build_rpg` precondition
trigger index — already tried, already reverted. The scattered loads lose to the
sequential CSR scan's cache locality on shallow graphs. Old ground, don't dig it twice.
