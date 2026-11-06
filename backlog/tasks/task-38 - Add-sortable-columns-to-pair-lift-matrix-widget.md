---
id: TASK-38
title: Add sortable columns to pair lift matrix widget
status: Done
assignee: []
created_date: '2026-11-06T07:29:00Z'
updated_date: '2026-11-06T07:31:00Z'
labels:
  - ui
  - chatgpt
  - widgets
dependencies:
  - TASK-36.1
references:
  - crates/menuyukti-mcp/assets/pair_lift_matrix_widget.html
priority: medium
type: enhancement
ordinal: 82000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners comparing combo affinity need to reorder the pair lift grid by menu name or by lift against a focus dish. The matrix currently renders focus_menus in fixed tool order, so high-lift partners can be buried mid-table.

Make every column header clickable: Menu sorts rows alphabetically; each focus-menu column sorts rows by that column numeric lift (null/diagonal last). Keep the square matrix consistent (reorder both axes together when sorting). Persist direction toggle on re-click.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Clicking the Menu header sorts the matrix alphabetically by menu name (toggle asc/desc)
- [x] #2 Clicking any focus-menu column header sorts by that column numeric lift (null/diagonal treated as lowest; toggle asc/desc)
- [x] #3 Row and column order stay aligned so the diagonal remains same-item cells
- [x] #4 Sort state is visible in the header (aria-sort and/or sort indicator)
- [x] #5 make check passes; pair-lift widget URI bumped for ChatGPT cache
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Refactor pair-lift render to keep focus/matrix state and draw from a sorted index order.
2. Clickable headers: Menu = alpha; focus cols = numeric lift (nulls last); toggle asc/desc; aria-sort + indicator.
3. Reorder both axes with the same permutation so diagonal stays same-item.
4. Bump URI to v4; assert sort markers in mcp_http HTML test; make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Pair lift headers are buttons with aria-sort + ↑/↓. Menu sorts A–Z; focus columns sort by lift (null/diagonal as lowest). Same permutation reorders rows and columns so diagonal stays aligned. URI pair-lift-matrix-v4. HTTP test asserts sortable markers.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Pair lift matrix columns are sortable: Menu alphabetically, lift columns numerically, with rows/columns kept aligned and visible sort indicators. Widget URI bumped to v4. make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
