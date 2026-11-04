---
id: TASK-19.2
title: Calculate menu engineering matrix from sales and COGS
status: To Do
assignee: []
created_date: '2026-11-04T08:14:00Z'
updated_date: '2026-11-04T08:16:00Z'
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
- [ ] #1 Library function accepts normalized sales line items plus a menu→unit-cost map and returns thresholds, quadrant distribution, and per-item matrix rows
- [ ] #2 Items classify into star, plow_horse, puzzle, and weak using popularity and contribution-margin averages consistent with the v3 calculator semantics
- [ ] #3 Items without usable COGS (or zero/invalid cost) are excluded from matrix membership with documented behavior; coverage gaps are visible to callers
- [ ] #4 Unit tests use a small crafted sales+COGS fixture that asserts all four categories (star, plow_horse, puzzle, weak), thresholds, and action strings
- [ ] #5 Unit tests cover exclusion of items with missing/zero/invalid COGS and surface coverage gaps to the caller
- [ ] #6 Unit tests cover empty sales and empty COGS-map edge cases with documented empty/error behavior
<!-- AC:END -->
