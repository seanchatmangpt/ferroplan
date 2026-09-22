---
id: fond-htn-56-fixtures-dedupe
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: fixtures footprint audit — byte-identical IPC copies deduped to one canonical set"
standing: CLOSED
branch: chore/fixtures-dedupe
worktree: ~/ferroplan-worktrees/wt-d56
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. Three suites vendored overlapping IPC-2023 files independently (wave-3 `tests/fixtures/htn-oracle/`, `tests/fixtures/htn-ipc2023/`; wave-4's `ipc-sweep/` lands via ticket 41). Duplication risks silent divergence (a "fixed" copy in one suite, stale in another).

Scope:
1. Hash every fixture file under `crates/ferroplan/tests/fixtures/` (and ferroplan-hddl's fixtures); build the duplicate map (byte-identical, and near-identical same-basename pairs listed separately).
2. Canonicalize: keep ONE canonical copy per byte-identical IPC domain set — `tests/fixtures/ipc/` (new canonical home; move the oldest copy there), repoint the consuming tests via their existing path constants/loaders (minimal diffs; no symlink — cross-platform). Near-identical pairs: leave, but list them with their diff summary (they are deliberate per-suite variants until proven otherwise).
3. Footprint table committed in the ticket History: before/after file count + bytes for the deduped set only.
4. Gates prove no suite broke: every consuming test target green (scoped to those you touched).

CAUTION: the wave-4 `ipc-sweep/` fixtures do NOT exist at your base — do not create or reference them; the integrator will route ticket 41's copies to the canonical home afterward (note this in your History row).

Gates: `cargo test -p ferroplan --test htn_oracle --test htn_ipc2023 && cargo test -p ferroplan-hddl` exit 0; duplicate map shows zero byte-identical IPC duplicates remaining under fixtures/ (script-checkable one-liner in History).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | chore/fixtures-dedupe @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | chore/fixtures-dedupe @ 75870de (wt-d56) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
| 2026-09-22T00:00:00Z | CLOSED | fer-07-respawns (merged to release/v26.9.22) | RETIRED, recorded as topology not silent pruning: the work was never started twice (wave-5 cut mid-flight, wave-6 respawn produced zero commits — worktree sat at 75870de); the v26.9.22 lane retires it rather than re-respawning a third time. Scope above stays the record of what a future cycle would owe. Worktree wt-d56 pruned (branch had 0 commits outside main). | none (re-open as a fresh ticket if the capability is still wanted) |
