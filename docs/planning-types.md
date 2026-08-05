# Ferroplan planning types

Ferroplan uses one typed planning constitution so RDF, ggen, MCP, A2A, and
human-facing interfaces do not reinterpret the word `planning` independently.

The registry is implemented in `crates/ferroplan/src/planning_types.rs`.
Routing proves required semantic capabilities, authority, verifier, and receipt
closure. A successful route is a selection result only; it never grants
actuation authority.

| Planning type | Rail | Current claim |
|---|---|---|
| classical | native deterministic | Native Ferroplan solve path |
| cost optimal | native deterministic | Native cost/optimal search path |
| numeric | native deterministic | Native numeric fluent path |
| temporal | native deterministic | Native PDDL2.1 path |
| preferences | native deterministic | Native PDDL3 path |
| probabilistic | native probabilistic | Native PPDDL policy path |
| partial order | composed | Compose existing contracts and validators |
| workflow | composed | Compose tasks with receipt joins |
| flow constrained | policy overlay | Enforce queue and WIP bounds over a policy |
| resolution adaptive | policy overlay | Decompose until primitive closure is proven |
| RDF derived | graph projection | Manufacture a bounded planner capsule from an admitted graph |
| A2A delegated | delegation | Assign a compound task to a capability-bearing peer |
| MCP bound | capability binding | Bind primitive tasks to authorized local tools |
| FOND | external planner | Typed handoff; no native-solver claim |
| conformant | external planner | Typed handoff; no native-solver claim |
| contingent | external planner | Typed handoff; no native-solver claim |
| hierarchical | external planner | HDDL/HTN handoff; no native-solver claim |
| multi-agent | external planner | Coordination handoff; no native-solver claim |

## Admission law

A request is routable only when all four boundaries close:

```text
required semantic capabilities
+ bounded authority
+ independent verifier
+ receipt obligation
```

Otherwise Ferroplan emits a typed refusal:

- `REFUSED:EMPTY_SUBJECT`
- `REFUSED:MISSING_PLANNING_CAPABILITIES`
- `REFUSED:AUTHORITY_UNBOUND`
- `REFUSED:VERIFIER_UNBOUND`
- `REFUSED:RECEIPT_UNBOUND`

## Architectural correspondence

```text
RDF/SPARQL  -> rdf_derived
HDDL/HTN    -> hierarchical
PPDDL       -> probabilistic
POWL        -> partial_order / workflow
Little law  -> flow_constrained
MFW         -> resolution_adaptive
A2A         -> a2a_delegated
MCP         -> mcp_bound
BRCE        -> outside this module; exclusive DO path
Truex       -> verifier/receipt/consequence lifecycle
```

The registry preserves the distinction between a paradigm being represented,
routed, natively solved, externally delegated, and actually executed.
