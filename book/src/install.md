# Install & quick start

Both crates sit on crates.io, one command deep:

```sh
cargo install ferroplan-cli    # -> the `ff` binary
```

Or pull from source and build it yourself:

```sh
git clone https://github.com/seanchatmangpt/ferroplan
cd ferroplan
cargo build --release      # -> target/release/ff
```

Point it at a problem, get a plan:

```sh
ff -o domain.pddl -f problem.pddl
```

Or skip the binary — `cargo add ferroplan`, then call it direct:

```rust,no_run
let domain  = std::fs::read_to_string("domain.pddl").unwrap();
let problem = std::fs::read_to_string("problem.pddl").unwrap();
let sol = ferroplan::solve(&domain, &problem, &ferroplan::Options::default()).unwrap();
println!("{:?}", sol.plan.map(|p| p.length));
```
