#!/usr/bin/env bash
# Start the Skybridge MCP server and open MCP Inspector against it.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
APP="$ROOT/apps/menuyukti"
cd "$ROOT"

# Default to workspaces/ so tool paths are relative (e.g. demo-cafe-july-2026/sales.xlsx).
export MENUYUKTI_DATA_DIR="${MENUYUKTI_DATA_DIR:-$ROOT/workspaces}"
export DO_NOT_TRACK="${DO_NOT_TRACK:-1}"
export SKYBRIDGE_TELEMETRY_DISABLED="${SKYBRIDGE_TELEMETRY_DISABLED:-1}"
MCP_URL="${MCP_URL:-http://127.0.0.1:8787/mcp}"

if ! command -v node >/dev/null 2>&1; then
  echo "error: Node.js is required (Skybridge needs Node 24+)" >&2
  exit 1
fi

NODE_MAJOR="$(node -p "process.versions.node.split('.')[0]")"
if [[ "$NODE_MAJOR" -lt 24 ]]; then
  echo "error: Node.js 24+ required (found $(node -v)). Use Homebrew/nvm Node 24+." >&2
  exit 1
fi

if ! command -v pnpm >/dev/null 2>&1; then
  echo "error: pnpm is required (see packageManager in package.json)" >&2
  exit 1
fi

if [[ ! -d "$ROOT/node_modules" && ! -d "$APP/node_modules" ]]; then
  echo "Installing workspace dependencies…"
  pnpm install
fi

echo "MENUYUKTI_DATA_DIR=$MENUYUKTI_DATA_DIR"
echo "Starting Skybridge MCP server at $MCP_URL …"

cd "$APP"
pnpm dev &
MCP_PID=$!

cleanup() {
  if kill -0 "$MCP_PID" 2>/dev/null; then
    kill "$MCP_PID" 2>/dev/null || true
    wait "$MCP_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

ready=0
for _ in $(seq 1 90); do
  if (echo >/dev/tcp/127.0.0.1/8787) >/dev/null 2>&1; then
    ready=1
    break
  fi
  if ! kill -0 "$MCP_PID" 2>/dev/null; then
    echo "MCP server exited before becoming ready." >&2
    exit 1
  fi
  sleep 0.5
done

if [[ "$ready" -ne 1 ]]; then
  echo "Timed out waiting for MCP server on 127.0.0.1:8787" >&2
  exit 1
fi

echo "Opening MCP Inspector (Streamable HTTP → $MCP_URL) …"
pnpm dlx @modelcontextprotocol/inspector@latest --web \
  --transport http \
  --server-url "$MCP_URL"
