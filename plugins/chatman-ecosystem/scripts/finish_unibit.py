#!/usr/bin/env python3
"""Let Gemma actually diagnose and fix unibit's build, iteratively, for real.

This is deliberately a general mechanism, not a hand-authored fix: build the
real workspace, capture the real compiler error, hand the real error and the
real file it points at to Gemma, apply whatever Gemma proposes, rebuild for
real, and let the *next* real error (or success) decide what happens next.
No part of the diagnosis or the fix is pre-computed by this script -- only
the loop (build -> extract -> propose -> apply -> verify -> repeat) is fixed.

Runs entirely inside an isolated git worktree
(`~/unibit-overnight-worktree`, branch `overnight/gemma-finish-consolidation`,
created from unibit's last commit) so the user's real `~/unibit` working
tree -- which has its own real, separate 144-file uncommitted work-in-
progress -- is never touched. Commits land only on the isolated branch, and
this script never pushes.

Usage:
    python3 finish_unibit.py cycle --max-attempts 3
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

import dspy

sys.path.insert(0, str(Path(__file__).resolve().parent))
from mustar_agent import configure_gemma  # noqa: E402

WORKTREE = Path.home() / "unibit-overnight-worktree"
MAX_ERROR_CHARS = 6000
MAX_FILE_CHARS = 20000


class RustBuildFixSignature(dspy.Signature):
    """Fix one real rustc compiler error in one real file.

    You are given the exact error rustc reported and the full current
    content of the file it points at. Propose the smallest correct change
    that resolves this specific error without breaking the file's existing
    public API unless the error itself requires an API change. Return the
    complete corrected file content, not a diff or a partial snippet --
    the caller writes your output directly over the file.
    """

    file_path: str = dspy.InputField(desc="Path (relative to the crate) of the file rustc pointed at.")
    file_content: str = dspy.InputField(desc="The file's exact current content.")
    compiler_error: str = dspy.InputField(desc="The exact rustc error block (error code, message, span, note).")

    fixed_file_content: str = dspy.OutputField(
        desc="The complete corrected file content, ready to write in place of the original."
    )
    rationale: str = dspy.OutputField(desc="One or two sentences: what was wrong and what this change does.")


def run(cmd: list[str], *, cwd: Path, timeout: int) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=timeout)


def build(worktree: Path, timeout: int = 400) -> tuple[bool, str]:
    try:
        result = run(["cargo", "build", "--workspace"], cwd=worktree, timeout=timeout)
        return result.returncode == 0, (result.stdout + result.stderr)
    except subprocess.TimeoutExpired as error:
        return False, f"cargo build timed out after {timeout}s\n{error.stdout or ''}\n{error.stderr or ''}"


def extract_first_error(build_output: str) -> dict[str, Any] | None:
    """Real parse of rustc's own error format -- no synthetic error shape."""
    lines = build_output.splitlines()
    for index, line in enumerate(lines):
        if line.startswith("error[") or line.startswith("error:"):
            block = lines[index : index + 12]
            path_line = next((candidate_line for candidate_line in block if "-->" in candidate_line), None)
            if not path_line:
                continue
            match = re.search(r"-->\s+([^\s:]+):(\d+):(\d+)", path_line)
            if not match:
                continue
            return {
                "file": match.group(1),
                "line": int(match.group(2)),
                "column": int(match.group(3)),
                "error_block": "\n".join(block)[:MAX_ERROR_CHARS],
            }
    return None


def propose_and_apply_fix(worktree: Path, error: dict[str, Any]) -> dict[str, Any]:
    file_path = worktree / error["file"]
    if not file_path.exists():
        return {"applied": False, "reason": f"file not found in worktree: {error['file']}"}

    original_content = file_path.read_text(encoding="utf-8", errors="replace")
    if len(original_content) > MAX_FILE_CHARS:
        return {"applied": False, "reason": f"file too large for a single fix pass ({len(original_content)} chars)"}

    fixer = dspy.Predict(RustBuildFixSignature)
    prediction = fixer(
        file_path=error["file"], file_content=original_content, compiler_error=error["error_block"]
    )

    fixed_content = _strip_markdown_fence(prediction.fixed_file_content)
    if not fixed_content.strip():
        return {"applied": False, "reason": "gemma returned empty content"}

    file_path.write_text(fixed_content, encoding="utf-8")
    return {
        "applied": True,
        "file": error["file"],
        "rationale": prediction.rationale,
        "bytes_before": len(original_content),
        "bytes_after": len(fixed_content),
    }


def _strip_markdown_fence(text: str) -> str:
    match = re.search(r"```(?:\w+)?\s*\n([\s\S]*?)\n```", text)
    return match.group(1) if match else text


def commit_progress(worktree: Path, message: str) -> None:
    run(["git", "add", "-A"], cwd=worktree, timeout=30)
    run(["git", "commit", "-m", message, "--no-verify"], cwd=worktree, timeout=30)


def cycle(max_attempts: int) -> dict[str, Any]:
    if not WORKTREE.exists():
        return {"error": f"worktree not found at {WORKTREE}; create it first with `git worktree add`"}

    configure_gemma()

    log: dict[str, Any] = {"attempts": []}
    for attempt in range(1, max_attempts + 1):
        success, output = build(WORKTREE)
        if success:
            log["final_status"] = "build_clean"
            if log["attempts"]:
                commit_progress(WORKTREE, f"overnight autonomics: build clean after {len(log['attempts'])} Gemma fix attempt(s)")
            return log

        error = extract_first_error(output)
        if error is None:
            log["final_status"] = "build_failed_unparseable"
            log["build_tail"] = output[-2000:]
            return log

        fix = propose_and_apply_fix(WORKTREE, error)
        log["attempts"].append({"attempt": attempt, "error": error, "fix": fix})

        if not fix.get("applied"):
            log["final_status"] = "fix_not_applied"
            return log

        commit_progress(
            WORKTREE,
            f"overnight autonomics: gemma attempt at {error['file']}:{error['line']} -- {fix.get('rationale', '')[:200]}",
        )

    log["final_status"] = "max_attempts_reached"
    return log


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    cycle_parser = sub.add_parser("cycle")
    cycle_parser.add_argument("--max-attempts", type=int, default=3)
    args = parser.parse_args()

    if args.command == "cycle":
        print(json.dumps(cycle(args.max_attempts), indent=2, default=str))


if __name__ == "__main__":
    main()
