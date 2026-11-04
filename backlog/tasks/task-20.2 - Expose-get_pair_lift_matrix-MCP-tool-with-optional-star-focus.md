---
id: TASK-20.2
title: Expose get_pair_lift_matrix MCP tool with optional star focus
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:32:00Z'
updated_date: '2026-11-04T08:43:00Z'
labels: []
dependencies:
  - TASK-20.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
  - crates/menuyukti/src/analytics/menu_engineering.rs
  - ../v3/apps/graphql/services/menu_combos.py
  - ../v3/apps/web/lib/analytics/load-pair-lift-matrix-for-chat.ts
parent_task_id: TASK-20
priority: high
type: feature
ordinal: 37000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Affinity math alone is not reachable from ChatGPT. Heatmaps and menu engineering already follow load-via-path + JSON MCP tools; pair lift needs the same surface. v3 chat visualisation only needs focusMenus + matrixLift (+ totals/scope). When COGS is available the cloud stack prefers menu-engineering stars as focus—local MCP should do the same without making COGS mandatory, so sales-only workspaces still get a useful matrix.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `get_pair_lift_matrix` (or equivalent) is listed by tools/list and accepts a path under MENUYUKTI_DATA_DIR (workspace folder or sales Excel, consistent with other sales tools)
- [x] #2 Successful call returns JSON including focus_menus, matrix_lift, total_orders, multi_item_order_count, and scope from local sales
- [x] #3 Sales-only / missing COGS path still succeeds with top-by-presence focus (does not hard-fail like analyze_menu_engineering)
- [x] #4 When workspace COGS loads and menu engineering yields >=2 stars, those menus are passed as focus and scope reflects stars
- [x] #5 Path gating rejects paths outside MENUYUKTI_DATA_DIR the same way other sales tools do
- [x] #6 INSTRUCTIONS and the tool description explain combo/pair-lift use and that COGS is optional for star focus
- [x] #7 MCP HTTP integration test covers a successful demo/fixture call; library affinity unit tests stay green under make check
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add get_pair_lift_matrix MCP tool: load sales via path gate; optionally load COGS + menu engineering for star focus (>=2 stars).
2. Return JSON with focus_menus, matrix_lift, totals, scope, pairs, pos; sales-only falls back to top_by_presence.
3. Update INSTRUCTIONS and tools/list expectations in mcp_http tests; assert demo-cafe success and sales Excel path.
4. make check; finalize TASK-20.2.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Wired get_pair_lift_matrix with optional_star_focus via load_workspace_cogs + menu engineering. Updated INSTRUCTIONS. MCP HTTP asserts tools/list, demo-cafe stars scope, sales.xlsx top_by_presence, and path-gate reject. Demo cafe fixture test in analytics_tools.rs. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed get_pair_lift_matrix over MCP. Sales-only uses top_by_presence; workspace+COGS with >=2 stars uses star focus. HTTP + fixture tests cover success and path rejection. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
