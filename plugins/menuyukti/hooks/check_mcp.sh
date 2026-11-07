#!/usr/bin/env bash
# Reachability check only — does not start the Menuyukti server or read POS files.
set -euo pipefail

URL="${MENUYUKTI_MCP_URL:-http://127.0.0.1:8787/mcp}"

if command -v curl >/dev/null 2>&1; then
  if curl -sS -o /dev/null -m 2 -X POST "$URL" \
    -H 'content-type: application/json' \
    -H 'accept: application/json, text/event-stream' \
    -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"menuyukti-plugin-hook","version":"0.1.0"}}}' \
    2>/dev/null; then
    echo "Menuyukti MCP looks reachable at ${URL}."
    exit 0
  fi
fi

cat <<EOF
Menuyukti MCP did not respond at ${URL}.
Start the local Skybridge server (pnpm mcp; MENUYUKTI_DATA_DIR set), then reconnect:
  docs/howtos/install-and-run.md
This hook never launches the server and never reads sales files.
EOF
exit 0
