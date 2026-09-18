---
id: fond-htn-47-ignored-inventory
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: ignored-test inventory + stale-ignore sweep"
standing: BLOCKED
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
