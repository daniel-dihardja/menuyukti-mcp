#!/usr/bin/env bash
# Start the local Rust MCP Apps widget preview server (mock data + reload).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PORT="${MENUYUKTI_WIDGET_PREVIEW_PORT:-8790}"
URL="http://127.0.0.1:${PORT}/"

echo "Starting widget preview at $URL …"
echo "(edit crates/menuyukti-mcp/assets/*.html — the browser reloads)"
exec cargo run -p menuyukti-mcp --bin menuyukti-widget-preview --quiet
