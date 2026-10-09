# CERTIFY-BLOCKED — doc-hdit certify re-run at HEAD (v26.10.8)

Lane: ferro-receipt, 2026-10-08. Canonical doc-hdit pipeline (extractor
`scripts/gen_doc_surface.py` + pack binary `doc-hdit` @ ggen-marketplace
`hdit-v2-structs` `6c2e0a41a`, binary built 2026-10-08 21:17) run with full
4-array inputs (claims / directories / known_external / modules / paths,
corrected merge carrying the code surface's `paths`/`directories`/`known_external`)
against ferroplan at HEAD.

## Verdict

**BLOCKED — typed certify refusal, no receipt minted.**

```
REFUSED:DOC_HDIT_CERTIFY_GATE_FAIL:Phi_halluc value=0.2229 threshold=0.0010
REFUSED:DOC_HDIT_CERTIFY:gate failure — no receipt minted
```

- Working tree at extraction time (r2 lane in-flight) and clean-HEAD snapshots
  at both `3da5adc` and `47aa362` all refuse identically: **the phantom mass is
  committed, not an in-flight-edit artifact**.
- Confound check: the ferro-r2 lane had an in-flight regen of
  `docs/reference/generated/` (−6600 lines) during the first extraction; the
  clean-HEAD re-runs rule it out as the cause.

## Gates at HEAD 47aa362 (inputs `/tmp/ferro-receipt/head2.inputs.json`, sha256 `51d271e6...`)

| gate | value | threshold | verdict |
|---|---|---|---|
| S_coverage | 0.9149 | >= 0.90 | PASS |
| Phi_halluc (phantom) | 0.2229 | <= 0.001 | **FAIL** |
| Q_density | 0.7771 | >= 0.65 | PASS |

Offending claims (indices into the extracted inputs; all `table_row_scaffold`
claims whose subject is the generated reference):

| idx | id | kind | subject | object |
|---|---|---|---|---|
| 2988 | `c-6beac2d4ae765a90` | table_row_scaffold | docs/reference/generated/reference.md | cores |
| 2989 | `c-db0756be8df6cb02` | table_row_scaffold | docs/reference/generated/reference.md | cores |
| 3262 | `c-4440fe99d4da1860` | table_row_scaffold | docs/reference/generated/reference.md | forge |

## [86] pointer — generated-reference phantom mass

Blocked on coordinator backlog item **[86]**: the generated reference
(`docs/reference/generated/reference.md`) carries a phantom mass of
`table_row_scaffold` rows mentioning `cores`/`forge` that do not ground against
the code surface. Until [86] fixes the generated-reference phantom mass (or the
extractor/court scopes generated-reference scaffold rows), certify refuses and
no doc-hdit receipt can be minted for ferroplan.

## Replay

```sh
# extractor @ ggen-marketplace hdit-v2-structs (6c2e0a41a), pack binary rebuilt
cd /Users/sac/ggen-marketplace
python3 scripts/gen_doc_surface.py code <checkout> > code.json
python3 scripts/gen_doc_surface.py doc  <checkout> --code-json code.json > doc.json
# merge: claims get ids; carry paths, directories, known_external from the code surface
#   (see ex4pm CERTIFY-VERIFY.md seam-bug note)
cd packs/rust-doc-hdit-pack
target/release/doc-hdit vectorize <inputs>.json
target/release/doc-hdit audit     <inputs>.json courts/doc_quality.court
target/release/doc-hdit certify   <inputs>.json courts/doc_quality.court --chain chain.jsonl
# expect: REFUSED:DOC_HDIT_CERTIFY_GATE_FAIL:Phi_halluc value=0.2229 threshold=0.0010
```

## Standing

- ferroplan doc-hdit certify standing at HEAD `47aa362`: **BLOCKED** (typed
  refusal above; coverage and density gates clear — the sole blocker is the
  [86] generated-reference phantom mass).
- Receipts at earlier subjects (SEMANTIC-WAVE-RECEIPT ferroplan PASS 0.9825;
  `/tmp/ferro_audit.out` PASS at cache `80257c6f`) are bound to their subjects
  and do not transfer to HEAD.
- Falsifier: re-run certify at any HEAD once [86] lands; ACCEPTED replaces this
  BLOCKED receipt.
