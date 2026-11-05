---
id: TASK-27.2
title: Add get_setup_status MCP tool for first-run diagnosis
status: To Do
assignee: []
created_date: '2026-11-05T08:50:00Z'
updated_date: '2026-11-05T08:50:00Z'
labels:
  - ux
  - onboarding
dependencies: []
references:
  - crates/menuyukti-mcp/src/main.rs
parent_task_id: TASK-27
priority: high
type: feature
ordinal: 59000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When the server is misconfigured or folders lack workspace.yaml, owners only see opaque tool failures. A setup/status tool lets ChatGPT diagnose first-run health in-chat (data dir, workspace inventory, which folders need config, demo availability, server version) instead of sending the owner back to install docs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 tools/list exposes get_setup_status (or equivalent) with no required path argument
- [ ] #2 Response includes resolved MENUYUKTI_DATA_DIR, server/version identity, workspace count, folders missing workspace.yaml, and whether the demo workspace is present
- [ ] #3 Paths and listing stay sandboxed under MENUYUKTI_DATA_DIR; no escape outside the data root
- [ ] #4 HTTP MCP integration covers tools/list + a successful setup-status call
- [ ] #5 Tool description (and a minimal INSTRUCTIONS mention if needed) makes setup status discoverable; full first-contact coaching lands in TASK-27.1
<!-- AC:END -->
