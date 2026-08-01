#!/usr/bin/env python3
"""Bridge: local Gemma (TurboFieldfareServer) drives one chatman-ecosystem
agent's MCP tool-calling loop, exposed over a minimal A2A HTTP surface.

This exists because Claude Code's subagent `model:` field only accepts
Claude models -- it cannot point at an arbitrary OpenAI-compatible endpoint.
To let TurboFieldfare's Gemma model be the LLM that drives a
chatman-ecosystem agent (e.g. `ferroplan-planner`), the agent loop has to
run outside Claude Code entirely. This script is that loop:

1. Load an agent definition (`agents/<name>.md` frontmatter + system prompt).
2. List `ferroplan-mcp`'s tools (via McpClient.list_tools) and keep only the
   ones the agent's frontmatter `tools:` allows, converting each MCP
   `inputSchema` into an OpenAI `function` tool schema. Also lists lumen's
   real semantic-search MCP server's tools (health_check/index_status/
   semantic_search) and merges them in, unconditionally (read-only, not
   gated by the agent's ferroplan-specific allowlist) unless `--no-lumen`
   is passed -- so Gemma can ground generated PDDL/Mermaid/docs in real
   repository symbols instead of inventing them.
3. Run the OpenAI-style client-side tool loop against TurboFieldfareServer's
   `/v1/chat/completions` (see docs/OPENAI_SERVER.md `## Tool calls`):
   send tools -> on `finish_reason == "tool_calls"` execute each call against
   `ferroplan-mcp` over MCP -> append `tool` results -> resend -> repeat
   until the model returns a plain text answer or a call-count ceiling hits.
4. Expose that loop as a minimal A2A surface: `GET /.well-known/agent.json`
   (agent card) and `POST /tasks` (submit a task, get the loop's final
   message back synchronously). No task queue, no streaming, no A2A-client
   delegation to other agents yet -- v1 proves one Gemma-driven agent
   end-to-end; multi-agent handoff is a follow-on slice.

Usage:
    # one-shot, no HTTP server -- prove the loop works
    python3 gemma_swarm.py run ferroplan-planner "What is the session status?"

    # A2A server for one agent
    python3 gemma_swarm.py serve ferroplan-planner --port 9001
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

from mcp_client import McpClient, McpToolError, tool_structured_result
from ocel import OcelLog

AGENTS_DIR = Path(__file__).resolve().parent.parent / "agents"
DEFAULT_MODEL_BASE_URL = "http://127.0.0.1:8080/v1"
DEFAULT_MODEL_NAME = "gemma-4-26b-a4b-it"
MAX_TOOL_ROUNDS = 8

# Real, local semantic-code-search MCP server (same one available to Claude
# Code itself), wired in so Gemma can ground generated PDDL/Mermaid/docs in
# real repository symbols and structure instead of inventing them -- read
# only, always safe to offer regardless of an agent's ferroplan-mcp
# frontmatter allowlist (see LUMEN_TOOLS below).
LUMEN_LAUNCHER = Path(__file__).resolve().parent / "run-lumen-mcp.sh"
LUMEN_TOOLS = {"health_check", "index_status", "semantic_search"}

# Real, tool-validated ferroplan PDDL fixture -- the exact domain/problem
# `crates/ferroplan-mcp/tests/dogfood_chain.rs` (lines 57-59) exercises
# end-to-end. Used to seed a real `session_open` before handing the agent a
# task, so session_status/session_think/session_advance calls the model
# makes have a real session to act on instead of failing on a missing
# `session_id`.
SEED_DOMAIN = (
    "(define (domain loc) (:requirements :strips) "
    "(:predicates (at-a) (at-b) (at-c)) "
    "(:action ab :precondition (at-a) :effect (and (not (at-a)) (at-b))) "
    "(:action bc :precondition (at-b) :effect (and (not (at-b)) (at-c))))"
)
SEED_PROBLEM = "(define (problem locp) (:domain loc) (:init (at-a)) (:goal (at-c)))"

# MCP tool names in agent frontmatter are namespaced as
# `mcp__plugin_chatman-ecosystem_ferroplan__<tool>` by Claude Code's own MCP
# wiring. `ferroplan-mcp` itself exposes bare tool names, so strip the prefix
# when filtering, and re-add it (or not) is irrelevant -- Gemma only ever
# sees the bare name.
MCP_TOOL_PREFIX = "mcp__plugin_chatman-ecosystem_ferroplan__"


class AgentDefinition:
    def __init__(self, name: str, system_prompt: str, allowed_tools: set[str]):
        self.name = name
        self.system_prompt = system_prompt
        self.allowed_tools = allowed_tools


def load_agent(name: str) -> AgentDefinition:
    path = AGENTS_DIR / f"{name}.md"
    if not path.exists():
        raise SystemExit(f"no agent definition at {path}")
    text = path.read_text()
    match = re.match(r"^---\n(.*?)\n---\n(.*)$", text, re.DOTALL)
    if not match:
        raise SystemExit(f"{path} is missing YAML frontmatter")
    frontmatter, body = match.group(1), match.group(2)

    tools_line = ""
    for line in frontmatter.splitlines():
        if line.startswith("tools:"):
            tools_line = line[len("tools:") :].strip()
            break
    allowed = set()
    for raw in tools_line.split(","):
        raw = raw.strip()
        if raw.startswith(MCP_TOOL_PREFIX):
            allowed.add(raw[len(MCP_TOOL_PREFIX) :])
    return AgentDefinition(name=name, system_prompt=body.strip(), allowed_tools=allowed)


def _sanitize_schema(schema: Any, defs: dict[str, Any]) -> Any:
    """Rewrite a `schemars`-generated JSON Schema into the subset
    TurboFieldfareServer's grammar-constrained tool-call decoder accepts.

    Confirmed by direct experiment against the real server (curl against
    /v1/chat/completions with single-tool payloads): it 500s with
    "generation failed" on (a) `$ref`/`$defs`/`oneOf` union schemas, and
    (b) empty `{}` "any JSON value" schemas -- concrete `"type": "object"`
    and a flattened `"enum"` both work. `ferroplan-mcp`'s real schemas (see
    e.g. the `Mode` enum in session_open/solve/decompose, and the
    intentionally-untyped `envelope`/`value`/`candidates` fields in
    verify_receipt/canonical_digest/bind_allocation_receipt) use exactly
    these two constructs, so every tool call the model attempts on them
    fails without this rewrite. This is a real compatibility transform
    between two real schema dialects, not a stand-in for either side.
    """
    if not isinstance(schema, dict):
        return schema

    if "$ref" in schema:
        ref_name = schema["$ref"].rsplit("/", 1)[-1]
        return _sanitize_schema(defs.get(ref_name, {"type": "string"}), defs)

    if "oneOf" in schema or "anyOf" in schema:
        branches = schema.get("oneOf", schema.get("anyOf", []))
        consts = [b.get("const") for b in branches if isinstance(b, dict) and "const" in b]
        if consts and len(consts) == len(branches):
            return {"type": "string", "enum": consts}
        # Non-const union (e.g. a real polymorphic value): first branch is the
        # closest single concrete type the grammar decoder can constrain to.
        return _sanitize_schema(branches[0], defs) if branches else {"type": "string"}

    result = {k: v for k, v in schema.items() if k not in ("$schema", "$defs")}

    schema_type = result.get("type")
    if isinstance(schema_type, list):
        # Grammar decoder rejects union types; drop `null` and keep the
        # concrete type since these fields are never in `required` anyway.
        concrete = [t for t in schema_type if t != "null"]
        result["type"] = concrete[0] if concrete else "string"
    elif schema_type is None and "properties" not in result and "enum" not in result:
        # An empty `{}` schema (arbitrary JSON) -- the decoder needs a
        # concrete type. `object` matches every real use in ferroplan-mcp
        # (receipts, candidates, envelopes are all JSON objects).
        result["type"] = "object"

    if "properties" in result:
        result["properties"] = {k: _sanitize_schema(v, defs) for k, v in result["properties"].items()}
    if "items" in result:
        result["items"] = _sanitize_schema(result["items"], defs)

    return result


def mcp_tools_as_openai_functions(client: McpClient, allowed: set[str]) -> list[dict[str, Any]]:
    functions = []
    for tool in client.list_tools():
        if tool["name"] not in allowed:
            continue
        raw_schema = tool.get("inputSchema", {"type": "object", "properties": {}})
        defs = raw_schema.get("$defs", {})
        functions.append(
            {
                "type": "function",
                "function": {
                    "name": tool["name"],
                    "description": tool.get("description", ""),
                    "parameters": _sanitize_schema(raw_schema, defs),
                },
            }
        )
    return functions


def _build_tool_routing(
    ferroplan_mcp: McpClient, allowed: set[str],
    lumen_mcp: McpClient | None,
) -> tuple[list[dict[str, Any]], dict[str, McpClient]]:
    """Merge ferroplan-mcp's allowed tools with lumen's real semantic-search
    tools (always offered when a lumen client is passed -- read-only, not
    gated by an agent's ferroplan-specific frontmatter allowlist) into one
    OpenAI-function list, plus a name -> client map so each tool call gets
    dispatched to the MCP server that actually owns it."""
    tools = mcp_tools_as_openai_functions(ferroplan_mcp, allowed)
    routing = {t["function"]["name"]: ferroplan_mcp for t in tools}

    if lumen_mcp is not None:
        lumen_tools = mcp_tools_as_openai_functions(lumen_mcp, LUMEN_TOOLS)
        tools.extend(lumen_tools)
        routing.update({t["function"]["name"]: lumen_mcp for t in lumen_tools})

    return tools, routing


def run_agent_loop(
    agent_name: str,
    user_task: str,
    *,
    base_url: str = DEFAULT_MODEL_BASE_URL,
    model: str = DEFAULT_MODEL_NAME,
    watch: bool = False,
    ocel_path: Path | None = None,
    with_lumen: bool = True,
) -> dict[str, Any]:
    """Runs the full tool-calling loop once and returns
    {"answer": str, "tool_calls": [...]} -- the trace of every MCP call made
    along the way, for auditability.

    `watch=True` prints each round's raw model output and tool call/result to
    stdout as it happens, so you can see Gemma think instead of only the
    final answer. Every round is also recorded as an OCEL 2.0 event log
    (session/agent/model/tool objects, llm_completion/tool_call events) and
    written to `ocel_path` (default `logs/gemma-<agent>-<pid>.ocel.json`)
    whether or not `watch` is set -- OCEL is the durable trace, `watch` is
    just its live tail.

    `with_lumen=True` (default) also wires in lumen's real semantic-search
    MCP server (the same one available to Claude Code itself) alongside
    ferroplan-mcp -- read-only, offered regardless of the agent's
    frontmatter tool allowlist, so Gemma can ground generated PDDL/Mermaid/
    docs in real repository symbols instead of inventing them."""
    from openai import OpenAI

    agent = load_agent(agent_name)
    openai_client = OpenAI(base_url=base_url, api_key="local")
    messages: list[dict[str, Any]] = [{"role": "system", "content": agent.system_prompt}]
    trace: list[dict[str, Any]] = []

    log = OcelLog()
    import os
    import time

    session_id = f"session-{agent_name}-{int(time.time())}-{os.getpid()}"
    log.object("session", session_id, agent=agent_name, task=user_task)
    log.object("agent", agent_name, kind="chatman-ecosystem-subagent")
    log.object("model", model, base_url=base_url)
    log.event(
        "loop_started",
        relationships=[(session_id, "runs_in"), (agent_name, "driven_by"), (model, "backed_by")],
        task=user_task,
    )

    def emit(line: str) -> None:
        if watch:
            print(line, flush=True)

    emit(f"[{agent_name}] task: {user_task}")

    from contextlib import ExitStack

    with ExitStack() as stack:
        mcp = stack.enter_context(McpClient())
        lumen_mcp = stack.enter_context(McpClient(launcher=LUMEN_LAUNCHER)) if with_lumen else None
        tools, tool_routing = _build_tool_routing(mcp, agent.allowed_tools, lumen_mcp)
        emit(f"[{agent_name}] {len(tools)} tools available: {', '.join(t['function']['name'] for t in tools)}")

        # Seed a real, tool-validated session before the model ever sees a
        # prompt -- so any session_status/session_think/session_advance call
        # the model decides to make has a real session_id to act on. This is
        # a real ferroplan session (same domain/problem the Rust test suite
        # validates end-to-end), not a canned tool result.
        if "session_open" in agent.allowed_tools:
            emit(f"[{agent_name}] seeding session_open({session_id})")
            open_result = mcp.call_tool(
                "session_open",
                {"session_id": session_id, "domain": SEED_DOMAIN, "problem": SEED_PROBLEM},
            )
            open_content = json.dumps(tool_structured_result(open_result))
            emit(f"[{agent_name}] session_open ok: {open_content[:200]}")
            log.event(
                "tool_call",
                relationships=[(session_id, "in"), ("session_open", "invokes")],
                round=0,
                arguments=json.dumps({"session_id": session_id, "domain": SEED_DOMAIN, "problem": SEED_PROBLEM}),
                result=open_content,
            )
            messages.append(
                {"role": "user", "content": f"Session '{session_id}' is open. {user_task}"}
            )
        else:
            messages.append({"role": "user", "content": user_task})

        for round_number in range(1, MAX_TOOL_ROUNDS + 1):
            emit(f"\n--- round {round_number}: asking gemma ---")
            response = openai_client.chat.completions.create(
                model=model,
                messages=messages,  # type: ignore[arg-type]
                tools=tools,  # type: ignore[arg-type]
                tool_choice="auto",
                temperature=0,
            )
            choice = response.choices[0]
            messages.append(choice.message.model_dump(exclude_none=True))

            call_names = [c.function.name for c in (choice.message.tool_calls or [])]
            emit(f"[{agent_name}] gemma says: {choice.message.content or '(no text)'}")
            if call_names:
                emit(f"[{agent_name}] gemma wants tools: {call_names}")
            log.event(
                "llm_completion",
                relationships=[(session_id, "in"), (model, "generated_by")],
                round=round_number,
                finish_reason=choice.finish_reason,
                content=choice.message.content or "",
                tool_calls=json.dumps(call_names),
            )

            if choice.finish_reason != "tool_calls":
                log.event(
                    "loop_finished",
                    relationships=[(session_id, "concludes")],
                    round=round_number,
                    answer=choice.message.content or "",
                )
                if ocel_path is None:
                    ocel_path = Path(__file__).resolve().parent.parent / "logs" / f"gemma-{agent_name}-{os.getpid()}.ocel.json"
                log.write(ocel_path)
                emit(f"\n[{agent_name}] done. OCEL log: {ocel_path}")
                return {"answer": choice.message.content or "", "tool_calls": trace, "ocel_log": str(ocel_path)}

            for call in choice.message.tool_calls or []:
                arguments = json.loads(call.function.arguments or "{}")
                emit(f"[{agent_name}] -> calling {call.function.name}({arguments})")
                target_client = tool_routing.get(call.function.name, mcp)
                source_label = "lumen" if call.function.name in LUMEN_TOOLS else "ferroplan-mcp"
                log.object("tool", call.function.name, source=source_label)
                try:
                    result = target_client.call_tool(call.function.name, arguments)
                    content = json.dumps(tool_structured_result(result))
                    emit(f"[{agent_name}] <- {call.function.name} ok: {content[:200]}")
                except McpToolError as error:
                    content = json.dumps({"error": str(error)})
                    emit(f"[{agent_name}] <- {call.function.name} ERROR: {error}")
                trace.append({"tool": call.function.name, "arguments": arguments, "result": content})
                log.event(
                    "tool_call",
                    relationships=[(session_id, "in"), (call.function.name, "invokes"), (agent_name, "on_behalf_of")],
                    round=round_number,
                    arguments=json.dumps(arguments),
                    result=content,
                )
                messages.append({"role": "tool", "tool_call_id": call.id, "content": content})

    log.event("loop_finished", relationships=[(session_id, "concludes")], round=MAX_TOOL_ROUNDS, answer="(round limit)")
    if ocel_path is None:
        ocel_path = Path(__file__).resolve().parent.parent / "logs" / f"gemma-{agent_name}-{os.getpid()}.ocel.json"
    log.write(ocel_path)
    return {
        "answer": "(stopped: exceeded tool-call round limit)",
        "tool_calls": trace,
        "ocel_log": str(ocel_path),
    }


def build_agent_card(agent: AgentDefinition, *, url: str) -> dict[str, Any]:
    """Minimal A2A agent card -- enough for a peer to discover this agent
    and its declared skills. Not the full A2A spec surface (no auth schemes,
    no streaming capability declaration) -- v1 scope."""
    return {
        "name": f"gemma-{agent.name}",
        "description": agent.system_prompt.splitlines()[0] if agent.system_prompt else agent.name,
        "url": url,
        "version": "0.1.0",
        "capabilities": {"streaming": False, "pushNotifications": False},
        "skills": [
            {
                "id": agent.name,
                "name": agent.name,
                "description": f"Drives {agent.name} via local Gemma + ferroplan-mcp tools",
                "tools": sorted(agent.allowed_tools),
            }
        ],
    }


def serve(agent_name: str, port: int, *, base_url: str, model: str, with_lumen: bool = True) -> None:
    from fastapi import FastAPI
    from pydantic import BaseModel
    import uvicorn

    agent = load_agent(agent_name)
    app = FastAPI(title=f"gemma-{agent_name}")

    class TaskRequest(BaseModel):
        message: str

    @app.get("/.well-known/agent.json")
    def agent_card() -> dict[str, Any]:
        return build_agent_card(agent, url=f"http://127.0.0.1:{port}")

    @app.post("/tasks")
    def submit_task(task: TaskRequest) -> dict[str, Any]:
        result = run_agent_loop(agent_name, task.message, base_url=base_url, model=model, watch=True, with_lumen=with_lumen)
        return {
            "status": "completed",
            "artifacts": [{"type": "text", "text": result["answer"]}],
            "trace": result["tool_calls"],
            "ocel_log": result["ocel_log"],
        }

    uvicorn.run(app, host="127.0.0.1", port=port)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    run_parser = sub.add_parser("run", help="one-shot loop, no HTTP server")
    run_parser.add_argument("agent")
    run_parser.add_argument("task")
    run_parser.add_argument("--base-url", default=DEFAULT_MODEL_BASE_URL)
    run_parser.add_argument("--model", default=DEFAULT_MODEL_NAME)
    run_parser.add_argument("--watch", action="store_true", help="print each round live as it happens")
    run_parser.add_argument("--ocel", type=Path, default=None, help="OCEL 2.0 JSON log output path")
    run_parser.add_argument("--no-lumen", action="store_true", help="disable lumen semantic-search tools")

    serve_parser = sub.add_parser("serve", help="A2A HTTP server for one agent")
    serve_parser.add_argument("agent")
    serve_parser.add_argument("--port", type=int, default=9001)
    serve_parser.add_argument("--base-url", default=DEFAULT_MODEL_BASE_URL)
    serve_parser.add_argument("--model", default=DEFAULT_MODEL_NAME)
    serve_parser.add_argument("--no-lumen", action="store_true", help="disable lumen semantic-search tools")

    args = parser.parse_args()

    if args.command == "run":
        result = run_agent_loop(
            args.agent,
            args.task,
            base_url=args.base_url,
            model=args.model,
            watch=args.watch,
            ocel_path=args.ocel,
            with_lumen=not args.no_lumen,
        )
        print(json.dumps(result, indent=2))
    elif args.command == "serve":
        serve(args.agent, args.port, base_url=args.base_url, model=args.model, with_lumen=not args.no_lumen)


if __name__ == "__main__":
    sys.exit(main())
