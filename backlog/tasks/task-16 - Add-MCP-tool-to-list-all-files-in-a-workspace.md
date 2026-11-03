---
id: TASK-16
title: Add MCP tool to list all files in a workspace
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T16:13:00Z'
updated_date: '2026-11-03T16:15:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/paths.rs
  - crates/menuyukti-mcp/src/main.rs
  - docs/howtos/workspaces.md
ordinal: 24000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
list_workspaces only returns Excel files. Owners and ChatGPT need to see workspace.json, COGS, notes, and other files inside a chosen workspace folder so they can pick the right paths and understand what is already configured.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP tools/list exposes a path-gated tool that lists files under a workspace directory relative to MENUYUKTI_DATA_DIR
- [x] #2 Listing returns non-Excel files such as workspace.json and skips Excel lock files (~$*)
- [x] #3 Paths outside MENUYUKTI_DATA_DIR and missing workspace dirs are rejected with a clear error
- [x] #4 Unit tests cover listing and path gating; HTTP MCP integration covers the new tool
- [x] #5 Docs mention the new tool alongside list_workspaces
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add resolve_allowed_dir and list_workspace_files in paths.rs (recurse under a gated workspace folder; return all regular files except ~$* locks).
2. Export the new API from menuyukti lib; add unit tests for listing, lock skip, missing dir, and traversal rejection.
3. Expose list_workspace_files MCP tool in menuyukti-mcp with a workspace path argument; update server instructions.
4. Extend HTTP MCP integration to require the tool and call it on the demo workspace.
5. Document the tool in docs/howtos/workspaces.md (and workspaces README if it lists tools).
6. Run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added resolve_allowed_dir + list_workspace_files (all files under a gated workspace; skips ~backlog task edit TASK-16 \
  --check-ac 1 --check-ac 2 --check-ac 3 --check-ac 4 --check-ac 5 \
  --append-notes "Added resolve_allowed_dir + list_workspace_files (all files under a gated workspace; skips ~$*). MCP tool list_workspace_files wired; docs updated. make check passed (unit + mcp_http)." \
  --final-summary "Exposed list_workspace_files so owners/ChatGPT can see every file in a workspace folder (workspace.json, Excel, notes), not just Excel via list_workspaces. Path-gated under MENUYUKTI_DATA_DIR; lock files skipped. Verified with unit tests and HTTP MCP integration; make check green." \
  -s Done). MCP tool list_workspace_files wired; docs updated. make check passed (unit + mcp_http).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed list_workspace_files so owners/ChatGPT can see every file in a workspace folder (workspace.json, Excel, notes), not just Excel via list_workspaces. Path-gated under MENUYUKTI_DATA_DIR; lock files skipped. Verified with unit tests and HTTP MCP integration; make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
