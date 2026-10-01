# Workspace crates

GENERATED from `ontology/ferroplan-crates.ttl` — edit the ontology, not this page.

| crate | kind | published | what |
|---|---|---|---|
| [`ferroplan`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan) | library | yes | the library: engine + modes + `solve` / `decompose` / `Session` API |
| [`ferroplan-cli`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-cli) | binary | yes | the `ff` binary (clap + JSON) |
| [`ferroplan-mcp`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-mcp) | binary | yes | an MCP server exposing `solve` / `validate` / `decompose` over stdio — so an LLM agent can author PDDL and drive the planner |
| [`ferroplan-bevy`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-bevy) | binary | no | Bevy app: visualize, inspect & animate a domain+problem (`cargo run -p ferroplan-bevy [domain.pddl problem.pddl]`) |
| [`ferroplan-wasm`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-wasm) | binding | no | WebAssembly binding behind the client-side [browser demo](https://seanchatmangpt.github.io/ferroplan/demo/index.html) — `solve` a domain+problem entirely in-page |
| [`ferroplan-py`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-py) | binding | no | Python binding (`pip`-installable extension module) exposing `solve` for embedding in Python tools |
| [`ferroplan-hddl`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-hddl) | library | yes | HDDL front-end: parses typed HDDL domain/problem text with `oneof` effects, grounds it and translates it into the ground IR the FOND solver runs over |
| [`ferroplan-sat`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-sat) | library | yes | in-tree CDCL SAT solver, absorbed from varisat 0.2.2 and carried forward as ferroplan code |
| [`ferroplan-runtime`](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan-runtime) | library | no | runtime substrate: authority, budgets, circuit breakers and dispatch for hosting the planner (in-workspace, not published) |

