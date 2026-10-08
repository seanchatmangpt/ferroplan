# Gall Checkpoints Audit — 2026-07-29

Cold read on someone else's ledger. Forty entries in `docs/gall-checkpoints.md`
— zero through 21 written in prose, CE-GALL-22 through CE-GALL-40 wired to
receipts. CE-GALL-27's a ghost in the numbering, not a body; details below.
Nothing here gets touched — no edit to `docs/gall-checkpoints.md`, no
`CE-GALL-NN.json` receipt rewritten, no test file altered. Walk in, verify,
walk out. Everything below is what the record said back.

## How this was produced

Ran the suites cold, no assumptions. `cd plugins/chatman-ecosystem &&
python3 -m pytest tests/ -q` — green across the board. `cargo test -p
ferroplan-mcp` — also green, 48 tests spread across `protocol.rs`,
`session_protocol.rs`, `session_goal_advance.rs`,
`session_lifecycle_bookends.rs`, `merged_server.rs`, `dogfood_chain.rs`,
plus the crate's own unit tests.

Two tracks, two disciplines. Track A (CE-GALL-22–40): pulled every
receipt's `standing`/`reason` and set it against its doc section, line by
line. Chased down every named `positive_witness`/`negative_falsifier` test
function — confirmed each one exists at, or within a few lines of, the
spot it claims. Re-ran `test_receipts.py` (166/166 passed — schema
validation and the promotion-law test both in that count), then went back
and hand-checked five items that had flagged themselves. Track B
(checkpoints 0–21): the four that pointed at something concrete and
re-runnable got re-run, live. The other eighteen got sorted by what their
own words actually hand a reader to check — nothing invented, nothing
assumed.

## Track A — CE-GALL-22 through CE-GALL-40 (receipt-backed)

Eighteen checkpoints, eighteen clean matches. **Doc prose and receipt
JSON agree exactly** on `standing`/`reason` — nothing off. Every named
test function is where it says it is (line numbers drifted a few lines
under later edits in five cases — 35, 36, 37, 38, 39 — but never wrong
file, never missing). Nothing here wears `ALIVE`; every receipt carries
`replayed_outside_session: false` and `sealed_at_commit: null`, holding
the promotion law's line (`test_promotion_law_actually_refuses` passes).

| Checkpoint | Standing (reason) | Verdict | Note |
|---|---|---|---|
| CE-GALL-22 | PARTIAL_ALIVE (NO_FALSIFIER) | CONFIRMED | No falsifier by design; doc and receipt agree |
| CE-GALL-23 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | Named tests exist and pass |
| CE-GALL-24 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | " |
| CE-GALL-25 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | " |
| CE-GALL-26 | PARTIAL_ALIVE (NO_FALSIFIER) | CONFIRMED | " |
| CE-GALL-28 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | " |
| CE-GALL-29 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | " |
| CE-GALL-30 | PARTIAL_ALIVE (MOCKED) | CONFIRMED | No positive witness by design (refuted claim) |
| CE-GALL-31 | UNSUPPORTED (DEPENDENCY_MISSING) | CONFIRMED | `verify_chain` genuinely absent repo-wide |
| CE-GALL-32 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | Named test exists and passes |
| CE-GALL-33 | PARTIAL_ALIVE (DEPENDENCY_MISSING) | CONFIRMED | Open defect, no falsifier by design |
| CE-GALL-34 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | Named tests exist and pass |
| CE-GALL-35 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | Line drift only (97/191 doc vs 104/201 actual) |
| CE-GALL-36 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | Line drift only |
| CE-GALL-37 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | Line drift only |
| CE-GALL-38 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | Line drift only |
| CE-GALL-39 | UNSUPPORTED (DEFECT_OPEN) | CONFIRMED | Falsifier present and passes (this is the open-defect demonstration) |
| CE-GALL-40 | PARTIAL_ALIVE (NO_REPLAY) | CONFIRMED | `dogfood_chain.rs` is the sole file, no stale duplicate |

## Track B — Checkpoints 0 through 21 (prose-only)

Four checkpoints handed over something solid — a file, a command, a name
to run. All four ran clean:

| # | Artifact | Result |
|---|---|---|
| 0 | `pytest tests/test_phase_space.py::test_every_invariant_key_is_understood` | PASS |
| 3 | `pytest tests/test_authority.py::test_single_actuator_policy_is_enforced` | PASS |
| 7 | `cargo check --workspace` + `cargo test --workspace` | PASS (clean, all green) |
| 13 | `benchmarks/get-val.sh` / VAL binary, run against `kiln-pack-domain.pddl`/`kiln-pack-6.pddl` | RECONFIRMED, not stale (exit 0, real output) |

The other eighteen got no such gift. Sorted them by what their own words
actually support — never by evidence they didn't cite:

| # | Standing | Verdict | Note |
|---|---|---|---|
| 1 | ALIVE (fixture scope) | UNVERIFIABLE FROM RECORD | Narrative only |
| 2 | PARTIAL_ALIVE | UNVERIFIABLE FROM RECORD | Names a defect, not independently re-run |
| 4 | PARTIAL_ALIVE | UNVERIFIABLE FROM RECORD | Narrative only |
| 5 | ALIVE (fixture scope) | UNVERIFIABLE FROM RECORD | Narrative only |
| 6 | PARTIAL_ALIVE | UNVERIFIABLE FROM RECORD | No file/command cited |
| 8 | PARTIAL_ALIVE | UNVERIFIABLE FROM RECORD | Cites a file, not independently re-checked |
| 9 | PARTIAL_ALIVE | UNVERIFIABLE FROM RECORD | See contradiction chain below |
| 10 | UNSUPPORTED | UNVERIFIABLE FROM RECORD | Bare assertion (consistent with its own honesty) |
| 11 | UNSUPPORTED | RECONFIRMED | Absence-of-mechanism claim checked and confirmed absent |
| 12 | PARTIAL_ALIVE | UNVERIFIABLE FROM RECORD | No independent command |
| 14 | PARTIAL_ALIVE | **STALE (uncorrected in place)** | See contradiction chain below |
| 15 | ALIVE (fixture scope) | RECONFIRMED | Cited scripts exist, claim already scoped carefully |
| 16 | PARTIAL_ALIVE | RECONFIRMED | Cited script exists, no overclaim |
| 17 | UNKNOWN | UNVERIFIABLE FROM RECORD | Explicitly "not attempted" — honest by design |
| 18 | UNSUPPORTED | UNVERIFIABLE FROM RECORD | Bare assertion |
| 19 | PARTIAL_ALIVE | **STALE (uncorrected in place)** | See contradiction chain below |
| 20 | PARTIAL_ALIVE | RECONFIRMED | Own text already carries the CE-GALL-30 caveat |
| 21 | PARTIAL_ALIVE | RECONFIRMED | Concrete claims (PR #2, commit, CI job), internally consistent; GitHub state not re-checked live in this pass |

## Checkpoints the audit could **not** independently confirm

`1, 2, 4, 5, 6, 8, 9, 10, 12, 17, 18` — eleven doors that wouldn't open.
No file behind them, no command, no quoted output — nothing to press
against. That's not proof they're lying. That's the ceiling on what
prose alone lets you check. Checkpoints 10, 17, 18 already carry
`UNSUPPORTED`/`UNKNOWN` on their own label, so nothing contradicts them.
Checkpoints 1, 2, 4, 5, 6, 8, 9, 12 stand on `PARTIAL_ALIVE` or `ALIVE
(fixture scope)` with nothing but narrative behind it — closing that gap
takes the original session transcripts, or a fresh live replay. Neither
is on the table here.

## Contradiction chains

| Chain | Status | Detail |
|---|---|---|
| Checkpoint 13 → CE-GALL-30 → CE-GALL-38 | **Self-correcting** | CE-GALL-30's hand-fabrication finding sits untouched in the record, where it belongs; CE-GALL-38 calls itself a partial mechanical re-witness straight up, not a fix. Nothing slips back in. |
| Checkpoint 14/19 → CE-GALL-31 → CE-GALL-39 | **NOT self-correcting** | Checkpoint 14 and Checkpoint 19 both still run their "Required proof" bullets asserting fork detection/refusal, no qualifier, no flinch. The correction lives in a prepended update box — never stitched into the checklist itself. A reader who skips the box and reads only the bullets walks away misled. |
| Checkpoint 9 → CE-GALL-37 | **NOT self-correcting** | Checkpoint 9's body still calls it "architecturally absent from the MCP tool schema," flat, no hedge. CE-GALL-37 cuts that down — recursive descent exists via `cmca_allocate_recursive`; the actual remaining gap is `bind_allocation_receipt`'s flat `previous_receipt` chaining, nothing broader. No pointer was ever dropped back into Checkpoint 9's text. |
| Checkpoint 20 | **Self-correcting** | The update box already carries the CE-GALL-30 hand-fabrication caveat, in place, no gap. |

**Follow-up candidate (not fixed by this audit):** Checkpoints 9, 14, and
19 are still open wounds — a one-line strike-through or forward-pointer
worked into the original bullet text, not just parked in a prepended
update box, would stop a reader who lands on the checkpoint's own section
from walking away holding a claim later checkpoints already refuted.

## CE-GALL-27 status

Checked, closed: **not a standalone checkpoint.** It surfaces exactly
once in the whole file, an inline note buried in Checkpoint 3's section
body — no standalone `## ... (CE-GALL-27)` header anywhere (the header
sequence runs CE-GALL-26 straight to CE-GALL-28), and no
`plugins/chatman-ecosystem/receipts/CE-GALL-27.json` on disk. A
revision-event label, not a gap in the numbering worth chasing.

## Non-claims

- This audit doesn't replay anything in a separate session — it can't
  promote a single checkpoint's standing under the promotion law. Call
  it evidence staged for a future replay, not the replay itself.
- Nothing found here got fixed. The already-known
  `bind_allocation_receipt` / `cmca_allocate_recursive` schema mismatch
  surfaced by CE-GALL-40's own receipt stays open, same as the two
  uncorrected-in-place contradiction chains (14/19, 9) above — flagged
  as follow-up candidates, touched by nothing else.
- Checkpoints 1, 2, 4, 5, 6, 8, 9, 12's `PARTIAL_ALIVE`/`ALIVE` claims
  leave this pass exactly as they entered it — neither confirmed nor
  refuted, exactly as verifiable (or not) as before. "UNVERIFIABLE FROM
  RECORD" reads on the record, not a downgrade stamped on the checkpoint.
