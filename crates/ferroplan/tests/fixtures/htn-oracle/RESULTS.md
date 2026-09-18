# RESULTS — solve_hddl vs koala/pandaPI oracle on the deterministic HTN corpus

Ticket: `docs/jira/v26.9.17/fond-htn-05-diff-htn.md` (branch `test/htn-ipc-oracle`).

- Oracle: koala/pandaPI via `/tmp/fond-oracle/oracle-run.sh` (flock-serialized;
  rebuild recipe: `bash /tmp/fond-oracle/build.sh`). Mode `flexible`
  (`fixed-ld` cross-check on NOSOLUTION/ERROR); verdict rule: SOLVED if any
  mode solves, NOSOLUTION only if both agree. Goldens:
  `oracle-goldens.json` (verdicts + wall only).
- Ferroplan: `cargo test -p ferroplan --test htn_oracle
  measure_translation_walls_for_results_md -- --ignored --test-threads=1
  --nocapture` (serial, idle machine, debug build; `solve_hddl` wall budget
  60 000 ms).
- `translation_ms` = parse→ground→translate with the exact phases and limit
  defaults `solve_hddl_inner` uses internally. SLOW = translation > 10 000 ms.
- 21 instances: 13 IPC-2023 + 4 PANDA feature tests + 4 SHOP3 hand-ports.
  Agreement: 14/21 strict, 1/21 open-verdict class, 6/21 admitted mismatches
  (each with an `#[ignore]`d demonstration test in `htn_oracle.rs`).

| instance | source | oracle status (mode) | oracle wall_s | ferroplan status | translation_ms | total_ms | flag |
|---|---|---|---|---|---|---|---|
| Blocksworld-GTOHP | IPC-2023 | SOLVED (flexible) | 0.578 | SOLVED | 6 | 17 | |
| Blocksworld-HPDDL | IPC-2023 | SOLVED (flexible) | 0.524 | SOLVED | 13 | 99 | |
| Depots | IPC-2023 | SOLVED (flexible) | 0.401 | SOLVED | 112 | 1065 | |
| Factories-simple | IPC-2023 | SOLVED (flexible) | 0.324 | SOLVED | 22 | 74 | |
| Lamps | IPC-2023 | SOLVED (flexible) | 0.343 | SOLVED | 2 | 3 | |
| Multiarm-Blocksworld | IPC-2023 | SOLVED (flexible) | 1.237 | SOLVED | 43 | 1323 | |
| PCP_1 | IPC-2023 | SOLVED (flexible) | 0.288 | TRANSLATE_ERROR | 10230 | 10311 | SLOW; MISMATCH |
| PO_Satellite | IPC-2023 | SOLVED (flexible) | 0.382 | SOLVED | 4 | 3 | |
| PO_Transport | IPC-2023 | SOLVED (flexible) | 0.338 | TRANSLATE_ERROR | 10400 | 10438 | SLOW; MISMATCH |
| Robot | IPC-2023 | SOLVED (flexible) | 0.406 | SOLVED | 2 | 2 | |
| Satellite-GTOHP | IPC-2023 | SOLVED (flexible) | 1.229 | TRANSLATE_ERROR | 10430 | 10425 | SLOW; MISMATCH |
| Towers | IPC-2023 | SOLVED (flexible) | 0.451 | SOLVED | 9 | 9 | |
| Transport | IPC-2023 | SOLVED (flexible) | 0.327 | TRANSLATE_ERROR | 10601 | 10488 | SLOW; MISMATCH |
| panda-conditional-effect | PANDA | SOLVED (flexible) | 0.301 | GROUND_ERROR | 1 | 0 | MISMATCH |
| panda-empty | PANDA | SOLVED (fixed-ld; flexible NOSOLUTION quirk) | 0.290 | SOLVED | 0 | 0 | |
| panda-interleaving | PANDA | SOLVED (flexible) | 0.273 | SOLVED | 0 | 0 | |
| panda-method-effect | PANDA | SOLVED (flexible) | 0.284 | GROUND_ERROR | 0 | 0 | MISMATCH |
| shop3-port-ab-ordering | SHOP3 port | SOLVED (flexible) | 0.393 | SOLVED | 0 | 1 | |
| shop3-port-loan-credit | SHOP3 port | SOLVED (flexible) | 0.338 | SOLVED | 0 | 0 | |
| shop3-port-loan-noplan | SHOP3 port | ERROR (both modes — oracle harness) | 0.292 | NOSOLUTION | 0 | 0 | open verdict |
| shop3-port-swap | SHOP3 port | SOLVED (flexible) | 0.544 | SOLVED | 0 | 1 | |

For every strict agreement on a SOLVED golden, the gate test additionally
verifies the returned policy is outcome-closed against the translated ground
IR (determinism witness: every transition at full 1 000 000 ppm; policy
outcomes == IR transition targets; every policy-following execution from an
initial state reaches a goal state — no non-goal dead ends).

## Admitted mismatches (6) — oracle artifact paths

| instance | oracle artifact (koala policy) | ferroplan error | classification |
|---|---|---|---|
| PCP_1 | `/tmp/fond-oracle/runs/20260917T214802Z-domain-p-pcp01-31904/policy.txt` | `translate wall-clock limit exceeded: 10000ms elapsed, limit 10000ms` (~16 800 states still queued) | translate composite-BFS capacity, internal `TranslateLimits::default()` 10 s hardcoded in `solve_hddl`'s pipeline |
| PO_Transport | `/tmp/fond-oracle/runs/20260917T215151Z-domain-pfile01-33690/policy.txt` | same 10 s translate limit (~46 661 states queued) | translate capacity |
| Satellite-GTOHP | `/tmp/fond-oracle/runs/20260917T215216Z-domain-p01-39529/policy.txt` | same 10 s translate limit (~19 266 states queued) | translate capacity |
| Transport | `/tmp/fond-oracle/runs/20260917T215221Z-domain-pfile01-39770/policy.txt` | same 10 s translate limit (~43 373 states queued) | translate capacity |
| panda-conditional-effect | `/tmp/fond-oracle/runs/20260917T215222Z-domain-problem-39842/policy.txt` | `unsupported construct: nested 'when' is out of scope` | grounder honest refusal: action-level conditional effects unsupported (loud, no panic) |
| panda-method-effect | `/tmp/fond-oracle/runs/20260917T215229Z-domain-problem-40353/policy.txt` | `unbound variable '?x'` | grounder cannot ground method-`:effect`/`when` zero-action domains (loud, no panic) |

## Open-verdict note

`shop3-port-loan-noplan` (deliberately unsolvable): koala's own pipeline
ERRORs before search (`htn_serializer.py` `KeyError: -1` on the empty task
network) in BOTH modes, so the golden is the open ERROR class ("no panic,
bounded exit" only). Ferroplan returns a clean `PlannerError::NoPlan` —
pinned by the `#[ignore]`d tripwire
`loan_noplan_ferroplan_returns_clean_no_plan_where_oracle_errors`.
Oracle artifact: `/tmp/fond-oracle/runs/20260917T215232Z-domain-problem-40622/run.log`.

## Known oracle quirk exercised

`panda-empty`: koala `--flexible` reports "Problem has no solution" (success
probability 0.0000) while `--fixed-ld` returns the real empty-plan policy
(`Task __noop`, `Method ε`) — the wave-context "treat flexible+NOSOLUTION
carefully" quirk. Golden = SOLVED via fixed-ld; ferroplan agrees (SOLVED).

## Addendum (2026-09-17, ticket fond-htn-24, branch `fix/ground-conditional-effects`)

The two PANDA rows above are historical wave-3 records. Ticket fond-htn-24
implemented conditional-effect grounding (action-level `when`, one
`when`-in-`when` level flattened to a conjunctive guard; deeper nesting keeps
the typed refusal), PANDA-style method `:effect` (applied at decomposition,
guards against the source state), and existential binding of the root
`:htn`'s own `:parameters` (one ground root network / initial state per
admissible binding). Both rows are flipped out of `KNOWN_MISMATCHES` into the
agreement gate (with the outcome-closure check), and their old `#[ignore]`d
GROUND_ERROR demonstrations are now active SOLVED tripwires in
`htn_oracle.rs`.

Re-measured full sweep (serial, idle machine, same command as above; machine
note: `Apple M3 Max`, macOS 25.2.0, debug build):

```
RESULTS|panda-conditional-effect|oracle=SOLVED|ferroplan=SOLVED|translation_ms=1|total_ms=0
RESULTS|panda-method-effect|oracle=SOLVED|ferroplan=SOLVED|translation_ms=0|total_ms=0
```

(All other instances re-measured unchanged: the four translate-capacity
mismatches remain TRANSLATE_ERROR at the same 10 s limit — those belong to
their own tickets — and every strict-agreement instance stays SOLVED.) The
mismatch count is now 4/21 admitted (translate capacity only); agreement
16/21 strict + 1/21 open verdict.

Reproduction note (証): at wave-3 the recorded ground errors were
`nested 'when' is out of scope` (direct grounder call) and
`unbound variable '?x'`; through the full `solve_hddl` pipeline both
fixtures actually refused earlier, at validation, with
`NonGroundRootSubtaskArg` — the root `:htn`'s `:parameters` were dropped at
parse time, so `?x` could never bind. Both refusal layers are gone: the
parameters now parse, validate (declared-variable check with the same
subtype rule method variables get), and ground existentially.
