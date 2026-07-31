# Working agreements

## Dogfood only

This repository is the Chatman ecosystem's own first managed world. Any
repository-management work here that the phase engine governs — observation,
CMCA allocation, Ferroplan planning, manufacturing, validation, admission —
goes exclusively through the real tool chain: the `ferroplan` MCP tools
(`session_open`/`session_observe`/`session_think`/`session_advance`,
`cmca_allocate`, `bind_allocation_receipt`, `bind_plan_receipt`,
`verify_receipt`) and `plugins/chatman-ecosystem/scripts/loop.py`.

Concretely, this rules out:

- **Hand-authored receipts.** Never write JSON that stands in for a
  `bind_*_receipt` result. Call the tool; use exactly what it returns.
- **Placeholder allocation factors.** `cmca_allocate` factors must come from
  real repository measurements (diff size, conflict count, age, commits
  ahead, whatever the work actually is) — not uniform `1`s used to satisfy
  the arity requirement.
- **Narrated ceremony.** Don't describe what the loop would do; run it.
- **No temp envelope files.** Pipe a tool result straight into
  `--envelope -` (stdin) on `loop.py admit` / `phase.py transition`. Never
  write a `bind_*_receipt` result to a scratch file (`/tmp/*.json` or
  otherwise) as an intermediate step just to hand it to the CLI.
- **Repeating a workaround instead of fixing the gap it works around.**
  `loop.py admit` and `phase.py transition` used to accept only
  `--envelope <path>`, with no stdin/inline form, forcing an agent to write
  a tool's JSON result to a file before admitting it — a defect in the
  tool, not a permanent hand step. That gap is fixed (`--envelope -` reads
  stdin in both). If a future pipeline step forces the same kind of manual
  transcription, fix the tool the same way rather than re-performing the
  manual bridge every session.

When the admission path is broken in the current environment (missing
binary, unbuilt crate, stale cache), say so plainly and fix the actual
dependency (e.g. `cargo build -p ferroplan-mcp`) — do not paper over it by
hand-crafting the receipt the broken tool was supposed to produce.

- **Finish in main.** Every completed phase/cycle push goes to `main` as
  well as the working branch — `publish.sh` runs from `main`, and a
  release isn't "ready" until `main` has it. Push the working branch,
  then fast-forward `main` (`git push origin <branch>:main`).
- Cycle discipline lives in `docs/roadmap-0.N.md`: measured win or
  recorded negative, fixtures first, scoreboards defend themselves.
- Full pre-flight before any cut: latest stable, `fmt --check`, clippy
  `--all-targets --all-features -D warnings`, `test --all` (release),
  doc `-D warnings`, `bench --no-run`, ferroplan-py version+re-lock,
  `publish -p ferroplan --dry-run`, build-check `ferroplan-mcp`, and
  the maturin wheel build (0.14+). See `RELEASING.md`.
