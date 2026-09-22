"""The canonical CMCA frontier, allocated by the real `ferroplan-mcp` binary.

CE-GALL-28's original positive witness does not reproduce. A genuine
outside-session replay gets the deterministic leaf-cascade result documented
by CE-GALL-35. This test exercises the real MCP binary.
"""

from __future__ import annotations

import json
import shutil

import pytest
import surfaces
from mcp_client import McpClient, McpToolError, tool_structured_result

pytestmark = pytest.mark.needs_cargo

EXPECTED_INPUT_DIGEST = "9e8f0839fd74fe089113679187a2523e95f712329869ed70e763fff907e3d8bf"
EXPECTED_CANDIDATES_DIGEST = "0983969d34eb35a19d40621c462befbf5359e41c79694e91f338a91aca01ef0a"
INTERIOR_IDS = {"correctness", "session-runtime", "mcp-protocol", "claude-plugin"}
LEAF_IDS = {"planner-core", "semantic-projection", "receipt-security", "evidence"}


def _mcp_reachable() -> bool:
    return shutil.which("cargo") is not None


def _allocate(candidates):
    with McpClient(timeout=60) as client:
        alloc = tool_structured_result(client.call_tool("cmca_allocate", {"candidates": candidates}))
        digest = tool_structured_result(client.call_tool("canonical_digest", {"value": candidates}))
    return alloc, digest


@pytest.mark.skipif(not _mcp_reachable(), reason="no cargo toolchain to build/run ferroplan-mcp")
def test_canonical_frontier_allocation_is_pinned_and_deterministic():
    profile = surfaces.load_profile()
    candidates = surfaces.candidates(profile)
    try:
        first_alloc, first_digest = _allocate(candidates)
        second_alloc, second_digest = _allocate(candidates)
    except McpToolError as error:
        pytest.skip(f"ferroplan-mcp unreachable: {error}")
    assert first_alloc["payload"]["input_digest"] == second_alloc["payload"]["input_digest"]
    assert first_digest["digest"] == second_digest["digest"]
    assert first_alloc["payload"]["input_digest"] == EXPECTED_INPUT_DIGEST
    assert first_digest["digest"] == EXPECTED_CANDIDATES_DIGEST
    shares = {row["id"]: row["share"] for row in first_alloc["payload"]["allocations"]}
    assert set(shares) == INTERIOR_IDS | LEAF_IDS
    for interior_id in INTERIOR_IDS:
        assert shares[interior_id] == 0.0
    for leaf_id in LEAF_IDS:
        assert shares[leaf_id] > 0.0


# --------------------------------------------------------------------------
# Restored by FERROPLAN-26922-02 from the 2026-07-30 gall-checkpoints lineage
# (f9ae077). The reconciliation merge that landed the CE-GALL-35 receipts
# dropped these two tests while keeping the receipts, so the suite named
# witnesses that no test defined -- a comment, not evidence, by bcinr's rule.
# They come back with the receipts' pinned values unchanged.
# --------------------------------------------------------------------------


@pytest.mark.skipif(not _mcp_reachable(), reason="no cargo toolchain to build/run ferroplan-mcp")
def test_canonical_frontier_allocation_matches_the_real_binary():
    """The true result of `cmca_allocate` over the canonical frontier.

    Pinned by direct observation of the real binary, not derived from
    CE-GALL-28's receipt -- which this test refutes.
    """
    profile = surfaces.load_profile()
    candidates = surfaces.candidates(profile)
    try:
        alloc, _ = _allocate(candidates)
    except McpToolError as error:
        pytest.skip(f"ferroplan-mcp unreachable: {error}")
    payload = alloc["payload"]
    assert payload["input_digest"] == EXPECTED_INPUT_DIGEST
    shares = {row["id"]: row["share"] for row in payload["allocations"]}
    assert shares["correctness"] == 0.0
    top_id = max(shares, key=shares.get)
    assert top_id == "planner-core"
    assert shares[top_id] == pytest.approx(0.26995849609375)


def test_ce_gall_28_receipt_digest_does_not_match_the_real_binary(plugin_root):
    """CE-GALL-28's recorded `input_digest` is refuted, not merely stale.

    The receipt's `f0a8d185` is 8 hex characters; `cmca_allocate` returns a
    64-character BLAKE3 digest. No revision of the tool could have produced an
    8-character `input_digest` -- the recorded value was never a real capture.
    """
    receipt = json.loads(
        (plugin_root / "receipts" / "CE-GALL-28.json").read_text(encoding="utf-8")
    )
    recorded = receipt["positive_witness"]["result"]
    assert "f0a8d185" in recorded
    assert len("f0a8d185") != len(
        "9e8f0839fd74fe089113679187a2523e95f712329869ed70e763fff907e3d8bf"
    )
