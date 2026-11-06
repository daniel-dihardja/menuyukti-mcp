---
id: TASK-33
title: Stop double venue-slot visualisation (widget + markdown)
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:44:00Z'
updated_date: '2026-11-06T04:45:00Z'
labels:
  - codex
  - ux
  - analytics
dependencies: []
priority: high
type: bug
ordinal: 74000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners see venue slot strength twice: MCP Apps widget from render_venue_slot_strength, then a skill-drawn markdown table (skill also said "Also" print markdown after HTML).

Make presentation exclusive: one chart per ask. Prefer widget when already mounted; otherwise HTML or markdown fallback — never both. Align MCP server playbook + menuyukti-mcp coaching so agents do not call render_* and redraw in the same turn.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Venue skill: if MCP Apps widget already rendered, narrate only — no HTML/markdown redraw
- [x] #2 Venue skill: never emit HTML and markdown for the same ask; markdown only when no widget and no HTML
- [x] #3 MCP server instructions + menuyukti-mcp skill: one presentation path per ask (render OR get+skill), never both
- [x] #4 Plugin skills synced and version bumped for reinstall
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update venue-slot SKILL + heatmap-recipe: exclusive present (widget → narrate only; else HTML XOR markdown).
2. Update menuyukti-mcp coaching + MCP SERVER_INSTRUCTIONS: one path per ask, never render + redraw.
3. Fix weak-items "Also" if same pattern; sync plugin, bump 0.1.3, reinstall; make check if Rust instructions changed.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated venue skill exclusive present; MCP SERVER_INSTRUCTIONS one-path; menuyukti-mcp coaching; weak-items Also→Else. make check passed. Plugin 0.1.3 reinstalled.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed double venue-slot charts: agents must use either render_venue_slot_strength (widget) or get_venue_slot_strength + one skill redraw (HTML XOR markdown), never both. MCP playbook and skills updated; plugin 0.1.3. Restart the local MCP binary so new server instructions load.
<!-- SECTION:FINAL_SUMMARY:END -->
