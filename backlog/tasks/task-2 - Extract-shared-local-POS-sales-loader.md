---
id: TASK-2
title: Extract shared local POS sales loader
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T10:49:00Z'
updated_date: '2026-11-03T10:50:00Z'
labels: []
dependencies: []
references:
  - src/menuyukti_gpt/sales.py
  - packages/menuyukti/src/menuyukti/core/analytics/registry.py
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
summarize_local_sales embeds detect-normalize inline. The menu engineering matrix tool needs the same gated POS load without forking that pipeline, so both tools can share one loader that returns the normalized frame and detected POS.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 load_normalized_sales gates the path under MENUYUKTI_DATA_DIR before any file read
- [x] #2 load_normalized_sales detects POS, normalizes via menuyukti registry, and returns pos plus a DataFrame with POSTransactionLineItem columns
- [x] #3 summarize_local_sales uses load_normalized_sales and keeps the same aggregate summary shape
- [x] #4 Existing path-gate and ESB fixture tests still pass
- [x] #5 Unit test covers load_normalized_sales against the ESB demo fixture
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add NormalizedSales TypedDict and load_normalized_sales in sales.py: path gate, detect POS, get_normalizer/get_config, normalize, optional rename, return pos + DataFrame.
2. Refactor summarize_local_sales to call load_normalized_sales then calculate_sales_analytics.
3. Add unit test for load_normalized_sales on ESB demo (pos, required columns, non-empty).
4. Run pytest to confirm existing path-gate and summary tests still pass.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added NormalizedSales + load_normalized_sales (path gate → detect → registry normalizer → optional rename). summarize_local_sales now calls the loader. Path-gate tests target the loader; added ESB load_normalized_sales column contract test. uv run pytest tests/test_sales.py tests/test_mcp_server.py: 6 passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Extracted load_normalized_sales so POS detect/normalize is shared for summarize_local_sales and upcoming matrix tools. Returns pos + normalized DataFrame with POSTransactionLineItem columns after MENUYUKTI_DATA_DIR gating. Verified with 6 pytest tests including ESB fixture load and HTTP smoke. Follow-up: wire analyze_menu_engineering on top of this loader plus a local COGS file.
<!-- SECTION:FINAL_SUMMARY:END -->
