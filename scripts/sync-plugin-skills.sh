#!/usr/bin/env bash
# Copy authoring skills from .agents/skills into the installable plugin package.
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

mkdir -p "${DEST}"

for name in "${SKILLS[@]}"; do
  if [[ ! -d "${SRC}/${name}" ]]; then
    echo "missing skill source: ${SRC}/${name}" >&2
    exit 1
  fi
  rm -rf "${DEST}/${name}"
  mkdir -p "${DEST}/${name}"
  # Copy tree; keep skill packaging files (SKILL.md, agents/, references/, scripts/, assets/).
  if command -v rsync >/dev/null 2>&1; then
    rsync -a --delete \
      --exclude '.DS_Store' \
      "${SRC}/${name}/" "${DEST}/${name}/"
  else
    cp -R "${SRC}/${name}/." "${DEST}/${name}/"
  fi
  echo "synced ${name}"
done

echo "Plugin skills are up to date under plugins/menuyukti/skills/"
echo "Authoring source of truth remains .agents/skills/"
