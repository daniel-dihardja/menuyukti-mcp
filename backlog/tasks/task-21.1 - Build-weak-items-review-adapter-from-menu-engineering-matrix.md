---
id: TASK-21.1
title: Build weak items review adapter from menu engineering matrix
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:49:00Z'
updated_date: '2026-11-04T08:51:00Z'
labels: []
dependencies: []
references:
  - ../v3/apps/web/lib/analytics/weak-page-adapter.ts
  - ../v3/apps/web/tests/lib/analytics/weak-page-adapter.test.ts
  - crates/menuyukti/src/analytics/menu_engineering.rs
  - crates/menuyukti/src/line_item.rs
parent_task_id: TASK-21
priority: high
type: feature
ordinal: 40000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The full matrix from `compute_menu_engineering_from_orders` includes all quadrants. v3 chat visualisation depth comes from `adaptWeakPage`: filter category weak, derive inclusive period days, avg qty/day, and portfolio KPIs (item/margin share, margin amount). The local library needs that adapter over matrix items + distribution + sales period so the MCP tool can return visualisation-identical fields without ChatGPT re-deriving them.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Library function accepts a menu-engineering matrix result (items + distribution) plus optional period_start/period_end (or derives period from sales line item order times) and returns weak rows and portfolio stats
- [x] #2 Each weak row includes menu, units_sold/quantity, avg_qty_per_day (null when period unknown), contribution_margin, contribution_margin_share, and action
- [x] #3 Stats include item_count, item_share, margin_share, margin_amount, total_menu_items, and period_days aligned with v3 weak-page-adapter semantics
- [x] #4 Empty weak quadrant returns structured empty rows with zero/empty stats rather than erroring
- [x] #5 Unit tests cover filtering to weak only, avg_qty_per_day when period is known vs null, portfolio share math, and empty-weak edge case
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add analytics/weak_review.rs with inclusive calendar-day helper, adapt_weak_review from matrix items+distribution+optional period dates, and period derivation from LineItem order times.
2. Export from analytics/mod.rs and lib.rs.
3. Unit tests aligned with v3 weak-page-adapter tests (filter, avg/day, missing period, empty weak).
4. make check; finalize TASK-21.1.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added analytics/weak_review.rs with inclusive_calendar_days, period_from_line_items, adapt_weak_review (+ from_matrix / from_orders helpers). Exported from analytics/mod.rs and lib.rs. Six unit tests mirror v3 weak-page-adapter cases plus period derivation and distribution fallback. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Library adapts menu-engineering matrix into weak-items review rows and portfolio stats with period depth (avg_qty_per_day). Empty weak quadrant returns structured empty. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
