# Beam-host template divergences

Local copies of `~/ggen-marketplace/packs/qri-qualification-profile-pack/templates/beam-host/*`
(frontmatter-mode ggen forbids `..` paths). Each divergence is also in the template header.

| template | what | why |
|---|---|---|
| abi.ex.tmpl, engine_load.ex.tmpl, host.ex.tmpl, wasm_config.ex.tmpl | `Refusal.new(code, msg, %{...})` rewritten to `Refusal.new(code, msg, details: %{...})` | `Ex4pm.Refusal.new/3` takes keyword opts (`:subject`, `:details`); a bare map raises in `Keyword.get` |
| pool.ex.tmpl | `Supervisor.init(..., strategy: :one_for_one)` instead of `:{{ p.pool_strategy }}`; pool size remains ontology `qri:poolSize` 1 | profile `poolStrategy "single"` is a routing label, not a Supervisor strategy; `:single` is invalid |

Not changed: `qri:lenType` (left as upstream; not proven safe to alter).
Ledger: UNSUPPORTED(generator-capability) until upstream pack carries these fixes.

## Defects found by the real-wasmex run (host-real-wasm lane, 2026-10-01)

| template | what | why (observed against real wasmex 0.15.1 + built ferroplan wasm32-wasip1) |
|---|---|---|
| host.ex.tmpl `classify/1` | `{:exit, {:timeout, _}}` -> `:call_timeout` (was `:call_exited`) | a 20 ms deadline on a 120-block plan surfaced as `:call_exited`; the instance is still recycled (recycle_codes) but the code was wrong |
| host.ex.tmpl `emit_call/3` | `:refused` only for `"ok" => false` or an `"error"` envelope; otherwise `:ok` | ferroplan responses carry no `"ok"` key, so every success was telemetered `:refused` |
| wasm_config.ex.tmpl / engine_load.ex.tmpl | allowlist also accepts `"wasi_snapshot_preview1.<name>"` string rows (name-only, `:any` signature) | the real ex4pm `priv/ferroplan/MANIFEST.json` stores imports as strings; the map-only decoder returned `:unavailable` and refused every engine (`wasm_import_surface_mismatch`) |

Not changed (reported): the engine's `{"error":{code,message,retryable}}` envelope is returned as `{:ok, map}`
(host contract: only transport failures are `{:error, _}`); `FerroplanTransport.call/4` converts it to
`:ferroplan_engine_error`. Callers of the generated host must check `"error"` themselves.

## Added after audit (2026-10-01)

- `engine_load.ex.tmpl`: the `wasm_import_surface_mismatch` and `wasm_missing_export`
  `Refusal.new/3` calls now pass `details: %{...}` (they still passed a bare map).
- `host.ex.tmpl`: the `wasm_not_vendored` message no longer points at a `mix <app>.vendor` task,
  because `vendor_task` is not generated.
- Registry (not beam-host): the local registry template under ontology/templates (a copy of the
  wasi-json-abi-pack registry template; guarded by sync determinism, not copy_drift) listing the live exports `fp_alloc`, `fp_call`,
  `fp_dealloc`, `memory` (upstream hardcodes `<prefix>_abi_version` and `<prefix>_free`).
- `registry/ARTIFACTS.sha256` is now rendered with `artifacts.rq` (pin facts present); the earlier
  `registry.rq` binding produced the placeholder skeleton, and Create mode never replaced it.
- Generated `.ex` files are not `mix format`-clean; format them in the consuming project.
