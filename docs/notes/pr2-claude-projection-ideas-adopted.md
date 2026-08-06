# Ideas adopted from PR #2 (draft, not merged)

PR #2, `agent/v26.7.29-claude-projection`, lands as a full,
self-consistent rewrite of `plugins/chatman-ecosystem/`
(marketplace/plugin manifests, all 8 agents, all 13 skills, hooks,
ontology, profiles, generated-guard, README) pinned to a single
coordinated release, `26.7.29`. Draft. Not merged.

This note logs what got pulled out of that PR straight into `main`
without merging the PR itself and without touching anything the PR
also touches — per an explicit "merge the ideas, do not overwrite"
directive.

## Adopted (added as new, standalone files)

Six files, each clean of the rest of PR #2's rewrite, none asserting
anything about this repo's current state (no version pins, no "this
is version 26.7.29" claims), each checked to parse/compile cleanly —
and, for `effective-phase.py`, run correctly against this project's
live ledger — before landing:

- `plugins/chatman-ecosystem/ontology/authority-graph.ttl` — a
  timeless RDF vocabulary for agent tool grants/denials, claim
  ceilings, and spawn authority. Nothing calls it yet; sitting ready
  for a future SHACL pass over the existing agents.
- `plugins/chatman-ecosystem/profiles/actuation-intent.schema.json`
  — a JSON Schema for structured actuation intents / derived
  execution grants. Not wired to any hook yet.
- `plugins/chatman-ecosystem/scripts/effective-phase.py` — projects
  a `canonical_vector` (the receipted phase state) apart from an
  `effective_vector` (canonical + pending-observation frontier),
  instead of conflating the two the way `phase.py status` does
  today. Stands on its own; run `python3
  plugins/chatman-ecosystem/scripts/effective-phase.py --project
  <path>` to see it work.
- `plugins/chatman-ecosystem/scripts/actuation-intent.py`,
  `grant-actuation.py` — a two-step "manufacture an intent, then
  derive a bounded grant from a verified receipt" pattern for
  protected Bash actuation. Standalone; `hooks/hooks.json` doesn't
  call it.
- `plugins/chatman-ecosystem/scripts/event-summary.py` — records
  bounded lifecycle candidates and summarizes parallel tool batches.
  Standalone; `hooks/hooks.json` doesn't call it either.

None of these are invoked by any existing hook, skill, or script —
capabilities on the shelf, not switched on. Wiring them in is a
separate call, made deliberately (see below).

## Deliberately not adopted, and why

Everything else in PR #2 is welded to the full `26.7.29` rewrite —
pulling any of it in piecemeal breaks CI/tests or makes this repo
claim things about itself that aren't true:

- `plugins/chatman-ecosystem/scripts/validate-claude-projection.py`
  and `.github/workflows/chatman-ecosystem.yml` — the validator
  hard-requires `plugin.json` version `26.7.29`,
  `defaultEnabled: false`, no `lspServers`, no `.lsp.json`, every
  agent declaring `effort`/`maxTurns` and denying
  `Write`/`Edit`/`NotebookEdit` (except `source-manufacturer`,
  isolated in a worktree), specific new hook events, and
  `actuation-intent.py`/`event-summary.py` wired into `hooks.json`.
  Landing the workflow alone ships a CI job that fails on every push.
- `plugins/chatman-ecosystem/tests/test_claude_projection.py` —
  asserts the above validator passes; same trap.
- `plugins/chatman-ecosystem/profiles/claude-projection.json` and
  `profiles/artifact-ownership.json` — both self-declare
  `"release": "26.7.29"` and describe file ownership for the
  rewritten layout. Landing them as-is means this repo claims a
  release state that isn't real.
- `docs/architecture/claude-projection.md`,
  `docs/migration/v26.7.29.md`, `docs/releases/v26.7.29.md`,
  `docs/verification/v26.7.29-claude-projection.md` — same problem;
  they describe `26.7.29` as shipped/verified.
- All modifications to existing tracked files —
  `.claude-plugin/marketplace.json`,
  `plugins/chatman-ecosystem/.claude-plugin/plugin.json`,
  `README.md`, all 8 `agents/*.md`, all 13 `skills/*/SKILL.md`,
  `hooks/hooks.json`, `monitors/monitors.json`, the existing
  `ontology/*.ttl` files,
  `profiles/{config-schema-epoch,phase-space,self-hosting,work-surfaces}.json`,
  and `scripts/generated-guard.py` — left alone, per "do not
  overwrite." The PR's ideas there (source-manufacturer as the sole
  editor, worktree isolation, treating hook events as intent
  candidates rather than admitted truth, recursive CMCA) hold up —
  worth weighing later — but pulling them in means rewriting files
  this session's live receipt chain and phase-vector work lean on. A
  deliberate call for later, not something to fold in quietly now.
- `plugins/chatman-ecosystem/.lsp.json` removal — a deletion, not an
  addition; left in place.

## If the full rewrite is wanted later

PR #2 itself is still open (draft) at
`https://github.com/seanchatmangpt/ferroplan/pull/2` and can be
reviewed and merged as a coordinated whole — the only way its
coupled pieces (validator, CI, tests, hooks, agent restrictions)
hold together.
</content>
