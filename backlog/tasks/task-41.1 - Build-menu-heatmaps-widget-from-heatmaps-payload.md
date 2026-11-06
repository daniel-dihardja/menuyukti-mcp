---
id: TASK-41.1
title: Build menu heatmaps widget from heatmaps payload
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T07:51:00Z'
updated_date: '2026-11-06T07:56:00Z'
labels:
  - ui
  - chatgpt
  - analytics
dependencies: []
references:
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - crates/menuyukti-mcp/assets/pair_lift_matrix_widget.html
  - crates/menuyukti-mcp/assets/shared/widget-theme.css
  - crates/menuyukti/src/analytics/heatmaps.rs
  - >-
    .agents/skills/menuyukti-visualize-menu-heatmaps/references/menu-heatmap-recipe.md
parent_task_id: TASK-41
priority: high
type: feature
ordinal: 88000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ChatGPT needs a self-contained MCP Apps HTML widget that can render dish timing from structuredContent the same way venue slot strength and pair lift mount heatmaps. Without the asset, render wiring has nothing to mount, and make widgets-preview cannot list a menu heatmap entry.

Build the widget HTML under crates/menuyukti-mcp/assets from get_menu_heatmaps-shaped heatmaps rows (menu, daily_heatmap, weekly_heatmap, optional matrix_category). Support daily and weekly views, quantity color scale, shared host theme tokens, and interactive matrix_category toggles with weak off by default when categories exist. Add local preview harness pages and fixtures (populated + empty) so developers can iterate with make widgets-preview.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Widget HTML renders daily and/or weekly per-menu quantity grids from structuredContent.heatmaps
- [x] #2 When matrix_category values are present, checkboxes enable/disable star, plow_horse, puzzle, and weak rows; weak starts unchecked
- [x] #3 When matrix_category is absent/null for all rows, the chart still renders and the filter bar is omitted or inert without inventing categories
- [x] #4 Widget follows shared host theme / style patterns used by existing heatmap widgets
- [x] #5 Preview fixtures exist for populated and empty/edge menu-heatmap payloads; preview index links them
- [x] #6 widget_preview_fixtures (or equivalent) covers fixture shape; make check passes for touched tests
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Study venue/pair widget host bridge + theme + preview harness.
2. Add menu_heatmaps_widget.html (daily/weekly toggle, quantity scale, matrix_category filters with weak off by default).
3. Add populated + empty preview fixtures and harness pages; link from preview/index.html.
4. Extend widget_preview_fixtures.rs; verify preview lists entries.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added menu_heatmaps_widget.html with daily/weekly toggle + matrix_category filters (weak unchecked). Preview fixtures/pages linked from index. widget_assets + fixture tests updated.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Menu heatmaps MCP Apps widget asset ships with daily/weekly views, quantity color scale, shared theme, and category toggles (weak off by default). Preview index lists populated + empty fixtures. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
