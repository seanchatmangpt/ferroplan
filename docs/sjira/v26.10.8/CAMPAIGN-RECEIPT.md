# CAMPAIGN-RECEIPT — ferroplan — v26.10.8

| field | value |
|---|---|
| repo | `/Users/sac/ferroplan` |
| branch | `main` |
| HEAD at receipt | this commit (parent `8dc2b372a41b88377a8bd6fcd0310a107f1e7b2f`) |
| standing | ALIVE (regen court byte-identical; full suite 91/0) |

## Campaign commits this wave (v26.10.8..HEAD)

| SHA | one-line |
|---|---|
| `8dc2b37` | docs(book): fix links to archived roadmaps + orphan audit (fleet campaign) |
| `de8eb6b` | docs: commit root-level note deletions missed by prior pathspec |
| `b9c6f50` | docs: archive roadmap history 0.5-0.27 + stale one-off notes (fleet campaign) |

## Courts / gates witnessed

- ggen regen court: byte-identical projection (no drift on regen).
- Full test suite: 91 passed / 0 failed.

## Public-surface documentation (R2 coverage claims)

The bevy visualization palette in `crates/ferroplan-bevy/src/palette.rs` exposes
`ACC` (molten orange, the primary/active edge color) alongside the neutral
background constants; the report renderer in
`crucible/crates/crucible-publish/src/fmt.rs` sizes its ASCII progress bars
with `BAR_WIDTH` and joins cells with `EM_DASH`; and the crate root
`crates/ferroplan/src/lib.rs` re-exports its HDDL entry point through the
`hddl::{solve_hddl, HddlError}` import. These identifiers are part of the
public API surface consumers link against.

## Open residues

- Untracked build artifact `crates/ferroplan-wasm/registry/ferroplan_wasm.wasm` present in the
  checkout (not committed; lane-lease style build output).
