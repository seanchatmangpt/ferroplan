# Command line (`ff`)

Same call signature, different engine. The `ff` binary drops in for
Metric-FF's `ff -o domain -f problem`.

| flag | meaning |
|---|---|
| `-o, --domain <FILE>` | PDDL domain |
| `-f, --problem <FILE>` | PDDL problem |
| `--json` | emit a structured JSON `Solution` instead of classic text |
| `--json-request <FILE>` | self-contained `{domain, problem, options}` JSON job (`-` = stdin) |
| `--mode <auto\|ff\|partition\|pddl3\|temporal\|portfolio\|optimal\|sat>` | planning strategy (default `auto`) |
| `--search <auto\|ehc\|best-first\|ehc-then-best-first>` | search strategy |
| `--weight-g <W>` / `--weight-h <W>` | best-first path-length / heuristic weights |
| `--max-evaluated <N>` | cap on nodes evaluated before giving up |
| `--satisfice` | PDDL3: stop at the first plan instead of optimizing the metric |
| `--decompose` | split a too-big temporal goal into ordered contracts and stitch them |
| `--validate <PLAN>` | replay a plan file under ferroplan's own apply semantics (exit 0/1) |
| `--threads <N>` | worker threads (`0` = auto) |
| `--ipc` | IPC time-stamped plan format (text mode) |
| `--readiness` | emit the canonical capability manifest and its deterministic fingerprint — a contract report, not a self-authored production-admission verdict |
| `--production` | execute through the bounded candidate-only production envelope: always JSON, stable exit classes |
| `--request-id <ID>` | optional request/correlation ID for `--production` |
| `--max-domain-bytes <B>` / `--max-problem-bytes <B>` | production input caps (default: the library's `ProductionLimits::default()`) |
| `--max-plan-steps <N>` | production cap on emitted plan steps |
| `--max-output-bytes <B>` | production cap on serialized solution bytes |
| `--max-workers <N>` | production cap on worker count |

`--production` exits with stable classes: `0` solved; `2` refused (parse /
model / invalid-request / input-limit); `3` no plan; `4` unsupported
(`FP_UNSUPPORTED`); `5` limit exceeded; `7` refused (other); `70` failed.
Independently of these, every file read is hard-capped at 64 MiB and a
`--json-request` job at 16 MiB.


`auto` reads the problem and picks its own weapon: classic FF for
classical/numeric, the PDDL3 metric optimizer when preferences are on the
table, the decision-epoch temporal search when the domain carries
`:durative-action`s. `partition` forces the SGPlan-style partition-and-resolve
mode; `portfolio` runs the budget-aware sequential strategy ladder on one
shared eval pool. `ff --help` has the rest.

```sh
ff -o domain.pddl -f problem.pddl --json
ff -o temporal-domain.pddl -f problem.pddl --mode temporal --decompose
ff -o domain.pddl -f problem.pddl --validate plan.txt
echo '{"domain":"...","problem":"...","options":{"mode":"ff"}}' | ff --json-request -
```
