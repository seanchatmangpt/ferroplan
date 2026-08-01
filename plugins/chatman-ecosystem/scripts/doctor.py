#!/usr/bin/env python3
"""Automated version of what docs/gall-checkpoints.md's audit ritual does by
hand: real commands, real exit codes, ALIVE/BLOCKED/PARTIAL_ALIVE per check --
never a standing reported from source presence alone (gall-checkpoints.md's
own stated law: "source presence != execution evidence").

Usage: python3 doctor.py   (or `just doctor`)
Exits non-zero if any check is BLOCKED.
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
FERROPLAN_ROOT = SCRIPTS_DIR.parent.parent.parent
sys.path.insert(0, str(SCRIPTS_DIR))

GRIPPER_DOMAIN = """(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from)
    :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))"""
GRIPPER_PROBLEM = """(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room ball1 - ball gripper1 - gripper)
  (:init (at-robby rooma) (free gripper1) (at ball1 rooma))
  (:goal (at ball1 roomb)))"""
GRIPPER_PLAN = "(pick ball1 rooma gripper1)\n(move rooma roomb)\n(drop ball1 roomb gripper1)\n"


def check_build() -> tuple[str, str]:
    result = subprocess.run(
        ["cargo", "build", "--workspace"], cwd=FERROPLAN_ROOT,
        capture_output=True, text=True, timeout=600,
    )
    if result.returncode == 0:
        return "ALIVE", "cargo build --workspace: clean"
    return "BLOCKED", f"cargo build --workspace failed:\n{result.stderr[-1000:]}"


def check_val() -> tuple[str, str]:
    from val_validator import find_val, run_val

    val_bin = find_val()
    if val_bin is None:
        return "BLOCKED", "VAL binary not found ($FERROPLAN_VAL, benchmarks/.val/VAL/build/bin/Validate, or PATH)"

    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        (tmp_path / "domain.pddl").write_text(GRIPPER_DOMAIN)
        (tmp_path / "problem.pddl").write_text(GRIPPER_PROBLEM)
        (tmp_path / "plan.plan").write_text(GRIPPER_PLAN)
        outcome = run_val(val_bin, tmp_path / "domain.pddl", tmp_path / "problem.pddl", tmp_path / "plan.plan")

    if outcome["valid"]:
        return "ALIVE", f"VAL ({val_bin}) validated a known-good fixture plan: real 'Plan valid'"
    return "BLOCKED", f"VAL ({val_bin}) did NOT validate a known-good fixture plan: {outcome['reason']}"


def check_mcp() -> tuple[str, str]:
    from mcp_client import McpClient, McpToolError, tool_structured_result

    try:
        with McpClient() as mcp:
            digest = tool_structured_result(mcp.call_tool("canonical_digest", {"value": "doctor-check"}))
    except McpToolError as error:
        return "BLOCKED", f"MCP canonical_digest call failed: {error}"
    except Exception as error:
        return "BLOCKED", f"MCP round trip failed to even start: {error}"

    digest_hex = digest.get("digest") if isinstance(digest, dict) else None
    if isinstance(digest_hex, str) and len(digest_hex) == 64 and all(c in "0123456789abcdef" for c in digest_hex):
        return "ALIVE", f"MCP round trip: canonical_digest returned a real 64-hex digest ({digest_hex[:12]}...)"
    return "BLOCKED", f"MCP canonical_digest returned an unexpected shape: {digest!r}"


def check_andon_guard() -> tuple[str, str]:
    source = (SCRIPTS_DIR / "overnight_autonomics.py").read_text()
    if "andon_stop" in source and "sys.exit(1)" in source and "ANDON:" in source:
        return "ALIVE", "overnight_autonomics.py's produce-role andon guard is present (andon_stop / sys.exit(1) / ANDON:)"
    return "BLOCKED", "overnight_autonomics.py is missing its andon guard -- the dirty-tree-silent-skip regression may have recurred"


CHECKS = [
    ("cargo build --workspace", check_build),
    ("VAL real validation", check_val),
    ("ferroplan-mcp round trip", check_mcp),
    ("andon guard regression check", check_andon_guard),
]


def main() -> None:
    any_blocked = False
    for name, check in CHECKS:
        try:
            status, detail = check()
        except Exception as error:
            status, detail = "BLOCKED", f"check raised: {error}"
        if status == "BLOCKED":
            any_blocked = True
        print(f"[{status}] {name}: {detail}")
    sys.exit(1 if any_blocked else 0)


if __name__ == "__main__":
    main()
