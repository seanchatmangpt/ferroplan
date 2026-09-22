#!/usr/bin/env python3
"""Guard: a ticket's frontmatter standing must agree with its last History row.

FERROPLAN-26922-06. The docs/jira tickets carry their standing in two
places: the YAML frontmatter (`standing:`) and the append-only History
table at the bottom (`| ts | standing | ... |`). The History table is the
ledger — standing-as-input is read from its LAST row — while the
frontmatter is a projection for tooling. When the two disagree the
ticket's standing is undefined, which is exactly how 60 of the 67
v26.9.17 tickets drifted to `BLOCKED` in frontmatter while their ledgers
said ALIVE / PARTIAL_ALIVE.

Law enforced here (fail-closed, exit 1 with a report):

* every ticket markdown (any `*.md` under the given root, except `_`-led
  context/runbook files) that has YAML frontmatter with a `standing:` key
  must have that value equal — after whitespace stripping — to the standing
  TOKEN of the LAST `|`-delimited History row in the file (the first word
  of the row's standing cell; trailing parentheticals are annotation);
* a frontmatter ticket with no History rows is a NEW ticket, not a
  contradiction (nothing to agree with yet);
* files without frontmatter (prose survey tickets, `_`-led context docs)
  are outside this guard's law;
* rows whose standing cell is a continuation table fragment are not
  rows: a History row is a line starting with `|` whose second cell
  parses as a timestamp-ish first cell — in practice every real row's
  first cell starts with a digit (ISO-8601). Rows that fail that shape
  are ignored rather than guessed at.

Usage: python3 scripts/verify_ticket_standing.py docs/jira
"""

from __future__ import annotations

import sys
from pathlib import Path


def frontmatter_standing(text: str) -> str | None:
    """The `standing:` value from the YAML frontmatter, or None."""
    if not text.startswith("---"):
        return None
    end = text.find("\n---", 3)
    if end == -1:
        return None
    for line in text[:end].splitlines():
        stripped = line.strip()
        if stripped.startswith("standing:"):
            value = stripped[len("standing:"):].strip().strip("\"'")
            return value or None
    return None


def last_history_standing(text: str) -> str | None:
    """The standing cell of the last well-formed History row, or None."""
    last: str | None = None
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped.startswith("|"):
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if len(cells) < 2:
            continue
        if not cells[0][:1].isdigit():
            # separator rows and continuation fragments are not rows
            continue
        standing = cells[1]
        if standing and standing not in ("standing", "-"):
            # The standing TOKEN is the first word of the cell; anything
            # after it (e.g. `(downgraded by observation)`) is annotation.
            last = standing.split()[0]
    return last


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print(__doc__)
        return 2
    root = Path(argv[1])
    if not root.is_dir():
        print(f"FAIL: {root} is not a directory")
        return 2

    contradictions: list[str] = []
    checked = 0
    for path in sorted(root.rglob("*.md")):
        if path.name.startswith("_"):
            continue
        text = path.read_text(encoding="utf-8")
        front = frontmatter_standing(text)
        if front is None:
            continue
        checked += 1
        last = last_history_standing(text)
        if last is not None and last != front:
            contradictions.append(
                f"{path}: frontmatter says `{front}`, last History row says `{last}`"
            )

    if contradictions:
        print(f"FAIL: {len(contradictions)} ticket(s) disagree with their last History row:\n")
        for line in contradictions:
            print(f"  - {line}")
        print(
            "\nReconcile by appending a new dated History row (the ledger is"
            " append-only) and setting the frontmatter to the same standing."
        )
        return 1

    print(f"OK: {checked} frontmatter ticket(s) agree with their last History row")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
