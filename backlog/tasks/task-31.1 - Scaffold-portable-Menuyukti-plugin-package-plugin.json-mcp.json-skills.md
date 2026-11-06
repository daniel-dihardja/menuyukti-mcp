---
id: TASK-31.1
title: Scaffold portable Menuyukti plugin package (plugin.json, mcp.json, skills)
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:08:00Z'
updated_date: '2026-11-06T04:11:00Z'
labels:
  - codex
  - plugin
dependencies: []
references:
  - https://learn.chatgpt.com/docs/plugins
  - .agents/skills/
parent_task_id: TASK-31
priority: high
type: feature
ordinal: 69000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Codex plugins discover skills from `skills/` and MCP servers from `mcp.json` automatically when a root `plugin.json` is present. Without that layout, Menuyukti skills stay repo-local folders that owners must add one by one.

Add a portable plugin package (recommended path `plugins/menuyukti/`) with identity/metadata, OpenAI interface extensions, localhost streamable-HTTP MCP entry for Menuyukti, and bundled skill trees. Authoring remains in `.agents/skills/`; packaging should copy or sync into `plugins/menuyukti/skills/` so the installable unit is self-contained. Include brand assets placeholders under `assets/`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 plugins/menuyukti/ (or documented equivalent) has plugin.json with stable kebab-case name, version, description, and OpenAI interface metadata
- [x] #2 mcp.json declares the local Menuyukti MCP streamable-HTTP endpoint (default http://127.0.0.1:8787/mcp)
- [x] #3 skills/ includes menuyukti-mcp and the visualisation skills (venue slot, menu heatmaps, weak items); offline venue-slot skill included or explicitly deferred in notes
- [x] #4 Packaging approach keeps `.agents/skills/` as authoring source (sync script and/or documented copy step) so the plugin does not fork skill content permanently
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create plugins/menuyukti/{plugin.json,mcp.json,assets,README}.
2. Add scripts/sync-plugin-skills.sh and sync coaching + visualise + offline venue-slot skills.
3. Wire make target if present; smoke-check layout.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created plugins/menuyukti with plugin.json, mcp.json, assets, hooks; scripts/sync-plugin-skills.sh + make sync-plugin-skills. Bundled coaching, 3 visualise skills, and offline venue-slot.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Scaffolded portable plugins/menuyukti (plugin.json v0.1.0, localhost mcp.json, SVG assets, optional SessionStart hook). Sync script copies five skills from .agents/skills into plugins/menuyukti/skills. make sync-plugin-skills verified.
<!-- SECTION:FINAL_SUMMARY:END -->
