# Introduction

**ferroplan** runs cold and fast: a data-parallel [PDDL](https://en.wikipedia.org/wiki/Planning_Domain_Definition_Language)
planner in Rust, rebuilt from scratch out of the FF planner lineage — a
**deterministic planning core for the age of AI**.

The wager: the model authors, the planner executes. You don't hand a language model a
column of numbers and ask it to add them — you make it emit code that does the
arithmetic, deterministic and free. Same move, one level up. Run a village of agents'
decisions through an LLM every tick and you pay for it — expensive, non-reproducible,
unbounded. Instead the model **authors a PDDL domain**. The domain plans itself:
deterministic, cheap, inspectable, at scale. The model only nudges it at runtime.
PDDL sits as the auditable interface — intent in, authored domain, fast solver out.
A domain and an axiom, you can read. A model's weights, you can't.

It combines:

- a **delete-relaxation FF heuristic** over a data-oriented task representation
  (bitset states, structure-of-arrays / CSR operator tables);
- **data parallelism** — parallel grounding and parallel batch heuristic
  evaluation, with bit-for-bit identical plans regardless of thread count;
- **ADL** (conditional effects, `forall`/`exists`, equality) and **numeric
  fluents**;
- **derived predicates / axioms** (`:derived`, static/stratified);
- **PDDL3 preferences** with anytime branch-and-bound metric optimization,
  and **PDDL3 trajectory constraints** (`(:constraints ...)`) — the six
  untimed modal operators enforced via monitor-automaton compilation,
  hard and soft alike;
- **PDDL2.1 temporal** planning — durative actions with constant,
  parameter-dependent, or state-dependent durations, timed initial
  literals, and required concurrency (see
  [Temporal planning](./temporal.md));
- a **game-embedding `Session`** — ground a world once, then run a whole
  population of minds in it: bounded deterministic thinks, free
  plan-validity replays, retargetable goals, cheap forks, scheduled
  events, in-flight intervals (see
  [Game embedding](./session.md));
- an optional **SGPlan-style partition-and-resolve** mode, and a
  budget-aware sequential **portfolio** mode (`--mode portfolio`).

Ships two ways: a Rust **library** with a structured, JSON-serializable API, and the
**`ff`** command-line binary — a drop-in for Metric-FF.

## Acknowledgments

Every planner here traces a lineage. Above all **SGPlan** (Chih-Wei Hsu and Benjamin
W. Wah, University of Illinois) — nearly two decades setting the standard in
satisficing planning with preferences and temporal/resource constraints. Landing
even *close* on a slice of the benchmarks is an honor, earned against a team's
long-running research. Jörg Hoffmann's **FF / Metric-FF** supplied the backbone:
the relaxed-plan heuristic, enforced hill-climbing. **VAL** (Derek Long & Maria Fox)
stands watch on temporal-plan validation, independent of this engine.
