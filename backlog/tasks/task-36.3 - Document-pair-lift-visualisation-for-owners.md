---
id: TASK-36.3
title: Document pair lift visualisation for owners
status: Done
assignee: []
created_date: '2026-11-06T07:14:00Z'
updated_date: '2026-11-06T07:18:00Z'
labels:
  - docs
  - chatgpt
  - ui
dependencies:
  - TASK-36.2
parent_task_id: TASK-36
priority: medium
type: docs
ordinal: 80000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners already know get_pair_lift_matrix returns combo numbers (TASK-20.3). Without docs for the render/widget path, ChatGPT may only summarise the JSON and owners will not know how to ask for the heatmap or that sales alone is enough.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 docs/howtos/workspaces.md (or equivalent owner howto) mentions the pair lift visualisation / render tool and that sales alone powers it
- [x] #2 workspaces/README.md or MCP-facing docs briefly note how to request the pair lift heatmap in ChatGPT
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Document render_pair_lift_matrix in workspaces howto + README.
2. Align menuyukti-mcp skill / root README ask examples.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Documented render_pair_lift_matrix in docs/howtos/workspaces.md, workspaces/README.md, root README, and menuyukti-mcp skill (agents + plugin).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs mention render_pair_lift_matrix and how to request the pair lift heatmap in ChatGPT; sales alone is enough.
<!-- SECTION:FINAL_SUMMARY:END -->
