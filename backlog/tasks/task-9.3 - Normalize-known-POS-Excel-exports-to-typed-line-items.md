---
id: TASK-9.3
title: Normalize known POS Excel exports to typed line items
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:13:00Z'
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
- [x] #1 ESB, Moka, and Quino exports are auto-detected from spreadsheet markers and normalized into the shared line-item schema
- [x] #2 Invalid or unconvertible rows are dropped consistently enough that fixture totals remain useful for analytics
- [x] #3 Unit tests cover each known POS fixture under packages/menuyukti/fixtures/pos-sales-reports (or sales-reports demos)
- [x] #4 Unsupported/unknown POS without a mapping config fails with a clear error
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add LineItem model + POS detect from sheet A1 via calamine.
2. Port ESB, Moka, Quino normalizers to Vec<LineItem> with skiprows/header coerce/drop-invalid.
3. Expose load_normalized_sales(path) using path gate + detect + registry.
4. Unit-test each known POS fixture; clear error for unknown POS without mapping.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added LineItem, calamine excel helpers, POS detect (esb/moka/quino), normalizers, load_normalized_sales. Tests: ESB demo (9 rows), Moka demo, synthetic Quino xlsx, unknown POS error, gated ESB load, demo-cafe large ESB.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rust now detects ESB/Moka/Quino Excel exports and normalizes them to typed LineItem rows behind path gating. Unknown POS without mapping returns a clear error. Fixture tests cover ESB, Moka, synthetic Quino, and the sales-reports demo cafe file.
<!-- SECTION:FINAL_SUMMARY:END -->
