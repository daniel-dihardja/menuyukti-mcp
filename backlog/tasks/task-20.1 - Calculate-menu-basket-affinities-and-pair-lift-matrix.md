---
id: TASK-20.1
title: Calculate menu basket affinities and pair lift matrix
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:32:00Z'
updated_date: '2026-11-04T08:41:00Z'
labels: []
dependencies: []
references:
  - >-
    ../v3/packages/menuyukti/src/menuyukti/core/analytics/calculate_menu_basket_affinities.py
  - >-
    ../v3/packages/menuyukti/tests/unit/core/analytics/test_calculate_menu_basket_affinities.py
  - crates/menuyukti/src/analytics/
  - crates/menuyukti/src/line_item.rs
parent_task_id: TASK-20
priority: high
type: feature
ordinal: 36000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Normalized LineItem rows already expose bill_number and menu, which is enough for market-basket lift, but crates/menuyukti has no affinity calculator. v3 `calculate_menu_basket_affinities` defines the product semantics ChatGPT already sees in the cloud pair-lift chart (MIN_CO_OCCURRENCE, MAX_FOCUS_ITEMS, lift = support / (P(A)*P(B)), null diagonal matrix). The local library needs that pure function so the MCP layer can return visualisation-identical fields without GraphQL.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Library function accepts normalized sales line items and optional focus_menus and returns total_orders, multi_item_order_count, avg_distinct_items_per_order, scope, focus_menus, pairs, and matrix_lift
- [x] #2 Lift, support, and confidence match v3 basket-affinity semantics (distinct menus per bill; qty ignored; pairs below min co-occurrence omitted from pairs but matrix only filled when lift was computed)
- [x] #3 With no focus_menus, focus is top menus by order presence (capped) and scope is top_by_presence; with >=2 provided focus menus, those are used (capped) and scope is stars
- [x] #4 matrix_lift is square over focus_menus, symmetric, with null diagonal; empty or single-item-only inputs return structured empty focus/pairs/matrix without panicking
- [x] #5 Unit tests cover empty input, single-item orders, correlated pair lift > 1, focus filtering, top-by-presence fallback, and diagonal/symmetry cases (aligned with v3 affinity unit tests)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Port calculate_menu_basket_affinities into crates/menuyukti/src/analytics/basket_affinities.rs (constants, focus resolution, lift/support/confidence, matrix_lift).
2. Export from analytics/mod.rs and lib.rs.
3. Add unit tests aligned with v3 test_calculate_menu_basket_affinities (empty, single-item, correlated lift, focus filter, top_by_presence, diagonal/symmetry).
4. Run make check; finalize TASK-20.1.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented analytics/basket_affinities.rs port of v3 calculate_menu_basket_affinities; exported from analytics/mod.rs and lib.rs. Eight unit tests mirror v3 affinity cases plus min co-occurrence omission. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added compute_menu_basket_affinities_from_orders with v3-compatible lift/support/confidence, focus resolution (stars vs top_by_presence), and matrix_lift. Unit tests cover empty/single-item/correlated/focus/symmetry/min-co-occurrence. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
