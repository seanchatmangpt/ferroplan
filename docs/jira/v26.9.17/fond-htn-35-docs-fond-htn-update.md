---
id: fond-htn-35-docs-fond-htn-update
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Docs: FOND-HTN.md wave-4 refresh (Phase 3, budget knobs, deviations table)"
standing: BLOCKED
branch: docs/fond-htn-wave4
worktree: ~/ferroplan-worktrees/wt-d35
created: 2026-09-17T23:50:00Z
---

Read `_WAVE4-CONTEXT.md`. Base document: `docs/FOND-HTN.md` (wave-3, commit 3a001c3 lineage, now on main).

Scope — update, don't rewrite:
1. Semantics section: document Phase 3 (committable goal-reachability prune alternating with the witness prune; the FOUND_BUG_1 lineage and the always-on guard in `tests/fond_property.rs`), and the loop-bound policy (structural fixpoints bounded by states+1, caller caps advisory — reflect tickets 25's outcome as "landing" if unmerged).
2. Budget knobs: document `PlannerLimits::max_wall_ms`, `GroundingLimits`, `TranslateLimits` and their defaults; mark the internal 10 s translate cap honestly (ticket 23 pending — state the current behavior, not the intended one).
3. Deviations table refresh: add the parity facts — koala parser silently drops undeclared-typed objects where ferroplan rejects (T02); koala = strong-only decision procedure (T06 frozen evidence); koala `Success probability` is a leaf count.
4. Leave a `<!-- WAVE4-NUMBERS -->` placeholder block where the benchmark capacity numbers (tickets 26–28) will slot in at integration — do not invent numbers.

Gates: `cargo test -p ferroplan --doc` exit 0; one atomic commit.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T23:50:00Z | BLOCKED | docs/fond-htn-wave4 @ 90c2ae2 | — | all |
| 2026-09-17T2xxx | IN_PROGRESS | docs/fond-htn-wave4 @ 6d14813 | — | orient: read FOND-HTN.md + sources |
| 2026-09-18T00:51:44Z | PARTIAL_ALIVE | docs/fond-htn-wave4 @ f0034fb | gate `cargo test -p ferroplan --doc` exit 0 (1 passed, 0 failed); committed-blob grep: 6/6 load-bearing claims present (WAVE4-NUMBERS, FOUND_BUG_1 reproducer name, ValidationError::UndefinedType, LIMIT:translate-wall, 7fea6cc, oracle-goldens.json); note: start row above has malformed ts literal `2xxx` (written without clock read; actual ~2026-09-17T23:59Z) — left verbatim per append-only | history+receipt |
| 2026-09-18T00:52:10Z | ALIVE | docs/fond-htn-wave4 @ f0034fb (base 90c2ae2; parent 6d14813; not pushed) | gate `cargo test -p ferroplan --doc` exit 0 (1 passed); 1 atomic commit, 98 ins / 10 del, docs/FOND-HTN.md only | none — awaiting coordinator integration; WAVE4-NUMBERS left empty for tickets 26-28 |
