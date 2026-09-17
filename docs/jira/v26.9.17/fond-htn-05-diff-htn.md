---
id: fond-htn-05-diff-htn
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Differential: solve_hddl vs oracle on deterministic HTN corpora"
standing: BLOCKED
branch: test/htn-ipc-oracle
worktree: ~/ferroplan-worktrees/wt-oracle-htn
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. IPC competition files MAY be copied into repo fixtures (canonical data); PANDA/SHOP3 files likewise from `/tmp/fond-corpus/htn-other/curated/` (they carry upstream provenance headers — keep them).

## Scope
1. Fixtures: copy the 13 IPC-2023 instances (`/tmp/fond-corpus/htn-ipc2023/`) + the 8 curated `htn-other` pairs into `crates/ferroplan/tests/fixtures/htn-oracle/` (preserve per-domain subdirs). Where an instance is > 100 KB, include only domain + the single chosen problem.
2. Oracle goldens: harvest each copied instance via `/tmp/fond-oracle/oracle-run.sh` (self-sufficient protocol per T02) — prefer reusing `/tmp/fond-oracle/verdicts/htn.json` if T03 already produced it; else harvest yourself. Commit `crates/ferroplan/tests/fixtures/htn-oracle/oracle-goldens.json` (verdicts + wall only).
3. Agreement test `crates/ferroplan/tests/htn_oracle.rs`: for every fixture, `solve_hddl` with a ≤ 60 s wall budget must agree with the golden where golden status ∈ {SOLVED, NOSOLUTION}; golden TIMEOUT/PARSE_ERROR cases assert only "no panic, bounded exit". Deterministic domains: SOLVED ⇒ returned plan/policy must be outcome-closed (in-test checker). Translation wall time per instance recorded into `RESULTS.md` committed alongside — flag SLOW > 10 s.
4. Mismatches: `#[ignore]` test per mismatch + History row with oracle artifact path.

## Gates
`cargo test -p ferroplan --test htn_oracle` exit 0; RESULTS.md committed with ≥ 21 rows.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | test/htn-ipc-oracle @ base d2faf4d | — | all |
