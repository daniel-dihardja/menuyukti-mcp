#!/usr/bin/env bash
# Start the local MCP server and open MCP Inspector against it.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Default to sales-reports/ so tool paths are just filenames (e.g. demo-cafe-sales-july-2026.xlsx).
export MENUYUKTI_DATA_DIR="${MENUYUKTI_DATA_DIR:-$ROOT/sales-reports}"
MCP_URL="${MCP_URL:-http://127.0.0.1:8787/mcp}"

echo "MENUYUKTI_DATA_DIR=$MENUYUKTI_DATA_DIR"
echo "Starting Rust MCP server at $MCP_URL …"

cargo run -p menuyukti-mcp --quiet &
MCP_PID=$!

cleanup() {
  if kill -0 "$MCP_PID" 2>/dev/null; then
    kill "$MCP_PID" 2>/dev/null || true
    wait "$MCP_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

ready=0
for _ in $(seq 1 50); do
  if (echo >/dev/tcp/127.0.0.1/8787) >/dev/null 2>&1; then
    ready=1
    break
  fi
  if ! kill -0 "$MCP_PID" 2>/dev/null; then
    echo "MCP server exited before becoming ready." >&2
    exit 1
  fi
  sleep 0.2
done

if [[ "$ready" -ne 1 ]]; then
  echo "Timed out waiting for MCP server on 127.0.0.1:8787" >&2
  exit 1
fi

echo "Opening MCP Inspector (Streamable HTTP → $MCP_URL) …"
npx --yes @modelcontextprotocol/inspector@latest --web \
  --transport http \
  --server-url "$MCP_URL"
