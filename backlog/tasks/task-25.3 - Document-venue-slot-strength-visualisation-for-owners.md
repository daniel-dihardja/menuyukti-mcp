---
id: TASK-25.3
title: Document venue slot strength visualisation for owners
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T13:33:00Z'
updated_date: '2026-11-04T13:37:00Z'
labels:
  - docs
  - chatgpt
  - ui
dependencies:
  - TASK-25.2
parent_task_id: TASK-25
priority: medium
type: docs
ordinal: 55000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners already know `get_venue_slot_strength` returns timing numbers (TASK-22.3). Without docs for the render/widget path, ChatGPT may only summarise the JSON and owners will not know how to ask for the heatmap or that sales alone is enough.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 docs/howtos/workspaces.md (or equivalent owner howto) mentions the venue slot strength visualisation / render tool and that sales alone powers it
- [x] #2 workspaces/README.md or MCP-facing docs briefly note how to request the heatmap in ChatGPT
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update docs/howtos/workspaces.md venue slot section with render_venue_slot_strength. 2. Note heatmap ask path in workspaces/README.md.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Documented render_venue_slot_strength in docs/howtos/workspaces.md (tool list + venue slot section) and workspaces/README.md ChatGPT heatmap ask path.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs mention render_venue_slot_strength and how to request the venue slot strength heatmap in ChatGPT; sales alone is enough.
<!-- SECTION:FINAL_SUMMARY:END -->
