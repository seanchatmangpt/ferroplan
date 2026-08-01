#!/usr/bin/env python3
"""Detects drift between docs/gall-checkpoints.md's hand-maintained prose
standings and the machine-readable receipts/CE-GALL-*.json files those
checkpoints cite -- the one automated check this ledger has never had.

The ledger's own stated law is "source presence != execution evidence";
this script applies the same discipline to the ledger itself: a receipt
that no longer matches its checkpoint's current prose standing is exactly
the kind of stale claim the ledger exists to prevent, just turned inward.

Real bugs this caught on first run (not hypothetical):
- receipts/CE-GALL-30.json still says "reason": "MOCKED", but the prose's
  sixth-pass note explicitly relabels it to "NOT_INDEPENDENT" and says the
  receipt "was not updated this pass" -- a self-admitted, now-detectable drift.
- receipts/CE-GALL-35.json does not parse as JSON at all: an unresolved git
  merge conflict (<<<<<<< HEAD) is committed directly into the file.

Usage: python3 gall_drift_check.py   (or `just doctor`, once wired in)
Exits non-zero if any DRIFT or BROKEN_RECEIPT is found.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
FERROPLAN_ROOT = SCRIPTS_DIR.parent.parent.parent
CHECKPOINTS_MD = FERROPLAN_ROOT / "docs" / "gall-checkpoints.md"
RECEIPTS_DIR = SCRIPTS_DIR.parent / "receipts"

SECTION_RE = re.compile(r"^## (\d+)\. (.+)$", re.MULTILINE)
STANDING_RE = re.compile(r"\*\*Current standing:\*\*\s*`([A-Z_]+)`")
CE_GALL_RE = re.compile(r"\bCE-GALL-(\d+)\b")


def parse_checkpoints(text: str) -> list[dict[str, object]]:
    """One entry per `## N. Title` section: its own current prose standing
    (the LAST `**Current standing:**` line in the section -- the ledger's
    own convention is append-only updates, so the last one is authoritative)
    and every CE-GALL-NN tag mentioned anywhere in the section body."""
    headers = list(SECTION_RE.finditer(text))
    sections = []
    for index, match in enumerate(headers):
        start = match.end()
        end = headers[index + 1].start() if index + 1 < len(headers) else len(text)
        body = text[start:end]
        standings = STANDING_RE.findall(body)
        tags = sorted({int(tag) for tag in CE_GALL_RE.findall(body)})
        sections.append({
            "number": match.group(1),
            "title": match.group(2).strip(),
            "current_standing": standings[-1] if standings else None,
            "ce_gall_tags": tags,
        })
    return sections


def check() -> list[dict[str, str]]:
    findings: list[dict[str, str]] = []
    if not CHECKPOINTS_MD.exists():
        return [{"status": "BLOCKED", "detail": f"{CHECKPOINTS_MD} not found"}]

    sections = parse_checkpoints(CHECKPOINTS_MD.read_text(encoding="utf-8"))

    # A CE-GALL-NN tag can be mentioned again later by unrelated summary/
    # rollup sections (e.g. a later "vX.Y Crown" section listing many prior
    # tags in one audit-log table) -- confirmed real on this ledger for
    # CE-GALL-23 etc. Only the FIRST section (document order) that cites a
    # tag is its home for standing comparison; later re-mentions are noise,
    # not a second authoritative source.
    # A section citing many distinct CE-GALL tags is a rollup/summary
    # (e.g. a later "vX.Y Crown" audit-log table listing several prior
    # checkpoints' tags together) -- confirmed real on this ledger. Its own
    # "Current standing" is an aggregate over those tags, not a second
    # authoritative source for any one tag's individual standing; diffing a
    # child receipt against the rollup's aggregate standing is a category
    # error, not real drift. Existence/parseability are still checked there
    # -- that's where CE-GALL-9/13/35 were actually caught.
    ROLLUP_TAG_THRESHOLD = 3

    home_section: dict[int, dict[str, object]] = {}
    for section in sections:
        is_rollup = len(section["ce_gall_tags"]) > ROLLUP_TAG_THRESHOLD  # type: ignore[arg-type]
        for tag_number in section["ce_gall_tags"]:  # type: ignore[union-attr]
            if tag_number not in home_section:
                home_section[tag_number] = {**section, "is_rollup_mention": is_rollup}

    seen_tags = set(home_section.keys())

    for tag_number, section in home_section.items():
            receipt_path = RECEIPTS_DIR / f"CE-GALL-{tag_number}.json"
            label = f"CE-GALL-{tag_number} (## {section['number']}. {section['title']})"

            if not receipt_path.exists():
                findings.append({"status": "BLOCKED", "detail": f"{label}: no receipt file at {receipt_path}"})
                continue

            try:
                receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            except json.JSONDecodeError as error:
                findings.append({"status": "BROKEN_RECEIPT", "detail": f"{label}: {receipt_path.name} does not parse as JSON ({error})"})
                continue

            if section.get("is_rollup_mention"):
                continue

            receipt_standing = receipt.get("standing")
            prose_standing = section["current_standing"]
            if receipt_standing and prose_standing and receipt_standing != prose_standing:
                findings.append({
                    "status": "DRIFT",
                    "detail": f"{label}: receipt says standing={receipt_standing!r}, prose's Current standing is {prose_standing!r}",
                })

    # Receipt filenames aren't all a clean `CE-GALL-<N>.json` -- some carry a
    # descriptive suffix (`CE-GALL-35.agent-tool-grants.json`). Extract the
    # leading number robustly rather than assuming the simple shape.
    receipt_files = sorted(RECEIPTS_DIR.glob("CE-GALL-*.json")) if RECEIPTS_DIR.exists() else []
    orphan_numbers: set[int] = set()
    for receipt_file in receipt_files:
        match = re.match(r"CE-GALL-(\d+)", receipt_file.stem)
        if match and int(match.group(1)) not in seen_tags:
            orphan_numbers.add(int(match.group(1)))
    for tag_number in sorted(orphan_numbers):
        findings.append({"status": "ORPHAN_RECEIPT", "detail": f"CE-GALL-{tag_number}.* receipt(s) exist but CE-GALL-{tag_number} is not cited anywhere in {CHECKPOINTS_MD.name}"})

    return findings


def main() -> None:
    findings = check()
    if not findings:
        print("[ALIVE] gall-checkpoints drift check: every cited CE-GALL-*.json receipt parses and matches its checkpoint's current prose standing")
        sys.exit(0)

    for finding in findings:
        print(f"[{finding['status']}] {finding['detail']}")
    sys.exit(1)


if __name__ == "__main__":
    main()
