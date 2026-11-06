---
id: TASK-42.3
title: Document weak items visualisation for owners
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T10:49:00Z'
updated_date: '2026-11-06T11:04:00Z'
labels:
  - docs
  - chatgpt
  - ui
dependencies:
  - TASK-42.2
references:
  - README.md
  - .agents/skills/menuyukti-mcp/SKILL.md
  - .agents/skills/menuyukti-visualize-weak-items/SKILL.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-42
priority: medium
type: docs
ordinal: 94000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners already know `get_weak_items_review` returns weak-quadrant JSON and Codex can draw HTML via the visualisation skill. Without docs for the render/widget path, ChatGPT may only summarise JSON or write a local HTML file while `make widgets-preview` and ChatGPT Desktop mounting go unused.

Document how to request the mounted weak items table, that sales+COGS are required, and which path to prefer in ChatGPT vs Codex (widget when the host mounts MCP Apps; Codex skill otherwise — never both in one turn).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 docs/howtos/workspaces.md (or equivalent owner howto) mentions the weak items visualisation / render tool
- [x] #2 README or MCP-facing docs briefly note how to request the weak items table in ChatGPT and that preview lists it under `make widgets-preview`
- [x] #3 Coaching / visualisation skills are updated so show/table weak-items asks prefer the widget when the host mounts MCP Apps, without double-drawing
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update workspaces howto + README chart table.
2. Update coaching/viz skills + sync plugin skills.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated workspaces howto, README, workspaces README, menuyukti-mcp + visualize-weak-items skills; synced plugin skills.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs and skills document render_weak_items_review, preview listing, and one-path show/table routing (widget vs Codex skill).
<!-- SECTION:FINAL_SUMMARY:END -->
