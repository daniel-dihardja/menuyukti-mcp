---
id: TASK-42.1
title: Build weak items table widget from get_weak_items_review payload
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T10:49:00Z'
updated_date: '2026-11-06T11:04:00Z'
labels:
  - ui
  - chatgpt
  - analytics
dependencies: []
references:
  - crates/menuyukti-mcp/assets/pair_lift_matrix_widget.html
  - crates/menuyukti-mcp/assets/shared/widget-theme.css
  - crates/menuyukti/src/analytics/weak_review.rs
  - ../v3/apps/web/components/analytics/weak/weak-table.tsx
  - >-
    .agents/skills/menuyukti-visualize-weak-items/references/weak-table-recipe.md
parent_task_id: TASK-42
priority: high
type: feature
ordinal: 92000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ChatGPT needs a self-contained MCP Apps HTML widget that can render weak-quadrant review from structuredContent the same way pair lift and menu heatmaps mount charts. Without the asset, render wiring and local preview have nothing to mount.

Build the widget HTML under `crates/menuyukti-mcp/assets` from `get_weak_items_review`-shaped payloads (`items`/`rows`, `stats`, period fields). Match v3 WeakTable / the Codex weak-table recipe: KPI strip, sortable columns (default margin share ascending; toggle asc/desc), omit Action column, omit avg qty/day only when every row has null. Use shared host theme tokens. Local preview harness pages and fixtures are TASK-42.4.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Widget HTML renders a KPI strip from structuredContent.stats plus a table of weak items from structuredContent.items (or rows)
- [x] #2 Columns are menu, avg qty/day (when any row has a value), units sold, contribution margin, and margin share — no Action column
- [x] #3 Default sort is contribution_margin_share ascending with menu tie-break; clickable headers toggle asc/desc with visible sort indicators
- [x] #4 Empty items list shows an empty-state message without inventing rows; null numeric cells render as em dash
- [x] #5 Widget follows shared host theme / style patterns used by existing MCP Apps widgets
- [x] #6 `make check` passes for touched tests
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Study host bridge + shared theme from pair/menu widgets.
2. Add weak_items_review_widget.html with KPI strip and sortable columns (margin share asc default).
3. Wire widget_assets include + unit asserts; leave preview to 42.4.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added weak_items_review_widget.html (KPI strip, sortable columns, default margin-share asc, no Action column, shared theme). Wired widget_assets include + unit asserts.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Weak items MCP Apps widget asset ships with KPI strip, sortable columns (margin share asc default), shared theme, and no Action column. Preview left to TASK-42.4. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
