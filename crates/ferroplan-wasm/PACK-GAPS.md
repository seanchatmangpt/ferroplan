# PACK-GAPS

Upstream gap report for the packs consumed by `ferroplan-wasm`, with evidence, plus the
ex4pm/beam4pm handoff. Analysis only: ferroplan edits nothing outside its own repository.

Subjects (observed 2026-10-01): `qri-qualification-profile-pack` 26.9.30 and
`wasi-json-abi-pack` 26.9.29 in `/Users/sac/ggen-marketplace` (HEAD 8971f8754);
ggen 26.9.28; ex4pm 30d5cbb (HEAD now 60a43b1, a descendant; ex4pm citations re-checked at
60a43b1); beam4pm fee843a2; ferroplan a03b5cc. Paths below are relative to
`/Users/sac/ggen-marketplace/packs/qri-qualification-profile-pack` unless prefixed `wja:`
(`wasi-json-abi-pack`).

## Gaps

### 1. FM-GEN-008: beam-host cannot be driven from `[[packs]]`

- Templates are frontmatter-schema: `templates/beam-host/pool.ex.tmpl:1-70` declares
  `to:`, a named `sparql: profile:` query and `force: true`. `host.ex.tmpl:3-100` declares
  four named queries (`profile`, `limits`, `exports`, `recycle`).
- A declarative rule pointing at `beam-host/host.ex.tmpl` with `50-profile.rq` fails exit 1,
  `FM-GEN-008 Variable profile[0] not found`: frontmatter `sparql:` is ignored and only one
  query binds (`/private/tmp/ferro-w1/f.md:92`).
- Omitting `query` in a rule is a parse error; `[[packs]]` inside a frontmatter config is
  refused. Consequence: a consumer must copy templates and ontology into a capsule
  (`ggen-beam-host.toml:1-19` documents the same copy-and-rename procedure).
- Ferroplan state: `crates/ferroplan-wasm/ggen.toml` plus local `beam-host-templates/`
  copies. Drift risk: copies are not bound to the pack SHA.
- Ask upstream: either rule-level multi-query binding (`query = { profile=..., limits=... }`)
  or a documented pack output mode for frontmatter templates.

### 2. `lenType` accepts only `u32`

- `templates/beam-host/host.ex.tmpl:112` emits the undefined variable
  `UNSUPPORTED_host_profile_not_packed_u64_consumed_u32` unless
  `out_mode == "packed_u64"`, `req_consumed == true` and `len_type == "u32"`.
- Ferroplan exports `fp_alloc(len: usize)` / `fp_dealloc(ptr, len: usize)`
  (`src/wasi_abi.rs:194,213,236`). On wasm32 `usize` and `u32` have the same wire width, so
  the contract declares `qri:lenType "u32"` (`ontology/ferroplan-host-contract.ttl:63`) and
  records `UNSUPPORTED(ontology-semantics: usize==u32 on wasm32)`.
- Ask upstream: accept `usize` as an alias of `u32` for wasm32 targets, or model
  `qri:lenWidthBits 32`.

### 3. `Refusal.new` third argument: map vs keyword

- Pack passes a bare map: `templates/beam-host/abi.ex.tmpl:131,164`,
  `engine_load.ex.tmpl:224,283,295` (multi-line calls at 255 and 269 also pass a bare map), `host.ex.tmpl:212,215,249,304,415,418,461,529,533,550`,
  `wasm_config.ex.tmpl:258,265`.
- ex4pm's refusal is `Ex4pm.Refusal.new(code, message, opts \\ [])` reading
  `Keyword.get(opts, :details, %{})` (`/Users/sac/ex4pm/lib/ex4pm/core.ex:30-52`). A bare map
  raises in `Keyword.get/3` at runtime.
- Ferroplan patched the local template copies to `details: %{...}` and tagged each with a
  `LOCAL DIVERGENCE` header (`beam-host-templates/*.tmpl`; each tag references `DIVERGENCE.md`, which lives at
  `beam-host-templates/DIVERGENCE.md`, not at the crate root).
- Ask upstream: add `qri:refusalCallStyle "keyword-details" | "positional-map"` to the host
  profile, default keyword to match the `taxonomyNamespace`/`refusalModule` consumers.

### 4. `poolStrategy` flows into `Supervisor.init`

- `templates/beam-host/pool.ex.tmpl:132` renders `strategy: :{{ p.pool_strategy }}`; the
  ferroplan contract sets `qri:poolStrategy "single"`
  (`ontology/ferroplan-host-contract.ttl:72`), giving `strategy: :single`, which is not a
  Supervisor strategy (`:one_for_one`, `:one_for_all`, `:rest_for_one`).
- The pool docstring (`pool.ex.tmpl:79`) treats the value as a routing strategy; the code
  uses it as a supervision strategy. Two meanings, one field.
- Ferroplan patched the local copy to `:one_for_one`
  (`generated/beam-host/pool.ex:60`).
- Ask upstream: split into `qri:poolRouting` and `qri:supervisorStrategy`.

### 5. `verify_task` / `vendor_task` assumptions

- `templates/beam-host/verify_task.ex.tmpl:103,127-132` requires the manifest to carry an
  integer `abi_version` and the probe response to carry `ok: true` and
  `<probeExpectKey> == manifest abi_version`.
- Ferroplan's `version` op returns only `{"version": <crate version>}`; the contract notes
  this (`ontology/ferroplan-host-contract.ttl:15-16`). The generated task could never pass
  unless the manifest `abi_version` equals the crate version string.
- `vendor_task.ex.tmpl:124,142` requires `artifact.url` in `MANIFEST.json`; ferroplan's
  manifest schema is `ferroplan.wasm.manifest/v1` with a local artifact path, no URL.
- Ferroplan state: both templates not copied; recorded in `HANDWRITTEN.md`.
- Ask upstream: make the probe comparison optional (`qri:probeExpectMode "present"`) and the
  `url` optional (skip download when absent).

### 6. Query 51 (limits) coupling

- `queries/51-limits.rq` returns every `qri:hostLimit` row ordered by `?order`
  (`51-limits.rq:1-11`). The ferroplan contract declares 18 limits.
- Templates select by name with `set_global` loops: `host.ex.tmpl:104-109` reads six names
  (`abi_timeout`, `margin`, `retry_base_ms`, `retry_max_ms`, `initialize_timeout_mult`,
  `backoff_exp_cap`); `vendor_task.ex.tmpl:87` reads `max_redirects`. A misspelled
  `limitName` leaves the variable unset instead of failing, so a typo is a silent default.
- Several declared limits are ASSUMED, not enforced by existing hosts
  (`contract.ttl` header: memory_limit_bytes, recycle_bytes, fuel_per_ms, instantiate_fuel).
- Ask upstream: fail generation when a template-required limit name is absent from query 51
  rows; add a SHACL shape listing the required names.

### 7. wja `ffi` / `guards` do not apply to the `fp_*` usize ABI

- `wja:templates/wasm_ffi.rs.tmpl:89-163,212,222` renders `{prefix}_abi_version`, `_alloc(len:
  u32)`, `_free`, `_call` with u32 lengths and a `_free` suffix. Ferroplan's pinned ABI is
  `fp_alloc(usize)`, two-argument `fp_dealloc(ptr, len)`, packed-u64 `fp_call` and no
  `fp_abi_version` export; adopting the template renames or retypes exported symbols that
  ex4pm and beam4pm hosts bind (`/Users/sac/ex4pm/lib/ex4pm_engine/wasm/ferroplan_transport.ex`
  calls `fp_call`; beam4pm `lib/beam4pm_ferroplan.ex:17-25` documents `fp_alloc/fp_call/
  fp_dealloc`).
- `wja:templates/wasm_guards.rs.tmpl:31-112` provides `json_depth`, `op_index`, `limit`,
  `limit_response` returning typed `AbiError`; these are usable only after an ffi shell in
  the same shape, so they inherit the same incompatibility.
- Ferroplan state: ffi and guards are not projected; recorded `UNSUPPORTED(generator-capability)`
  in `HANDWRITTEN.md`. `wasm_abi_meta.rs.tmpl` (constants only) remains a candidate.
- Ask upstream: parameterize ffi by `qri:lenType`, free symbol name and arity
  (`qri:freeSymbol`, `qri:freeArity` already exist in the ferroplan contract).

### 8. Missing `[pack.outputs]` (benign WARN)

- `qri-qualification-profile-pack` has `pack.toml` only; there is no `package.toml`
  (`ls` evidence: no such file) and `pack.toml` has no `[pack.outputs]` table.
- ggen warns `no [pack.outputs] entry for queries/templates` and falls back to the literal
  directory names; exit 0, output identical (`/private/tmp/ferro-w1/f.md`, experiment results).
  `wasi-json-abi-pack` has `package.toml` yet the same WARN appeared in the same run.
- Ask upstream: add `[pack.outputs] queries = "queries"` and `templates = "templates"`.

## ex4pm handoff

Goal: ex4pm consumes `generated/beam-host` and `registry`, then retires its hand-written
transport. Nothing below is executed by ferroplan; each step is for the ex4pm owner and
needs that repository's own gates. Subjects: ex4pm 30d5cbb, beam4pm fee843a2.

### A. Consume (ex4pm)

1. Build and pin the artifact in ferroplan:
   `cargo build -p ferroplan-wasm --release --target wasm32-wasip1`, then record the sha256
   in the pin line of `crates/ferroplan-wasm/registry/ARTIFACTS.sha256`.
2. Vendor `generated/beam-host/{abi,engine_load,host,pool,wasm_config}.ex` into ex4pm
   `lib/ex4pm_engine/ferroplan/` (module root `Ex4pmEngine.Ferroplan`, per
   `qri:moduleRoot`). Vendor the `.wasm` and a `priv/ferroplan/MANIFEST.json` with schema
   `ferroplan.wasm.manifest/v1` (`artifact.file`, `artifact.sha256`).
3. Write `mix ex4pm.verify` by hand (gap 5): probe `version`, compare the crate version to
   the manifest, fail closed on digest mismatch.
4. Replace the callers in `lib/ex4pm/engine/ferroplan.ex:49,59,292`
   (`FerroplanTransport.stop/start/call`) with `Ex4pmEngine.Ferroplan.Host` request calls;
   per-call timeout 120000 for `plan`, `plan_production`, `explain`, `session_new`,
   `session_think` (contract header).
5. Port `test/wasm/ferroplan_transport_test.exs` (103 lines) to the new host module; keep the
   `version` and `readiness` assertions, add a refusal case (null/oversize request).
6. Optionally read `registry/capability-registry.json` and `op-examples.json` to replace
   hand-listed op names in `docs/FERROPLAN-RUNTIME.md:34-60`.

### B. Retire (ex4pm)

Preconditions: A.1-A.5 green with the same fixtures; one release cycle with both paths
present behind a config flag.

1. Delete `lib/ex4pm_engine/wasm/ferroplan_transport.ex` (255 lines) and
   `test/wasm/ferroplan_transport_test.exs`.
2. Remove the `Ex4pm.Engine.Ferroplan.Host` GenServer cache (`lib/ex4pm/engine/ferroplan.ex`)
   if the generated `Pool` supersedes it; keep the public facade functions.
3. Update `docs/FERROPLAN-RUNTIME.md:19,34` and `CHANGELOG.md`.

### C. Retire (beam4pm)

`BeamPM.Ferroplan` (`/Users/sac/beam4pm/lib/beam4pm_ferroplan.ex:2`, plus
`BeamPM.Ferroplan.Health` at :881 and `BeamPM.Ferroplan.Bridge` in
`lib/beam4pm_ferroplan_bridge.ex`) is rendered by the beam4pm `bpm:Engine` graph, not by this
contract. Bridge needed: `bpm:EngineOp` facts (beam4pm) vs `wja:Op` facts (ferroplan).

1. Write a CONSTRUCT bridge from `wja:Op` to `bpm:EngineOp` so the op surface is derived from
   `registry/capability-registry.json` rather than hand-listed.
2. Replace the wasmex hosting block in the beam4pm template with a delegate to the vendored
   `Ex4pmEngine.Ferroplan.Host`, or regenerate beam4pm's host from the same qri contract.
3. Regenerate `beam4pm_ferroplan.ex` and the Erlang/Gleam facades; do not hand-edit.
4. Update the 33 test files under `test/` that name `BeamPM.Ferroplan` (observed by grep at
   fee843a2, e.g. `test/beam4pm_ferroplan_test.exs`,
   `beam4pm_parity_ferroplan_hddl_solve_vs_lab_fabric_solve_test.exs`, `test/beam_pm/
   ferroplan_bridge/*`) and rerun them.
5. Only after step 4 passes, delete the hand-hosting code paths.

### Falsifiers

- ex4pm tests pass with the old transport deleted but `generated/beam-host` absent:
  the consumer is not actually using it.
- Digest-mismatch artifact starts successfully: admission is vacuous.
- A ferroplan op added to the ontology does not appear in ex4pm's registry read: the bridge
  is decorative.
