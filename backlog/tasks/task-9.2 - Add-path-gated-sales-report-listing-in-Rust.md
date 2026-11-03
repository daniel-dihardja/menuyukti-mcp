---
id: TASK-9.2
title: Add path-gated sales report listing in Rust
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.1
references:
  - src/menuyukti_mcp/
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Users need to discover which Excel files they can analyze without guessing paths. The Python server already sandboxes all reads under MENUYUKTI_DATA_DIR; that privacy boundary must exist before any Excel parse runs in Rust.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 list_sales_reports MCP tool lists .xlsx/.xls under MENUYUKTI_DATA_DIR as relative paths
- [x] #2 Paths outside the data directory are rejected before any file content is read
- [x] #3 Missing or invalid MENUYUKTI_DATA_DIR returns a clear tool error
- [x] #4 Tests cover listing and path rejection
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add menuyukti paths module: MENUYUKTI_DATA_DIR resolution, PathNotAllowedError, resolve_allowed_path, list_sales_reports.
2. Mirror Python Excel suffix / ~$ ignore / relative path listing behavior.
3. Wire list_sales_reports MCP tool to real implementation returning JSON matching sales_report_list_as_dict.
4. Unit-test listing + path rejection + missing env; cargo test + smoke tools/call.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added menuyukti::paths with get_data_dir, resolve_allowed_path, list_sales_reports. Wired list_sales_reports MCP tool. cargo test -p menuyukti: 5 passed (missing env, outside path, traversal, listing, not-a-dir). Smoke tools/call listed sales-reports Excel files and returned clear error when MENUYUKTI_DATA_DIR unset.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rust MCP list_sales_reports now lists .xlsx/.xls under MENUYUKTI_DATA_DIR with relative paths; resolve_allowed_path rejects traversal/outside paths before reads; missing data dir returns a clear tool error. Covered by unit tests and HTTP smoke calls.
<!-- SECTION:FINAL_SUMMARY:END -->
