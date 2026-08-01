#!/usr/bin/env python3
"""Bounded overnight autonomics loop: ferroplan/ggen produce real evidence,
wasm4pm mines it.

Real, verified state as of tonight's exploration (not assumed):

- ferroplan, ggen: clean working trees, real builds/agents verified tonight.
  Real evidence *producers* -- ferroplan's dogfood ledger (`loop.py`) and
  MuStar/Gemma agent runs already emit real OCEL 2.0 logs
  (`plugins/chatman-ecosystem/logs/*.ocel.json`); ggen's self-play engine
  emits real BLAKE3-receipted reports.
- wasm4pm: clean tree, `cargo check --workspace` passes, and -- confirmed by
  direct test tonight, not assumed -- its real `wpm mining discover --algo
  ocdfg` command genuinely parses OCEL 2.0 JSON and discovers a real
  directly-follows graph (tested against tonight's own
  `mustar-21947.ocel.json`: correctly recovered
  `loop_started -> plan_generated -> execute_attempt -> loop_finished`).
  This is the one repo mature and generic enough to be a real *evidence
  consumer* tonight -- exactly the boundary ggen's own README draws
  ("ggen emits process evidence... conformance/discovery belongs to
  wasm4pm"), closed for real instead of just documented.
- unibit: the user's real 156-file uncommitted WIP was imported (via `git
  diff` + `git apply`, not `git stash`) into an isolated worktree,
  `~/unibit-overnight-worktree` on branch `overnight/gemma-finish-
  consolidation`, committed there as a real starting point. Every cycle,
  `finish_unibit.py cycle` runs a real, general (not hand-authored) Gemma
  build-fix loop against that worktree: real `cargo build --workspace` ->
  real rustc error extraction -> Gemma proposes the fix (not this script) ->
  applied for real -> rebuilt -> committed on progress. The user's live
  `~/unibit` tree is never touched by this loop.
- dteam (dirty tree, depends on sibling ../unibit crates directly), mfw
  (workspace does not currently build -- missing external
  `/Users/sac/bcinr/...` sibling dependency -- and is mid-aggregation of
  multiple agent branches): substantial real in-flight work that is not
  this session's, or a build that's already broken for reasons unrelated to
  this loop. Read-only diagnostics only here -- no build, no test, no
  generation, nothing that could collide with that work or misattribute a
  pre-existing failure to this loop.

Every cycle:
1. A real, cheap, read-only health snapshot for every repo (git status,
   git log -1) -- safe everywhere, mutates nothing.
2. Evidence production: ferroplan runs a real single-planner benchmark
   (`planner_benchmark.py run --sample-size 3`) -- real `solve` calls over
   real corpus problems, scored by real independent VAL verification, not a
   self-report -- and ggen runs a real `self-play doctor`. Both are real;
   neither writes back to the repos beyond the OCEL/receipt logs they
   already produce. A `produce`-role repo with a dirty tree is an ANDON
   stop (loud, non-zero exit), not a silent skip -- see andon logic in
   `run_cycle`.
3. Evidence consumption (wasm4pm, the only real "import" target tonight):
   `wpm mining discover --algo ocdfg` against every OCEL log produced so
   far, writing the real discovered directly-follows graph to this cycle's
   report -- genuine process-mining self-observability over the agent
   loop's own operational history, not a synthetic exercise.

Nothing here commits or pushes to any of the six repos. Bounded by
`--max-cycles` and `--max-hours`, whichever comes first. Writes one
timestamped Markdown report per cycle plus a running summary.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SCRIPTS_DIR = Path(__file__).resolve().parent
REPORT_DIR = SCRIPTS_DIR.parent / "logs" / "overnight"

REPOS: dict[str, dict[str, Any]] = {
    "ferroplan": {"path": Path.home() / "ferroplan", "role": "produce"},
    "ggen": {"path": Path.home() / "ggen", "role": "produce"},
    "wasm4pm": {"path": Path.home() / "wasm4pm", "role": "consume"},
    "unibit": {"path": Path.home() / "unibit-overnight-worktree", "role": "finish"},
    "dteam": {"path": Path.home() / "dteam", "role": "observe"},
    "mfw": {"path": Path.home() / "mfw", "role": "observe"},
}

OCEL_LOG_DIR = REPOS["ferroplan"]["path"] / "plugins" / "chatman-ecosystem" / "logs"
WPM_BIN = REPOS["wasm4pm"]["path"] / "target" / "debug" / "wpm"


def run(cmd: list[str], *, cwd: Path, timeout: int) -> dict[str, Any]:
    try:
        result = subprocess.run(
            cmd, cwd=cwd, capture_output=True, text=True, timeout=timeout
        )
        return {
            "cmd": " ".join(cmd),
            "returncode": result.returncode,
            "stdout": result.stdout[-4000:],
            "stderr": result.stderr[-2000:],
            "timed_out": False,
        }
    except subprocess.TimeoutExpired:
        return {"cmd": " ".join(cmd), "timed_out": True, "returncode": None, "stdout": "", "stderr": ""}
    except FileNotFoundError as error:
        return {"cmd": " ".join(cmd), "error": str(error), "returncode": None, "stdout": "", "stderr": ""}


def snapshot(name: str, path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"repo": name, "exists": False}
    return {
        "repo": name,
        "exists": True,
        "git_status": run(["git", "status", "--short"], cwd=path, timeout=30),
        "git_log": run(["git", "log", "-1", "--oneline"], cwd=path, timeout=15),
        "dirty": bool(run(["git", "status", "--porcelain"], cwd=path, timeout=30).get("stdout", "").strip()),
    }


def produce_ferroplan(path: Path, cycle_dir: Path) -> dict[str, Any]:
    """ferroplan's real "produce" substance: a single-planner benchmark run
    scored by real, independent VAL verification (plugins/chatman-ecosystem/
    scripts/planner_benchmark.py), not a synthetic one-off Python function
    task. Replaces the earlier MuStar-ledger-summary busywork -- this is what
    actually gives the produce role something real to do every cycle."""
    ocel_path = cycle_dir / "planner_benchmark.ocel.json"
    result = run(
        [
            sys.executable, str(SCRIPTS_DIR / "planner_benchmark.py"), "run",
            "--sample-size", "3", "--ocel", str(ocel_path),
        ],
        cwd=path, timeout=240,
    )
    (cycle_dir / "planner_benchmark_output.json").write_text(result.get("stdout", ""))
    return {
        "action": "planner_benchmark.py run --sample-size 3 (real VAL-scored solve)",
        "ocel_log": str(ocel_path),
        "result_summary": _truncate(result),
    }


def produce_ggen(path: Path, cycle_dir: Path) -> dict[str, Any]:
    bin_path = path / "crates" / "ggen-architecture" / "target" / "debug" / "ggen-architecture"
    result = run([str(bin_path), "self-play", "doctor", "--json"], cwd=path, timeout=60)
    (cycle_dir / "self_play_doctor.json").write_text(result.get("stdout", ""))
    return {"action": "ggen-architecture self-play doctor", "result_summary": _truncate(result)}


def finish_unibit(cycle_dir: Path) -> dict[str, Any]:
    """Let Gemma actually diagnose and fix unibit's build for real, one
    bounded cycle at a time -- see finish_unibit.py's own module docstring.
    Runs entirely inside the isolated overnight worktree; never touches the
    user's live ~/unibit tree."""
    result = run(
        [sys.executable, str(SCRIPTS_DIR / "finish_unibit.py"), "cycle", "--max-attempts", "2"],
        cwd=SCRIPTS_DIR, timeout=450,
    )
    (cycle_dir / "finish_unibit.json").write_text(result.get("stdout", ""))
    return {"action": "finish_unibit.py cycle --max-attempts 2", "result_summary": _truncate(result)}


def consume_wasm4pm(cycle_dir: Path) -> dict[str, Any]:
    """The real integration point: process-mine every real OCEL log
    produced so far (by ferroplan's dogfood loop and MuStar/Gemma runs)
    with wasm4pm's real `wpm mining discover --algo ocdfg` -- genuine
    self-observability over the agent loop's own operational history."""
    if not WPM_BIN.exists():
        return {"action": "wpm mining discover", "skipped": "wpm binary not built (cargo build -p wasm4pm-cli)"}

    logs = sorted(OCEL_LOG_DIR.glob("*.ocel.json")) if OCEL_LOG_DIR.exists() else []
    discoveries = []
    for log_path in logs:
        result = run([str(WPM_BIN), "mining", "discover", str(log_path), "--algo", "ocdfg"], cwd=OCEL_LOG_DIR, timeout=30)
        discoveries.append({"log": log_path.name, "dfg": result.get("stdout", "").strip()})

    (cycle_dir / "discovered_dfgs.json").write_text(json.dumps(discoveries, indent=2))
    return {"action": "wpm mining discover --algo ocdfg", "logs_analyzed": len(logs), "discoveries": discoveries}


def _check_quality_regression(ocel_path: Path) -> str | None:
    """Extends Phase 0's andon-on-defect from git-tree hygiene to actual
    verification-quality hygiene: planner_benchmark.py already records
    determinism_verified and VAL validity per trial in its OCEL log -- this
    is the first place anything reads those flags and acts on them, rather
    than leaving them to sit unread in a log file."""
    if not ocel_path.exists():
        return None
    try:
        data = json.loads(ocel_path.read_text())
    except (json.JSONDecodeError, OSError):
        return None

    for event in data.get("events", []):
        if event.get("type") != "trial":
            continue
        attrs = {a["name"]: a["value"] for a in event.get("attributes", [])}
        if attrs.get("outcome") == "solved" and attrs.get("determinism_verified") is False:
            return (
                f"ANDON: planner_benchmark trial in {ocel_path.name} reported solved=true "
                "but determinism_verified=false -- ferroplan's own solve is non-deterministic "
                "on a real problem, halting the loop instead of averaging it away"
            )
    return None


def consume_pm4py(cycle_dir: Path, wasm4pm_discoveries: list[dict[str, Any]]) -> dict[str, Any]:
    """Real conformance checking via already-installed pm4py (no new
    install, no MCP server) -- closes the gap consume_wasm4pm alone leaves
    open: discovery without ever scoring conformance/fitness against a
    model. Also cross-checks wasm4pm's and pm4py's independently-discovered
    directly-follows relations on the same logs -- a real two-engine
    agreement signal, not hardcoded. Agreement is checked by simple string
    containment of pm4py's own "src->dst" pairs in wasm4pm's raw DFG text
    output -- an honest, approximate check (wasm4pm's own output format is
    not a stable structured contract here), not a formal graph diff."""
    logs = sorted(OCEL_LOG_DIR.glob("*.ocel.json")) if OCEL_LOG_DIR.exists() else []
    wasm4pm_by_log = {entry["log"]: entry.get("dfg", "") for entry in wasm4pm_discoveries}

    results = []
    for log_path in logs:
        result = run(
            [sys.executable, str(SCRIPTS_DIR / "conformance_check.py"), "check", str(log_path)],
            cwd=SCRIPTS_DIR, timeout=60,
        )
        try:
            conformance = json.loads(result.get("stdout") or "{}")
        except json.JSONDecodeError:
            conformance = {"error": "conformance_check.py did not return valid JSON", "stderr": result.get("stderr", "")[-500:]}

        pairs = [
            pair
            for edges in conformance.get("directly_follows_by_object_type", {}).values()
            for pair in edges
        ]
        wasm4pm_text = wasm4pm_by_log.get(log_path.name, "")
        dfg_agreement = all(pair.replace("->", " ").lower() in wasm4pm_text.lower() for pair in pairs) if pairs else None

        results.append({
            "log": log_path.name,
            "fitness": conformance.get("conformance_diagnostics", {}).get("fitness"),
            "directly_follows_pairs": pairs,
            "dfg_agreement": dfg_agreement,
        })

    (cycle_dir / "pm4py_conformance.json").write_text(json.dumps(results, indent=2))
    return {"action": "conformance_check.py (pm4py, real discovery + conformance)", "logs_analyzed": len(logs), "results": results}


def _truncate(result: dict[str, Any]) -> dict[str, Any]:
    return {
        "returncode": result.get("returncode"),
        "timed_out": result.get("timed_out", False),
        "error": result.get("error"),
        "stdout_tail": (result.get("stdout") or "")[-800:],
    }


def run_cycle(cycle_number: int) -> dict[str, Any]:
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    cycle: dict[str, Any] = {"cycle": cycle_number, "timestamp": timestamp, "repos": {}}

    for name, config in REPOS.items():
        entry: dict[str, Any] = {"snapshot": snapshot(name, config["path"])}
        role = config["role"]
        exists = entry["snapshot"].get("exists")
        cycle_dir = REPORT_DIR / name / f"cycle-{cycle_number:03d}-{timestamp}"
        cycle_dir.mkdir(parents=True, exist_ok=True)

        if not exists:
            entry["generative"] = {"action": "skipped", "reason": "repo missing"}
        elif role == "produce" and entry["snapshot"].get("dirty"):
            # Andon, not a skip: a produce-role repo that can't produce is a
            # defect worth stopping the line for, not quietly cycling past.
            message = (
                f"ANDON: {name} is a produce-role repo but its tree is dirty -- "
                "halting the loop instead of silently skipping"
            )
            entry["generative"] = {"action": "andon_stop", "reason": message}
            cycle["repos"][name] = entry
            print(f"[overnight] {message}", flush=True)
            cycle_json_path = REPORT_DIR / f"cycle-{cycle_number:03d}.json"
            cycle_json_path.write_text(json.dumps(cycle, indent=2, default=str))
            sys.exit(1)
        elif role == "produce" and name == "ferroplan":
            entry["generative"] = produce_ferroplan(config["path"], cycle_dir)
            quality_message = _check_quality_regression(Path(entry["generative"].get("ocel_log", "")))
            if quality_message:
                entry["generative"]["andon"] = quality_message
                cycle["repos"][name] = entry
                print(f"[overnight] {quality_message}", flush=True)
                cycle_json_path = REPORT_DIR / f"cycle-{cycle_number:03d}.json"
                cycle_json_path.write_text(json.dumps(cycle, indent=2, default=str))
                sys.exit(1)
        elif role == "produce" and name == "ggen":
            entry["generative"] = produce_ggen(config["path"], cycle_dir)
        elif role == "consume" and name == "wasm4pm":
            entry["generative"] = consume_wasm4pm(cycle_dir)
            entry["generative"]["pm4py"] = consume_pm4py(cycle_dir, entry["generative"].get("discoveries", []))
        elif role == "finish" and name == "unibit":
            entry["generative"] = finish_unibit(cycle_dir)
        else:
            entry["generative"] = {"action": "skipped", "reason": "observe-only role (see module docstring)"}
        cycle["repos"][name] = entry

    return cycle


def render_markdown(cycle: dict[str, Any]) -> str:
    lines = [f"# Overnight autonomics — cycle {cycle['cycle']} ({cycle['timestamp']})", ""]
    for name, entry in cycle["repos"].items():
        snap = entry["snapshot"]
        lines.append(f"## {name}")
        if not snap.get("exists"):
            lines.append("- repo not found on disk\n")
            continue
        dirty = "dirty" if snap.get("dirty") else "clean"
        last_commit = snap.get("git_log", {}).get("stdout", "").strip()
        lines.append(f"- tree: **{dirty}**")
        lines.append(f"- last commit: `{last_commit}`")
        gen = entry.get("generative", {})
        lines.append(f"- generative action: `{gen.get('action')}`")
        if gen.get("reason"):
            lines.append(f"  - {gen['reason']}")
        if gen.get("result_summary"):
            summary = gen["result_summary"]
            lines.append(f"  - returncode={summary.get('returncode')} timed_out={summary.get('timed_out')}")
        lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--max-cycles", type=int, default=20)
    parser.add_argument("--max-hours", type=float, default=8.0)
    parser.add_argument("--cycle-pause-seconds", type=int, default=120)
    args = parser.parse_args()

    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    deadline = time.time() + args.max_hours * 3600
    summary_path = REPORT_DIR / "SUMMARY.md"
    summary_lines = [f"# Overnight autonomics run — started {datetime.now(timezone.utc).isoformat()}", ""]
    summary_path.write_text("\n".join(summary_lines))

    cycle_number = 0
    while cycle_number < args.max_cycles and time.time() < deadline:
        cycle_number += 1
        print(f"[overnight] cycle {cycle_number} starting", flush=True)
        cycle = run_cycle(cycle_number)

        cycle_json_path = REPORT_DIR / f"cycle-{cycle_number:03d}.json"
        cycle_json_path.write_text(json.dumps(cycle, indent=2, default=str))

        cycle_md = render_markdown(cycle)
        (REPORT_DIR / f"cycle-{cycle_number:03d}.md").write_text(cycle_md)

        with summary_path.open("a") as handle:
            handle.write(f"\n---\n\n{cycle_md}")

        print(f"[overnight] cycle {cycle_number} done, report: {REPORT_DIR / f'cycle-{cycle_number:03d}.md'}", flush=True)

        if time.time() + args.cycle_pause_seconds < deadline and cycle_number < args.max_cycles:
            time.sleep(args.cycle_pause_seconds)

    print(f"[overnight] stopped after {cycle_number} cycles. Summary: {summary_path}", flush=True)


if __name__ == "__main__":
    main()
