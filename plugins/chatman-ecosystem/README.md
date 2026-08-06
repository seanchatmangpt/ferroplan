# Chatman Phase Engine

A phase-changing Claude Code plugin. Ferroplan is its first managed world —
the territory it watches, allocates, plans, and rebuilds, one receipted step
at a time.

It composes:

- current Claude Code loader validation;
- `claude-code-config-lsp` diagnostics, completion, semantic tokens, and Declare conformance;
- RDF/PROV/SHACL-shaped repository observation;
- the Chatman Multifractal Cascade Allocator (CMCA);
- stateless and persistent Ferroplan planning;
- reversible manufacturing agents;
- independent validation;
- protected hooks, monitors, and canonical BLAKE3 receipt chains.

## Design law

The plugin follows **design for combinatorial maximalism**. There is no
single fixed workflow baked into the machine — only orthogonal primitives,
and the laws that govern how they may be wired together.

The live operating state is a product of six dimensions:

| Dimension | States |
|---|---|
| Epistemic | latent, observed, admitted |
| Allocation | unallocated, allocated |
| Planning | unplanned, candidate, validated |
| Actuation | sealed, manufacturing, receipted, publishable |
| Drift | stable, drifted, refused |
| Configuration | unknown, nonconformant, conformant |

Six dimensions fan out to 648 raw combinations. Only **136 (21.0%) are
lawful** — `profiles/phase-space.json` declares the transitions and
invariants that admit those and refuse the rest. Both numbers are computed
at runtime by `phase.py status` (see the `census` field), never hardcoded,
so the day an invariant stops pulling weight, the lawful count itself moves
and says so. Publication sits at the far edge of the space: exactly one
lawful vector reaches `publishable`.

The active agents, skills, and capabilities at any moment are the set union
tied to whichever vector the world currently occupies.

A repository mutation is a small earthquake. It collapses the affected
dimensions straight back to:

```text
observed × unallocated × unplanned × sealed × drifted × unknown
```

Nothing climbs back to an advanced phase without a receipt to show for it.

## Authority graph

| Component | Maximum claim |
|---|---|
| Claude | model authoring and supervision |
| Claude Code loader | plugin load/install conformance |
| claude-code-config-lsp | conformance for its modeled schema epoch |
| RDF observer | bounded semantic projection |
| CMCA | bounded allocation |
| Ferroplan | deterministic candidate plan and suffix validity |
| Source manufacturer | reversible source construction |
| Independent validator | exercised validation result |
| Admission MCP | canonical evidence envelope |
| Hooks | observation and protected-command refusal |
| Receipt auditor | replay and maximum lawful standing |

These ceilings are read, not enforced by machinery — a review discipline,
not a runtime lock. `phase.py` computes the set union of capabilities,
agents, and skills across the selected dimension states; nothing in that
projection checks the result against the table above, because no ordering
over these claim values exists anywhere in code. Agents and skills are
instructed to respect the ceilings, an auditor is expected to flag
violations — but a composition that overstepped one would still load, and
still run.

## Installation

From Claude Code:

```text
/plugin marketplace add seanchatmangpt/ferroplan
/plugin install chatman-ecosystem@chatman-ecosystem --scope project
```

The repository also declares the marketplace and plugin in `.claude/settings.json`, so trusted project sessions can enable the plugin at project scope.

At enable time, the plugin can accept optional checkout locations for:

- `claude-code-config-lsp`;

If omitted, resolvers first reach for an installed binary, then a sibling
checkout beside Ferroplan. They never phone the network on their own.

## Main skills

| Skill | Purpose |
|---|---|
| `/chatman-ecosystem:self-host` | Run the complete dogfooding loop |
| `/chatman-ecosystem:phase-change` | Inspect or advance the product-state vector |
| `/chatman-ecosystem:compose` | Manufacture a new capability from existing primitives |
| `/chatman-ecosystem:configure` | Federate loader and config-LSP conformance |
| `/chatman-ecosystem:observe` | Build the RDF-shaped repository world |
| `/chatman-ecosystem:allocate` | Run CMCA and bind allocation evidence |
| `/chatman-ecosystem:plan` | Retain or replan a persistent Ferroplan Session |
| `/chatman-ecosystem:manufacture` | Execute one reversible plan step |
| `/chatman-ecosystem:validate` | Independently exercise the changed surface |
| `/chatman-ecosystem:admit` | Bind canonical plan and validator evidence |
| `/chatman-ecosystem:audit` | Replay receipts and determine standing |
| `/chatman-ecosystem:doctor` | Diagnose every plugin surface |
| `/chatman-ecosystem:publish` | Explicitly perform protected publication |

`publish` is the one door the model can never open by itself.

## MCP servers

The plugin starts two independent stdio authorities:

- `ferroplan`: stateless parse/solve/validate/decompose, persistent `Session`
  (observation, suffix replay, bounded replanning, CMCA), and Chatman
  admission (canonical digest, allocation envelope, plan envelope, receipt
  verification) — all 16 tools in one process, one `rmcp` server;

## Live self-hosting world

The repository world is represented by:

- `world/ferroplan-self-host-domain.pddl`;
- `scripts/project-world.py`;
- the hook ledger;
- the current phase vector.

Generate a live problem:

```sh
python3 "$CLAUDE_PLUGIN_ROOT/scripts/project-world.py" \
  --project "$CLAUDE_PROJECT_DIR" \
  --goal receipt \
  --output /tmp/ferroplan-live.pddl \
  --metadata /tmp/ferroplan-live.json
```

Feed it the same ledger and the same phase state twice, and it hands back
the same problem twice — Ferroplan plans against the repository's actual
observed standing, never a static fixture wearing the world's clothes.

## Configuration schema epochs

`claude-code-config-lsp` is ontology-generated and genuinely useful, but the
ontology it carries models an earlier Claude Code plugin schema — a map
drawn before the territory finished moving. `profiles/config-schema-epoch.json`
records the known gap, including:

- optional commit-SHA plugin versions;
- object marketplace sources;
- plugin dependencies;
- experimental monitors;
- user configuration;
- expanded hook types;
- plugin-root agent and skill locations.

The current Claude loader and `claude plugin validate` govern loadability.
The LSP governs only the surfaces its ontology actually reaches. A known
epoch gap can never masquerade as a false refusal. An unknown disagreement
stays `UNKNOWN` until someone reconciles it — it doesn't get rounded up or
down.

## Receipt chain

Allocation and plan envelopes bind canonical forms of:

- observation frontier;
- eight CMCA candidates;
- CMCA result and BCINR revision;
- PDDL domain and problem commitments through the Session receipt;
- candidate plan;
- independent validator result;
- predecessor receipt.

The admission server uses recursively key-sorted JSON, length-framed inputs,
and BLAKE3. Verification recomputes both the payload digest and the receipt
— trust nothing that wasn't rebuilt from the raw bytes.

## Protected actuation

Hooks deny protected Bash operations the moment repository observations run
ahead of the admitted receipt frontier. Protected surfaces include
publication, destructive git operations, package publishing, recursive
forced deletion, and state-changing HTTP requests.

A source change is only ever allowed as reversible manufacturing. The
instant it lands, it becomes a new observation, and it seals advanced
actuation shut until the loop closes around it again.

## Standing

- `ALIVE`: exact runtime and replay evidence establishes the complete stated claim.
- `PARTIAL_ALIVE`: a bounded subset is evidenced and the remaining obligations are named.
- `BUILD_BROKEN`: an exercised build, validation, or execution surface failed.
- `UNKNOWN`: the required executor or evidence was unavailable.

Source presence, plans, confidence, and prose buy none of these on their
own.

## Development check

Run the plugin doctor inside Claude Code:

```text
/chatman-ecosystem:doctor
```

The doctor checks loader validation, LSP resolution, Python syntax, shell
resolvers, Rust binaries, MCP startup, live PDDL projection, phase
invariants, and receipt replay.
