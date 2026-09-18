---
id: fond-htn-53-ipc2020-profile
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Bench: profile the IPC-2020 blocksworld translate blowup — name the top term"
standing: BLOCKED
branch: bench/ipc2020-profile
worktree: ~/ferroplan-worktrees/wt-d53
created: 2026-09-18T00:55:00Z
source: CHANGELOG admission (fixture f never finishes translating in 120 s)
---

Read `_WAVE4-CONTEXT.md`. Evidence ticket — profile, don't fix (the fix follows the numbers next wave).

Scope:
1. Locate the IPC-2020 blocksworld fixture in `crates/ferroplan-hddl` (fixtures dir, "fixture f" per the changelog lineage; also check `tests/fixtures/`).
2. Instrumented run: parse+ground fine (they finish); translate under a 120 s wall with counters sampled every 5 s (thread a sampling hook or use a wrapper test printing frontier size, interned states, transitions built, method expansions — whatever the translate loop exposes; if nothing is exposed, add `#[cfg(test)]`-gated counters local to your test via a copied minimal harness? NO copying — instead use the pub API + a sampling thread reading a progress callback if one exists; if none exists, run with `perf`/`sample` (macOS) and attribute by stack).
3. Deliver `crates/ferroplan/tests/fixtures/ipc2020-profile/RESULTS.md`: wall-time curve table, the dominant term named with evidence (e.g., "decomposition offer fan-out: method m expanded k× per composite state; offers/s flat, states/s declining ⇒ queue growth is the carrier"), and the top-3 ranked fix hypotheses with estimated leverage (order-of-magnitude reasoning, stated as hypotheses).
4. One `#[ignore]`d reproducer test running the profile bounded (120 s) for future comparison.

Gates: RESULTS.md committed with the curve + named term; reproducer exits 0 under `-- --ignored` (it may end in a typed wall refusal — that IS the honest outcome).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | bench/ipc2020-profile @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | bench/ipc2020-profile @ 75870de (wt-d53) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
