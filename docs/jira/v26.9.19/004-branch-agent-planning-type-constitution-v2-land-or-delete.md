# ferroplan: land or delete remote branch `agent/planning-type-constitution-v2`

- Standing: CLOSED
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `agent/planning-type-constitution-v2` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/ferroplan --head agent/planning-type-constitution-v2`) or delete (`git push origin --delete agent/planning-type-constitution-v2`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `agent/planning-type-constitution-v2`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | agent/planning-type-constitution-v2 | decision pending
- 2026-09-22 | CLOSED | FERROPLAN-26922-04 | deleted on origin; tree of 2c983aa verified identical to squash-merge 282fae4 in main | none
