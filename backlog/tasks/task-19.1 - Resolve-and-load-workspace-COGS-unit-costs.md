---
id: TASK-19.1
title: Resolve and load workspace COGS unit costs
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:14:00Z'
updated_date: '2026-11-04T08:22:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/workspace.rs
  - crates/menuyukti/src/load.rs
  - workspaces/demo-cafe-july-2026/cogs.xlsx
  - docs/howtos/workspaces.md
parent_task_id: TASK-19
priority: high
type: feature
ordinal: 31000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Menu engineering cannot run until unit costs are available on disk. Workspaces already store an optional files.cogs role in workspace.yaml and the demo ships cogs.xlsx (Menu + COGS columns), but there is no path resolver or Excel loader—only sales resolve exists today. Without this, the matrix tool has nothing safe to join against sales menus.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 resolve_workspace_cogs_path (or equivalent) returns the gated COGS Excel path from files.cogs for a workspace folder under MENUYUKTI_DATA_DIR
- [x] #2 Missing files.cogs role, missing file, or path outside the data dir returns a clear typed/string error naming the gap
- [x] #3 Loader returns a menu-name → unit-cost map from the demo cogs.xlsx shape (Menu + COGS columns)
- [x] #4 Unit tests cover successful demo cafe COGS load (non-empty menu→cost map with expected columns)
- [x] #5 Unit tests cover missing files.cogs role, missing COGS file, and path-outside-MENUYUKTI_DATA_DIR failures
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add resolve_workspace_cogs_path (require files.cogs) and fix join_workspace_role_path to take a role name.
2. Extend excel helpers to read a named sheet (COGS sheet in the demo workbook).
3. Add load_cogs / load_workspace_cogs returning HashMap<String, f64> from Menu+COGS columns with path gating.
4. Unit tests: demo cafe load success; missing role; missing file; path outside data dir.
5. Export from lib.rs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added resolve_workspace_cogs_path, excel named-sheet read, and load_workspace_cogs (COGS sheet Menu+COGS → HashMap). Unit tests: demo load, missing role/file, path outside data dir.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Workspace COGS resolve/load is path-gated and returns a menu→unit-cost map from the demo cogs.xlsx COGS sheet. Missing role/file and outside-sandbox paths error clearly. Verified with menuyukti unit tests and make check.
<!-- SECTION:FINAL_SUMMARY:END -->
