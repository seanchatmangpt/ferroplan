# ferroplan: land or delete remote branch `agent/sync-upstream-c8d43f9`

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `agent/sync-upstream-c8d43f9` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/ferroplan --head agent/sync-upstream-c8d43f9`) or delete (`git push origin --delete agent/sync-upstream-c8d43f9`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `agent/sync-upstream-c8d43f9`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | agent/sync-upstream-c8d43f9 | decision pending
