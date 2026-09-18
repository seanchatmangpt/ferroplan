# Non-deterministic hierarchical planning (FOND-HTN)

Classic planners assume the world obeys. FOND planners assume it sometimes
doesn't: an action fires and the world lands in one of several outcomes, and
you don't get to choose which. Hierarchical planners assume purpose arrives
decomposed: an abstract task is refined by methods into subtasks, recursively,
until only executable primitives remain.

ferroplan accepts problems that have both at once — fully observable
non-deterministic (FOND) planning written in a subset of HDDL — grounds and
translates them into its flat FOND model, and solves them with strong and
strong-cyclic policy fixpoints. This chapter is the narrative tour; the
reference document `docs/FOND-HTN.md` in the repository carries the full
language surface, semantics tables, and deviation ledger, with the same facts.

## The problem: branching outcomes under a hierarchy

The dialect is HDDL (Höller et al., IJCAI 2020): typed parameters and objects,
abstract `:task` declarations, primitive `:action`s, and `:method`s that
decompose a task into a totally or partially ordered subtask network. On top
of that, ferroplan admits one non-deterministic construct, in exactly one
position: `oneof` as the *entire* top-level effect of an action.

```text
(:action reboot
  :precondition (machine-down)
  :effect (oneof (and (not (machine-down)) (machine-up))
                 ()))               ; or nothing happens
```

The restriction is a design stance, not a limitation of ambition. Every
out-of-scope shape is refused *loudly at the correct pipeline stage* rather
than silently dropped or mis-parsed:

- nested `oneof`, `oneof` under a `when`, or `oneof` in any goal-description
  position — hard, typed parse errors;
- `:durative-action` and `:functions` — refused at parse (temporal/numeric
  planning is a permanent non-goal here);
- `:constraints`, numeric fluents, `increase`/`decrease` — parsed into real
  AST nodes, then refused the moment grounding starts;
- `or`, `imply`, and quantifiers inside `when`-effect conditions — refused.

One deliberate deviation is worth naming. A reference implementation in the
wild *parses* `when` inside a `oneof` branch and then silently drops the
conditional effect downstream, manufacturing a silently-weaker domain.
ferroplan refuses the same shape with a typed error: a domain it cannot solve
honestly is refused, never mis-solved. A weighted `(:probabilistic w1 e1 ...)`
extension exists as pure syntactic sugar — a text pre-pass rewrites it into a
standard `oneof` plus a side-channel weight map before tokenizing.

The genuinely subtle part is not the branching, it is where branching meets
the hierarchy. A policy for a FOND-HTN problem is not defined over bare
states; it is defined over pairs *(remaining task network, state)* — the
policy must prescribe a move for every pair it can reach, and method
commitment interleaves with outcome resolution: committing to a decomposition
before its non-deterministic outcomes are known is exactly what makes
flexible hierarchical FOND distinct from flattening the hierarchy first.
That formalization is due to Chen and Bercher (AAAI 2021); ferroplan
realizes the pair concretely as the *composite state*: (ground-fact-set,
task-network frontier).

## Strong vs strong-cyclic, and the fairness intuition

Two solution concepts dominate FOND planning (Cimatti, Pistore, Roveri,
Traverso — AIJ 2003):

**Strong.** The goal is reached in a bounded number of steps under *every*
possible resolution of every non-deterministic choice. Winning states are
grown as a least fixpoint from the goal states: repeatedly admit any state
from which some action exists whose outcomes *all* land in the winning set.
No fairness assumption is needed. The cost: cyclic solution structure cannot
be expressed — a state whose action self-loops would need itself already in
the set at admission time.

**Strong-cyclic.** The goal is reached with probability 1 in the limit:
every *fair* execution eventually reaches the goal, possibly after
unboundedly many retries. Winning states are computed as a greatest fixpoint
*within* the weakly-reachable set: optimistically keep every state from
which the goal is reachable under some lucky run, then repeatedly prune any
non-goal state with no action whose outcomes all still survive. Because the
set starts full rather than empty, a self-loop survives as long as its state
does — exactly what admits retry loops a least-fixpoint construction cannot.

The fairness intuition is a coin: flip it infinitely often and it cannot come
up tails forever. Every outcome that stays reachable infinitely often
eventually occurs, so a policy that always retries the branching action
cannot lose eternally — but it promises *eventual* success, never a bounded
step count.

Every strong solution is also a strong-cyclic solution; the converse fails
precisely on domains whose only solutions contain retry loops. ferroplan's
dispatch exploits the inclusion: the `Fond` planning type tries
`fond_policy` (strong) first and falls back to `fond_policy_strong_cyclic`
only on `NoPlan`, so domains the strong fixpoint already solves are
unchanged and the strong-cyclic solver strictly extends coverage.

## The pipeline

Five stages, each owning its refusals:

| Stage | Where | What happens |
|---|---|---|
| probabilistic pre-pass | `crates/ferroplan-hddl/src/probabilistic.rs` | `(:probabilistic ...)` sugar rewritten to `oneof` + weight map, before tokenizing |
| parse + validate | `parser.rs`, `validate.rs` | HDDL text → typed AST; static checks (resolution, arity, duplicates) |
| ground | `grounder.rs` | Type closure, object indexing, substitution; `oneof`/`when` expand into grounded effect branches |
| translate | `translate.rs` | BFS over the reachable *composite* (fact-set, TN-frontier) space; decomposition-aware — only actions reachable via an actual decomposition become transitions; `oneof` outcomes become probability-weighted transitions; TN completion is the synthetic `htn:done` fact |
| solve | `solve_hddl` → `solve_planning_type` | Adapt into `PlanningProblem` (decomposition transitions get zero cost — bookkeeping, not world action), run the strong/strong-cyclic fixpoints |

Two structural facts carry the design. First, every phase after the parser
enforces its own internal state/iteration/wall-clock limits with loud typed
refusals, and `solve_hddl` adds a watchdog over the entire call keyed off
`limits.max_wall_ms` — the caller never waits past budget (a worker thread
cannot be forcibly killed, but it can be abandoned on deadline). Second, the
dependency direction is one-way: `ferroplan-hddl` has zero dependency on
`ferroplan`; the front-end stays a library you can use without the engine.

## A retry loop, worked

One action, two outcomes: reboot fixes the machine, or nothing happens.

```text
                  outcome 1: (and (not (down)) (up))
        ┌───────────────────────────────────────────►  up    (goal)
        │
  down ─┤
        │
        └───────────────────────────────────────────►  down  (no change)
                  outcome 2: () — retry
```

The entire policy is one entry: `down ↦ reboot`.

- **Strong says no.** Grow the winning set from `{up}`: the reboot action's
  outcomes are `{up, down}`, and `down` is not in the set — so `down` is
  never admitted. No bounded guarantee exists, and that is a theorem, not a
  search failure.
- **Strong-cyclic says yes.** Start optimistic with the weakly-reachable set
  `{down, up}` and prune: `down` has an action whose outcomes all survive,
  so it stays. The policy is closed, and under fairness the "nothing
  happened" outcome cannot repeat forever — the machine eventually comes up.

Note the extra conjunct a hierarchical problem adds: the goal is not merely
`(machine-up)` but `htn:done` *and* the domain goal facts — the task network
must be fully reduced to executed primitives with the goal holding on the
final facts, never on a half-decomposed state.

## War story: differential testing and FOUND_BUG_1

Policy fixpoints are easy to write and easier to get subtly wrong, so
ferroplan's FOND-HTN verdicts are differentially tested against an external
oracle — a third-party FOND-HTN planner used strictly as a test oracle. Its
code lives only under `/tmp`, is never copied into any repository, and
contributes *concepts* only. A flock-serialized runner invokes it (its own
tooling writes shared temp paths, so concurrent runs would corrupt each
other) and emits exactly one JSON verdict line: `status`, `mode`, `wall_s`,
artifact path. A `NOSOLUTION` verdict is a *successful* oracle run; only
harness failure exits non-zero. The protocol: solve with ferroplan, solve
with the oracle, compare verdict class and policy shape per corpus instance —
disagreement is a defect report, not noise. Recorded verdicts are committed
as golden JSON fixtures, treated as facts: they record what the oracle said,
not behavior to reproduce blindly. Known oracle quirks are documented and
expected — `Success probability: 4.0000` is a solved-leaf count, not a
probability, and a `flexible + NOSOLUTION` verdict is cross-checked against
a second mode before it is allowed to overrule ferroplan.

The sweep earned its keep immediately. Across the committed goldens, the
FOND-HTN harvest recorded 31 verdicts — 18 solved, 12 timeouts, zero
`NOSOLUTION` verdicts, plus one parse-stage negative case — over seven
domains at 90-second walls
(`crates/ferroplan/tests/fixtures/fond-htn/oracle-harvest-full-summary.md`;
timeouts are wall exhaustion, never unsolvability evidence). In that
negative case the oracle silently mis-compiled a domain referencing an
undeclared type — exactly the shape ferroplan refuses loudly — and the
deterministic HTN comparison covered 21 corpus instances with 14 strict
agreements, 6 admitted mismatches (each pinned by an `#[ignore]`d
demonstration test — translate-capacity walls and honest grounding refusals),
and 1 open verdict
(`crates/ferroplan/tests/fixtures/htn-oracle/RESULTS.md`).

The best catch, though, was ferroplan's own. A property test
(`crates/ferroplan/tests/fond_property.rs`) generates 320 randomized FOND
problems from a fixed seed (`0x5EED_2026_0917`) and checks the solver against
an independent reference: literal policy enumeration over every generated
instance. The sweep found that the strong-cyclic solver returned a bogus
"solved" policy on **79 of the 320 instances** — every one the same shape:

```text
         a0 (pure self-loop)
         ┌──────┐
         ▼      │
       ┌────────┐    a1, outcome 1    ┌───────┐
       │  s0    │ ──────────────────► │   g   │ goal
       └────────┘                     └───────┘
           │
           └──── a1, outcome 2 ──────► u   (unsafe)
```

The greatest-fixpoint prune kept `s0` alive through the closed `a0` self-loop
and returned a policy whose reachable region could never reach the goal —
closed under outcomes, yet winning under none of them. The repair is a third
phase in `fond_policy_strong_cyclic`: a *committable goal-reachability* prune,
alternating with the witness prune to a joint fixpoint. An edge counts only
when some action takes *all* its outcomes into the surviving region (so the
controller can commit to it) and at least one outcome lands closer to a goal;
states without such a path are pruned along with their choices. The
reproducer was promoted to an always-on guard, and the wave's final gate ran
695 tests with 0 failures.

The methodology paid twice: the oracle flagged where ferroplan was behind
(translate capacity), and the property sweep flagged where it was silently
wrong (FOUND_BUG_1). Both findings became permanent tripwires rather than
fixes-and-forgets.

## Where it runs

- **Library.** `crates/ferroplan-hddl` is the front-end (parse → ground →
  translate); `solve_hddl` in `crates/ferroplan/src/hddl.rs` is the one-call
  entry that adapts and solves. The result reports the `Fond` planning type —
  honestly naming the solver that ran; HDDL is a front-end, not an eighteenth
  paradigm.
- **WASM.** Three ops in `crates/ferroplan-wasm/src/wasi_abi.rs`:
  `hddl_solve` (HDDL text → plan JSON, with per-stage error codes
  `FP_PARSE` / `FP_HDDL_GROUND` / `FP_HDDL_TRANSLATE` / `FP_MODEL`),
  `htn_plan` (JSON problem, forces the deterministic `Hierarchical` planning
  type), and `fond_policy` (JSON problem, forces the `Fond` dispatch).
- **Eve.** The Genesis lifecycle carries an `EveStage::DecomposeHddl` stage
  between `ProjectGenesis` and `GovernUncertaintyPpddl`
  (`crates/ferroplan/src/eve.rs`), and the capability manifest admits
  `fp.core.hddl` and `fp.core.fond` entries.

One boundary applies everywhere: planner output is a *candidate policy*. It
carries no execution authority — actuation remains the exclusive business of
the BRCE boundary (see [Eve and the Genesis planning
lifecycle](./eve-genesis.md)).

## References

- Alessandro Cimatti, Marco Pistore, Marco Roveri, Paolo Traverso. *Weak,
  Strong, and Strong Cyclic Planning via Symbolic Model Checking.*
  Artificial Intelligence 147(1–2), 2003.
- Yuchen Chen, Daniel Bercher. *Fully Observable Nondeterministic
  Hierarchical Planning.* AAAI 2021.
- Daniel Höller, Gregor Behnke, Daniel Bercher, Susanne Biundo, Humbert
  Fiorino, Damien Pellier, Ronald Alford. *HDDL: An Extension to PDDL for
  Expressing Hierarchical Planning Tasks.* IJCAI 2020.
