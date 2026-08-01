"""MuStar task — domain-neutral specification for semantic planning.

Ported from `~/chatmangpt/ostar/src/ostar/process/mu_star_task.py`, trimmed
of ostar-specific convenience constructors that assumed ostar's own
artifact-type conventions.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class MuStarDomain(str, Enum):
    """Registered MuStar domains."""

    ALGORITHM = "ALGORITHM"
    BACKEND_API = "BACKEND_API"
    DATA_PIPELINE = "DATA_PIPELINE"
    DATABASE_DESIGN = "DATABASE_DESIGN"
    FRONTEND_COMPONENT = "FRONTEND_COMPONENT"
    ONTOLOGY = "ONTOLOGY"
    WORKFLOW = "WORKFLOW"
    SYSTEM_DESIGN = "SYSTEM_DESIGN"
    SECURITY_REVIEW = "SECURITY_REVIEW"


@dataclass(frozen=True)
class MuStarTask:
    """Domain-neutral task specification for two-pass semantic planning.

    A MuStarTask describes a problem that MuStarAgent will solve via:
    1. Plan phase (generate build order)
    2. Execute phase (generate artifact)

    The task specifies WHAT to solve and constraints; the agent determines HOW.
    """

    domain: str
    problem_statement: str
    constraints: str = ""
    title: str = ""
    context: dict[str, Any] = field(default_factory=dict)
    artifact_type: str = ""

    def __str__(self) -> str:
        """Human-readable short form."""
        domain_str = f"[{self.domain}]" if self.domain else "[?]"
        title_str = self.title or "Untitled"
        problem_short = (
            self.problem_statement[:70] + "..."
            if len(self.problem_statement) > 70
            else self.problem_statement
        )
        return f"{domain_str} {title_str}: {problem_short}"

    @classmethod
    def code(
        cls,
        title: str,
        problem_statement: str,
        constraints: str = "",
        domain: str = "ALGORITHM",
        **context,
    ) -> "MuStarTask":
        """Create a code-solving task."""
        return cls(
            domain=domain,
            title=title,
            problem_statement=problem_statement,
            constraints=constraints,
            context=context,
            artifact_type="python_code",
        )


__all__ = ["MuStarDomain", "MuStarTask"]
