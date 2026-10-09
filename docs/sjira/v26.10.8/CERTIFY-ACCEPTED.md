# CERTIFY-ACCEPTED — doc-hdit certify at HEAD (v26.10.8)

Lane: ferro-cov120, 2026-10-08. Closes backlog [120]+[129]: this receipt
replaces the earlier BLOCKED record at the same path (CERTIFY-BLOCKED.md,
git history). Pipeline identical to the BLOCKED run: extractor
`scripts/gen_doc_surface.py` + pack binary `doc-hdit` @ ggen-marketplace
`hdit-v2-structs` (working tree 2026-10-08, post-[105] multi-ident +
[109] env_key/str_key surface), full 4-array inputs (claims / directories /
known_external / modules / paths) against ferroplan.

## Verdict

**ACCEPTED — receipt minted.**

```
verdict: ACCEPTED
subject:  961d4e0cb0f57a820fe04afeb8aca377f0f840b3ca5e7ba9ef2de8672e7fe408
hash:     0e74b46d4631397237ed783b2c7aab5050c74299ed53fb60e178da6c7ee0cd15
```

Receipt appended out-of-subject to
`ggen-marketplace/packs/rust-doc-hdit-pack/doc-hdit.receipts.jsonl`
(subject-keyed; a commit cannot contain its own hash).

## Gates

| gate | value | threshold | verdict |
|---|---|---|---|
| S_coverage | 0.9783 (1400/1431) | >= 0.90 | PASS |
| Phi_halluc (phantom) | 0.000527 | <= 0.001 | PASS |
| Q_density | 0.9995 | >= 0.65 | PASS |

Denominator: 1431 gated items (1709 raw; collapsed delta 278) — 177
env_key items (151 distinct FF_*/harness keys), 398 str_key items, the
remainder symbols/paths. The BLOCKED run's blocker (generated-reference
phantom mass, 0.2229) is gone at the current extractor: the [105]
per-ident grounding split of use-list spans removed the mass (phantom
0.0006 pre-documentation, 0.0005 post).

## What landed this pass (docs/reference/config-surface.md)

1. Env-key inventory by module ([120] policy: env_key and str_key items
   stay IN the gated denominator, documented, no threshold change) — the
   FF_* keys with meanings written from the read sites.
2. Public re-export surface: all 25 uncovered `pub use` statements quoted
   byte-exactly as the extractor lifts them (multi-ident normalized to
   one line, per [105]). Grounding note: per [105] a use-list doc span is
   ALWAYS split into per-ident claims, so the `use` items themselves
   remain structurally uncovered at this extractor — the prose documents
   the real re-export map and grounds its members, but does not move the
   gated per-item number. Honest residual: 31 uncovered items
   (25 structural `use` items + 6 fixture str_key items, the latter
   disclosed as extraction over-approximation, out of documentation
   policy). Gate closes at 0.9783 regardless — no threshold was moved.

## Subject

Audit + certify ran on the ferroplan working tree at `6847e87` (HEAD at
extraction) with this lane's docs edits in flight (config-surface.md +
this file) — the exact tree the extracted inputs describe. Inputs:
`/tmp/ferro-cov120/inputs2.json` (8094 claims). Vectorize cache
`de63d50b266374da`, derived `a80071a9b6865aeb`.

## Replay

```sh
cd /Users/sac/ggen-marketplace
python3 scripts/gen_doc_surface.py code <checkout> > code.json
python3 scripts/gen_doc_surface.py doc  <checkout> --code-json code.json > doc.json
# merge: claims (+ids) + code surface's paths/directories/known_external/modules
cd packs/rust-doc-hdit-pack
target/release/doc-hdit vectorize <inputs>.json
target/release/doc-hdit audit     <inputs>.json courts/doc_quality.court
# expect: PASS coverage 0.9783 / PASS phantom 0.0005 / PASS density 0.9995
target/release/doc-hdit certify   <inputs>.json courts/doc_quality.court
# expect: ACCEPTED, receipt appended to doc-hdit.receipts.jsonl
```

## Standing

- ferroplan doc-hdit certify standing at subject `961d4e0c…`: **ALIVE**
  (ACCEPTED receipt minted 2026-10-08, timestamp 1791529936).
- Falsifier: re-run certify at any new HEAD; drift re-opens the gate and a
  future BLOCKED record replaces this one.

## Drift re-close at new HEAD (backlog [145], lane ferro-reclose, 2026-10-09)

Falsifier executed: certify re-run at the post-re-close HEAD (see the
verdict block appended below). The earlier BLOCKED→ACCEPTED transition
stands; the new receipt supersedes the 2026-10-08 subject only in the
receipt ledger (`doc-hdit.receipts.jsonl`), not by replacing this record.

### Re-export map in prose ([105]-residual class)

The generated reference (docs/reference/generated/reference.md) carries four
`use` rows the extractor lifts as inline spans — two `model::*` glob rows and
two `crate::packed::PackedTask as Task` alias rows. Per [105] a use-list span
is split into per-ident claims that stay structurally uncovered at this
extractor; these are documented here with real symbols only, not backticked
scaffold:

1. `crates/ferroplan-cli/src/harvest/mod.rs` — `pub use model::*` glob
   re-exports the harvest data model from crates/ferroplan-cli/src/harvest/model.rs:
   the schema constants OBSERVATION_SCHEMA, ADMISSION_SCHEMA, CATALOG_SCHEMA,
   RECEIPT_SCHEMA and the ObservationWindow / ObservationPack /
   ObservedWorkItem / TransportFailure structs (plus their fields).
2. `crates/ferroplan/src/ground.rs` — `pub use crate::packed::PackedTask as
   Task` alias re-export (commented "re-export for the heuristic/search
   modules"); PackedTask itself lives in crates/ferroplan/src/packed.rs and
   is additionally imported by name in api.rs, costs.rs, espc.rs and
   ground.rs via use crate::packed::{CondEff, CsrBuilder, PackedTask, State}.

These rows appear twice each in the generated reference because the pack
renders the harvest module in both the CLI and workspace surfaces; the map
above is the single real source.

### Fresh audit verdict at the re-close tree

```
PASS  coverage value=0.9783 threshold=0.9000
PASS  phantom  value=0.0005 threshold=0.0010
PASS  density  value=0.9995 threshold=0.6500
```

Denominator unchanged: 1431 gated items (1709 raw, collapsed delta 278).
Inputs: /tmp/ferro-reclose/inputs2.json (8121 claims; code+doc surface
extracted from the ferroplan working tree carrying this lane's edits).
Standing after the falsifier run: **ALIVE** — the gate did not re-open.

## Extractor identity pin (ggen-marketplace fleet law [150], 2026-10-09)

Receipts in this doc predate the extractor pin and are **grandfathered**
(valid as bound). Certify now embeds an `extractor` field (BLAKE3 over the
extractor source bytes) into every new receipt and refuses typed
(`REFUSED:EXTRACTOR_MISMATCH`) on replay when the current extractor identity
differs from the recorded one; `--force-rebaseline` mints a NEW baseline
receipt acknowledging the drift. Pass `--extractor scripts/gen_doc_surface.py`
(ggen-marketplace) when replaying so new receipts carry the pin.
