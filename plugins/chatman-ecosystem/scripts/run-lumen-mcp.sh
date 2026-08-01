#!/bin/sh
# Launch lumen's real MCP server (semantic code search), same stdio JSON-RPC
# transport as run-ferroplan-mcp.sh, so McpClient's existing single-arg
# subprocess spawn works unmodified -- this script's only job is resolving
# lumen's versioned plugin-cache path and appending its required `stdio` arg
# (lumen's own .claude-plugin/plugin.json declares
# `"command": "${CLAUDE_PLUGIN_ROOT}/scripts/run", "args": ["stdio"]`).
#
# Picks the highest installed version under the plugin cache rather than
# hardcoding one, so an unrelated lumen upgrade doesn't silently break this.
set -eu

cache_root="$HOME/.claude/plugins/cache/claude-plugins-official/lumen"
if [ ! -d "$cache_root" ]; then
  echo "lumen plugin not found at $cache_root" >&2
  exit 69
fi

version_dir=$(ls -1 "$cache_root" | sort -V | tail -n1)
if [ -z "$version_dir" ]; then
  echo "no lumen version installed under $cache_root" >&2
  exit 69
fi

run_script="$cache_root/$version_dir/scripts/run"
if [ ! -x "$run_script" ]; then
  echo "lumen launcher not executable: $run_script" >&2
  exit 69
fi

CLAUDE_PLUGIN_ROOT="$cache_root/$version_dir" exec "$run_script" stdio
