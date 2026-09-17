# FOND-HTN wave context — v26.9.17 (shared; every ticket links this)

day: prov:Activity "v26.9.17-fond-htn-hardening"
operator cut: migrate koala-planner → ferroplan. koala is an **external test oracle only**.

## KOALA POLICY (operator order, binding on every ticket)

- koala runs ONLY from `/tmp/fond-review/Planner` (its own checkout, fine) and `/tmp/fond-oracle` (our runner harness).
- NEVER copy koala code, grammar, build scripts, or verbatim files from koala-planner org repos (`Planner`, `HDDL-Parser`, `domains`) into ANY git repo or worktree — including fixtures. Use koala **concepts** only.
- Ferroplan-repo fixtures are either (a) IPC/competition files from canonical repos already fetched under `/tmp/fond-corpus/` (ipc2023-htn, panda-planner-dev/ipc2020-domains — competition data, not koala code; these MAY be copied into repo fixtures), or (b) hand-authored by you (add a one-line provenance comment).
- Golden JSON files recording oracle RESULTS (status/wall/policy shape) are facts, not code — committing them is allowed and encouraged.
- Sleath–Bercher flawed-model files live inside the koala HDDL-Parser repo → run them from `/tmp` as an EXTERNAL negative corpus; do not vendor them.

## Repo state at wave start

- main checkout `/Users/sac/ferroplan`, HEAD `d2faf4d` (do not mutate unless your ticket says so; never push).
- Untracked pre-existing: `plugins/chatman-ecosystem/scripts/*` — leave alone.
- Ferroplan FOND-HTN architecture: `crates/ferroplan-hddl` (parse → ground → translate) → `crates/ferroplan/src/hddl.rs::solve_hddl` → `crates/ferroplan/src/planning_runtime.rs` (`fond_policy` strong fixpoint ~L721, `fond_policy_strong_cyclic` ~L811, dispatch ~L380). WASM ops `hddl_solve`/`htn_plan`/`fond_policy` in `crates/ferroplan-wasm/src/wasi_abi.rs`.
- FROZEN landed branches (from wave 2, tests green): `fix/htn-method-backtracking` (tip `0f9ea8f`), `fix/oneof-koala-semantics` (tip `4d40f99`), `fix/fond-minors` (frozen commit `00fe98d`; the branch may advance during this wave — merge the SHA only).
- Known defects (wave-1 audit): strong-cyclic truncation dead-sink; `contingent_policy` depth-cache false-NoPlan; dead tautological post-check in `fond_policy`; Eve `DecomposeHddl` stage has no consumer; capability manifest has no HDDL/FOND entries; `hierarchical_plan` was state-blind (backtracking fix lands via frozen branch).

## Oracle (built and smoke-tested 2026-09-17)

Binaries (all built, verified end-to-end on Transport pfile01, flexible mode → policy found):
- parser: `/tmp/fond-review/Planner/parser/pandaPIparser`
- grounder: `/tmp/fond-review/Planner/grounder/pandaPIgrounder/pandaPIgrounder`
- planner: `/tmp/fond-review/Planner/planner/target/release/planner`
- wiring: `cd /tmp/fond-review/Planner && python3 solve.py <domain.hddl> <problem.hddl> [--flexible|--fixed-ld|--fixed] [--ff|--add|--max|--lmcut] [--output FILE] [--keep-json]` (timeout minutes default 30).
- macOS build notes (already applied; re-apply if /tmp is wiped): brew bison/flex/gengetopt; grounder built with `make CXX=g++ CC=gcc`; bliss required unzip→memleak-patch→`sed 's|(compiled "__DATE__")|(compiled " __DATE__ ")|'`→`make CC=g++` (the cpddl Makefile re-unzips bliss, so patch AFTER its unzip).
- CONCURRENCY HAZARD: `solve.py` writes shared temp files (`grounder/pandaPIgrounder/parsed.htn`, `serializer/result.sas+`, `planner/result.json`). ALL oracle invocations must be serialized through the flock-protected runner (T01 contract) — never call solve.py concurrently.
- Oracle quirks (expected, do not "fix"): `Success probability: 4.0000` is a solved-leaf count, not a probability; koala "flexible" finds strong plans (its strong-cyclic is preliminary — treat flexible+NOSOLUTION carefully; cross-check with `--fixed-ld`).

## Runner contract (T01 delivers; everyone else consumes)

- `/tmp/fond-oracle/build.sh` — flock `/tmp/fond-oracle/.lock`; idempotent; verifies/rebuilds binaries; exit 0 when ready.
- `/tmp/fond-oracle/oracle-run.sh <domain.hddl> <problem.hddl> [--mode flexible|fixed-ld|fixed] [--timeout SEC] [--heur ff|add|max|lmcut]` — flock-serialized; prints ONE line JSON: `{"status":"SOLVED|NOSOLUTION|ERROR|TIMEOUT|PARSE_ERROR","mode":...,"wall_s":...,"artifact":"/tmp/fond-oracle/runs/<id>/policy.txt"}`; non-zero exit only on harness failure (a NOSOLUTION verdict is a successful oracle run).
- If runner is missing when you need it: run `bash /tmp/fond-oracle/build.sh` (flock handles races), then retry. If still broken, record `HARNESS_UNAVAILABLE` and continue with what you can.

## Corpora (on disk now)

- `/tmp/fond-review/domains/` — 7 koala FOND-HTN domains × 15 problems. AssemblyHierarchical is BROKEN upstream (undeclared `FaultyPort` type + suspected modeling deadlock) → parse-rejection case only.
- `/tmp/fond-corpus/htn-ipc2023/` — 13 curated IPC-2023 HTN instances + README (verify results, compat-watch).
- `/tmp/fond-corpus/htn-other/` — curated 12 (4×IPC-2020, 4×PANDA feature tests, 4×SHOP3 hand-ports) + src clones.
- `/tmp/fond-corpus/fond-flat/` — may exist from T16 (race-safe: assume absent, self-author what you need).

## Ticket mechanics (票)

- Standing vocabulary: `ALIVE | BLOCKED | BUILD_BROKEN | PARTIAL_ALIVE | REFUSED_* | UNKNOWN | UNSUPPORTED`.
- History is append-only; append a row at start, after each gate, at end: `| ts UTC | standing | branch+SHA | gates+exits | remaining |`.
- Worktrees: `~/ferroplan-worktrees/<name>` (pre-created on your branch; if missing: `git -C /Users/sac/ferroplan worktree add -b <branch> ~/ferroplan-worktrees/<name> <base-SHA-if-given>`).
- Gates: run scoped (`cargo test -p <crate> ...`) from your worktree root; avoid `--workspace` unless your ticket says so.
- If you hit `[1302]` rate-limit errors: wait ~60 s, retry up to 3 times, then record BLOCKED with the error id.
- Never push. Never rebase shared branches. Commits atomic, imperative subject.
- Final report to coordinator: standing, branch+SHA, gates+exit codes, deliverable paths, remaining.
