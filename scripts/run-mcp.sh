#!/usr/bin/env bash
# Start the local Rust Menuyukti MCP server (streamable HTTP).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export MENUYUKTI_DATA_DIR="${MENUYUKTI_DATA_DIR:-$ROOT/sales-reports}"
MCP_URL="${MCP_URL:-http://127.0.0.1:8787/mcp}"

echo "MENUYUKTI_DATA_DIR=$MENUYUKTI_DATA_DIR"
echo "Starting Rust MCP server at $MCP_URL …"
exec cargo run -p menuyukti-mcp --quiet
