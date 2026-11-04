---
id: TASK-22.2
title: Expose get_venue_slot_strength MCP tool
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T09:35:00Z'
updated_date: '2026-11-04T09:40:00Z'
labels: []
dependencies:
  - TASK-22.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
  - ../v3/apps/web/lib/analytics/load-slot-demand-profile-for-chat.ts
  - ../v3/apps/agents/agents/core/chat/chart_data.py
parent_task_id: TASK-22
priority: high
type: feature
ordinal: 45000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Slot demand math alone is not reachable from ChatGPT. Heatmaps and pair lift already follow load-via-path + JSON MCP tools; venue slot strength needs the same surface for timing questions. Sales alone powers the profile—do not hard-require COGS.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `get_venue_slot_strength` (or equivalent) is listed by tools/list and accepts a path under MENUYUKTI_DATA_DIR (workspace folder or sales Excel, consistent with other sales tools)
- [x] #2 Successful call returns JSON including pos and a 35-cell slot_demand_profile (or equivalent field name) from local sales
- [x] #3 Path gating rejects paths outside MENUYUKTI_DATA_DIR the same way other sales tools do
- [x] #4 INSTRUCTIONS and the tool description explain venue timing / demand-slot use and that sales alone is enough
- [x] #5 MCP HTTP integration test covers a successful demo/fixture call; library slot unit tests stay green under make check
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Wire get_venue_slot_strength MCP tool (sales path gate, no COGS).
2. Update INSTRUCTIONS.
3. MCP HTTP + fixture assertions.
4. make check; finalize.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Wired get_venue_slot_strength in main.rs (sales path gate, no COGS). INSTRUCTIONS updated. mcp_http asserts tools/list, 35-cell demo call, and path rejection. analytics_tools venue_slot_strength_for_demo_cafe fixture added. make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed get_venue_slot_strength MCP tool returning pos + 35-cell slot_demand_profile from sales alone. MCP HTTP and fixture tests green; make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
