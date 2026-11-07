#!/usr/bin/env bash
# Quality gate for the Skybridge (TypeScript) Menuyukti MCP app.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [[ ! -d node_modules && ! -d apps/menuyukti/node_modules ]]; then
  echo "==> pnpm install (workspace)"
  pnpm install
fi

echo "==> biome check"
pnpm lint

echo "==> plugin skills sync"
bash scripts/check-plugin-skills-sync.sh

echo "==> tsc --noEmit"
pnpm --filter menuyukti typecheck

echo "==> vitest"
pnpm --filter menuyukti test

echo "All checks passed."
