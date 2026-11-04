---
id: TASK-19.3
title: Expose analyze_menu_engineering MCP tool
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:14:00Z'
updated_date: '2026-11-04T08:22:00Z'
labels: []
dependencies:
  - TASK-19.1
  - TASK-19.2
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
parent_task_id: TASK-19
priority: high
type: feature
ordinal: 33000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Library loaders and matrix math are useless to ChatGPT until they are one MCP call. Soft prompt-only prerequisites fail; the tool must load sales and COGS itself, refuse when COGS is unavailable, and teach the model via description plus server INSTRUCTIONS how to recover (list_workspace_files / write_workspace_config / ask owner).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 analyze_menu_engineering (or equivalent name) is listed by tools/list and accepts a workspace folder path under MENUYUKTI_DATA_DIR
- [x] #2 Successful call returns JSON with thresholds, distribution, and items from joined local sales + COGS
- [x] #3 Missing COGS role/file returns HTTP-success tool JSON with an error field that names the missing prerequisite and next step
- [x] #4 INSTRUCTIONS and the tool description state that menu engineering requires files.sales and files.cogs on the workspace
- [x] #5 MCP HTTP integration test covers success on demo-cafe-july-2026 and at least one missing-COGS failure path
- [x] #6 Existing menuyukti library unit tests for COGS load and matrix calc stay green under make check alongside the new MCP HTTP coverage
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Wire analyze_menu_engineering MCP tool with hard COGS gate.
2. Update INSTRUCTIONS and tool description.
3. HTTP tests: demo success + missing COGS failure.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Wired analyze_menu_engineering: load COGS first (hard gate), then sales, then matrix. Updated INSTRUCTIONS. MCP HTTP tests cover demo success and sales-only missing-COGS error.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
analyze_menu_engineering is exposed over MCP for workspace folders with sales+COGS. Missing COGS returns actionable JSON error. HTTP + library tests pass make check.
<!-- SECTION:FINAL_SUMMARY:END -->
