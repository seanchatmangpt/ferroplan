# ferroplan: land or delete remote branch `agent/planning-type-constitution`

- Standing: CLOSED
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `agent/planning-type-constitution` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/ferroplan --head agent/planning-type-constitution`) or delete (`git push origin --delete agent/planning-type-constitution`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `agent/planning-type-constitution`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | agent/planning-type-constitution | decision pending
- 2026-09-22 | CLOSED | FERROPLAN-26922-04 | superseded draft deleted on origin (85d8fe1, 4 unique patches superseded by v2/squash 282fae4); tip preserved locally as preserve/v26922-planning-type-constitution | none
