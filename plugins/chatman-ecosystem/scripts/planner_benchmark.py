#!/usr/bin/env python3
"""Single-planner PDDL benchmark, scored by a real independent validator (VAL),
receipted and OCEL-logged.

Honest scope: this is NOT a multi-planner tournament. No second real PDDL
planner (distinct search engine/config) is confirmed to exist anywhere in
this ecosystem as of this session -- claiming a tournament today would
overclaim. This runs ferroplan's own real solve against real corpus problems,
scores every plan with VAL (val_validator.py -- a genuinely independent
engine, not ferroplan's own semantics), chains a real BLAKE3 receipt per
trial (same mechanic as mustar_agent.py's `_bind_attempt_receipt`), and logs
everything to a real OCEL 2.0 file. `PLANNERS` below is a config table so a
second planner is a one-entry addition later, not a rewrite -- but only one
entry exists today.

Usage:
    python3 planner_benchmark.py run --sample-size 5 --ocel PATH
    python3 planner_benchmark.py report <ocel-log-path>
"""

from __future__ import annotations

import argparse
import json
import random
import re
import sys
from pathlib import Path
from typing import Any

SCRIPTS_DIR = Path(__file__).resolve().parent
FERROPLAN_ROOT = SCRIPTS_DIR.parent.parent.parent
sys.path.insert(0, str(SCRIPTS_DIR))
from mcp_client import McpClient, tool_structured_result  # noqa: E402
from ocel import OcelLog  # noqa: E402
from val_validator import find_val, run_val  # noqa: E402

CORPUS_DIRS = [FERROPLAN_ROOT / "benchmarks" / "ipc", FERROPLAN_ROOT / "examples"]

#: Config surface for future multi-planner extension. Today: one real entry
#: (ferroplan's own MCP `solve`). Not a working tournament until a second,
#: genuinely different planner is added here for real.
PLANNERS: dict[str, dict[str, Any]] = {
    "ferroplan": {"kind": "mcp_solve"},
}


def sample_problems(n: int, *, seed: int = 0) -> list[tuple[Path, Path]]:
    """Real domain/problem pairs from the existing corpus -- no synthetic
    generation. Pairs a problem file with its sibling domain.pddl (shared)
    or <stem>-domain.pddl (per-instance, IPC-2008 style)."""
    pairs: list[tuple[Path, Path]] = []
    for corpus_dir in CORPUS_DIRS:
        if not corpus_dir.exists():
            continue
        for problem_path in corpus_dir.rglob("p*.pddl"):
            if not re.fullmatch(r"p\d+\.pddl", problem_path.name):
                continue
            per_instance = problem_path.parent / f"{problem_path.stem}-domain.pddl"
            shared = problem_path.parent / "domain.pddl"
            domain_path = per_instance if per_instance.is_file() else shared
            if domain_path.is_file():
                pairs.append((domain_path, problem_path))

    if not pairs:
        raise SystemExit(f"no real domain/problem pairs found under {CORPUS_DIRS}")

    rng = random.Random(seed)
    rng.shuffle(pairs)
    return pairs[:n]


def run_planner(mcp: McpClient, planner_name: str, domain_src: str, problem_src: str) -> dict[str, Any]:
    """Real subprocess/tool call -- real plan or real failure, no self-report."""
    config = PLANNERS[planner_name]
    if config["kind"] != "mcp_solve":
        raise SystemExit(f"unknown planner kind {config['kind']!r}")
    try:
        solution = tool_structured_result(
            mcp.call_tool("solve", {"domain": domain_src, "problem": problem_src})
        )
        return {"outcome": "solved" if solution.get("solved") else "unsolved", "solution": solution}
    except Exception as error:
        return {"outcome": "tool_failed", "error": str(error)[:500]}


def _plan_steps_to_val_format(steps: list[dict[str, Any]]) -> str:
    return "\n".join(
        "(" + " ".join([step["action"]] + step.get("args", [])).lower() + ")" for step in steps
    ) + "\n"


def score_trial(val_bin: Path, domain_path: Path, problem_path: Path, steps: list[dict[str, Any]], tmp_dir: Path) -> dict[str, Any]:
    plan_path = tmp_dir / "trial.plan"
    plan_path.write_text(_plan_steps_to_val_format(steps))
    return run_val(val_bin, domain_path, problem_path, plan_path)


def _bind_trial_receipt(
    mcp: McpClient, *, trial_id: str, domain_digest: Any, problem_digest: Any,
    plan_digest: Any, validator_result: dict[str, Any], previous_receipt: str | None,
) -> dict[str, Any]:
    """Same canonical_digest chaining mechanic as mustar_agent.py's
    `_bind_attempt_receipt` -- generic, real, no fabricated shape."""
    digest_input = {
        "trial_id": trial_id,
        "domain_digest": domain_digest,
        "problem_digest": problem_digest,
        "plan_digest": plan_digest,
        "validator_result": validator_result,
        "previous_receipt": previous_receipt,
    }
    digest = tool_structured_result(mcp.call_tool("canonical_digest", {"value": digest_input}))
    return {"receipt": digest, "previous_receipt": previous_receipt}


def verify_determinism(mcp: McpClient, domain_src: str, problem_src: str, first_solution: dict[str, Any]) -> bool:
    """Re-run the same planner on the same problem once more; True iff the
    plan is identical (structural compare, matching self_play.rs's
    verify_report discipline -- a full re-run, not a cached hash)."""
    rerun = tool_structured_result(mcp.call_tool("solve", {"domain": domain_src, "problem": problem_src}))
    return rerun.get("plan") == first_solution.get("plan") and rerun.get("solved") == first_solution.get("solved")


def run(sample_size: int, ocel_path: Path, *, seed: int = 0) -> dict[str, Any]:
    val_bin = find_val()
    if val_bin is None:
        raise SystemExit("VAL binary not found -- run benchmarks/get-val.sh first")

    problems = sample_problems(sample_size, seed=seed)
    ocel = OcelLog()
    summary: list[dict[str, Any]] = []
    previous_receipt: str | None = None

    import tempfile

    with McpClient() as mcp:
        for index, (domain_path, problem_path) in enumerate(problems):
            trial_id = f"trial-{index}"
            domain_src = domain_path.read_text(encoding="utf-8", errors="replace")
            problem_src = problem_path.read_text(encoding="utf-8", errors="replace")

            problem_obj = ocel.object("problem", trial_id, domain_path=str(domain_path), problem_path=str(problem_path))
            planner_obj = ocel.object("planner", "ferroplan")

            for planner_name in PLANNERS:
                run_result = run_planner(mcp, planner_name, domain_src, problem_src)
                trial_entry: dict[str, Any] = {"trial_id": trial_id, "planner": planner_name, **run_result}

                if run_result["outcome"] == "solved":
                    solution = run_result["solution"]
                    steps = solution["plan"]["steps"]
                    domain_digest = tool_structured_result(mcp.call_tool("canonical_digest", {"value": domain_src}))
                    problem_digest = tool_structured_result(mcp.call_tool("canonical_digest", {"value": problem_src}))
                    plan_digest = tool_structured_result(mcp.call_tool("canonical_digest", {"value": solution["plan"]}))

                    with tempfile.TemporaryDirectory() as tmp:
                        validator_result = score_trial(val_bin, domain_path, problem_path, steps, Path(tmp))
                    trial_entry["validator_result"] = validator_result

                    determinism_ok = verify_determinism(mcp, domain_src, problem_src, solution)
                    trial_entry["determinism_verified"] = determinism_ok

                    receipt_info = _bind_trial_receipt(
                        mcp, trial_id=trial_id, domain_digest=domain_digest,
                        problem_digest=problem_digest, plan_digest=plan_digest,
                        validator_result=validator_result, previous_receipt=previous_receipt,
                    )
                    previous_receipt = receipt_info["receipt"]
                    trial_entry["receipt"] = receipt_info["receipt"]

                    ocel.event(
                        "trial", relationships=[(problem_obj, "in"), (planner_obj, "by")],
                        outcome="solved", val_valid=validator_result["valid"],
                        determinism_verified=determinism_ok, receipt=receipt_info["receipt"],
                    )
                else:
                    ocel.event(
                        "trial", relationships=[(problem_obj, "in"), (planner_obj, "by")],
                        outcome=run_result["outcome"], error=run_result.get("error"),
                    )

                summary.append(trial_entry)

    ocel.write(ocel_path)
    return {"trials": summary, "ocel_log": str(ocel_path)}


def report(ocel_path: Path) -> dict[str, Any]:
    """Tally Valid/Invalid/PlannerFailed per planner from the receipted VAL
    verdicts already written to `ocel_path` -- not from self-report."""
    data = json.loads(ocel_path.read_text())
    tally: dict[str, dict[str, int]] = {}
    determinism_flags = 0
    for event in data.get("events", []):
        if event.get("type") != "trial":
            continue
        attrs = {a["name"]: a["value"] for a in event.get("attributes", [])}
        planner = next((rel["objectId"] for rel in event["relationships"] if rel["qualifier"] == "by"), "unknown")
        bucket = tally.setdefault(planner, {"valid": 0, "invalid": 0, "unsolved": 0, "tool_failed": 0})
        if attrs.get("outcome") == "solved":
            bucket["valid" if attrs.get("val_valid") else "invalid"] += 1
            if attrs.get("determinism_verified") is False:
                determinism_flags += 1
        elif attrs.get("outcome") == "unsolved":
            bucket["unsolved"] += 1
        else:
            bucket["tool_failed"] += 1
    return {"tally": tally, "determinism_flags": determinism_flags}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    run_parser = sub.add_parser("run")
    run_parser.add_argument("--sample-size", type=int, default=5)
    run_parser.add_argument("--ocel", type=Path, required=True)
    run_parser.add_argument("--seed", type=int, default=0)

    report_parser = sub.add_parser("report")
    report_parser.add_argument("ocel_log", type=Path)

    args = parser.parse_args()

    if args.command == "run":
        args.ocel.parent.mkdir(parents=True, exist_ok=True)
        result = run(args.sample_size, args.ocel, seed=args.seed)
        print(json.dumps({"trials": len(result["trials"]), "ocel_log": result["ocel_log"]}, indent=2))
    elif args.command == "report":
        print(json.dumps(report(args.ocel_log), indent=2))


if __name__ == "__main__":
    main()
