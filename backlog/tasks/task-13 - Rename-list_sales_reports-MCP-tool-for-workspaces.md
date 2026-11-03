---
id: TASK-13
title: Rename list_sales_reports MCP tool for workspaces
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T14:53:00Z'
updated_date: '2026-11-03T14:54:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti-mcp/src/main.rs
  - README.md
type: enhancement
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After renaming the data root to workspaces/, MCP Inspector and ChatGPT still expose list_sales_reports, which reads as POS-file listing rather than the owner workspace model.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP tools/list exposes a workspaces-oriented list tool instead of list_sales_reports
- [x] #2 Tool description refers to workspaces under MENUYUKTI_DATA_DIR
- [x] #3 HTTP/integration tests call the renamed tool
- [x] #4 Docs that name the list tool use the new name
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Rename list_sales_reports MCP tool to list_workspaces; update description.
2. Keep listing Excel files under the data dir as relative paths for tool chaining (same payload shape, clearer name).
3. Update README/docs and HTTP tests.
4. Run make check.

2b. Change list payload to group Excel files by workspace folder (workspace_count/workspaces/files).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Renamed MCP tool and library API to list_workspaces; payload now groups Excel files by workspace folder. Updated README/workspaces docs and HTTP test. make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced list_sales_reports with list_workspaces. The tool groups Excel files by workspace folder under MENUYUKTI_DATA_DIR and returns workspace_count/workspaces/files. Docs and HTTP tests updated; make check passed. Restart mcp-inspect to pick up the new tool list.
<!-- SECTION:FINAL_SUMMARY:END -->
