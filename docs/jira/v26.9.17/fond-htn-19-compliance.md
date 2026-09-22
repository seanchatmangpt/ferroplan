---
id: fond-htn-19-compliance
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Compliance sweep: zero koala-derived code/data in any repo we own"
standing: ALIVE
worktree: none (read-only sweep; report to /tmp)
created: 2026-09-17T21:30:00Z
---

Read `_FOND-HTN-WAVE-CONTEXT.md` first. READ-ONLY: you edit nothing in any repo/worktree. Your report is `/tmp/fond-oracle/COMPLIANCE.md` (create dir if needed).

## Scope
1. Scan `/Users/sac/ferroplan` and every `/Users/sac/ferroplan-worktrees/*` (exclude `target/`, `.git/`): grep -ri for koala markers — `koala`, `pandaPI`, `fond_act__`, `fond_merger`, `htn_serializer`, `panda-planner`, `hddl.y`, `pandaPIparser`, `pandaPIgrounder`.
2. For each hit: classify — ALLOWED (prose citation/provenance comment/golden verdict), SUSPECT (code or fixture resembling koala implementation), VIOLATION (copy of koala code/grammar/verbatim domain file without canonical provenance). For SUSPECT: byte-diff against the corresponding koala file(s) under /tmp/fond-review; record diff result.
3. Specifically audit the frozen branches' diffs: `git -C /Users/sac/ferroplan diff d2faf4d..0f9ea8f`, `..4d40f99`, `..00fe98d` — confirm fixes are concept-authored (e.g. `MalformedOneof` is our error type), not ported koala lines.
4. Check in-repo fixture provenance comments exist for every new fixture on branch worktrees you scan (spot-check ≥ 10).
5. Verdict per repo path: CLEAN / VIOLATION(evidence) / REVIEW-NEEDED(why). Sweep TWICE — once at start, once at end (branches evolve during the wave); final verdict uses the end sweep.

## Gates
COMPLIANCE.md exists with both sweeps, per-path verdicts, and the three branch audits.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-17T21:30:00Z | BLOCKED | none | — | both sweeps |
| 2026-09-17T22:32:40Z | ALIVE | none (read-only) | start sweep begun: 17 worktrees + main, grep koala-marker set | start sweep, 3 branch audits, provenance spot-check, end sweep, COMPLIANCE.md |
| 2026-09-17T22:36:00Z | ALIVE | none (read-only) | start sweep done (407 hits, 0 VIOLATION; 1 byte-collision resolved to canonical corpus IPC data); 3 branch diffs audited concept-authored (0f9ea8f: 0 koala lines, 4d40f99: 27 prose-citation lines only, MalformedOneof pre-existing our type, 00fe98d: 0); provenance spot-check 14/14 PASS; probabilistic.rs vs prob_preprocessor.py = concept reimpl with documented divergences | end sweep, COMPLIANCE.md |
| 2026-09-17T22:44:33Z | ALIVE | none (read-only) | GATE MET: end sweep 449 hits, drift = 4 prose-only files (FOND-HTN.md, htn_ipc2023.rs provenance header, ast.rs/hddl.rs oneof citations); re-hash of 3 advanced repos: 0 new koala byte-collisions; COMPLIANCE.md written with both sweeps + per-path verdicts (17/17 CLEAN + main CLEAN) + 3 branch audits | none — final verdict CLEAN, 0 VIOLATION, 0 REVIEW-NEEDED |
