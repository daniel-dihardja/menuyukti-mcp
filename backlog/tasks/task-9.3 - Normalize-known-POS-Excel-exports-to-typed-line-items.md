---
id: TASK-9.3
title: Normalize known POS Excel exports to typed line items
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.2
references:
  - packages/menuyukti/src/menuyukti/core/models/pos_transaction.py
  - packages/menuyukti/src/menuyukti/core/analytics/esb/normalizer.py
  - packages/menuyukti/src/menuyukti/core/analytics/moka/normalizer.py
  - packages/menuyukti/src/menuyukti/core/analytics/quino/normalizer.py
  - packages/menuyukti/src/menuyukti/core/analytics/pos_detector.py
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The core user workflow is dropping POS Excel exports and getting clean line items. Python relies on pandas/openpyxl for ESB, Moka, and Quino. Rust must own detect + normalize into a typed line-item model so later tools do not depend on DataFrames or a Python runtime.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ESB, Moka, and Quino exports are auto-detected from spreadsheet markers and normalized into the shared line-item schema
- [ ] #2 Invalid or unconvertible rows are dropped consistently enough that fixture totals remain useful for analytics
- [ ] #3 Unit tests cover each known POS fixture under packages/menuyukti/fixtures/pos-sales-reports (or sales-reports demos)
- [ ] #4 Unsupported/unknown POS without a mapping config fails with a clear error
<!-- AC:END -->
