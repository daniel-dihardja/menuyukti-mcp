---
id: TASK-36.1
title: Build pair lift matrix heatmap widget from focus_menus and matrix_lift
status: Done
assignee: []
created_date: '2026-11-06T07:14:00Z'
updated_date: '2026-11-06T07:18:00Z'
labels:
  - ui
  - chatgpt
  - analytics
dependencies: []
references:
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - ../v3/apps/web/components/chat/visualizations/pair-lift-matrix-card.tsx
  - crates/menuyukti/src/analytics/basket_affinities.rs
parent_task_id: TASK-36
priority: high
type: feature
ordinal: 78000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ChatGPT needs a self-contained MCP Apps HTML widget that can render pair lift from structuredContent the same way venue slot strength renders slot_demand_profile. Without the asset, render wiring has nothing to mount.

Build the heatmap HTML under crates/menuyukti-mcp/assets from focus_menus + matrix_lift (null diagonal, lift color scale, host theme tokens). Add local preview harness pages and fixtures (populated + empty) so developers can iterate with make widgets-preview.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Widget HTML renders a square focus_menus × focus_menus grid from structuredContent.focus_menus and structuredContent.matrix_lift
- [x] #2 Diagonal / null cells are visually distinct and not treated as lift 0
- [x] #3 Color scale and labels make lift > 1 (affinity above chance) vs < 1 understandable; legend explains lift
- [x] #4 Widget follows host theme / style variable patterns used by venue_slot_strength_widget.html
- [x] #5 Preview fixtures exist for populated and empty/edge pair-lift payloads; preview index links them
- [x] #6 make check / widget_preview_fixtures covers fixture shape
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Study venue_slot_strength_widget.html host bridge + v3 pair-lift-matrix-card lift semantics.
2. Add pair_lift_matrix_widget.html (focus_menus × matrix_lift, null diagonal, sequential color for lift, host theme tokens).
3. Add preview fixtures (populated + empty) and harness pages; link from preview/index.html.
4. Extend widget_preview_fixtures.rs; run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added pair_lift_matrix_widget.html (focus×focus teal heatmap, null diagonal, host theme bridge). Preview fixtures + harness pages linked from index. widget_preview_fixtures tests cover populated square matrix and empty focus.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Built pair lift MCP Apps heatmap widget from focus_menus + matrix_lift with null diagonal and lift legend. Preview fixtures (populated + empty) and harness pages added; fixture shape tests pass. make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
