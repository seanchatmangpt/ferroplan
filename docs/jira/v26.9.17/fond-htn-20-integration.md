---
id: fond-htn-20-integration
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Serial integration of the three FROZEN wave-2 branches into main"
standing: ALIVE
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
| 2026-09-17T22:35:32Z | PARTIAL_ALIVE | main checkout @ 2a09e48 (d2faf4d + docs-only wave cut; branch `main` is unrelated upstream lineage 2d5b891 — integration line = this checkout) | pre-flight: 3 SHAs verified on frozen branches; merge1/merge3 both touch planning_runtime.rs (conflict risk); tree = 43 append-only History insertions (rider-allowed wave docs, not committed) + pre-existing untracked plugins/*; gate set confirmed | 3 merges |
| 2026-09-17T22:37:21Z | PARTIAL_ALIVE | merge 1/3: 0f9ea8f → 3a05d48 clean --no-ff (ort, no conflict; planning_runtime.rs +86, tests +159) | gates: planning_runtime lib 6 passed/exit 0; planning_runtime test 17 passed/exit 0; ferroplan-hddl 91+7 passed/exit 0; hddl lib 3 passed/exit 0 | 2 merges |
| 2026-09-17T22:39:07Z | PARTIAL_ALIVE | merge 2/3: 4d40f99 → b1d3d95 clean --no-ff (ort, no conflict; ferroplan-hddl parser/translate/ast + hddl.rs, +755/−41) | gates: planning_runtime lib 6 passed/exit 0; planning_runtime test 17 passed/exit 0; ferroplan-hddl 103+7 passed/exit 0 (oneof tests landed); hddl lib 5 passed/exit 0 | 1 merge |
| 2026-09-17T22:43:00Z | PARTIAL_ALIVE | merge 3/3: 00fe98d → 6a86b80 clean --no-ff (ort auto-merge of planning_runtime.rs + tests, disjoint regions, no conflict) | gates: planning_runtime lib 7 passed/exit 0; planning_runtime test 20 passed/exit 0 (fond-minors tests landed); ferroplan-hddl 103+7 passed/exit 0; hddl lib 5 passed/exit 0 | 0 merges |
| 2026-09-17T22:50:56Z | ALIVE | main checkout integration line @ 6a86b80 = 2a09e48 + 3 merges (0f9ea8f, 4d40f99, 00fe98d); `git log --oneline -6` shows all three; nothing pushed; tree = wave-doc History appends (uncommitted, rider-allowed) + pre-existing untracked plugins/* only | final gate `cargo test -p ferroplan -p ferroplan-hddl` exit 0 (64 ok result-blocks, 0 failed); no [1302] events; operator wrote zero bytes | none — ticket complete |
