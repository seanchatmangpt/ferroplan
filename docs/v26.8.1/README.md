# Ferroplan v26.8.1 — Full Planning Under Uncertainty

This implementation branch turns the integrated PPDDL rail into the first
bounded full-planning product surface.

## Delivered in source

- explicit deterministic/probabilistic rail dispatch;
- ggen-owned enum contracts, SHACL gates, SPARQL queries, Tera templates, and a
  generated capability matrix;
- hard goal-probability, unsafe-reachability, reward, and cost constraints;
- structural plus quantitative policy verification;
- observable-state `PolicySession` with decide, observe, advance, goal retarget,
  objective retarget, bounded replan, fork, status, and close;
- deterministic policy explanations;
- BLAKE3 policy receipts and predecessor-chain verification;
- `ferroplan ppddl` CLI commands for parse, solve, validate, simulate, explain,
  and JSON-lines policy sessions;
- fourteen PPDDL/full-planning MCP tools merged into the existing server, with
  ontology-sourced resources and generated input schemas;
- repository-uncertainty and TCPS application-pipeline reference domains;
- positive and negative fixtures for constraints, observation, reference-domain
  simulation, session transitions, and receipt tampering.

## Preserved fences

- deterministic PDDL and probabilistic PPDDL remain separate semantic rails;
- planning never actuates;
- ambiguous observation is refused rather than replaced by a likely state;
- a successful verifier report is capped at `PARTIAL_ALIVE` with `NO_REPLAY`;
- the example probabilities are fixtures, not empirical claims;
- release promotion still requires exact-head execution and independent replay.

## Open release checkpoints

LAO*-class partial-graph search, deterministic relaxation, exact-rational
verification, Python/WASM parity, IPC-4 standings, comparative reports, full
normalized-MDP receipt identity, and independent exact-head replay remain
explicit checkpoints. They are not represented as complete by this branch.
