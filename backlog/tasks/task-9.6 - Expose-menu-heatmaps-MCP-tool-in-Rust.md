---
id: TASK-9.6
title: Expose menu heatmaps MCP tool in Rust
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:02:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.3
  - TASK-9.4
references:
  - src/menuyukti_gpt/server.py
  - packages/menuyukti/src/menuyukti/core/analytics/calculate_menu_heatmaps.py
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
- [ ] #1 get_menu_heatmaps returns per-menu daily and weekly heatmap payloads for a path under the data dir
- [ ] #2 Empty or no-datetime inputs follow a documented empty/error behavior consistent with useful ChatGPT answers
- [ ] #3 Tests cover heatmap structure against a known POS fixture
<!-- AC:END -->
