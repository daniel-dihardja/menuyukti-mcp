---
id: TASK-40.1
title: Enrich get_menu_heatmaps items with menu-engineering category
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T07:44:00Z'
updated_date: '2026-11-06T07:47:00Z'
labels:
  - mcp
  - analytics
dependencies: []
references:
  - crates/menuyukti/src/analytics/heatmaps.rs
  - crates/menuyukti/src/analytics/menu_engineering.rs
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti/tests/analytics_tools.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
parent_task_id: TASK-40
priority: high
type: feature
ordinal: 85000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Agents and the heatmap visualisation need each dish row labeled as star / plow_horse / puzzle / weak without calling `analyze_menu_engineering` separately. Today `MenuHeatmapPayload` only carries POS `menu_category` fields and quantity grids, so BCG role filtering depends on a second tool and is easy to skip.

Reuse the existing menu-engineering classifier when the path is a workspace with usable COGS (same optional enrichment idea as pair-lift star focus). Keep heatmaps sales-only when COGS is missing: do not fail closed. Prefer a field name that cannot be confused with POS `menu_category` (for example `matrix_category`).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Each heatmap item exposes matrix category values star, plow_horse, puzzle, or weak when COGS/matrix can be computed for that workspace
- [x] #2 When COGS/matrix is unavailable, get_menu_heatmaps still returns heatmaps and the category field is null or omitted
- [x] #3 Category values match analyze_menu_engineering classifications for the same menu names on a COGS-ready fixture (e.g. demo cafe)
- [x] #4 Unit and/or MCP HTTP tests cover enrichment-present and enrichment-absent cases; make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add matrix_category: Option<String> to MenuHeatmapPayload; default None from calculate_menu_heatmaps.
2. Add library helper to apply categories from a menu→category map (keep heatmaps pure / COGS optional).
3. In get_menu_heatmaps, optionally load workspace COGS + compute matrix (same pattern as optional_star_focus) and attach categories; never fail the tool when COGS is missing.
4. Unit test: demo cafe enrichment matches analyze_menu_engineering; Excel-only path leaves category None.
5. MCP HTTP: assert demo cafe heatmaps have matrix_category aligned with matrix; sales-xlsx-only still returns heatmaps.
6. make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added matrix_category to MenuHeatmapPayload + apply_matrix_categories; get_menu_heatmaps optionally enriches via shared optional_matrix_categories (also used by star focus). Unit + MCP HTTP coverage for demo cafe match and sales-xlsx null categories.

Verified: demo cafe unit + MCP HTTP assert matrix_category matches analyze_menu_engineering; sales.xlsx path keeps matrix_category null; make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
get_menu_heatmaps now optionally enriches each heatmap item with matrix_category (star/plow_horse/puzzle/weak) when workspace COGS can classify the menu. Sales-only paths still return heatmaps with null categories. Shared optional_matrix_categories also feeds pair-lift star focus. Covered by analytics_tools + mcp_http; make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
