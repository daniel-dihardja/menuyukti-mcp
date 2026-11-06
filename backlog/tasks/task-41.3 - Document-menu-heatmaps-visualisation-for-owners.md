---
id: TASK-41.3
title: Document menu heatmaps visualisation for owners
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T07:51:00Z'
updated_date: '2026-11-06T07:56:00Z'
labels:
  - docs
  - chatgpt
  - ui
dependencies:
  - TASK-41.2
references:
  - README.md
  - .agents/skills/menuyukti-mcp/SKILL.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-41
priority: medium
type: docs
ordinal: 90000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners already know get_menu_heatmaps returns dish timing JSON and Codex can draw HTML via the visualisation skill. Without docs for the render/widget path, ChatGPT may only summarise JSON or write a local HTML file while make widgets-preview and ChatGPT Desktop mounting go unused.

Document how to request the mounted menu heatmaps chart, that sales alone is enough, that workspace COGS adds matrix_category filters (weak off by default), and which path to prefer in ChatGPT vs Codex.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 docs/howtos/workspaces.md (or equivalent owner howto) mentions the menu heatmaps visualisation / render tool
- [x] #2 README or MCP-facing docs briefly note how to request the menu heatmaps chart in ChatGPT and that preview lists it under make widgets-preview
- [x] #3 Coaching / visualisation skills are updated so show/chart dish timing prefers the widget when the host mounts MCP Apps, without double-drawing
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Done in same pass: workspaces howto, README, install-and-run, coaching/viz skills, workspaces README.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs and skills document render_menu_heatmaps, preview listing, and one-path show/chart routing (widget vs Codex skill).
<!-- SECTION:FINAL_SUMMARY:END -->
