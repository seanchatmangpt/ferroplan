# wasm-ontology

How to change, regenerate and verify the `ferroplan-wasm` WASI JSON ABI through its RDF
ontology, plus a reference of the files involved. Authority ceiling: NONE; every generated
file is a candidate description or host mechanism and grants no execution authority.

## Reference

### Sources of truth

| File | Role |
|---|---|
| `ontology/ferroplan-wasm.ttl` | module, one `wja:Op` per dispatched op, error codes, pin facts |
| `ontology/ferroplan-host-contract.ttl` | host contract: exports, WASI imports, limits, recycle rules |
| `crates/ferroplan-wasm/ontology/contract.ttl` | byte copy of the host contract (frontmatter-mode ggen forbids `..` paths) |
| `crates/ferroplan-wasm/qri-ontology.ttl` | copy of the qri pack ontology used by the capsule |

### Two ggen projects

| Project | Config | Mode | Outputs |
|---|---|---|---|
| root | `ggen.toml` | declarative `[[packs]]` + `[[generation.rules]]` | `crates/ferroplan-wasm/registry/*` |
| capsule | `crates/ferroplan-wasm/ggen.toml` | frontmatter (`[templates] dir`) | `crates/ferroplan-wasm/generated/beam-host/*.ex` |

The two schemas are exclusive in one file, and a declarative rule cannot drive a pack
frontmatter template (see `crates/ferroplan-wasm/PACK-GAPS.md`, gap 1). Hence two projects.

### Generated outputs (never hand-edit)

| Output | Rule or template | Mode |
|---|---|---|
| `registry/capability-registry.json` | `wasm-capability-registry` (registry.rq) | Overwrite |
| `registry/op-examples.json` | `wasm-op-examples` (examples.rq) | Overwrite |
| `registry/ARTIFACTS.sha256` | `wasm-artifacts` | Create (pin; never rewritten) |
| `generated/beam-host/abi.ex` | `beam-host-templates/abi.ex.tmpl` | frontmatter `force: true` |
| `generated/beam-host/engine_load.ex` | `beam-host-templates/engine_load.ex.tmpl` | same |
| `generated/beam-host/host.ex` | `beam-host-templates/host.ex.tmpl` | same |
| `generated/beam-host/pool.ex` | `beam-host-templates/pool.ex.tmpl` | same |
| `generated/beam-host/wasm_config.ex` | `beam-host-templates/wasm_config.ex.tmpl` | same |

`vendor_task` and `verify_task` are deliberately not generated; see `HANDWRITTEN.md`.

### Handwritten residue

Ledger lives in `crates/ferroplan-wasm/HANDWRITTEN.md`: the `fp_alloc` / `fp_call` /
`fp_dealloc` shell, the op bodies, the `lenType` modelling, `vendor_task` / `verify_task`.
Local forks of the beam-host templates carry `LOCAL DIVERGENCE` header comments; the
upstream gaps they work around are in `PACK-GAPS.md`.

### Pinned ABI

| Symbol | Signature |
|---|---|
| `fp_alloc` | `(len: usize) -> *mut u8` |
| `fp_call` | `(ptr: *mut u8, len: usize) -> u64` packed `(out_ptr << 32) \| out_len`; consumes request |
| `fp_dealloc` | `(ptr: *mut u8, len: usize)`; release the response buffer only |

Request cap 16777216 bytes; JSON depth cap 128 (`ontology/ferroplan-wasm.ttl`).

### Drift guard

`crates/ferroplan-wasm/tests/abi_ontology_drift.rs` compares the `"op" =>` arms of
`fn dispatch` in `src/wasi_abi.rs` with the `wja:opName` literals in the ontology (source
text, std only).

## How to add or remove an op

1. Implement or delete the `"op" =>` arm in `crates/ferroplan-wasm/src/wasi_abi.rs`.
2. Add or remove the matching `wja:Op` (with `wja:opName`) in `ontology/ferroplan-wasm.ttl`.
3. Regenerate the registry from the repo root:

   ```sh
   cd /Users/sac/ferroplan && ggen sync run
   ```

4. Run the drift court:

   ```sh
   cargo test -p ferroplan-wasm --test abi_ontology_drift
   ```

5. Commit ontology, source and regenerated `registry/*` together.

ARTIFACTS.sha256 is Create-mode: a rebuilt `.wasm` needs its pin line updated deliberately,
not by sync.

## How to change a host-contract fact (limits, timeouts, probe op)

1. Edit `ontology/ferroplan-host-contract.ttl` only.
2. Copy it to the capsule: `cp ontology/ferroplan-host-contract.ttl
   crates/ferroplan-wasm/ontology/contract.ttl`.
3. Regenerate the host:

   ```sh
   cd /Users/sac/ferroplan/crates/ferroplan-wasm && ggen sync run
   ```

4. Review the diff of `generated/beam-host/*.ex`; nothing there is hand-edited.

A second `ggen sync run` must report every file unchanged (content identical). A changed
byte on the second run means replay is broken.

## How to verify a change is not decorative

| Check | Command | Expected |
|---|---|---|
| replay | run `ggen sync run` twice | second run: all outputs unchanged |
| drift gate | `cargo test -p ferroplan-wasm --test abi_ontology_drift` | pass |
| gate has teeth | in a scratch copy, rename one `wja:opName`, rerun the test | fail |
| pin | edit ARTIFACTS.sha256 in a scratch copy, run sync | line left untouched (Create) |

## How to hand the host to ex4pm

Analysis only; ferroplan edits nothing outside this repository. The exact steps are in
`crates/ferroplan-wasm/PACK-GAPS.md`, section "ex4pm handoff".

## See Also

- `crates/ferroplan-wasm/HANDWRITTEN.md`
- `crates/ferroplan-wasm/PACK-GAPS.md`
- `crates/ferroplan-wasm/README.md`
