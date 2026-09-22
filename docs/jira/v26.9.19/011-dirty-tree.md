# ferroplan: commit or clean uncommitted changes

- Standing: CLOSED
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: working tree dirty at survey time
- Evidence: `git status --porcelain` → 1 path(s) (tracked-modified: 0, untracked: 1); sample: ?? docs/jira/v26.9.17/fond-htn-67-sa2a-goal-set-semantics-divergence.md;

## Work to complete
- For the 1 untracked path(s): add intentional files to git and commit; gitignore or delete build artifacts/temp files.
- Note: this survey's ticket files under docs/jira/v26.9.19/ are intentionally uncommitted; include or exclude them deliberately in the commit plan.

## Acceptance
- `git status --porcelain` is clean (except items deliberately deferred and recorded here).

## History
- 2026-09-19 | OPEN | survey found dirty tree | 1 paths (T0/U1) | commit/clean pending
- 2026-09-22 | CLOSED | FERROPLAN-26922-01..04 | both untracked paths landed intentionally: docs/jira/v26.9.17/fond-htn-67-*.md committed with the ruling (FERROPLAN-26922-03), docs/jira/v26.9.19/ committed by this WO | none
