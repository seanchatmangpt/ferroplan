# ferroplan (Python)

Python bindings for the [ferroplan](https://github.com/seanchatmangpt/ferroplan/tree/main/crates/ferroplan) PDDL planner —
the Rust engine reached across the FFI wall via [pyo3](https://pyo3.rs) +
[maturin](https://maturin.rs). One portable **abi3** wheel, built once, runs on
CPython 3.8 and every version after it without a rebuild.

## Install / build

```
pip install maturin
cd crates/ferroplan-py
maturin develop --release          # build + install into the active venv
# or: maturin build --release       # produce a wheel in target/wheels/
```

On a Python newer than your pyo3 knows about, prefix with
`PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1`.

## Use

```python
import ferroplan, json

domain  = open("domain.pddl").read()
problem = open("problem.pddl").read()

sol = json.loads(ferroplan.plan(domain, problem))   # mode="auto", threads=auto
if sol["solved"]:
    print(sol["plan"]["length"], "steps,", sol["statistics"]["evaluated_states"], "states")
    for step in sol["plan"]["steps"]:
        print(step["index"], step["action"], *step["args"])

print("ferroplan", ferroplan.version())
```

`plan(domain, problem, mode=None, threads=None)` returns a JSON string of the
`Solution` (or `{"error": "..."}`). `mode` ∈ `auto | ff | pddl3 | partition |
temporal` (`auto` routes durative-action problems to the temporal solver).

`plan_production(domain, problem, mode=None, threads=None, max_evaluated=None,
request_id=None, max_domain_bytes=None, max_problem_bytes=None,
max_plan_steps=None, max_output_bytes=None, max_workers=None)` runs the bounded
candidate-only production envelope and returns a JSON string carrying the
outcome class — new service integrations should use this rather than `plan`.
`readiness()` returns the capability-manifest contract report
(`schema_version: "ferroplan.readiness-contract.v1"`, manifest fingerprint,
product version) as a JSON string.
