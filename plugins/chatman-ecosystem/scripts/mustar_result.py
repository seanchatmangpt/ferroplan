"""MuStar result — output of two-pass semantic planning.

Ported from `~/chatmangpt/ostar/src/ostar/process/mu_star_result.py`, minus
the AlphaStar backward-compat aliases (no legacy callers exist here).
"""

import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass
class MuStarResult:
    """Result of MuStarAgent two-pass planning and execution.

    The result contains:
    - Build order: structured plan (POWL format)
    - Artifact: generated output (code, RDF, BPMN, etc.)
    - Metadata: success indicators and artifact type info
    - powl_model: POWL control flow model (optional)
    - problem_statement: Original problem statement (optional)
    """

    title: str
    domain: str
    build_order: str
    artifact: str
    artifact_type: str
    operator_notation: str
    build_order_adhered: bool
    implementation_complete: bool
    powl_model: str = ""
    problem_statement: str = ""

    def to_dict(self) -> dict[str, Any]:
        """Convert to JSON-serializable dict."""
        return {
            "title": self.title,
            "domain": self.domain,
            "build_order": self.build_order,
            "artifact": self.artifact,
            "artifact_type": self.artifact_type,
            "operator_notation": self.operator_notation,
            "build_order_adhered": self.build_order_adhered,
            "implementation_complete": self.implementation_complete,
            "powl_model": self.powl_model,
            "problem_statement": self.problem_statement,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "MuStarResult":
        """Reconstruct from dict."""
        return cls(
            title=data["title"],
            domain=data["domain"],
            build_order=data["build_order"],
            artifact=data["artifact"],
            artifact_type=data["artifact_type"],
            operator_notation=data.get("operator_notation", "unknown"),
            build_order_adhered=data["build_order_adhered"],
            implementation_complete=data["implementation_complete"],
            powl_model=data.get("powl_model", ""),
            problem_statement=data.get("problem_statement", ""),
        )

    def save(self, path: Path | str) -> None:
        """Save result to JSON file."""
        path = Path(path)
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "w") as f:
            json.dump(self.to_dict(), f, indent=2)

    @classmethod
    def load(cls, path: Path | str) -> "MuStarResult":
        """Load result from JSON file."""
        path = Path(path)
        with open(path) as f:
            data = json.load(f)
        return cls.from_dict(data)

    def __str__(self) -> str:
        """Human-readable summary."""
        lines = [
            f"Title: {self.title}",
            f"Domain: {self.domain}",
            f"Artifact Type: {self.artifact_type}",
            f"Operator: {self.operator_notation}",
            f"Build Order Adhered: {self.build_order_adhered}",
            f"Implementation Complete: {self.implementation_complete}",
            f"Build Order Length: {len(self.build_order)} chars",
            f"Artifact Length: {len(self.artifact)} chars",
        ]
        return "\n".join(lines)


__all__ = ["MuStarResult"]
