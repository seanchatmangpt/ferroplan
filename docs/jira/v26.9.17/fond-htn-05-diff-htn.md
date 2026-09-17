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
| 2026-09-17T22:05:00Z | BLOCKED | test/htn-ipc-oracle @ d2faf4d | worktree verified; corpora present (13 IPC-2023 + 12 curated); /tmp/fond-oracle harness WIPED (not in wave-start state) — rebuilding per T01 contract; koala binaries at /tmp/fond-review/Planner intact | harness rebuild, goldens, fixtures, test, gates |
| 2026-09-17T21:52:00Z | PARTIAL_ALIVE | test/htn-ipc-oracle @ d2faf4d (uncommitted work) | harness rebuilt in /tmp/fond-oracle (build.sh exit 0: parser/grounder/serializer/release planner verified; oracle-run.sh flock-serialized, one-line JSON contract); smokes: koala Transport pfile01 flexible SOLVED 0.229s; shop3-port-loan-noplan -> oracle ERROR (koala serializer KeyError: -1 on empty task network — honest ERROR verdict, kept); 42 fixture files copied to crates/ferroplan/tests/fixtures/htn-oracle/ (Lamps problem is pfile01.pddl per corpus README); htn_oracle.rs skeleton written; goldens harvest RUNNING serialized behind other wave tickets' long koala runs | goldens, RESULTS.md, mismatch tests, gate |
| 2026-09-17T22:12:00Z | PARTIAL_ALIVE | test/htn-ipc-oracle @ d2faf4d (uncommitted work) | goldens harvested 21/21 via /tmp/fond-oracle/oracle-run.sh (19 SOLVED flexible; panda-empty flexible-NOSOLUTION quirk cross-checked fixed-ld SOLVED -> golden SOLVED; loan-noplan stable ERROR both modes -> open verdict); verdicts at /tmp/fond-oracle/verdicts/htn.json + committed oracle-goldens.json (verdicts+wall only); serial measurement run: 14/21 strict agreement (each SOLVED policy outcome-closure-checked in-test), 6 mismatches: 4x TRANSLATE_ERROR (PCP_1, PO_Transport, Satellite-GTOHP, Transport — internal 10s translate limit, 14k-47k states queued), 2x GROUND_ERROR (panda-conditional-effect "nested 'when' out of scope"; panda-method-effect "unbound variable '?x'"); loan-noplan: ferroplan clean NoPlan where oracle harness crashes (tripwire pinned) | RESULTS.md, KNOWN_MISMATCHES + demos, final gate |
| 2026-09-17T22:20:00Z | ALIVE | test/htn-ipc-oracle @ cc0efe6 | GATE: cargo test -p ferroplan --test htn_oracle exit 0 (1 passed, 8 ignored, 0 warnings, 85.6s); all 7 #[ignore]d demos run green (exit 0, 127.4s) with live mismatch behaviors pinned; RESULTS.md committed with 21 instance rows + 6 mismatch artifact-path rows; atomic commit cc0efe6 (45 files, +3767) | none mandatory for this ticket; wave follow-ups: raise translate composite-BFS capacity (4 IPC instances), ground conditional effects (2 PANDA), koala flexible NOSOLUTION quirk documented in goldens note |
