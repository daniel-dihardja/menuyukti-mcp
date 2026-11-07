#!/usr/bin/env bash
# Skybridge MCP with Alpic HTTPS tunnel — needed for Codex Desktop widgets.
# Codex often times out loading widget JS/CSS from http://127.0.0.1; HTTPS assets work.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
APP="$ROOT/apps/menuyukti"

export MENUYUKTI_DATA_DIR="${MENUYUKTI_DATA_DIR:-$ROOT/workspaces}"
export DO_NOT_TRACK="${DO_NOT_TRACK:-1}"
export SKYBRIDGE_TELEMETRY_DISABLED="${SKYBRIDGE_TELEMETRY_DISABLED:-1}"

if ! command -v node >/dev/null 2>&1; then
  echo "error: Node.js is required (Skybridge needs Node 24+)" >&2
  exit 1
fi

NODE_MAJOR="$(node -p "process.versions.node.split('.')[0]")"
if [[ "$NODE_MAJOR" -lt 24 ]]; then
  echo "error: Node.js 24+ required (found $(node -v))." >&2
  exit 1
fi

if ! command -v pnpm >/dev/null 2>&1; then
  echo "error: pnpm is required" >&2
  exit 1
fi

if [[ ! -d "$ROOT/node_modules" && ! -d "$APP/node_modules" ]]; then
  (cd "$ROOT" && pnpm install)
fi

cd "$APP"
echo "MENUYUKTI_DATA_DIR=$MENUYUKTI_DATA_DIR"
echo "Starting Skybridge with --tunnel. Copy the printed https://…/mcp URL into Codex MCP settings"
echo "(replace http://127.0.0.1:8787/mcp), then start a new Codex session."
exec pnpm dev:tunnel
