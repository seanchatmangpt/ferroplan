# ferroplan: land or delete remote branch `feat/eve-genesis-lifecycle`

- Standing: CLOSED
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `feat/eve-genesis-lifecycle` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/ferroplan --head feat/eve-genesis-lifecycle`) or delete (`git push origin --delete feat/eve-genesis-lifecycle`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `feat/eve-genesis-lifecycle`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | feat/eve-genesis-lifecycle | decision pending
- 2026-09-22 | CLOSED | FERROPLAN-26922-04 | superseded draft deleted on origin (62767af, 5 unique patches superseded by v2/squash 42f9af3); tip preserved locally as preserve/v26922-eve-genesis-lifecycle | none
