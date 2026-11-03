---
id: TASK-9.6
title: Expose menu heatmaps MCP tool in Rust
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.3
  - TASK-9.4
references:
  - packages/menuyukti/src/menuyukti/core/analytics/calculate_menu_heatmaps.py
  - src/menuyukti_mcp/
  - src/menuyukti_mcp/server.py
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 15000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners ask when each dish sells most. The current get_menu_heatmaps tool answers with per-menu daily (hour) and weekly (weekday) demand from the same Excel load; Rust must keep that answer shape so ChatGPT prompts keep working.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 get_menu_heatmaps returns per-menu daily and weekly heatmap payloads for a path under the data dir
- [x] #2 Empty or no-datetime inputs follow a documented empty/error behavior consistent with useful ChatGPT answers
- [x] #3 Tests cover heatmap structure against a known POS fixture
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement calculate_menu_heatmaps over LineItem rows.
2. Wire get_menu_heatmaps MCP tool.
3. Tests for heatmap structure on ESB fixture.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Heatmaps already implemented with 9.5 analytics module; MCP tool wired; ESB fixture test asserts 24h/7d structure; empty input returns [].
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
get_menu_heatmaps returns per-menu daily/weekly demand payloads from local Excel loads. Empty inputs return an empty heatmap list; fixture tests cover structure.
<!-- SECTION:FINAL_SUMMARY:END -->
