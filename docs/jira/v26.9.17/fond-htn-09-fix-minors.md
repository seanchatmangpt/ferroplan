---
id: fond-htn-09-fix-minors
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "FOND minors: contingent depth-cache + dead post-check (mass fix already on branch)"
standing: ALIVE
branch: fix/fond-minors
worktree: ~/ferroplan-worktrees/wt-minor
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. NOTE: branch tip `00fe98d` already landed "stop enforcing probability-mass ceremony on mass-blind planners" (audited Fix 2, complete with tests — verify, don't redo). Do NOT restructure `fond_policy`/`fond_policy_strong_cyclic` (T07 owns those regions on another branch) — keep your diff inside `contingent_policy`/`solve_belief` and at most a comment-level touch elsewhere.

## Fix 1 — contingent depth-cache
`contingent_policy`/`solve_belief`: memo caches `None` depth-free, but failure can be depth-caused (`depth >= max_depth`); a belief failed deep is later wrongly unsolvable when re-reached shallow ⇒ false NoPlan. Fix: only memoize negative results that are depth-independent, or store the failing depth and re-search when re-reached with more remaining budget (choose the simpler correct variant; document choice). Test: belief reachable deep (beyond budget at first encounter) and shallow (within budget later), solvable — must now solve; pre-fix returned NoPlan.

## Fix 2 — dead post-check cleanup in fond_policy
Same as T07 item 3 — IF you get there first it's done once; coordinate via History note ("done here" / "deferred to T07"). Prefer deferring to avoid merge conflict: only add a code comment if you must touch it; leave the conversion itself to T07.

## Gates
`cd ~/ferroplan-worktrees/wt-minor && cargo test -p ferroplan --lib planning_runtime && cargo test -p ferroplan --test planning_runtime` exit 0. Atomic commits; History rows per fix.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | fix/fond-minors @ 00fe98d | — | fix 1, fix 2-coordination |
| 2026-09-17T21:35:48Z | ALIVE | fix/fond-minors @ 00fe98d | orient: fix 1 already landed on branch as 52c3501 — `ContingentMemo::Failed { at_depth }` + shallow-re-encounter re-search (chosen variant: store failing depth; documented in enum doc + test comment) + discriminating test. Verify-not-redo per ticket note; diff stays inside contingent_policy as required | falsify test claim, gates, fix-2 note |
| 2026-09-17T21:39:00Z | ALIVE | fix/fond-minors @ 00fe98d | falsifier by execution: test-only hunk of 52c3501 applied to d2faf4d scratch worktree, `cargo clean -p ferroplan` forced rebuild (`Compiling ferroplan (/private/tmp/wt-fix1-prefix)`) → 0 passed, 1 FAILED, panic "the shallow re-encounter of {m} must re-search…: NoPlan" — test discriminates the exact ticket scenario. First attempt VOIDED: stale shared-target binary ran from wrong cwd (false ok, no compile line) — redone with forced rebuild | gates |
| 2026-09-17T21:43:05Z | ALIVE | fix/fond-minors @ 00fe98d | gate1 `cargo test -p ferroplan --lib planning_runtime` exit 0 (7 passed; rebuilt from wt-minor tip path, compile line witnessed); gate2 `cargo test -p ferroplan --test planning_runtime` exit 0 (15 passed); also re-verifies 00fe98d mass-fix tests. Scratch worktree removed, tree clean | fix-2 note, final |
| 2026-09-17T21:43:37Z | ALIVE | fix/fond-minors @ 00fe98d (no new commit — fix 1 pre-landed + verified, nothing to commit) | fix 2: DEFERRED TO T07 per ticket preference — zero touches to fond_policy/fond_policy_strong_cyclic, no merge-conflict surface; T07 owns the conversion | none |
