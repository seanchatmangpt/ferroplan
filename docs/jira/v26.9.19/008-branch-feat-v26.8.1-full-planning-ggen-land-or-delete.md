# ferroplan: land or delete remote branch `feat/v26.8.1-full-planning-ggen`

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `feat/v26.8.1-full-planning-ggen` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/ferroplan --head feat/v26.8.1-full-planning-ggen`) or delete (`git push origin --delete feat/v26.8.1-full-planning-ggen`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `feat/v26.8.1-full-planning-ggen`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | feat/v26.8.1-full-planning-ggen | decision pending
