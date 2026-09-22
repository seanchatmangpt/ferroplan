# ferroplan: land or delete remote branch `feat/eve-genesis-lifecycle-v2`

- Standing: CLOSED
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `feat/eve-genesis-lifecycle-v2` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/ferroplan --head feat/eve-genesis-lifecycle-v2`) or delete (`git push origin --delete feat/eve-genesis-lifecycle-v2`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `feat/eve-genesis-lifecycle-v2`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | feat/eve-genesis-lifecycle-v2 | decision pending
- 2026-09-22 | CLOSED | FERROPLAN-26922-04 | deleted on origin; tree of a3abad5 verified identical to squash-merge 42f9af3 in main | none
