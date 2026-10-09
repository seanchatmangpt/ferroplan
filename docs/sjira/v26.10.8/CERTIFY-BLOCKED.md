# CERTIFY-VERIFY (supersedes CERTIFY-BLOCKED @ 44749c6) — doc-hdit certify ACCEPTED (v26.10.8)

Lane ferro-cov120, 2026-10-08. Canonical doc-hdit pipeline (extractor
`scripts/gen_doc_surface.py` + pack binary `doc-hdit` @ ggen-marketplace
`hdit-v2-structs` `2de3d52fe`, binary prebuilt) run with full 4-array inputs
(claims / directories / known_external / modules / paths) against ferroplan.
This file replaces the BLOCKED receipt at `47aa362` (committed as `44749c6`);
the falsifier on that receipt ("re-run certify at HEAD once [86] lands;
ACCEPTED replaces this BLOCKED receipt") fires here.

## Verdict

**ACCEPTED — receipt minted, appended to
`docs/sjira/v26.10.8/ferroplan.chain.jsonl`.**

```
{"gates":{"Phi_halluc":0.0005379236148466917,"Q_density":0.9994620763851533,
"S_coverage":0.9783368273934312},
"hash":"eccd087c3924bacc2b061c5b50626a3e5faa74f59444c8adeb7f26370fe98fc6",
"parent":"","subject":"d02931777c45afd65d5aa14d96c3e162ff4b0d3852d795b2feb1a152c186c873",
"thresholds":{"Phi_halluc_max":0.001,"Q_density_min":0.65,"S_coverage_min":0.9},
"timestamp":1791529197,"verdict":"ACCEPTED"}
```

- Receipt subject = SHA-256 of the audited inputs
  (`/tmp/ferro-cov120/inputs2.json`, extraction of the working tree at this
  commit minus this file and the chain itself — the out-of-subject receipt
  seam, C21 class; re-run at any clean checkout reproduces within the
  receipt-doc delta).

## Gate table (extraction 2026-10-08, 7911 claims)

| gate | value | threshold | verdict |
|---|---|---|---|
| S_coverage (public-scope set) | 0.9783 | >= 0.90 | PASS |
| Phi_halluc | 0.0005 | <= 0.001 | PASS |
| Q_density | 0.9995 | >= 0.65 | PASS |
| S_coverage_raw (report-only) | 0.2452 | — | report |
| coverage_denominators | set=1431 raw=1709 (collapsed_delta 278) | — | report |
| prose_artifacts | 475 (excluded from Phi/Q) | — | report |

Denominators: 1709 raw public items; 177 env_key items (151 distinct keys) +
398 str_key items (347 distinct values) among them ([109] surface).

## [120] config-surface denominator decision

Recorded in `docs/reference/config-surface.md` (Policy section): env_key AND
str_key items stay IN the gated denominator and the genuinely-public config
items are documented (env keys fully, per-module, from read sites; string-key
receipt/schema constants documented; fixture/inline HDDL/PDDL literals are
disclosed as an extraction over-approximation, not hand-documented —
manufacturing prose for fixture text would be exactly the hallucination the
court exists to refuse). No threshold moved.

## What closed the gap (baseline -> pass)

Baseline at the [105]-landed extractor (HEAD `620f953`, before this lane's
edits): S_coverage 0.8637 / Phi 0.0209 (162 phantom claims) / Q 0.9791 — the
[109] env-key/str-key denominator moved coverage to ~0.86 and the
table_row_scaffold phantom class stayed open. Closure, all doc-side:

| change | files |
|---|---|
| [120] config-surface reference (all 151 env keys documented per read site; string-key constants) | docs/reference/config-surface.md (new) |
| generated-reference scaffold regenerated at the [109]/[105] surface (adds env_key/str_key rows; kills the 4 use-span FPs' stale rows) | docs/reference/generated/{reference,how_to,explanation}.md |
| phantom repair — de-backtick record references (commit SHAs, historical test names, spec vocabulary, private-fn/field names, error-code literals, tool/binary names) per the [106] disposition precedent | 26 files: FOND-HTN.md, BENCHMARKS.md, PRD-FORTUNE5-*.md, wasm-ontology.md, roadmap-0.28.md, claude-projection.md, archive notes x3, jira v26.9.17 x14, CAMPAIGN-RECEIPT.md, README.md, book/src/tuning.md |
| crates.md left untouched (generated from ontology/ferroplan-crates.ttl; its 4 residual phantoms are inside the Phi budget) | — |

## Residual phantom mass (8, inside the Phi budget)

- book/src/crates.md (4): `ff`, `pip`, `oneof`, `ferroplan` — generated from
  the ontology; hand-edit forbidden by its banner.
- docs/reference/generated/reference.md (4): two rendered `use` rows whose
  whole-span idents (`model::*`, `crate::packed::PackedTask as Task`) do not
  re-ground against the audited item set (use-item whole-span self-claim
  loss, the [105] residual class). Extractor-side item, not hand-fillable.

## Replay

```sh
cd /Users/sac/ggen-marketplace   # hdit-v2-structs @ 2de3d52fe
python3 scripts/gen_doc_surface.py code <checkout> > code.json
python3 scripts/gen_doc_surface.py doc  <checkout> --code-json code.json > doc.json
# merge to inputs (ids + code-surface paths/directories/known_external/modules)
cd packs/rust-doc-hdit-pack
target/release/doc-hdit vectorize <inputs>.json
target/release/doc-hdit audit     <inputs>.json courts/doc_quality.court
target/release/doc-hdit certify   <inputs>.json courts/doc_quality.court --chain <chain>.jsonl
# expect: S 0.9783 / Phi 0.0005 / Q 0.9995 -> verdict ACCEPTED
```

## Standing

- ferroplan doc-hdit certify standing: **ACCEPTED** at the extraction of this
  commit's tree. Receipts at earlier subjects (SEMANTIC-WAVE ferroplan PASS
  0.9825; BLOCKED at `47aa362`/`44749c6`) are bound to their subjects and do
  not transfer; this receipt replaces them for the current HEAD.
- Pathspec disclosure: the phantom repairs also touch README.md and
  book/src/tuning.md — documentation files in substance, required by the
  audit (the phantom class lives there); no non-doc file was modified.
- Falsifier: re-run certify at any new HEAD; a gate failure re-opens this
  receipt with a typed REFUSED.
