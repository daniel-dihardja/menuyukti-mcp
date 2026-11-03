#!/usr/bin/env bash
# Format the Rust workspace (or check with CHECK=1).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [[ "${CHECK:-0}" == "1" ]]; then
  cargo fmt --all -- --check
else
  cargo fmt --all
fi
