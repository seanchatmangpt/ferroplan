# Architecture

No objects, no indirection. ferroplan runs **data-oriented**: states are
bitsets of fact ids plus dense fluent vectors; operators sit column-wise
(CSR). The hot loops stream contiguous memory, parallel over immutable shared
task data.

The pipeline, four stages:

1. **Parse** (`parser`, `lexer`) — PDDL domain + problem to an AST.
2. **Ground** (`ground`) — parallel per-action binding enumeration, DNF of
   preconditions, ADL expansion (`forall`/`exists`/`when`), negative-precondition
   compilation, relaxed-reachability pruning, CSR packing.
3. **Search** (`search`, `heuristic`) — weighted best-first (`1·g + 5·h`) with a
   delete-relaxation relaxed-plan heuristic; deferred (lazy) heuristic
   evaluation; parallel batch evaluation with order-preserving determinism.
4. **Modes** — classic FF; SGPlan-style `partition`+`resolve`; PDDL3
   `pddl3` (Keyder–Geffner soft-goal compilation + anytime branch-and-bound);
   the decision-epoch `temporal` search (snap-action compilation, pending-end
   agenda, symmetry-reduced since 0.13); a budget-aware sequential
   `portfolio`; and the game-embedding [`Session`](./session.md) (ground
   once, think forever — with a fixpoint grounding path whose transient
   memory is up to ~117× smaller on sparse-reachable worlds).

Under the hood: an in-tree FxHash hasher, a compact relevant-only visited key,
size-gated parallelism — serial when the frontier is small, threads capped
when it isn't. Small problems stay fast. Large ones don't stall.
