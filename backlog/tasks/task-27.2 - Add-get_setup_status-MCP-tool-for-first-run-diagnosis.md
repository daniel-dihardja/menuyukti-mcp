---
id: TASK-27.2
title: Add get_setup_status MCP tool for first-run diagnosis
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-05T08:50:00Z'
updated_date: '2026-11-05T08:59:00Z'
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
- [x] #1 tools/list exposes get_setup_status (or equivalent) with no required path argument
- [x] #2 Response includes resolved MENUYUKTI_DATA_DIR, server/version identity, workspace count, folders missing workspace.yaml, and whether the demo workspace is present
- [x] #3 Paths and listing stay sandboxed under MENUYUKTI_DATA_DIR; no escape outside the data root
- [x] #4 HTTP MCP integration covers tools/list + a successful setup-status call
- [x] #5 Tool description (and a minimal INSTRUCTIONS mention if needed) makes setup status discoverable; full first-contact coaching lands in TASK-27.1
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add menuyukti::setup_status (or equivalent) listing workspaces, missing workspace.yaml, demo presence under data dir.
2. Expose get_setup_status MCP tool with no args; sandboxed to MENUYUKTI_DATA_DIR.
3. Minimal tool description + brief INSTRUCTIONS mention.
4. HTTP MCP integration test for tools/list + call.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added menuyukti::get_setup_status + MCP get_setup_status; HTTP test mcp_http_setup_status_and_owner_playbook_instructions covers tools/list + call. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed get_setup_status for first-run diagnosis (data dir, version, workspace folders, missing workspace.yaml, demo presence). Sandboxed under MENUYUKTI_DATA_DIR; HTTP integration and unit tests pass via make check.
<!-- SECTION:FINAL_SUMMARY:END -->
