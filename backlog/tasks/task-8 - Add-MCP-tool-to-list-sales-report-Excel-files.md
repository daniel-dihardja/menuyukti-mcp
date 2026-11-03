---
id: TASK-8
title: Add MCP tool to list sales report Excel files
status: Done
assignee: []
created_date: '2026-11-03T11:26:00Z'
updated_date: '2026-11-03T11:27:00Z'
labels: []
dependencies: []
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners and ChatGPT need to discover which POS Excel files are available under MENUYUKTI_DATA_DIR before calling summarize or heatmap tools. Without a listing tool they must know filenames in advance.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP tool lists Excel sales reports under MENUYUKTI_DATA_DIR as relative paths
- [x] #2 Listing ignores non-Excel files such as README and mapping JSON
- [x] #3 Tests cover discovering the default demo file and an empty data dir
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add list_sales_reports helper in sales.py. 2. Register no-arg MCP tool. 3. Unit + HTTP tests. 4. Run pytest.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added list_sales_reports MCP tool; pytest sales+mcp → 13 passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added list_sales_reports MCP tool that returns Excel files under MENUYUKTI_DATA_DIR as relative paths (skips README/JSON). Verified with unit and HTTP tests (13 passed).
<!-- SECTION:FINAL_SUMMARY:END -->
