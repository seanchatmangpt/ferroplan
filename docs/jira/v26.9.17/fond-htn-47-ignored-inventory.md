---
id: fond-htn-47-ignored-inventory
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: ignored-test inventory + stale-ignore sweep"
standing: CLOSED
branch: test/ignored-inventory
worktree: ~/ferroplan-worktrees/wt-d47
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`.

Scope:
1. Inventory every `#[ignore]`/`#[ignore = "..."]` across `crates/**/tests/**` and `#[cfg(test)]` modules (script it): name, file:line, reason string, category ∈ {external-corpus (/tmp dependency), known-defect, long-run, flaky-guard, other}.
2. `docs/IGNORED-TESTS.md`: the table + the policy lines: external-corpus cases must skip-with-note when paths absent (verify each actually does — read the code); known-defect cases must name their owning ticket; long-run cases must carry their `-- --ignored` command.
3. Stale sweep: run each known-defect + long-run ignore explicitly (`-- --ignored` scoped); any that now PASS and have no open ticket → un-ignore (list them); any that fail with a DIFFERENT error than their reason states → History finding row.
4. Commit the inventory + any un-ignores (un-ignores in a separate commit with the reason in the message).

Gates: `cargo test -p ferroplan -p ferroplan-hddl` exit 0 after your changes (ignores that must stay ignored still ignored); inventory committed.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | test/ignored-inventory @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | test/ignored-inventory @ 75870de (wt-d47) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
| 2026-09-22T00:00:00Z | CLOSED | fer-07-respawns (merged to release/v26.9.22) | RETIRED, recorded as topology not silent pruning: the work was never started twice (wave-5 cut mid-flight, wave-6 respawn produced zero commits — worktree sat at 75870de); the v26.9.22 lane retires it rather than re-respawning a third time. Scope above stays the record of what a future cycle would owe. Worktree wt-d47 pruned (branch had 0 commits outside main). | none (re-open as a fresh ticket if the capability is still wanted) |
