#!/usr/bin/env python3
"""Have Gemma itself design and write the lumen-grounding integration into
mustar_agent.py -- the actual code is Gemma's real output, not hand-authored
here. This script is only the mechanism: build -> extract real error ->
Gemma proposes a complete replacement file -> apply -> verify for real
(syntax parse, then an actual `solve` run) -> repeat on real failure, same
shape as finish_unibit.py's real build-fix loop.

"Complete" means: not a dead unused helper, actually called from
MuStarAgent's real generation path (planner and/or executor), so a real
`solve` run demonstrably calls lumen's semantic_search before generating.

Usage: python3 close_loop_lumen_grounding.py run [--max-attempts 3]
"""

from __future__ import annotations

import argparse
import ast
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

import dspy

sys.path.insert(0, str(Path(__file__).resolve().parent))
from mustar_agent import DEFAULT_MODEL_BASE_URL, DEFAULT_MODEL_NAME  # noqa: E402

SCRIPTS_DIR = Path(__file__).resolve().parent
TARGET_FILE = SCRIPTS_DIR / "mustar_agent.py"
MAX_FILE_CHARS = 40_000

REQUIREMENTS = """\
Integrate real lumen semantic-code-search grounding into MuStarAgent's
generation path in this exact file, so PDDL/POWL/Mermaid/doc generation is
grounded in real repository symbols instead of invented ones.

Hard requirements (this is the "complete" bar -- a dead/unused helper does
not satisfy it):

1. Add a real MCP client connection to lumen's semantic-search server,
   reusing the exact pattern already proven in gemma_swarm.py in this same
   directory: `McpClient(launcher=Path(__file__).resolve().parent /
   "run-lumen-mcp.sh")`, used as a context manager, calling the tool named
   "semantic_search" with arguments `{"query": <str>, "path": <str>}`, and
   unwrapping the result with `tool_structured_result(...)` (already
   imported from mcp_client in this file). The launcher script
   run-lumen-mcp.sh already exists in this directory -- do not invent a
   different path or protocol.
2. Add a function `_ground_with_lumen(query: str, *, path: str | None =
   None, limit: int = 5) -> str` that opens that lumen MCP client, calls
   semantic_search, and returns the real result formatted as a plain string
   ready to prepend to a constraints/context field. It must degrade
   gracefully (return an empty string or a short "grounding unavailable"
   note) if the lumen launcher is missing or the call raises -- never let a
   lumen failure crash the whole MuStar run.
3. Wire a real call to `_ground_with_lumen` into `MuStarAgent.forward()`
   BEFORE `self.planner.forward(...)` is called, using `self.task.problem_statement`
   as the search query and appending the real grounding text it returns into
   the `constraints` string passed to `self.planner.forward(...)` (and, if
   you judge it useful, also into `self.executor.forward(...)`'s inputs) --
   real text from a real tool call must actually reach the dspy signature
   call, not just be computed and discarded.
4. Add a `--lumen-path` CLI argument (default: the ferroplan repo root,
   three parents up from this script's directory, matching the
   FERROPLAN_ROOT convention already used elsewhere in this directory) so
   the search path is configurable, and thread it through to
   `MuStarAgent`/`MuStarTask` or however you choose to wire it -- your
   design choice, as long as it reaches `_ground_with_lumen` for real.
5. Call `self._emit(...)` (the existing watch-mode logger already used
   throughout `forward()`) with a real, observable line describing the
   grounding step -- e.g. how many characters/results it returned -- right
   after `_ground_with_lumen` is called and BEFORE the planner call. This is
   not cosmetic: the verifier for this task runs `solve ... --watch` and
   greps the real stdout for evidence the grounding step actually executed;
   silently succeeding with no observable trace does not satisfy this
   requirement even if the underlying call worked.
6. Do not change any other public behavior: `MuStarTask`, `MuStarResult`,
   `MuStarPlanSignature`/`MuStarExecuteSignature`/`MuStarRefineSignature`,
   the CLI's `solve` subcommand's existing arguments, and the real
   subprocess-execute refine loop must all keep working exactly as before.

Return the COMPLETE new content of this file, ready to write in place of
the original -- not a diff, not a partial snippet, not just the new
function in isolation. It must be syntactically valid Python and must not
break `python3 mustar_agent.py solve <task> --domain ALGORITHM --watch`.
"""


class LumenGroundingIntegrationSignature(dspy.Signature):
    """Rewrite one real Python file to add a specific, fully-wired feature.

    You are given the exact current content of mustar_agent.py and a
    precise requirements spec. Design the integration yourself (function
    placement, exact prompt/format of the grounding text, error handling
    details) within the stated hard requirements, then return the complete
    corrected file content, ready to write in place of the original --
    the caller writes your output directly over the file.
    """

    current_file_content: str = dspy.InputField(desc="The file's exact current content.")
    requirements: str = dspy.InputField(desc="The precise, hard requirements this integration must satisfy.")

    updated_file_content: str = dspy.OutputField(
        desc="The complete corrected file content, ready to write in place of the original."
    )
    rationale: str = dspy.OutputField(desc="What was added, where, and why it satisfies every hard requirement.")


class LumenGroundingRefineSignature(dspy.Signature):
    """Fix one real defect in a previously-generated file, given the exact
    error that resulted from trying to use it (a syntax error, or a real
    failure running `mustar_agent.py solve` against it). Return the
    complete corrected file content, not a diff."""

    previous_file_content: str = dspy.InputField(desc="The complete content that failed.")
    requirements: str = dspy.InputField(desc="The original hard requirements, unchanged.")
    real_error: str = dspy.InputField(desc="The exact real error encountered trying to use the previous content.")

    updated_file_content: str = dspy.OutputField(
        desc="The complete corrected file content, ready to write in place of the original."
    )
    rationale: str = dspy.OutputField(desc="What was wrong and what this change fixes.")


def _strip_markdown_fence(text: str) -> str:
    import re

    match = re.search(r"```(?:\w+)?\s*\n([\s\S]*?)\n```", text)
    return match.group(1) if match else text


def _check_syntax(content: str) -> str | None:
    """Real syntax check. Returns None if valid, else the real error text."""
    try:
        ast.parse(content)
        return None
    except SyntaxError as error:
        return f"SyntaxError: {error}"


def _check_real_run(candidate_path: Path) -> str | None:
    """Real execution check: actually run `solve` against the candidate file
    (via a copy at the real module's path so its sibling imports resolve)
    and confirm it still produces a real result. Returns None if it ran
    successfully, else the real captured error output."""
    result = subprocess.run(
        [
            sys.executable, str(candidate_path), "solve",
            "Write a Python function that returns the square of an integer.",
            "--domain", "ALGORITHM", "--no-receipts", "--watch",
        ],
        cwd=SCRIPTS_DIR, capture_output=True, text=True, timeout=180,
    )
    if result.returncode != 0:
        return f"exit {result.returncode}\nstdout:\n{result.stdout[-2000:]}\nstderr:\n{result.stderr[-2000:]}"
    if "semantic_search" not in result.stdout and "lumen" not in result.stdout.lower():
        return (
            "Ran without crashing, but the real run's own --watch output shows no "
            "evidence semantic_search/lumen was actually called -- this does not "
            "satisfy requirement 3 (a real call must reach the generation path).\n"
            f"stdout tail:\n{result.stdout[-2000:]}"
        )
    return None


def run(max_attempts: int) -> dict[str, Any]:
    # A full-file rewrite (~4000 input tokens) plus a matching-size output
    # plus JSON structure overhead does not fit in mustar_agent.py's
    # configure_gemma() default max_tokens=4096 (confirmed: the first real
    # run's LM response was truncated mid-file, breaking JSON parsing).
    # 24576 leaves headroom for the ~4000-token input inside the server's
    # real 65536-token context window. Configured directly here rather than
    # reusing configure_gemma() so this harness-specific generation budget
    # doesn't change the default every other MuStar caller gets.
    lm = dspy.LM(
        model=f"openai/{DEFAULT_MODEL_NAME}", api_base=DEFAULT_MODEL_BASE_URL,
        api_key="local", temperature=0.2, max_tokens=24_576, cache=False,
    )
    dspy.configure(lm=lm)
    original_content = TARGET_FILE.read_text(encoding="utf-8")
    if len(original_content) > MAX_FILE_CHARS:
        return {"error": f"mustar_agent.py too large for a single-pass rewrite ({len(original_content)} chars)"}

    log: dict[str, Any] = {"attempts": []}
    integrator = dspy.Predict(LumenGroundingIntegrationSignature)
    refiner = dspy.Predict(LumenGroundingRefineSignature)

    prediction = integrator(current_file_content=original_content, requirements=REQUIREMENTS)
    candidate = _strip_markdown_fence(prediction.updated_file_content)

    backup_path = TARGET_FILE.with_suffix(".py.pre-lumen-close-loop.bak")
    backup_path.write_text(original_content, encoding="utf-8")

    for attempt in range(1, max_attempts + 1):
        error = _check_syntax(candidate)
        if error is None:
            TARGET_FILE.write_text(candidate, encoding="utf-8")
            error = _check_real_run(TARGET_FILE)
            if error is None:
                log["final_status"] = "closed_loop_verified"
                log["rationale"] = prediction.rationale
                log["attempts"].append({"attempt": attempt, "status": "passed_syntax_and_real_run"})
                return log
            # Real run failed -- restore original while we refine, so the
            # repo is never left mid-broken between attempts.
            TARGET_FILE.write_text(original_content, encoding="utf-8")

        log["attempts"].append({"attempt": attempt, "status": "failed", "error": error})
        if attempt == max_attempts:
            log["final_status"] = "max_attempts_reached"
            log["last_candidate_saved_to"] = str(backup_path.with_suffix(".rejected.py"))
            backup_path.with_suffix(".rejected.py").write_text(candidate, encoding="utf-8")
            return log

        refined = refiner(previous_file_content=candidate, requirements=REQUIREMENTS, real_error=error or "")
        candidate = _strip_markdown_fence(refined.updated_file_content)
        prediction = refined

    log["final_status"] = "unreachable"
    return log


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    run_parser = sub.add_parser("run")
    run_parser.add_argument("--max-attempts", type=int, default=3)
    args = parser.parse_args()

    if args.command == "run":
        print(json.dumps(run(args.max_attempts), indent=2, default=str))


if __name__ == "__main__":
    main()
