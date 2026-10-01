# HANDWRITTEN

Generated-vs-handwritten ledger for the `ferroplan-wasm` WASI module. The RDF
sources of truth are `ontology/ferroplan-wasm.ttl` (module, ops, error codes,
pin facts) and `ontology/ferroplan-host-contract.ttl` (host contract). Generated
files are projections: never edit them by hand, edit the ontology and run
`ggen sync`.

Authority ceiling: **NONE**. Every generated artifact here is a candidate
description or host mechanism. It grants no execution authority; planner outputs
remain evidence.

## Generated (do not hand-edit)

| Artifact | Source | Generator |
|---|---|---|
| `registry/*` (capability registry, op examples, artifact pin `sha256`) | `ontology/ferroplan-wasm.ttl` | `wasi-json-abi-pack` via `ggen.toml` |
| `generated/beam-host/*.ex` (abi, wasm_config, engine_load, host, pool) | `ontology/ferroplan-host-contract.ttl` | `qri-qualification-profile-pack` beam-host templates |

## Handwritten residue

| Surface | Location | Standing | Reason |
|---|---|---|---|
| `fp_alloc`, `fp_call`, `fp_dealloc` Rust shell | `src/wasi_abi.rs` | `UNSUPPORTED(generator-capability)` | The pack `ffi.rs` template is u32 and `_free` shaped. Adopting it would change the exported symbols and signatures (`fp_alloc(len: usize)`, 2-arg `fp_dealloc(ptr, len)`, packed-u64 `fp_call`) and break the pinned ABI that ex4pm and beam4pm hosts rely on. |
| Op bodies (the 38 `dispatch` arms: plan/htn/fond/hddl, `session_*`, probe, meta ops) | `src/wasi_abi.rs`, `src/dfcm_route.rs`, `src/probe_guard.rs` | `UNSUPPORTED(generator-capability)` | Planner semantics are domain logic, not derivable from the ontology; the pack generates metadata and examples, not op implementations. |
| `lenType` modelling | `ontology/ferroplan-host-contract.ttl` | `UNSUPPORTED(ontology-semantics)` | The beam-host `host.ex.tmpl` renders only `lenType "u32"`. The contract declares `u32`; on wasm32 `usize` and `u32` are the same wire width, so the ABI is unchanged. |
| `vendor_task` / `verify_task` | pack `beam-host` templates | `UNSUPPORTED(generator-capability)` | Not generated: `vendor_task` assumes an `artifact.url` and `verify_task` needs an integer `abi_version` plus an `ok:true` probe; ferroplan's `version` op returns only `{"version":...}`, so the generated task could never pass. |

## Drift guard

`tests/abi_ontology_drift.rs` asserts the set of `"op" =>` arms in
`fn dispatch` equals the set of `wja:opName` literals in the ontology. Adding or
removing an op requires editing both.

## Authority

authorityClaim: NONE. Hooks and generators express intent only; no generated
file authorizes actuation.
