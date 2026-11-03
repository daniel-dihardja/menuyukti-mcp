---
id: TASK-17.1
title: Expose write_workspace_config and onboard MCP instructions
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T16:40:00Z'
updated_date: '2026-11-03T16:44:00Z'
labels: []
dependencies:
  - TASK-18
references:
  - crates/menuyukti-mcp/src/main.rs
parent_task_id: TASK-17
ordinal: 27000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
list_workspace_files shows dropped Excels, but ChatGPT has no safe way to persist roles. An MCP write tool plus server instructions make the set-up ritual work in chat.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP tools/list exposes write_workspace_config that writes workspace.yaml inside MENUYUKTI_DATA_DIR only
- [x] #2 Server instructions describe: list files, confirm roles if needed, write config, then analyze
- [x] #3 HTTP MCP integration covers the new write tool
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add write_workspace_config MCP tool that accepts a workspace path + structured config and calls menuyukti::write_workspace_config.
2. Update server instructions with the onboard ritual (list files → write config → analyze).
3. Extend HTTP MCP tests to cover tools/list + a gated write into a temp data dir.
4. Run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added write_workspace_config MCP tool + onboard instructions. HTTP test covers write and outside-sandbox reject. MENUYUKTI_MCP_PORT for test isolation. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed write_workspace_config and documented the list→write→analyze onboard ritual in server instructions. HTTP integration covers the tool; make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
