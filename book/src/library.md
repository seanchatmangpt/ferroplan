# Library API

No strings to parse, no ad-hoc output. The library hands back **typed,
`serde`-serializable** structures. Every knob lives on one `Options` struct; every
field is optional via `Default`.

```rust,no_run
use ferroplan::{solve, Mode, Options};

let domain = std::fs::read_to_string("domain.pddl").unwrap();
let problem = std::fs::read_to_string("problem.pddl").unwrap();

let opts = Options { mode: Mode::Auto, ..Default::default() };
let sol = solve(&domain, &problem, &opts).unwrap();

if let Some(plan) = sol.plan {
    for step in &plan.steps {
        println!("{} {}", step.action, step.args.join(" "));
    }
    println!("metric: {:?}, makespan: {:?}", plan.metric, plan.makespan);
}
```

## The public surface

- **`solve(domain, problem, &Options)`** → `Result<Solution, SolveError>` — feed it
  a domain and a problem, get a plan back. `Mode::Auto` routes by features:
  temporal goes to decision-epoch, preferences to the PDDL3 metric optimizer,
  otherwise classical FF.
- **`parse(src)`** → `ParseReport` — a syntax check and summary of a domain *or*
  problem, no grounding, no solving. Fast feedback while you're authoring.
- **`decompose(domain, problem, &Options)`** → `Result<Decomposition, SolveError>`
  — a temporal goal too big to swallow whole gets split into ordered,
  individually-solved contracts, stitched back into one validated plan. Falls back
  to a monolithic solve if the goal won't split. See [`examples/decompose.rs`](https://github.com/seanchatmangpt/ferroplan/blob/main/crates/ferroplan/examples/decompose.rs).
- **`Session::new(domain, problem, &Options)`** — ground the world once, then think
  as many times as the world changes: classical *and* temporal domains, bounded
  deterministic thinks, free plan-validity replays, retargetable goals, cheap
  population forks, scheduled events, in-flight intervals. The whole
  game-embedding surface gets [its own chapter](./session.md); see also
  [`examples/game_think.rs`](https://github.com/seanchatmangpt/ferroplan/blob/main/crates/ferroplan/examples/game_think.rs)
  and [`examples/bazaar_live.rs`](https://github.com/seanchatmangpt/ferroplan/blob/main/crates/ferroplan/examples/bazaar_live.rs).
- **`plan::validate_plan(&domain, &problem, &plan)`** — an independent replay,
  ferroplan checking its own work under its own apply semantics. See
  [`examples/validate_plan.rs`](https://github.com/seanchatmangpt/ferroplan/blob/main/crates/ferroplan/examples/validate_plan.rs).

## Key types

- `Solution { solved, mode, plan: Option<Plan>, statistics, notes }`
- `Plan { steps: Vec<Step>, length, metric: Option<f64>, makespan: Option<f64> }`
- `Step { index, action, args, time, duration }` — `time`/`duration` are set on
  temporal plans.
- `SolveError` — `DomainParse` / `ProblemParse` / `EmptyType` / `Derived` /
  `Unsupported`, via `thiserror`.

Everything serializes to JSON. `solve` doubles as the core of a planning service
with no extra wiring. See [`examples/json_api.rs`](https://github.com/seanchatmangpt/ferroplan/blob/main/crates/ferroplan/examples/json_api.rs).
