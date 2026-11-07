---
id: TASK-44.4
title: Port workspace onboarding and path gating to TypeScript
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T17:44:00Z'
labels:
  - skybridge
  - migration
  - security
milestone: m-2
dependencies:
  - TASK-44.3
references:
  - crates/menuyukti/src/paths.rs
  - crates/menuyukti/src/workspace.rs
  - crates/menuyukti/src/setup.rs
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-44
priority: high
type: feature
ordinal: 104000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Local privacy depends on path gating under MENUYUKTI_DATA_DIR and the owner ritual tools (setup status, list workspaces/files, write config, knowledge). Porting analytics without this boundary would break the product promise and onboarding flow.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Equivalent setup/workspace tools work on the Skybridge server: get_setup_status, list_workspaces, list_workspace_files, write_workspace_config, read/write_workspace_knowledge
- [x] #2 User-supplied paths are rejected when outside the configured data dir (traversal and absolute escape covered by tests)
- [x] #3 demo-cafe-july-2026 (or fixture workspace) can be listed and configured through the TS tools
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Port resolve_allowed_path / data-dir helpers from crates/menuyukti/src/paths.rs into apps/menuyukti with unit tests.
2. Port setup/workspace list/config/knowledge behaviours needed by get_setup_status, list_workspaces, list_workspace_files, write_workspace_config, read/write_workspace_knowledge.
3. Register those tools on the Skybridge server alongside the hello-world smoke tool.
4. Verify against demo-cafe-july-2026 under MENUYUKTI_DATA_DIR; cover path escape rejection with tests.
5. Finalize TASK-44.4 ACs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ported paths/setup/workspace into apps/menuyukti/src/lib with vitest (6 passing). Registered get_setup_status, list_workspaces, list_workspace_files, write_workspace_config, read/write_workspace_knowledge on Skybridge. Verified against workspaces/demo-cafe-july-2026 via MCP on :8792; ../etc rejected with isError.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
TypeScript path gating and workspace onboarding tools now run on the Skybridge server: setup/list/config/knowledge APIs with MENUYUKTI_DATA_DIR sandboxing, unit tests for traversal rejection, and live MCP checks on demo-cafe-july-2026. Analytics Excel tools remain on Rust until TASK-44.5+.
<!-- SECTION:FINAL_SUMMARY:END -->
