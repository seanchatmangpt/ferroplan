"""Minimal OCEL 2.0 JSON writer -- just enough to log a `gemma_swarm.py` loop
run as an object-centric event log, no pm4py/OCEL library dependency.

Format follows the OCEL 2.0 JSON export shape: top-level `objectTypes`,
`eventTypes`, `objects`, `events`. Each event carries `relationships` to the
objects it touched (the agent, the model, the tool called), so a downstream
process-mining tool can replay "which objects were involved in which events"
without a bespoke parser for this script's log shape.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def _now_iso() -> str:
    import datetime

    return datetime.datetime.now(datetime.timezone.utc).isoformat()


class OcelLog:
    def __init__(self) -> None:
        self._object_types: dict[str, set[str]] = {}
        self._objects: dict[str, dict[str, Any]] = {}
        self._events: list[dict[str, Any]] = []
        self._next_event_id = 1

    def object(self, obj_type: str, obj_id: str, **attributes: Any) -> str:
        """Register (or update) an object, return its id for use in relationships."""
        self._object_types.setdefault(obj_type, set()).update(attributes.keys())
        self._objects[obj_id] = {
            "id": obj_id,
            "type": obj_type,
            "attributes": [
                {"name": name, "time": _now_iso(), "value": value}
                for name, value in attributes.items()
            ],
        }
        return obj_id

    def event(
        self,
        event_type: str,
        *,
        relationships: list[tuple[str, str]],
        **attributes: Any,
    ) -> str:
        """Log one event. `relationships` is a list of (object_id, qualifier)."""
        event_id = f"e{self._next_event_id}"
        self._next_event_id += 1
        self._events.append(
            {
                "id": event_id,
                "type": event_type,
                "time": _now_iso(),
                "attributes": [{"name": k, "value": v} for k, v in attributes.items()],
                "relationships": [
                    {"objectId": object_id, "qualifier": qualifier}
                    for object_id, qualifier in relationships
                ],
            }
        )
        return event_id

    def to_dict(self) -> dict[str, Any]:
        return {
            "objectTypes": [
                {
                    "name": type_name,
                    "attributes": [{"name": attr, "type": "string"} for attr in sorted(attrs)],
                }
                for type_name, attrs in self._object_types.items()
            ],
            "eventTypes": sorted({event["type"] for event in self._events}),
            "objects": list(self._objects.values()),
            "events": self._events,
        }

    def write(self, path: Path) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(self.to_dict(), indent=2))
