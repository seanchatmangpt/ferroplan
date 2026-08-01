#!/usr/bin/env python3
"""Real, independently-computed conformance checking over the OCEL 2.0 logs
this ecosystem already produces, via pm4py -- already installed on this
machine (editable local fork, confirmed importable), so this needs zero new
installs and no new MCP server. This is library code the autonomous
`overnight_autonomics.py` process calls directly on itself; it is not a
tool exposed to Claude Code.

Closes a real, previously-100%-aspirational gap: `overnight_autonomics.py`'s
`consume_wasm4pm()` only ever ran *discovery* (`wpm mining discover`) against
real OCEL logs -- never conformance/fitness scoring against a model. Nothing
computed that with any library before this file.

Uses `discover_ocdfg`/`conformance_ocdfg` -- the same object-centric DFG
family wasm4pm's own `--algo ocdfg` targets -- so `overnight_autonomics.py`
can directly diff the two tools' discovered relations on the same log
(see `consume_pm4py`), not just produce an unrelated second number.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import pm4py


def check(ocel_path: Path) -> dict[str, Any]:
    """Real discovery + real conformance over a real OCEL 2.0 JSON log --
    the discovered model and the fitness/conformance diagnostics are both
    computed by pm4py, not self-reported by whatever produced the log."""
    ocel = pm4py.read_ocel2_json(str(ocel_path))

    ocdfg = pm4py.discover_ocdfg(ocel)
    activities = sorted(ocdfg.get("activities", []))
    directly_follows: dict[str, list[str]] = {}
    for object_type, edges in ocdfg.get("edges", {}).get("event_couples", {}).items():
        directly_follows[object_type] = sorted({f"{src}->{dst}" for (src, dst) in edges.keys()})

    diagnostics = pm4py.conformance_ocdfg(ocel, ocdfg)

    return {
        "log": str(ocel_path),
        "activities": activities,
        "directly_follows_by_object_type": directly_follows,
        "conformance_diagnostics": _jsonable(diagnostics),
        "engine": "pm4py",
        "engine_version": pm4py.__version__,
    }


def _jsonable(value: Any) -> Any:
    """pm4py conformance results can carry non-JSON-native types (sets,
    tuples); normalize for a clean, real (not truncated/opaque) report."""
    if isinstance(value, dict):
        return {str(k): _jsonable(v) for k, v in value.items()}
    if isinstance(value, (list, tuple, set)):
        return [_jsonable(v) for v in value]
    if isinstance(value, (str, int, float, bool)) or value is None:
        return value
    return str(value)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    check_parser = sub.add_parser("check")
    check_parser.add_argument("ocel_log", type=Path)
    args = parser.parse_args()

    if args.command == "check":
        print(json.dumps(check(args.ocel_log), indent=2, default=str))


if __name__ == "__main__":
    main()
