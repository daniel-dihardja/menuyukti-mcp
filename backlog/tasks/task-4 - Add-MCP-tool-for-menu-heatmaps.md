---
id: TASK-4
title: Add MCP tool for menu heatmaps
status: Done
assignee: []
created_date: '2026-11-03T11:12:00Z'
updated_date: '2026-11-03T11:13:00Z'
labels: []
dependencies: []
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners asking when dishes sell need daily and weekly demand patterns, not only totals or raw rows. menuyukti already computes per-menu heatmaps; exposing that through a gated local MCP tool lets ChatGPT answer timing questions without sending POS files off the owner's machine.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP tool returns detected POS plus menu heatmap payloads for a path under MENUYUKTI_DATA_DIR
- [x] #2 Tool rejects paths outside MENUYUKTI_DATA_DIR the same way other sales tools do
- [x] #3 Tests cover successful heatmaps and path-gate failures
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add get_menu_heatmaps + JSON helper in sales.py that loads via load_normalized_sales and calls calculate_menu_heatmaps. 2. Register get_menu_heatmaps MCP tool with the same path-gate error handling. 3. Extend unit and MCP HTTP tests for success and path rejection. 4. Run pytest.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added get_menu_heatmaps + MCP tool; pytest tests/test_sales.py tests/test_mcp_server.py → 10 passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added get_menu_heatmaps MCP tool that returns detected POS plus per-menu daily/weekly heatmap payloads via calculate_menu_heatmaps on the gated local loader. Path gating matches other sales tools. Verified with unit and Streamable HTTP MCP tests (10 passed).
<!-- SECTION:FINAL_SUMMARY:END -->
