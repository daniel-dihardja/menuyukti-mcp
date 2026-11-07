---
id: TASK-51.2
title: Expose data-dir knowledge MCP tools without workspace path
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-07T05:38:00Z'
updated_date: '2026-11-07T05:42:00Z'
labels:
  - knowledge
  - mcp
dependencies:
  - TASK-51.1
references:
  - crates/menuyukti-mcp/src/main.rs
parent_task_id: TASK-51
priority: high
type: enhancement
ordinal: 102000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ChatGPT must persist venue notes once per data dir. Current read_workspace_knowledge / write_workspace_knowledge require a workspace folder path and write beside that folder, which fights the one-location-per-data-dir assumption.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP knowledge read/write tools operate on MENUYUKTI_DATA_DIR/knowledge.md and do not require a workspace folder path argument
- [x] #2 Server instructions / tool descriptions state knowledge is location-wide for the data dir
- [x] #3 HTTP/MCP tests cover missing, write, reread, and replace without a workspace path
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Drop workspace path from read/write knowledge MCP tools; write takes content only.
2. Remove files.knowledge from write_workspace_config args mapping.
3. Update INSTRUCTIONS + tool descriptions for data-dir-wide knowledge.
4. Update HTTP tests for missing/write/reread/replace without path.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MCP read takes no args; write takes content only. INSTRUCTIONS + tool descriptions updated for location-wide knowledge. HTTP test covers missing/write/reread/replace. make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed pathless knowledge MCP tools over MENUYUKTI_DATA_DIR/knowledge.md, dropped files.knowledge from write_workspace_config args, and updated HTTP coverage. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
