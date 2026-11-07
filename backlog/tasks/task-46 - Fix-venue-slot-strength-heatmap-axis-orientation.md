---
id: TASK-46
title: Fix venue slot strength heatmap axis orientation
status: Done
assignee:
  - '@daniel'
created_date: '2026-11-07T02:56:00Z'
updated_date: '2026-11-07T02:57:00Z'
labels: []
dependencies: []
references:
  - .agents/skills/menuyukti-venue-slot-strength/references/heatmap-recipe.md
  - apps/menuyukti/src/views/venue-slot-strength.tsx
ordinal: 112000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Skybridge venue slot strength widget currently shows weekdays as rows and meal periods as columns. That is the transpose of the intended chart: meal-period (slot hours) labels should be the left/vertical axis and weekdays Mon–Sun should be the top/horizontal columns, matching the Codex HTML heatmap recipe and owner expectations for staffing by daypart.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Venue slot strength React heatmap has meal periods (with hours labels) as row headers on the left
- [x] #2 Venue slot strength React heatmap has weekdays Mon–Sun as column headers across the top
- [x] #3 Cell values still map to the correct (day, meal_period) slot; metric toggle and coloring unchanged
- [x] #4 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Transpose the Skybridge `venue-slot-strength` table so meal periods are rows and Mon–Sun are columns (match heatmap-recipe.md).
2. Keep cell lookup keyed by day:meal_period; leave metric toggle, intensity coloring, and tooltips intact.
3. Run `pnpm check` and finalize TASK-46.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Transposed apps/menuyukti/src/views/venue-slot-strength.tsx: meal periods are rows (with hours labels), Mon–Sun are columns. Cell keying and metric toggle unchanged. pnpm check: tsc + 15 vitest tests passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed venue slot strength widget axis orientation so meal periods (slot hours) are vertical row labels and weekdays are horizontal columns, matching the Codex HTML heatmap recipe. Verified with pnpm check (typecheck + 15 tests).
<!-- SECTION:FINAL_SUMMARY:END -->
