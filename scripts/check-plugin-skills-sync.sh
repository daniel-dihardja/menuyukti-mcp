#!/usr/bin/env bash
# Fail when plugins/menuyukti/skills drifts from .agents/skills (authoring source).
# Compares SKILL.md + references/; plugin-only packaging (agents/, assets/) is ignored.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="${ROOT}/.agents/skills"
DEST="${ROOT}/plugins/menuyukti/skills"

SKILLS=(
  menuyukti-mcp
  menuyukti-venue-slot-strength
  menuyukti-visualize-menu-heatmaps
  menuyukti-visualize-weak-items
)

fail=0
for name in "${SKILLS[@]}"; do
  if [[ ! -d "${SRC}/${name}" ]]; then
    echo "missing skill source: ${SRC}/${name}" >&2
    fail=1
    continue
  fi
  if [[ ! -d "${DEST}/${name}" ]]; then
    echo "missing plugin skill copy: ${DEST}/${name}" >&2
    echo "Run: pnpm sync:plugin-skills" >&2
    fail=1
    continue
  fi

  tmp="$(mktemp)"
  if ! diff -rq \
    -x '.DS_Store' \
    -x 'agents' \
    -x 'assets' \
    "${SRC}/${name}" "${DEST}/${name}" >"${tmp}" 2>&1; then
    echo "Plugin skill drift for ${name} (source of truth: .agents/skills/${name}):" >&2
    cat "${tmp}" >&2
    echo "Run: pnpm sync:plugin-skills" >&2
    fail=1
  fi
  rm -f "${tmp}"
done

if [[ "${fail}" -ne 0 ]]; then
  exit 1
fi

echo "Plugin skills match .agents/skills (SKILL.md + references)."
