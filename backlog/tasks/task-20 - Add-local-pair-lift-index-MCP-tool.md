---
id: TASK-20
title: Add local pair lift index MCP tool
status: Done
assignee: []
created_date: '2026-11-04T08:31:00Z'
updated_date: '2026-11-04T08:43:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/analytics/
  - crates/menuyukti-mcp/src/main.rs
  - >-
    ../v3/packages/menuyukti/src/menuyukti/core/analytics/calculate_menu_basket_affinities.py
  - ../v3/apps/graphql/services/menu_combos.py
  - ../v3/apps/agents/agents/core/chat/chart_data.py
  - docs/howtos/workspaces.md
documentation:
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 35000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners ask which dishes belong in combos and upsells. v3 answers with the pair lift matrix (`pair_lift_matrix_heatmap` / `menuCombos` focusMenus + matrixLift). The local MCP already has heatmaps and menu engineering but no co-purchase lift, so ChatGPT cannot ground combo advice in basket affinity without inventing numbers.

Port the v3 market-basket affinity semantics (support, confidence, lift, focus matrix) into crates/menuyukti and expose a thin MCP tool that returns the same visualisation fields ChatGPT needs. Sales-only paths must still work via top-by-presence focus; when workspace COGS exists, prefer menu-engineering stars as focus (parity with v3 `build_menu_combos`).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT can call one MCP tool against a path under MENUYUKTI_DATA_DIR and receive pair-lift visualisation JSON with focus_menus, matrix_lift, total_orders, multi_item_order_count, and scope
- [x] #2 Without usable COGS, the tool still returns a lift matrix using top-by-presence focus rather than failing
- [x] #3 When workspace sales+COGS yield at least two menu-engineering stars, those stars are used as focus_menus and scope reflects that choice
- [x] #4 Demo cafe (or equivalent fixture) produces a non-empty focus set (>=2) and a square matrix_lift with null diagonal
- [x] #5 Owner/MCP docs mention the pair-lift tool and that sales alone is enough (COGS only improves star focus)
- [x] #6 Unit tests cover affinity math edge cases; MCP HTTP tests cover a successful call; make check passes
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All subtasks 20.1–20.3 done. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Local pair lift index is available via get_pair_lift_matrix. Library computes basket affinities; MCP returns visualisation fields with optional star focus when COGS exists; docs clarify sales-only vs COGS. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
