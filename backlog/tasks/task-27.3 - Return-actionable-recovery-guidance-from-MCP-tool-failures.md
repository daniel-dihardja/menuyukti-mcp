---
id: TASK-27.3
title: Return actionable recovery guidance from MCP tool failures
status: To Do
assignee: []
created_date: '2026-11-05T08:50:00Z'
labels:
  - ux
  - onboarding
dependencies:
  - TASK-27.1
references:
  - crates/menuyukti-mcp/src/main.rs
parent_task_id: TASK-27
priority: medium
type: enhancement
ordinal: 60000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Tool error text is often the only UI when analytics or onboarding fails mid-call. Generic failures leave ChatGPT and owners stuck. Errors should name the problem in owner language and the next MCP or physical step (confirm sales file, write_workspace_config, drop COGS, use a folder under the data dir).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Missing or incomplete workspace config errors name the gap (e.g. no files.sales / no files.cogs) and suggest list_workspace_files then write_workspace_config or asking the owner
- [ ] #2 Menu engineering and weak-items COGS failures explain the owner action (drop unit-cost Excel) plus recovery tools
- [ ] #3 Path-outside-sandbox / not-found failures tell the model to verify the folder is under the data directory and to call list_workspaces or setup status
- [ ] #4 HTTP or unit tests assert representative failure messages include the recovery hint (not only an error code)
<!-- AC:END -->
