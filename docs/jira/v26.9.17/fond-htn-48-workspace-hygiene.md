---
id: fond-htn-48-workspace-hygiene
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Finish: workspace hygiene — fmt, clippy on new code, intra-doc links"
standing: ALIVE
branch: chore/workspace-hygiene
worktree: ~/ferroplan-worktrees/wt-d48
created: 2026-09-18T00:55:00Z
---

Read `_WAVE4-CONTEXT.md`. Hygiene on the BASE tree (your branch = main@6d14813; wave-4 branches land separately — record findings that their merges will inherit, fix what is fixable at base without stepping on their lanes).

Scope:
1. `cargo fmt --check` workspace-wide: list offenders; fix ONLY files no wave-4 branch owns (check the runbook's ownership lanes); fmt-only commit.
2. `cargo clippy -p ferroplan -p ferroplan-hddl --all-targets`: triage every warning as pre-existing vs introduced-by-waves-3/4 code ALREADY ON MAIN (base 6d14813 already contains waves 1–3). Fix the main-side ones where mechanical (allow-with-reason only as last resort, each justified in the commit message); the known `translate.rs:647` clippy error: diagnose precisely (line content + lint) and record — fix ONLY if it is a trivially safe one-liner, else leave with a History note.
3. `cargo doc --no-deps -p ferroplan -p ferroplan-hddl`: fix broken intra-doc links on main-side code (warnings listed by rustdoc).
4. Record a findings table in the ticket History: file | issue | fixed-here | left-for-integration (with reason).

Gates: `cargo fmt --check` clean on the files you own; `cargo clippy --all-targets -p ferroplan -p ferroplan-hddl` no NEW warnings vs base; `cargo test -p ferroplan --lib planning_runtime` exit 0 (fmt must not break anything).

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T00:55:00Z | BLOCKED | chore/workspace-hygiene @ 6d14813 | — | all |
| 2026-09-18T03:02:00Z | respawn-in-flight | chore/workspace-hygiene @ 75870de (wt-d48) | — | coordinator-close: wave-5 attempt never started (no History rows after the cut); second attempt in flight (wave 6), worktree fast-forwarded to 75870de | wave-6 respawn owns |
| 2026-09-22T00:00:00Z | ALIVE | fer-07-respawns (merged to release/v26.9.22) | FINISHED by FERROPLAN-26922-07: all 43 merged worktrees under ~/ferroplan-worktrees/wt-* pruned (git worktree list 50 -> 7, the remainder being the lane's own per-WO checkouts, the release integration checkout, and another lane's checkout); every pruned branch verified 0 commits outside main before removal; the 3 dirty worktrees (wt-h60/61/62) had their residue preserved on preserve/v26922-wt-h60/61/62 first. | none |
