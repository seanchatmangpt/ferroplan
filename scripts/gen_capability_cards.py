#!/usr/bin/env python3
"""Generate one v1.0 A2A-style agent card per ferroplan-wasm op.

Source of truth: crates/ferroplan-wasm/registry/capability-registry.json
(the content-addressed qri:CapabilityContract). Cards are never hand-authored;
edit the registry and rerun.

Deterministic: double run is byte-identical (fixed field order, no timestamps).

Usage: python3 scripts/gen_capability_cards.py [--check]
"""

import hashlib
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
REGISTRY = REPO / "crates/ferroplan-wasm/registry/capability-registry.json"
CARDS = REPO / "crates/ferroplan-wasm/cards"
MANIFEST = CARDS / "MANIFEST.sha256"

CRATE_VERSION = "0.29.0"

LIMITS_TEXT = (
    "Limits: max request 16 MiB (16777216 bytes), max JSON depth 128, "
    "16 MiB stack. Requests exceeding these are refused with "
    "FP_LIMIT_INPUT (bytes/depth) or FP_LIMIT_SEARCH/FP_LIMIT_MEMORY at runtime."
)
AUTHORITY_LAW = (
    "Authority law: planner CONSTRUCT - candidate output, never actuation. "
    "Returned plans are candidates for host-side admission; the planner holds "
    "no DO authority."
)

OP_NOTES = {
    "plan": "Dev/inspect op: no request_id/authority/validation envelope.",
    "plan_production": "Production envelope: schema_version, request_id, "
    "input_fingerprint, authority, validation, counters.",
    "readiness": "Reports admission_state, manifest_fingerprint and contract "
    "validity; the qualification surface of the capability.",
    "version": "Returns the module version string.",
}


def sha256_file(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def op_tag(op_name: str) -> str:
    """Derive the skill tag from the op family (surface's own vocabulary)."""
    if op_name.startswith("session_"):
        return "session"
    if op_name.startswith(("fond_", "htn_", "hddl_", "plan")):
        return "plan"
    if op_name == "readiness":
        return "qualification"
    if op_name == "version":
        return "metadata"
    return "op"


def build_card(reg: dict, op: dict) -> dict:
    name = op["name"]
    note = OP_NOTES.get(
        name, "Callable via fp_call with op='%s'." % name
    )
    desc = (
        "ferroplan-wasm op `%s` (ABI v%d). %s %s %s"
        % (name, reg["abi_version"], note, LIMITS_TEXT, AUTHORITY_LAW)
    )
    return {
        "a2a_version": "1.0",
        "version": CRATE_VERSION,
        "id": "ferroplan.%s" % name,
        "kind": "agent-card",
        "name": "Ferroplan capability: %s" % name,
        "description": desc,
        "supportedInterfaces": [
            {"protocolVersion": "1.0", "protocolBinding": "WASM"}
        ],
        "skills": [
            {
                "id": "ferroplan.%s" % name,
                "name": name,
                "description": desc,
                "tags": [op_tag(name)],
            }
        ],
        "source": {
            "registry": "crates/ferroplan-wasm/registry/capability-registry.json",
            "registry_sha256": sha256_file(REGISTRY),
            "abi_version": reg["abi_version"],
            "crate": reg["crate"],
            "op": name,
        },
        "signature": {
            "request_fields": list(op["request_fields"]),
            "response_fields": list(op["response_fields"]),
        },
        "refusal_vocabulary": list(reg["error_codes"]),
        "limits": {
            "max_request_bytes": reg["limits"]["max_request_bytes"],
            "max_json_depth": reg["limits"]["max_json_depth"],
        },
        "authority": {
            "authority_claim": "NONE",
            "law": AUTHORITY_LAW,
        },
        "qualification": {
            "contract_class": "qri:CapabilityContract",
            "digest_property": "qri:contractDigest",
            "receipt_class": "qri:QualificationReceipt",
            "note": "Standing is derived from receipts; cards carry none.",
        },
        "abi": {
            "exports": list(reg["exports"]),
            "imports_policy": reg["imports_policy"],
        },
    }


def main() -> int:
    check = "--check" in sys.argv
    reg = json.loads(REGISTRY.read_text())
    names = [o["name"] for o in reg["ops"]]
    CARDS.mkdir(parents=True, exist_ok=True)

    if not check:
        # Remove stale cards from retired ops.
        for old in CARDS.glob("*.json"):
            if old.stem not in names:
                old.unlink()
        for op in reg["ops"]:
            card = build_card(reg, op)
            (CARDS / ("%s.json" % op["name"])).write_text(
                json.dumps(card, indent=2) + "\n"
            )

    manifest = "".join(
        "%s  %s.json\n" % (sha256_file(CARDS / ("%s.json" % n)), n)
        for n in sorted(names)
    )
    expected = MANIFEST.read_text() if MANIFEST.exists() else None
    if check:
        if manifest != expected:
            print("drift: cards do not match registry", file=sys.stderr)
            return 1
        print("ok: %d cards match registry" % len(names))
        return 0

    MANIFEST.write_text(manifest)
    return 0


if __name__ == "__main__":
    sys.exit(main())
