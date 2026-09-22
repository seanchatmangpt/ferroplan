---
id: fond-htn-12-test-fondhtn-regression
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Hand-authored FOND-HTN micro-domain regression suite for solve_hddl"
standing: ALIVE
branch: test/fond-htn-koala
worktree: ~/ferroplan-worktrees/wt-test-fondhtn
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. Worktree only. ALL fixtures hand-authored with provenance comments ("hand-authored, <pattern>-pattern") — no koala files in the repo.

## Scope
1. Fixtures under `crates/ferroplan/tests/fixtures/fond-htn-micro/` — micro HDDL domain+problem pairs (each ≤ 30 lines): drop-retry (oneof success/empty-branch — Transport pattern), tray-dirty (overlapping branches — Childsnack pattern), sense-then-branch (two-branch sensing — Satellite pattern), grow-loop (recursive method — Snake pattern), supervisor-fail (fail-then-retry via second method — Depots pattern), plain-chain (deterministic baseline).
2. `crates/ferroplan/tests/fond_htn_micro.rs`: read `solve_hddl`'s signature from `crates/ferroplan/src/hddl.rs`; per fixture assert: solved with outcome-closed policy (in-test closure checker) OR typed NoPlan where the fixture is deliberately unsolvable (add one such: a oneof whose both branches dead-end). Also assert: policy outcomes count == branch count for the oneof action (no outcome collapsing), empty branch yields a no-change outcome, and recursion fixture terminates within budget.
3. No-panic/no-hang contract: every fixture wrapped with a wall budget from solve_hddl's API.
4. Report any semantic surprise in History (e.g., decomposition-order effects) — findings, not fixes.

## Gates
`cd ~/ferroplan-worktrees/wt-test-fondhtn && cargo test -p ferroplan --test fond_htn_micro` exit 0.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/fond-htn-koala @ base d2faf4d | — | all |
| 2026-09-17T22:32:29Z | ALIVE | test/fond-htn-koala @ d2faf4d (worktree wt-test-fondhtn) | start: ticket+context read, worktree verified clean | orient: read solve_hddl signature, author fixtures+test |
| 2026-09-17T22:58:00Z | PARTIAL_ALIVE | test/fond-htn-koala @ c4137fb (wt-test-fondhtn) | cargo test -p ferroplan --test fond_htn_micro: run1 exit 101 (2 real defects in test draft, 0 in src), run2 exit 0 (9/9), run3 exit 0 (stable) | findings row + final receipt |
| 2026-09-17T22:59:00Z | PARTIAL_ALIVE | test/fond-htn-koala @ c4137fb | findings (report-only, no fixes): (1) retry patterns (drop-retry, supervisor-fail) are NOT solvable by the primary `fond_policy` strong fixpoint — they solve only via the `fond_policy_strong_cyclic` fallback (asserted via plan.notes); the self-loop is expressible only because translate's rank-canonicalized frontier marker collapses the post-empty-branch composite state back onto the initial state. (2) Empty oneof branch is a real distinct outcome (2 transitions, 500k/500k ppm split, no collapsing) whose landing state's world facts are identical to the source — "no-change outcome" confirmed at policy level. (3) Zero-arg ground actions/methods are named WITHOUT parens (`place`, not `place()`) — `atom_key` adds parens only when args exist; run1's failures were test-side needles assuming `place()`; hazard for any consumer keying on policy action names. (4) Decomposition is execution-gated: a later abstract subtask cannot decompose until its ordered predecessor has EXECUTED (grow-loop's inner `build` waits for `add-block`), which is what lets state-gated method preconditions (`(not (full))`) select the right branch. (5) All branch selection across 6 patterns is purely translate-time method-precondition gating; no observation channel needed. (6) 1ms wall budget returns typed `HddlError::Timeout` — watchdog contract live. (7) 6/7 fixtures needed no typing at all (propositional); only drop-retry exercises `:types`. | final receipt |
| 2026-09-17T23:00:00Z | ALIVE | test/fond-htn-koala @ c4137fb (worktree wt-test-fondhtn, clean; main checkout untouched except this ticket) | cargo test -p ferroplan --test fond_htn_micro exit 0 (9 passed; rerun stable; fixtures 10-24 lines each, all ≤30, provenance comments present) | none — deliverables complete: crates/ferroplan/tests/fixtures/fond-htn-micro/{drop-retry,tray-dirty,sense-then-branch,grow-loop,supervisor-fail,plain-chain,both-branches-deadend}/{domain,problem}.hddl + crates/ferroplan/tests/fond_htn_micro.rs |
