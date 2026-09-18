#!/usr/bin/env python3
"""Evidence-id existence verifier for the capability manifest (non-fictional rule).

Guards the rule introduced with `fp.core.fond` / `fp.core.hddl` (wave 3) and
hardened in wave 4: every evidence id in those families must name a REAL,
existing test function — rename-sensitive by construction. Thematic ids in the
other families (`core.*`, `cli.*`, `release.*`, ...) are category slugs and are
checked for verifier charset compliance only, matching the admission rules in
`docs/FORTUNE5-CAPABILITY-ADMISSION.md`.

This script is documentation/evidence-mapping enforcement only. It never
changes admission semantics: `evaluate_readiness` alone derives ADMITTED.

Exit codes: 0 = all ids verified; 1 = violation(s) found; 64 = harness misuse.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
MANIFEST_RS = REPO_ROOT / "crates" / "ferroplan" / "src" / "readiness.rs"
SCAN_DIRS = ["crates/*/src", "crates/*/tests"]

# Families whose evidence ids MUST be exact existing test function names
# (the non-fictional rule). Prefix match on `family.`.
TEST_NAME_FAMILIES = (
    "fond.",
    "hddl.",
    "fond_canonical.",
    "fond_property.",
    "fond_htn_micro.",
    "fond_htn_oracle.",
    "fond_flat_oracle.",
    "htn_oracle.",
)

EVIDENCE_CHARSET = re.compile(r"^[A-Za-z0-9._-]{1,256}$")
CONTRACT_BLOCK = re.compile(r"contract\((.*?)\n        \),", re.DOTALL)
EVIDENCE_SLICE = re.compile(r"&\[(.*?)\]", re.DOTALL)
STRING_LITERAL = re.compile(r'"([^"]+)"')
TEST_ATTR = re.compile(r"#\[(?:tokio::)?test\]")
TEST_FN = re.compile(r"\bfn\s+([A-Za-z0-9_]+)")


def manifest_evidence_ids() -> tuple[dict[str, list[str]], list[str]]:
    """Return ({capability_id: [evidence ids]}, all evidence ids in file order)."""
    text = MANIFEST_RS.read_text(encoding="utf-8")
    start = text.index("pub fn capability_manifest()")
    region = text[start:]
    end = region.index("\n    ];\n")
    region = region[:end]

    manifest: dict[str, list[str]] = {}
    order: list[str] = []
    for block in CONTRACT_BLOCK.finditer(region):
        body = block.group(1)
        cap_ids = STRING_LITERAL.findall(body)
        if not cap_ids:
            fail(f"contract block without a capability id literal near offset {block.start()}")
        slices = EVIDENCE_SLICE.findall(body)
        if not slices:
            fail(f"capability `{cap_ids[0]}` has no evidence slice")
        evidence = STRING_LITERAL.findall(slices[-1])
        manifest[cap_ids[0]] = evidence
        order.extend(evidence)
    return manifest, order


def collect_test_names() -> dict[str, list[str]]:
    """Map test function name -> [file:line] across all crate test surfaces."""
    names: dict[str, list[str]] = {}
    for pattern in SCAN_DIRS:
        for path in sorted(REPO_ROOT.glob(pattern + "/**/*.rs")):
            text = path.read_text(encoding="utf-8", errors="replace")
            for attr in TEST_ATTR.finditer(text):
                # Skip mentions of `#[test]` inside // or /// comments.
                line_start = text.rfind("\n", 0, attr.start()) + 1
                if text[line_start : attr.start()].lstrip().startswith("//"):
                    continue
                # Skip comments/other attributes between #[test] and `fn`.
                window = text[attr.end() : attr.end() + 400]
                match = TEST_FN.search(window)
                if match:
                    abs_offset = attr.end() + match.start()
                    rel = path.relative_to(REPO_ROOT)
                    line = text.count("\n", 0, abs_offset) + 1
                    names.setdefault(match.group(1), []).append(f"{rel}:{line}")
    return names


def fail(message: str) -> None:
    print(f"FAIL: {message}")
    sys.exit(64)


def main() -> int:
    if not MANIFEST_RS.is_file():
        fail(f"manifest source not found: {MANIFEST_RS}")
    manifest, order = manifest_evidence_ids()
    tests = collect_test_names()

    seen: dict[str, str] = {}
    failures: list[str] = []
    bound = 0
    thematic = 0

    for evidence in order:
        if not EVIDENCE_CHARSET.match(evidence):
            failures.append(f"`{evidence}` violates the verifier identifier charset")
            continue
        owner = next((cid for cid, ids in manifest.items() if evidence in ids), "?")
        if evidence in seen:
            failures.append(f"`{evidence}` is duplicated ({owner} and {seen[evidence]})")
            continue
        seen[evidence] = owner

        if evidence.startswith(TEST_NAME_FAMILIES):
            suffix = evidence.split(".", 1)[1]
            locations = tests.get(suffix)
            if not locations:
                failures.append(
                    f"`{evidence}` ({owner}) names no real test function — "
                    "non-fictional rule violated (renamed or fictional?)"
                )
                continue
            bound += 1
            print(f"ok  {evidence}  ({owner}) -> {', '.join(locations)}")
        else:
            thematic += 1
            print(f"ok  {evidence}  ({owner}) -> thematic slug (charset ok)")

    total = bound + thematic
    print(
        f"verify_evidence_ids: {total} ids checked "
        f"({bound} test-name-bound, {thematic} thematic), "
        f"{len(manifest)} capabilities"
    )
    if failures:
        for item in failures:
            print(f"FAIL: {item}")
        return 1
    if total == 0:
        print("FAIL: no evidence ids extracted — parser drift")
        return 1
    print("PASS: every manifest evidence id is verifiable")
    return 0


if __name__ == "__main__":
    sys.exit(main())
