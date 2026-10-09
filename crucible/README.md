# crucible

Standalone sweep harness for ferroplan plan quality. Vendored-by-intent: it is
its own Cargo workspace root (`crucible/Cargo.toml`, excluded from the ferroplan
workspace via `exclude = ["crucible"]` in the root `Cargo.toml`), is never
published (`publish = false`), and is deliberately invisible to ferroplan CI —
`crucible/preflight.sh` is its only gate, run before any crucible change lands.

Consequence for doc-surface audits: `crucible/` is a vendored tree, not
first-party surface. It is excluded from the public code-surface denominator
(same policy as `vendor/` in ggen-marketplace `gen_doc_surface.py`, which lists
`crucible` in `VENDOR_DIRS`).
