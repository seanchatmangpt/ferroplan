# Contributing

Thanks for looking at ferroplan. This document covers dev setup, the test
surface, the vendored `crucible/` boundary, and the commit/receipt conventions
this repository runs on.

## Dev setup

```sh
git clone https://github.com/seanchatmangpt/ferroplan
cd ferroplan
cargo build --release        # produces target/release/ff
cargo test --workspace
```

A current stable Rust toolchain is required (pinned in `rust-toolchain.toml`).
The full pre-flight for a cut is documented in [`RELEASING.md`](RELEASING.md):
`fmt --check`, clippy `--all-targets --all-features -D warnings`,
`cargo test --workspace` (release), `doc -D warnings`, `bench --no-run`, the
ferroplan-py re-lock, `publish --dry-run`, build-check `ferroplan-mcp`, and the
maturin wheel build. A normal contribution does not need all of it; a release
does.

## Testing

`cargo test --workspace` is the primary gate. It covers the engine, the CLI,
the HDDL front-end, the FOND solvers, the WASM bindings, and the IPC fixtures
(one `#[test]` per committed corpus instance, closure-checked).

Bigger questions go through the harness:

- **Benchmarks / sweeps** — `benchmarks/standings.py` regenerates the standings
  table; `benchmarks/perf.py run`/`compare` track regressions against a
  committed baseline (see [`PROFILING.md`](docs/archive/notes/PROFILING.md)).
- **Examples** — [`examples/README.md`](examples/README.md) maps every worked
  domain to the feature it exercises; new features should be witnessed by an
  example or a test, not prose alone.
- **PDDL features** — every claim in the README is pinned to the test or
  RESULTS file that witnesses it. If you add a claim, pin it the same way.

## The crucible boundary

[`crucible/`](crucible/README.md) is a **vendored** benchmark sweep harness.
It is its own Cargo workspace root, excluded from the ferroplan workspace via
`exclude = ["crucible"]` in the root `Cargo.toml`, invisible to
`cargo test --workspace`, the clippy gate, `publish.sh` and CI. Its only gate
is `crucible/preflight.sh`, run before any crucible change lands.

Consequence: ferroplan-level gates never exercise crucible changes. If your
change touches `crucible/`, you run its preflight yourself; a green CI run on a
crucible-touching PR proves nothing about it. Do not move crucible code into
the workspace or vice versa without an explicit boundary change.

## Commit and receipt conventions

- **Fix forward.** Never `git reset --hard`; commits are immutable
  (`revert` is the exception).
- Commits are one coherent change; messages name what changed and why
  (`docs:`, `fix:`, `feat:` prefixes are the house style).
- **Claims carry witnesses.** A performance claim cites the command and the
  scoreboard; a behavior claim cites the test. Unwitnessed claims in docs,
  README, or changelog entries get edited out in review.
- **Typed refusals over silent truncation.** Unsupported inputs fail loudly
  with a typed, stage-named error — the same discipline applies to docs: a gap
  is stated as a Limitation, not papered over.
- Status vocabulary: `READY` / `BLOCKED` / `UNKNOWN` — inspection is not
  execution, and a green CI run is evidence about the paths it covered, not a
  crown.

## Pull requests

Explain what changed, why, the exact base, commands run and their exits, and
the tests/witnesses for any new claim. CI is read-only evidence; it does not
push generated corrections to the branch under test.
