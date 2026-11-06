#!/usr/bin/env bash
# Sync plugin skills into the package, then reinstall Menuyukti into Codex CLI.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PLUGIN_ID="${MENUYUKTI_CODEX_PLUGIN_ID:-menuyukti@menuyukti-repo}"

if ! command -v codex >/dev/null 2>&1; then
  echo "codex CLI not found on PATH; install Codex CLI first." >&2
  exit 1
fi

bash "${ROOT}/scripts/sync-plugin-skills.sh"

echo "==> register marketplace (idempotent)"
codex plugin marketplace add "${ROOT}"

echo "==> reinstall ${PLUGIN_ID}"
codex plugin add "${PLUGIN_ID}"

echo "Codex plugin updated. Start a new Codex session so bundled skills reload."
