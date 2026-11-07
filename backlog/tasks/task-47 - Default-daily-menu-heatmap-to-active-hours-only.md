---
id: TASK-47
title: Default daily menu heatmap to active hours only
status: Done
assignee:
  - '@daniel'
created_date: '2026-11-07T03:00:00Z'
updated_date: '2026-11-07T03:01:00Z'
labels: []
dependencies: []
references:
  - apps/menuyukti/src/views/menu-heatmaps.tsx
ordinal: 113000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Skybridge menu heatmaps daily view always renders 24 hour columns (0–23), so early-morning and overnight zeros crowd the chart. Owners want the default daily view to show only hours where at least one unit was sold in the currently visible menus, so the grid focuses on operating hours that actually have demand.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Daily menu heatmap columns default to hours with total quantity >= 1 across currently visible menus
- [x] #2 Hours with no sales across visible menus are omitted from the daily columns
- [x] #3 Weekly view remains Mon–Sun unchanged
- [x] #4 If no hour has sales, daily view still renders safely (no crash; empty or clear empty state)
- [x] #5 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a pure helper that picks hours 0–23 with total quantity >= 1 across the given heatmap items.
2. Wire the daily menu-heatmaps view to use those active hours for columns and cell values; keep weekly Mon–Sun.
3. Cover the helper with a small vitest; run pnpm check and finalize.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added activeDailyHours/dailyValuesForHours helpers; daily view columns are hours with qty>=1 across visible menus. Weekly unchanged. Vitest covers helper; pnpm check 18 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Daily menu heatmap now defaults to hour columns that have at least one unit sold across the currently visible menus, instead of always showing 0–23. Weekly view unchanged. Verified with pnpm check.
<!-- SECTION:FINAL_SUMMARY:END -->
