---
id: TASK-39
title: Keep pair lift column headers fixed while sorting rows
status: Done
assignee: []
created_date: '2026-11-06T07:33:00Z'
updated_date: '2026-11-06T07:34:00Z'
labels:
  - ui
  - chatgpt
  - widgets
dependencies:
  - TASK-38
references:
  - crates/menuyukti-mcp/assets/pair_lift_matrix_widget.html
priority: high
type: bug
ordinal: 83000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Sorting a lift column currently reorders both axes with the same permutation so the diagonal stays aligned. That moves the clicked header (e.g. Espresso becomes Croissant), which makes the sort target feel like it jumped.

Owners expect fixed column names: only row order should change when sorting by Menu (A–Z) or by a lift column (numeric).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Column header labels stay in the original focus_menus order while sorting
- [x] #2 Sorting Menu or a lift column only reorders table rows
- [x] #3 Clicked lift column still sorts by that column numeric values (null/diagonal lowest)
- [x] #4 pair-lift widget URI bumped; make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Draw column headers from fixed focus indexes; keep order permutation for rows only.
2. Update sort hint copy; bump URI to v5; make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Column headers now always iterate original focus indexes; rowOrder permutation only reorders tbody rows. Sort hint updated. URI pair-lift-matrix-v5.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed pair lift sorting so column headers stay fixed in focus_menus order; only rows reorder by Menu (A–Z) or lift column values. URI bumped to v5. make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
