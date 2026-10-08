# Generated reference skeletons (doc-hdit scaffold)

GENERATED — do not hand-edit the reference tables.

- Source of truth: ferroplan code surface, extracted by
  `ggen-marketplace/scripts/gen_doc_surface.py code /Users/sac/ferroplan`.
- Renderer: `doc-hdit scaffold` from `ggen-marketplace/packs/rust-doc-hdit-pack`
  (templates in that pack's `templates/`).
- Regen command:

  ```sh
  python3 /Users/sac/ggen-marketplace/scripts/gen_doc_surface.py code /Users/sac/ferroplan \
    > /tmp/hdit/ferroplan.code.v3.json
  /Users/sac/ggen-marketplace/packs/rust-doc-hdit-pack/target/release/doc-hdit scaffold \
    --code /tmp/hdit/ferroplan.code.v3.json \
    --templates /Users/sac/ggen-marketplace/packs/rust-doc-hdit-pack/templates \
    --out /Users/sac/ferroplan/docs/reference/generated
  ```

- The reference tables under `reference.md` are rigid (see the
  AGENT-FORBIDDEN banner in the file): every row is rendered from the
  extracted code surface. Prose lives only in the fenced slot.
- Known extractor limit (disclosed, not hand-filled): `struct`/`enum`/`trait`
  rows carry an empty signature column (612 items) — the v1 scanner records
  type names only, not fields/generics. Function signatures (1120) are complete.
