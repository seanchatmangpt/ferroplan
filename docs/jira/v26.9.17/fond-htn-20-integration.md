---
id: fond-htn-20-integration
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Serial integration of the three FROZEN wave-2 branches into main"
standing: BLOCKED
worktree: MAIN CHECKOUT /Users/sac/ferroplan (sole authorized writer this wave)
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. You are the ONLY ticket authorized to touch the main checkout. Rider protocol: only merge when `git -C /Users/sac/ferroplan status` is clean apart from the pre-existing untracked `plugins/chatman-ecosystem/scripts/*` and this wave's docs (untracked docs are fine — do not commit them). Never push. Red gate → `git merge --abort` + standing BLOCKED + History.

## Scope
Merge BY EXACT SHA (never branch names — branches may advance during the wave), in this order, one `--no-ff` merge each:
1. `0f9ea8f` (fix/htn-method-backtracking: hierarchical_plan method backtracking)
2. `4d40f99` (fix/oneof-koala-semantics: oneof grammar alignment)
3. `00fe98d` (fix/fond-minors snapshot: mass-validation relaxation)

Between EVERY merge: `cargo test -p ferroplan --lib planning_runtime && cargo test -p ferroplan --test planning_runtime && cargo test -p ferroplan-hddl && cargo test -p ferroplan --lib hddl`. All exit 0 before the next merge. A merge conflict: resolve minimally in favor of semantic union (both features must survive), rerun gates. Two consecutive red gates on the same merge → abort that merge, record BLOCKED, continue with remaining SHAs only if independent.

Commit message convention: `Merge '<sha>' (<branch-name>) into main` (match repo style; `--no-edit`).

## Gates
Three `--no-ff` merges landed (or lawful BLOCKED record), gates green after each, `git log --oneline -6` shows the merges; final `cargo test -p ferroplan -p ferroplan-hddl` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | main @ d2faf4d | — | 3 merges |
