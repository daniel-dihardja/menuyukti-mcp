---
id: TASK-19.2
title: Calculate menu engineering matrix from sales and COGS
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:14:00Z'
updated_date: '2026-11-04T08:22:00Z'
labels: []
dependencies: []
references:
  - >-
    ../v3/packages/menuyukti/src/menuyukti/core/analytics/calculate_menu_engineering_matrix.py
  - crates/menuyukti/src/analytics/
  - crates/menuyukti/src/load.rs
parent_task_id: TASK-19
priority: high
type: feature
ordinal: 32000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Normalized sales alone only yield popularity. Owners ask which items are stars vs dogs; that needs contribution margin vs popularity thresholds. The v3 package already defines the matrix result shape (thresholds, distribution, items with category and action). The local Rust library needs the same classification so ChatGPT answers match product language without calling the cloud GraphQL stack.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Library function accepts normalized sales line items plus a menu→unit-cost map and returns thresholds, quadrant distribution, and per-item matrix rows
- [x] #2 Items classify into star, plow_horse, puzzle, and weak using popularity and contribution-margin averages consistent with the v3 calculator semantics
- [x] #3 Items without usable COGS (or zero/invalid cost) are excluded from matrix membership with documented behavior; coverage gaps are visible to callers
- [x] #4 Unit tests use a small crafted sales+COGS fixture that asserts all four categories (star, plow_horse, puzzle, weak), thresholds, and action strings
- [x] #5 Unit tests cover exclusion of items with missing/zero/invalid COGS and surface coverage gaps to the caller
- [x] #6 Unit tests cover empty sales and empty COGS-map edge cases with documented empty/error behavior
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Port calculate_menu_engineering_matrix semantics into crates/menuyukti analytics.
2. Return thresholds, distribution, items, and coverage for unmatched menus.
3. Unit tests for four quadrants, COGS exclusion, empty edges.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ported v3 matrix semantics to analytics/menu_engineering.rs with coverage.unmatched_menus. Unit tests cover four quadrants, COGS exclusion, empty sales/COGS. Demo cafe integration asserts all four categories.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
compute_menu_engineering_from_orders classifies star/plow_horse/puzzle/weak with thresholds, distribution, actions, and COGS coverage. Unit + demo cafe tests green under make check.
<!-- SECTION:FINAL_SUMMARY:END -->
