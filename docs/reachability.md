# Reachability Engine — `reachability.rs`

Source: `crates/ferroplan/src/reachability.rs` (204 lines).

## Theorem (dissertation Ch3, Theorem 3.1)

The module compiles a **safe set** over a finite state universe `X`:

```text
S_safe = X \ ∪_k Reach⁻ᵏ(S_prohibited)
```

A state is safe exactly when it cannot reach a prohibited state within `k`
steps, where `k = max_depth` is the compiled bound. This is the planner-side
analog of CASTLE's Γ_prohibited gating-mandate construction
(`reachability.rs:1-8`).

## Algorithm

Bounded backward BFS over predecessors:

1. **Invert the transition relation** — `from_successors` (`reachability.rs:36`)
   builds `preds[t]` = list of states with an edge `s → t`
   (`reachability.rs:41-47`), then delegates to `from_predecessors`.
2. **Seed at depth 0** — every prohibited state id is marked unsafe and
   pushed onto the frontier (`reachability.rs:65-78`).
3. **Level-by-level pull-in** — while `depth_reached < max_depth` and the
   frontier is non-empty, each level marks every unmarked predecessor of the
   current frontier unsafe (`reachability.rs:81-97`). An empty `next` frontier
   sets `saturated = true` and stops (`reachability.rs:91-94`).
4. **Post-loop saturation check** — if the frontier emptied, `saturated`
   (`reachability.rs:98-100`).

`max_depth` bounds termination; the `saturated` flag distinguishes "exhausted
the graph" from "stopped at the bound" (`reachability.rs:10-12`).

## API

All in `crates/ferroplan/src/reachability.rs`. Uses the crate's shared bitset
helpers `crate::bitset::{count, set, test}` (`reachability.rs:14`).

### `BackwardSafeSet` (`reachability.rs:19-30`)

`#[derive(Debug, Clone, PartialEq, Eq)]` struct with private fields
`n_states: usize`, `unsafe_words: Vec<u64>` (bit set = state UNSAFE, i.e. can
reach a prohibited state within the compiled depth), `depth_reached: u32`,
`saturated: bool`.

### Constructors

```rust
pub fn from_successors(
    successors: &[Vec<u32>],
    prohibited: &[u32],
    max_depth: u32,
) -> Self            // reachability.rs:36-49

pub fn from_predecessors(
    predecessors: &[Vec<u32>],
    prohibited: &[u32],
    max_depth: u32,
) -> Self            // reachability.rs:54-108
```

- `successors[s]` lists the states one step from `s`; `prohibited` lists
  prohibited state ids (each compiled at depth 0); `max_depth` bounds the BFS.
- `from_predecessors` is "the direct shape of the dissertation recursion"
  (`reachability.rs:51-53`).

### Queries

```rust
pub fn is_safe(&self, state: u32) -> bool       // reachability.rs:113-115
pub fn unsafe_count(&self) -> usize             // reachability.rs:118-120
pub fn depth_reached(&self) -> u32              // reachability.rs:123-125
pub fn saturated(&self) -> bool                 // reachability.rs:128-130
```

- `is_safe(s)` = `s ∉ ∪_k Reach⁻ᵏ(S_prohibited)`; an out-of-range id is safe
  (not in the compiled universe).
- `unsafe_count` = number of excluded states; `depth_reached` = depth actually
  compiled (`max_depth` unless the frontier emptied early); `saturated` = the
  backward closure exhausted before the depth bound.

## Complexity & Saturation Semantics

- **Time**: O(|E| + |X|) worst case — each state is enqueued at most once
  (bitset membership test guards re-enqueue) and each edge is scanned once
  during relation inversion plus once during BFS. **Space**: O(|X|/64) words
  for the unsafe bitset plus O(|E|) for the predecessor lists.
- **Saturation**: `saturated() == true` iff the predecessor closure closed
  (empty `next` frontier) before `max_depth`; then `depth_reached` is the
  closure diameter, not the bound. `saturated() == false` means the result is
  depth-bounded: states beyond `depth_reached` backward steps may be unsafe
  but are not yet marked, so `is_safe` answers are valid only for the
  compiled depth `k = depth_reached`.
- Out-of-range ids and depth-0 seeds: an out-of-range id is safe; prohibited
  states are unsafe at depth 0 regardless of `max_depth` (including
  `max_depth = 0`).

## Behavioral Spec — the Module's Tests

Six tests (not seven — `grep -c '#\[test\]'` = 6) in `reachability.rs:137-203`.

### `linear_chain_backward_reach` (line 141)

Chain `A(0) → B(1) → C(2, prohibited)`; `D(3)` has no path to C.
Asserts C, B, A unsafe; D safe; `saturated()`; `unsafe_count() == 3`;
`depth_reached() == 2`.

### `depth_bound_stops_before_full_closure` (line 155)

`X → Y → Z_prohibited`, `max_depth = 1`: Y unsafe, X stays safe at k=1,
`!saturated()`, `depth_reached() == 1`.

### `cycle_saturates_without_hang` (line 166)

`A ↔ B`, `B → C(prohibited)`, bound 5: both A and B unsafe; terminates with
`saturated() == true` (the visited-bitset prevents infinite cycling).

### `disconnected_component_stays_safe` (line 176)

Component `{0,1,2}` flows into prohibited 2; disjoint component `{3,4}`:
states 0,1 unsafe; 3,4 safe.

### `prohibited_seed_only_depth_zero` (line 187)

`max_depth = 0`: only the prohibited state itself is unsafe, `depth_reached()
== 0`, and `!saturated()` ("seed frontier unexpanded at bound → not
saturated").

### `out_of_range_state_is_safe` (line 198)

`is_safe(99)` on a 3-state universe returns true.

## Mutant Anchor

`NOTE(mutant-anchor)` at `reachability.rs:72-76`: the predecessor pull-in is
the load-bearing step of Theorem 3.1. Dropping it (or a single edge of it)
shrinks `∪_k Reach⁻ᵏ(S_prohibited)` and wrongly declares a state that reaches
prohibition "safe". The doc comment at `reachability.rs:133-135` names the
killer: `linear_chain_backward_reach` fails (A and B come out "safe") if the
pull-in is dropped. This is an anti-vacuity anchor — the test suite can detect
that specific mutation.

## See Also

`docs/FOND-HTN.md` · `docs/planning-types.md`
