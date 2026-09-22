---
id: fond-htn-67-sa2a-goal-set-semantics-divergence
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "adjudicate goal-set semantics: autofde-lab sa2a fixture yields NoPlan under full empty-task-network termination; beam4pm flip-audit escalates for upstream ruling (do NOT relax the beam4pm expectation)"
standing: PARTIAL_ALIVE
branch: (adjudication; no code yet)
worktree: n/a (analysis ticket)
created: 2026-09-18T18:20:00Z
source: beam4pm b4p-f5-06 flip audit (chore/ferroplan-verdict-flip-audit @ bda457f, FLIP-LEDGER.md; real run 2026-09-18, fp-probe scratch runner vs ferroplan main @ 6cacbda) + _CONTEXT.md session preview 2026-09-17
depends: none (pre-existing semantic question, not introduced by wave-6)
---

## The divergence (real run, witnessed)

The autofde-lab `sa2a-v26.9.17` HDDL domain/problem pair (borrowed fixture,
path recorded in beam4pm `FLIP-LEDGER.md` row "sa2a") is measured:

- ferroplan `main @ 6cacbda`: **NoPlan** in 4729 ms (real run, fp-probe).
- The fixture is believed solvable-by-design by its authors (autofde-lab's
  SA2A conformance pair): the problem expresses its objective as a goal SET
  over named fluents rather than a classical `(:goal)` network termination.

Two candidate semantic readings:

1. **Pure empty-network termination** (current behavior): a decomposed
   primitive network must reach `htn:done` with every method pre-condition
   satisfied; a problem whose goal facts are not encoded as task-network
   terminal conditions correctly yields NoPlan.
2. **Extended-goal typed-stop acceptance**: the solver should treat goal-set
   facts as admissible terminal states (`state |= goal` stops the search
   lawfully), which would make this fixture solvable.

This is the same divergence class the 2026-09-17 session preview recorded
(`_CONTEXT.md`: "a larger FOND fixture → NoPlan under full empty-task-network
semantics"). wave-6 did NOT introduce it (pre-existing on `main @ 6cacbda`).

## Scope

1. Adjudicate the intended semantics per the FOND-HTN line's own design docs
   (fond-htn-57..66 context; empty-task-network law from 57's
   choice-rewrite). Either (a) declare pure empty-network termination the
   law and the fixture out-of-contract, or (b) admit extended-goal typed
   stop as an explicit, gated solver mode.
2. If (b): spec + implement behind a `PlannerLimits`/request flag, with
   property tests that the classical path is byte-identical when the flag
   is off.
3. Record the ruling in beam4pm's flip ledger (b4p-f5-06) so the beam4pm
   expectation is re-derived from the ruling, not relaxed to green.

## Gates

- A written ruling in this ticket's History (a) or (b) with evidence.
- If (b): `cargo test -p ferroplan --lib planning_runtime` + the property
  suites green with the flag both off and on; no verdict change for any
  existing fixture with the flag off.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T18:20:00Z | BLOCKED | — | filed from beam4pm verdict-flip audit (real NoPlan run witnessed, command+exit in FLIP-LEDGER.md) | all |
| 2026-09-22T00:00:00Z | PARTIAL_ALIVE | fer-03-htn67 (merged to release/v26.9.22) | **RULING (a)** recorded: pure empty-network termination is the law; the sa2a-v26.9.17 goal-set fixture is OUT-OF-CONTRACT; option (b) (flag-gated accepted-terminal mode) is NOT implemented. Fresh-run evidence at HEAD: vendored fixture (byte-identical to autofde-lab @ 8ef74497, md5 595501e5/d884ad3b) runs in tests/sa2a_goal_set_ruling.rs — at 6cacbda the fp-probe saw NoPlan@4729ms; at the ruling SHA the pipeline is wall-bound instead (typed `Timeout{limit_ms:10000}` at defaults, still no verdict at 120 s, probe `--ignored`), so no plan and no verdict class form for a `:goal`-less goal-set problem — exactly ruling (a)'s content. Gates: `cargo test -p ferroplan --test sa2a_goal_set_ruling` exit 0 (2 passed, 1 ignored probe); `cargo test -p ferroplan --lib planning_runtime` exit 0 (15 passed). Remaining: beam4pm b4p-f5-06 flip-ledger write-back — re-derive the beam4pm expectation from this ruling (owning lane: beam4pm, not ferroplan). | flip-ledger write-back (beam4pm lane) |
