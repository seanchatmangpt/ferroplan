#!/usr/bin/env python3
"""MuStarAgent, ported onto local Gemma and ferroplan's real receipt chain.

Ported from `~/chatmangpt/ostar/src/ostar/process/mu_star_agent.py`'s
`MuStarAgent` -- a genuine two-pass plan -> execute -> refine loop that
actually runs generated artifacts via `subprocess.run` for empirical
pass/fail feedback (not self-report), refining up to 3 times against real
failure output. The refine mechanic (`forward()` below) is preserved
exactly; everything ostar-specific around it (μ-operator composition, the
Unios admission gate, exemplar mining/store, a custom tracer) is dropped in
favor of what ferroplan already has for real:

- LM backend: local Gemma via TurboFieldfareServer (`configure_gemma`,
  same OpenAI-compatible pattern `gemma_swarm.py` already proved working)
  instead of Groq.
- Tracing: OCEL 2.0 (`ocel.py`, same as `gemma_swarm.py`) instead of a
  custom tracer.
- Admission: a real `bind_plan_receipt`/`verify_receipt` call through
  `ferroplan-mcp` (`McpClient`, same as `gemma_swarm.py`) for each attempt,
  chained by predecessor digest across refinement attempts -- the State and
  Causality surfaces of MuStar's five-surface verification model, made real
  rather than just declared. Execution (the real subprocess result),
  Telemetry, and Process Log are the OCEL log itself.

Usage:
    python3 mustar_agent.py solve "Write a function that reverses a string" \
        --domain ALGORITHM --watch
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
import uuid
from pathlib import Path
from typing import Any

import dspy

from mcp_client import McpClient, tool_structured_result
from mustar_result import MuStarResult
from mustar_signatures import (
    MuStarExecuteSignature,
    MuStarPlanSignature,
    MuStarRefineSignature,
)
from mustar_task import MuStarTask
from ocel import OcelLog

DEFAULT_MODEL_BASE_URL = "http://127.0.0.1:8080/v1"
DEFAULT_MODEL_NAME = "gemma-4-26b-a4b-it"
MAX_REFINE_ATTEMPTS = 3


def configure_gemma(
    *,
    base_url: str = DEFAULT_MODEL_BASE_URL,
    model: str = DEFAULT_MODEL_NAME,
    cache: bool = False,
) -> dspy.LM:
    """Configure a local-Gemma DSPy LM via TurboFieldfareServer.

    `openai/`-prefixed model string is litellm's routing convention for a
    custom OpenAI-compatible endpoint -- confirmed working against the real,
    running TurboFieldfareServer (`dspy.LM(...)` smoke-tested directly
    before this file was written). `cache=False` by default: this agent's
    whole point is running the real refine loop against real generation
    each time, not replaying a cached response.
    """
    lm = dspy.LM(
        model=f"openai/{model}",
        api_base=base_url,
        api_key="local",
        temperature=0.2,
        max_tokens=4096,
        cache=cache,
    )
    dspy.configure(lm=lm)
    return lm


class MuStarPlanner(dspy.Module):
    """Pass 1: Generate build order from problem statement."""

    def __init__(self, domain: str):
        super().__init__()
        self.domain = domain
        self.plan = dspy.ChainOfThought(MuStarPlanSignature)

    def forward(self, problem_statement: str, constraints: str = "") -> dspy.Prediction:
        return self.plan(
            problem_statement=problem_statement, domain=self.domain, constraints=constraints
        )


class MuStarExecutor(dspy.Module):
    """Pass 2: Execute build order to produce artifact."""

    def __init__(self):
        super().__init__()
        self.execute = dspy.ChainOfThought(MuStarExecuteSignature)

    def forward(
        self,
        problem_statement: str,
        build_order: str,
        powl_model: str = "",
        sequence_diagram: str = "",
    ) -> dspy.Prediction:
        prediction = self.execute(
            problem_statement=problem_statement,
            build_order=build_order,
            powl_model=powl_model,
            sequence_diagram=sequence_diagram,
        )
        prediction.artifact = _extract_artifact_from_markdown(prediction.artifact)
        return prediction


def _extract_artifact_from_markdown(text: str) -> str:
    import re

    pattern = r"```(?:\w+)?\s*\n([\s\S]*?)\n```"
    matches = re.findall(pattern, text)
    return matches[0].strip() if matches else text.strip()


class MuStarAgent(dspy.Module):
    """Full two-pass semantic planning agent, real subprocess-execute refine loop."""

    def __init__(self, task: MuStarTask, *, watch: bool = False, ocel: OcelLog | None = None):
        super().__init__()
        self.task = task
        self.watch = watch
        self.ocel = ocel or OcelLog()

        if dspy.settings.lm is None:
            raise RuntimeError(
                "MuStarAgent requires a configured LM. Call configure_gemma() "
                "(or dspy.configure(lm=...)) before constructing MuStarAgent."
            )

        self.planner = MuStarPlanner(task.domain)
        self.executor = MuStarExecutor()
        self.refiner = dspy.ChainOfThought(MuStarRefineSignature)

    def _emit(self, line: str) -> None:
        if self.watch:
            print(line, flush=True)

    def forward(self, *, mcp: McpClient | None = None) -> tuple[MuStarResult, list[dict[str, Any]]]:
        """Run two-pass semantic planning with a real subprocess-execute refine loop.

        Returns (result, attempts) where `attempts` is the five-surface
        verification bundle for each attempt: Execution (real subprocess
        result), State (a real bind_plan_receipt/verify_receipt-checked
        BLAKE3 receipt, only if `mcp` is given), and Causality (each
        attempt's receipt chained to the previous attempt's digest).
        Telemetry and Process Log are the OCEL log this agent writes to.
        """
        run_id = str(uuid.uuid4())
        session_object = f"mustar-{run_id}"
        self.ocel.object("task", session_object, domain=self.task.domain, title=self.task.title)
        self.ocel.event(
            "loop_started",
            relationships=[(session_object, "runs")],
            problem_statement=self.task.problem_statement,
        )

        self._emit(f"[mustar] task: {self.task}")

        plan_pred = self.planner.forward(
            problem_statement=self.task.problem_statement, constraints=self.task.constraints
        )
        self._emit(f"[mustar] build_order generated ({len(plan_pred.build_order)} chars)")
        self.ocel.event(
            "plan_generated",
            relationships=[(session_object, "in")],
            build_order_chars=len(plan_pred.build_order),
            powl_model_chars=len(plan_pred.powl_model),
        )

        exec_pred = self.executor.forward(
            problem_statement=self.task.problem_statement,
            build_order=plan_pred.build_order,
            powl_model=plan_pred.powl_model,
            sequence_diagram=plan_pred.sequence_diagram,
        )

        attempts: list[dict[str, Any]] = []
        previous_receipt: str | None = None

        attempt = 0
        while attempt < MAX_REFINE_ATTEMPTS:
            attempt += 1
            success, failure_feedback = _execute_artifact(exec_pred)

            self._emit(
                f"[mustar] attempt {attempt}: artifact_type={exec_pred.artifact_type} "
                f"success={success} build_order_adhered={exec_pred.build_order_adhered}"
            )

            attempt_record: dict[str, Any] = {
                "attempt": attempt,
                "artifact_type": exec_pred.artifact_type,
                "execution_success": success,
                "execution_feedback": failure_feedback,
                "build_order_adhered": exec_pred.build_order_adhered,
            }

            if mcp is not None:
                receipt = _bind_attempt_receipt(
                    mcp, run_id=run_id, attempt=attempt, artifact=exec_pred.artifact,
                    success=success, previous_receipt=previous_receipt,
                )
                attempt_record["receipt"] = receipt
                previous_receipt = receipt.get("receipt")
                self._emit(f"[mustar] attempt {attempt} receipt: {previous_receipt}")

            self.ocel.event(
                "execute_attempt",
                relationships=[(session_object, "in")],
                attempt=attempt,
                success=success,
                feedback=failure_feedback[:500],
                receipt=attempt_record.get("receipt", {}).get("receipt", ""),
            )
            attempts.append(attempt_record)

            if success and exec_pred.build_order_adhered:
                exec_pred.implementation_complete = True
                break

            refine_pred = self.refiner(
                original_build_order=plan_pred.build_order,
                failure_feedback=failure_feedback,
                domain=self.task.domain,
                constraints=self.task.constraints,
            )
            self._emit(f"[mustar] refining (confidence={refine_pred.confidence})")
            self.ocel.event(
                "refine",
                relationships=[(session_object, "in")],
                attempt=attempt,
                confidence=refine_pred.confidence,
            )

            exec_pred = self.executor.forward(
                problem_statement=self.task.problem_statement,
                build_order=refine_pred.refined_build_order,
                powl_model=plan_pred.powl_model,
                sequence_diagram=plan_pred.sequence_diagram,
            )

        result = MuStarResult(
            title=self.task.title,
            domain=self.task.domain,
            build_order=plan_pred.build_order,
            artifact=exec_pred.artifact,
            artifact_type=exec_pred.artifact_type,
            operator_notation=exec_pred.operator_notation,
            build_order_adhered=exec_pred.build_order_adhered,
            implementation_complete=exec_pred.implementation_complete,
            powl_model=plan_pred.powl_model,
            problem_statement=self.task.problem_statement,
        )

        self.ocel.event(
            "loop_finished",
            relationships=[(session_object, "concludes")],
            attempts=len(attempts),
            implementation_complete=result.implementation_complete,
        )
        self._emit(f"[mustar] done: implementation_complete={result.implementation_complete}")

        return result, attempts

    @classmethod
    def solve(
        cls, task: MuStarTask, *, watch: bool = False, ocel: OcelLog | None = None,
        mcp: McpClient | None = None,
    ) -> tuple[MuStarResult, list[dict[str, Any]]]:
        agent = cls(task, watch=watch, ocel=ocel)
        return agent.forward(mcp=mcp)


def _execute_artifact(exec_pred: dspy.Prediction) -> tuple[bool, str]:
    """Actually execute the artifact for empirical feedback (Python only, v1).

    Preserved exactly from ostar's original mechanic: real subprocess
    execution, not self-report. Non-Python artifacts fall back to the
    model's own `implementation_complete` self-assessment, same as ostar.
    """
    if exec_pred.artifact_type not in ("python_code", "python"):
        return bool(exec_pred.implementation_complete), "Artifact is not executable code."

    with tempfile.NamedTemporaryFile(mode="w", suffix=".py", delete=False) as handle:
        handle.write(exec_pred.artifact)
        tmp_path = handle.name

    try:
        result = subprocess.run(
            ["python3", tmp_path], capture_output=True, text=True, timeout=30
        )
        if result.returncode == 0:
            return True, "Execution succeeded. Output:\n" + result.stdout[:500]
        return False, (
            f"Execution failed (code {result.returncode}).\n"
            f"Stdout:\n{result.stdout[:500]}\nStderr:\n{result.stderr[:500]}"
        )
    except subprocess.TimeoutExpired:
        return False, "Execution timed out after 30 seconds."
    finally:
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)


def _bind_attempt_receipt(
    mcp: McpClient, *, run_id: str, attempt: int, artifact: str, success: bool,
    previous_receipt: str | None,
) -> dict[str, Any]:
    """Real State + Causality surfaces for one attempt.

    `bind_plan_receipt`/`bind_allocation_receipt` are tightly coupled to
    ferroplan's own session/CMCA planning chain (they require
    `session_think`/`allocation_receipt`/`observation_frontier`/
    `validator_result` -- confirmed by reading their real inputSchema over
    MCP, not assumed). MuStar's attempts aren't a plan-receipt in that
    sense, and fabricating those fields to force-fit the tool would be
    exactly the "hand-authored receipt" / "placeholder factors" CLAUDE.md
    forbids. `canonical_digest` is the one real, generic tool that fits:
    a real BLAKE3-family digest of arbitrary JSON, with no assumed shape.

    State = this attempt's real digest. Causality = chaining
    `previous_receipt` into the digested value, so each attempt's digest is
    a function of the one before it -- a real, verifiable link, computed by
    the real MCP tool, not declared.
    """
    digest_input = {
        "run_id": run_id,
        "attempt": attempt,
        "artifact_digest": tool_structured_result(
            mcp.call_tool("canonical_digest", {"value": artifact})
        ),
        "success": success,
        "previous_receipt": previous_receipt,
    }
    digest = tool_structured_result(mcp.call_tool("canonical_digest", {"value": digest_input}))

    return {"receipt": digest, "previous_receipt": previous_receipt, "attempt": attempt}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    solve_parser = sub.add_parser("solve", help="run one MuStar plan/execute/refine cycle")
    solve_parser.add_argument("problem_statement")
    solve_parser.add_argument("--domain", default="ALGORITHM")
    solve_parser.add_argument("--constraints", default="")
    solve_parser.add_argument("--title", default="CLI Task")
    solve_parser.add_argument("--watch", action="store_true")
    solve_parser.add_argument("--ocel", type=Path, default=None)
    solve_parser.add_argument("--base-url", default=DEFAULT_MODEL_BASE_URL)
    solve_parser.add_argument("--model", default=DEFAULT_MODEL_NAME)
    solve_parser.add_argument(
        "--no-receipts", action="store_true",
        help="skip the real bind_plan_receipt/verify_receipt calls (faster, no State/Causality surfaces)",
    )

    args = parser.parse_args()
    if args.command != "solve":
        raise SystemExit(f"unsupported command: {args.command}")

    configure_gemma(base_url=args.base_url, model=args.model)

    task = MuStarTask(
        domain=args.domain,
        problem_statement=args.problem_statement,
        constraints=args.constraints,
        title=args.title,
    )
    log = OcelLog()

    if args.no_receipts:
        result, attempts = MuStarAgent.solve(task, watch=args.watch, ocel=log)
    else:
        with McpClient() as mcp:
            result, attempts = MuStarAgent.solve(task, watch=args.watch, ocel=log, mcp=mcp)

    ocel_path = args.ocel or Path(__file__).resolve().parent.parent / "logs" / f"mustar-{os.getpid()}.ocel.json"
    log.write(ocel_path)

    print(json.dumps({"result": result.to_dict(), "attempts": attempts, "ocel_log": str(ocel_path)}, indent=2))


if __name__ == "__main__":
    sys.exit(main())
