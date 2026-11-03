#!/usr/bin/env bash
# Lint the Rust workspace with clippy (warnings as errors).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo clippy --workspace --all-targets --locked -- -D warnings
