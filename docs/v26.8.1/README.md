# Ferroplan v26.8.1 — Full Planning Under Uncertainty

This implementation branch turns the integrated PPDDL rail into the first
bounded full-planning product surface.

## Delivered in this change

- explicit deterministic/probabilistic rail dispatch;
- ggen-owned public contracts and generated capability matrix;
- hard goal-probability, unsafe-reachability, reward, and cost constraints;
- structural plus quantitative policy verification;
- observable-state `PolicySession` with decide, observe, advance, goal retarget,
  objective retarget, bounded replan, fork, status, and close;
- deterministic policy explanations;
- BLAKE3 policy receipts and predecessor-chain verification;
- positive and negative fixtures for closure, constraints, observation, and
  receipt tampering.

## Preserved fences

- deterministic PDDL and probabilistic PPDDL remain separate semantic rails;
- planning never actuates;
- ambiguous observation is refused rather than replaced by a likely state;
- a successful verifier report is capped at `PARTIAL_ALIVE` with `NO_REPLAY`;
- release promotion still requires exact-head execution and independent replay.

## Not yet promoted

LAO*, deterministic relaxation, exact rational verification, CLI/MCP/SDK parity,
IPC-4 standings, and the two end-to-end dogfood domains remain explicit
checkpoints. They are not represented as complete by this branch.
