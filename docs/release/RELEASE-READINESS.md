# Release Readiness — ferroplan 1.0

Audit date: 2026-10-09 · auditor: ferro-release lane · subject: `main` @ `620f953` +
this lane's doc fixes. Scope: external-reader surface only. Statuses: READY (verified
against the tree) / BLOCKED (external action or version-gated work required).

| # | item | status | evidence |
|---|---|---|---|
| 1 | CI badge present, workflow exists, CI green on main | **READY** | README.md:6 badge → `.github/workflows/ci.yml`; last completed run `success` (`gh run list`, 2026-10-09) |
| 2 | LICENSE present and matches Cargo metadata | **READY** | `LICENSE-MIT` + `LICENSE-APACHE` at root; `[workspace.package] license = "MIT OR Apache-2.0"` (Cargo.toml:17); README License section links both |
| 3 | Cargo workspace metadata: repository, keywords, categories, authors | **READY** | `Cargo.toml` `[workspace.package]`: repository `seanchatmangpt/ferroplan`, homepage, keywords (pddl/planner/…), categories, authors; crates/ferroplan + ferroplan-cli add per-crate `description` + `readme` |
| 4 | README badges (CI, docs, demo, license) | **READY** | README.md lines 5–8, all four present and pointing at real URLs |
| 5 | README install instructions accurate | **READY (with caveat)** | `cargo install ferroplan-cli` documented; **caveat**: crates.io carries 0.27.1 while main is 0.29.0 — README status note (line ~72) already discloses this lag |
| 6 | README quickstart / CLI examples accurate | **READY** | `ff -o/-f`, `--json`, `--mode`, `--search`, `--decompose`, `--json-request` all map to clap flags in `crates/ferroplan-cli`; Library snippet matches `ferroplan::solve/parse/Session` API |
| 7 | docs/ structure coherent for an external reader | **READY (partial)** | `book/src/` (mdbook, 21 chapters incl. install/cli/library/tuning/standings) is the external path; `docs/` top level is internal-first (jira/, sjira/, archive/, roadmaps) — acceptable, but README's FOND links route externals into it deliberately |
| 8 | Vendored `crucible/` boundary documented for external users | **READY** | `crucible/README.md` states vendored-by-intent, own workspace root, `publish = false`, invisible to ferroplan CI, own gate `crucible/preflight.sh`; root `Cargo.toml` `exclude = ["crucible"]` matches; **gap**: root README never links `crucible/README.md` — external users discover the harness only via the what's-new mention (**fixed this pass**: README now carries a "Vendored harness" note linking `crucible/README.md`) |
| 9 | Dead / wrong-host links in the README | **BLOCKED → READY (fixed this pass)** | 4 links pointed at `github.com/hhh42/...` / `hhh42.github.io` (the upstream author's host) instead of `seanchatmangpt/...`: standings link (line 50), changelog links (lines 104, 121), tuning-chapter link (line 377). Rewritten to `seanchatmangpt` |
| 10 | Changelog current | **READY** | `CHANGELOG.md` + `CHANGELOG-ARCHIVE.md` present; WHATSNEW block in README generated/rolled by `scripts/release-notes-roll.py` |
| 11 | Release process documented | **READY** | `RELEASING.md` at root (pre-flight includes crucible exclusion check); `publish.sh` present |
| 12 | Examples + index for external users | **READY** | `examples/README.md` index, README Examples section links rpg/rpg-world/cabin/village/… with per-example claims |
| 13 | Benchmark claims carry evidence paths | **READY** | `benchmarks/ipc-standings.md`, `ipc5-scoreboard.md`, `results.md`, `COMPARING.md` (oracle licensing disclosed), `docs/BENCHMARKS.md` — all linked from README Benchmarks section |
| 14 | Limitations stated honestly | **READY** | README Limitations section (numeric, IPC-5 preference quality, PDDL3 timed constraints, temporal tails, derived predicates) with tracked tickets |
| 15 | Community health files: CONTRIBUTING / CODE_OF_CONDUCT / SECURITY | **BLOCKED** | None of the three exist at root. Not doc-trivial (each needs owner input on process/contacts); recommend adding before the 1.0 tag |
| 16 | Version / publication state | **BLOCKED (by design)** | workspace at 0.29.0 "release candidate"; crates.io at 0.27.1; 1.0 bump + tag + publish are the coordinator's gated transition, explicitly out of this lane's scope |
| 17 | Vendored oracle licensing boundary | **READY** | README Benchmarks: comparison oracles not bundled (GPL/non-commercial), reproduction via `benchmarks/COMPARING.md` — no license contamination in-tree |

**Counts: 14 READY, 3 BLOCKED.**

BLOCKED summary (none of these are doc-trivial fixes):

1. **Community files** (#15) — CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md absent.
2. **Version gate** (#16) — 0.29.0 → 1.0 bump, tag, crates.io publish, and re-measured
   standings for the cut (README admits the table is the 0.28 sweep) await the
   coordinator's release directive.
3. **(#9 residual)** — the four stale-host links were README-only; a repo-wide
   `grep -rn "hhh42" --include='*.md'` still hits `docs/archive/` and generated
   receipts, which are historical records and left as-is by policy.
