# ferroplan: land or delete remote branch `feat/v26.8.1-full-planning-ggen`

- Standing: CLOSED
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
- 2026-09-22 | CLOSED | FERROPLAN-26922-08 | RULING: delete, do not merge. Rationale: (1) the branch is the 2026-07-31 v26.8.1 ggen full_planning graph attempt — 69/70 unique patches that never reached main and were referenced by no open PR since July; (2) main has since landed the FOND-HTN waves (fond-htn-21..66) that redesigned the same seams (planning_runtime dispatch, grounding/relevance, receipts/standing now live in the ERRC + docs/jira form the branch's full_planning/{receipt,standing,verify}.rs and .ggen/ferroplan-v26.8.1/ ontology predate); (3) a 69-patch merge two months past its base is an unbounded-risk diff with no consumer, against the lane's bounded-diff law — the failed edge is recorded here rather than silently pruned; (4) both tips are preserved locally (preserve/v26922-integration-all-open-prs, preserve/v26922-feat-v26.8.1-full-planning-ggen) so any future cycle can re-derive from them. Origin branches deleted; `git ls-remote origin refs/heads/integration/all-open-prs-2026-07-31` returns empty. | none (re-open as fresh tickets if the v26.8.1 graph work is wanted)
