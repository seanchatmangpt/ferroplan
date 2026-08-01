#!/usr/bin/env python3
"""Real, independent PDDL plan validation via VAL (KCL-Planning/VAL).

Closes the still-open half of docs/gall-checkpoints.md section 13
"Independent PDDL Validation": ferroplan's own MCP `validate` tool checks a
plan against ferroplan's own semantics (self-validation); `bind_plan_receipt`
still binds that self-validation into every receipt's `validator_result`.
This module runs the real, separately-sourced VAL binary (vendored and built
by `benchmarks/get-val.sh`) the same way `benchmarks/run.py`/`ipc67.py`
already do, and normalizes its real stdout/exit code into a
`validator_result`-shaped dict that satisfies `bind_plan_receipt`'s only hard
requirement (a boolean `valid`/`ok` key) *and* records validator identity
(binary path + BLAKE3 digest of its bytes) -- the other still-open proof
item from section 13 ("validator executable identity is recorded").

Usage:
    python3 val_validator.py check --domain D.pddl --problem P.pddl --plan-file plan.txt [--temporal]
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any

import blake3

SCRIPTS_DIR = Path(__file__).resolve().parent
FERROPLAN_ROOT = SCRIPTS_DIR.parent.parent.parent
DEFAULT_VAL = FERROPLAN_ROOT / "benchmarks" / ".val" / "VAL" / "build" / "bin" / "Validate"


def find_val() -> Path | None:
    """Same lookup order as benchmarks/run.py/ipc67.py: $FERROPLAN_VAL, then
    the conventional local build path, then PATH."""
    env_path = os.environ.get("FERROPLAN_VAL")
    if env_path and Path(env_path).is_file():
        return Path(env_path)
    if DEFAULT_VAL.is_file():
        return DEFAULT_VAL
    which = shutil.which("Validate")
    return Path(which) if which else None


def _engine_binary_digest(val_bin: Path) -> str:
    """BLAKE3 of the validator binary's own bytes -- same hash family as
    ferroplan-mcp's own receipt chain (`digest_value`/`canonical_digest`) --
    so `validator_result` records which exact executable produced this
    verdict, not just its path (which could point at a rebuilt binary later)."""
    return blake3.blake3(val_bin.read_bytes()).hexdigest()


def run_val(
    val_bin: Path,
    domain_path: Path,
    problem_path: Path,
    plan_path: Path,
    *,
    temporal: bool = False,
    timeout: int = 60,
) -> dict[str, Any]:
    """Run VAL for real against a real domain/problem/plan file and return a
    normalized validator_result. Classification mirrors the exact substring
    checks already proven live in this session against real tampered/
    truncated/mismatched plans (docs/gall-checkpoints.md section 13)."""
    cmd = [str(val_bin)]
    if temporal:
        cmd += ["-t", "0.0005"]
    cmd += [str(domain_path), str(problem_path), str(plan_path)]

    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired as error:
        return {
            "valid": False,
            "reason": f"VAL timed out after {timeout}s",
            "engine": "VAL",
            "engine_source": "KCL-Planning/VAL",
            "engine_binary_path": str(val_bin),
            "engine_binary_digest": _engine_binary_digest(val_bin),
            "raw_stdout": (error.stdout or "")[:2000],
            "raw_stderr": (error.stderr or "")[:2000],
            "raw_returncode": None,
        }

    stdout = result.stdout or ""
    valid = result.returncode == 0 and "Plan valid" in stdout
    reason = None if valid else (stdout.strip()[-500:] or (result.stderr or "").strip()[-500:] or "VAL reported the plan invalid")

    return {
        "valid": valid,
        "reason": reason,
        "engine": "VAL",
        "engine_source": "KCL-Planning/VAL",
        "engine_binary_path": str(val_bin),
        "engine_binary_digest": _engine_binary_digest(val_bin),
        "raw_stdout": stdout[:2000],
        "raw_stderr": (result.stderr or "")[:2000],
        "raw_returncode": result.returncode,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    check_parser = sub.add_parser("check", help="validate one plan against one domain/problem with VAL")
    check_parser.add_argument("--domain", required=True, type=Path)
    check_parser.add_argument("--problem", required=True, type=Path)
    check_parser.add_argument("--plan-file", required=True, type=Path)
    check_parser.add_argument("--temporal", action="store_true")
    check_parser.add_argument("--timeout", type=int, default=60)

    args = parser.parse_args()

    if args.command == "check":
        val_bin = find_val()
        if val_bin is None:
            print(json.dumps({
                "valid": False,
                "reason": "VAL binary not found ($FERROPLAN_VAL, benchmarks/.val/VAL/build/bin/Validate, or PATH)",
                "engine": "VAL",
            }, indent=2))
            sys.exit(2)
        outcome = run_val(
            val_bin, args.domain, args.problem, args.plan_file,
            temporal=args.temporal, timeout=args.timeout,
        )
        print(json.dumps(outcome, indent=2))


if __name__ == "__main__":
    main()
