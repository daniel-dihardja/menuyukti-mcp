---
id: TASK-22.1
title: Calculate venue slot demand profile from sales line items
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T09:35:00Z'
updated_date: '2026-11-04T09:37:00Z'
labels: []
dependencies: []
references:
  - >-
    ../v3/packages/menuyukti/src/menuyukti/core/analytics/calculate_slot_demand_profile.py
  - ../v3/packages/menuyukti/src/menuyukti/core/analytics/meal_periods.py
  - ../v3/packages/menuyukti/src/menuyukti/core/analytics/demand_labels.py
  - ../v3/packages/menuyukti/src/menuyukti/core/analytics/slot_keys.py
  - >-
    ../v3/packages/menuyukti/tests/unit/core/analytics/test_calculate_slot_demand_profile.py
  - crates/menuyukti/src/analytics/
  - crates/menuyukti/src/line_item.rs
parent_task_id: TASK-22
priority: high
type: feature
ordinal: 44000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
LineItem already has bill_number and order_time, which is enough for venue-level day × meal-period demand, but crates/menuyukti has no slot demand calculator. v3 `calculate_slot_demand_profile` defines the product semantics ChatGPT sees in `venue_slot_strength_heatmap` (one timestamp per bill, 35 fixed cells, demand_index = traffic_share / (1/35), relative_demand low/average/high). The local library needs that pure function so the MCP layer can return visualisation-identical fields without GraphQL.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Library function accepts normalized sales line items and returns a list of slot demand cells with day, meal_period, meal_period_label, meal_period_hours_label, order_count, traffic_share, demand_index, and relative_demand
- [x] #2 Uses distinct bills via min order_time per bill_number; qty/menu lines do not inflate slot counts
- [x] #3 Meal periods and weekday order match v3 constants (breakfast/lunch/afternoon/dinner/late_night; mon–sun); non-empty input yields exactly 35 cells including zeros
- [x] #4 demand_index and relative_demand match v3 semantics (mean slot share 1/35; low < 0.9, high > 1.1)
- [x] #5 Empty input returns an empty profile without panicking
- [x] #6 Unit tests cover empty input, 35-cell shape, weak vs strong slots, and sparse breakfast staying below average (aligned with v3 slot demand unit tests)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add analytics/slot_demand.rs: meal period constants, relative demand thresholds, bill-min aggregation, 35-cell profile.
2. Export from analytics/mod.rs and lib.rs.
3. Unit tests: empty, 35 cells, weak mon lunch vs strong fri dinner, sparse breakfast.
4. make check; finalize 22.1 then continue 22.2/22.3.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added analytics/slot_demand.rs with meal periods, relative demand, bill-min aggregation, 35-cell profile. Five unit tests mirror v3 cases. Exported from mod/lib.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
compute_slot_demand_profile_from_orders returns v3-compatible 35-cell venue slot demand profiles from LineItem bill times. Unit tests green.
<!-- SECTION:FINAL_SUMMARY:END -->
