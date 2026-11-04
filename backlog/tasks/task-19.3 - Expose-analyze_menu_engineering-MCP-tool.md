---
id: TASK-19.3
title: Expose analyze_menu_engineering MCP tool
status: To Do
assignee: []
created_date: '2026-11-04T08:14:00Z'
updated_date: '2026-11-04T08:16:00Z'
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
- [ ] #1 analyze_menu_engineering (or equivalent name) is listed by tools/list and accepts a workspace folder path under MENUYUKTI_DATA_DIR
- [ ] #2 Successful call returns JSON with thresholds, distribution, and items from joined local sales + COGS
- [ ] #3 Missing COGS role/file returns HTTP-success tool JSON with an error field that names the missing prerequisite and next step
- [ ] #4 INSTRUCTIONS and the tool description state that menu engineering requires files.sales and files.cogs on the workspace
- [ ] #5 MCP HTTP integration test covers success on demo-cafe-july-2026 and at least one missing-COGS failure path
- [ ] #6 Existing menuyukti library unit tests for COGS load and matrix calc stay green under make check alongside the new MCP HTTP coverage
<!-- AC:END -->
