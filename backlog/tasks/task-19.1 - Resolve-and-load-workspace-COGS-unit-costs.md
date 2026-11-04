---
id: TASK-19.1
title: Resolve and load workspace COGS unit costs
status: To Do
assignee: []
created_date: '2026-11-04T08:14:00Z'
updated_date: '2026-11-04T08:16:00Z'
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
- [ ] #1 resolve_workspace_cogs_path (or equivalent) returns the gated COGS Excel path from files.cogs for a workspace folder under MENUYUKTI_DATA_DIR
- [ ] #2 Missing files.cogs role, missing file, or path outside the data dir returns a clear typed/string error naming the gap
- [ ] #3 Loader returns a menu-name → unit-cost map from the demo cogs.xlsx shape (Menu + COGS columns)
- [ ] #4 Unit tests cover successful demo cafe COGS load (non-empty menu→cost map with expected columns)
- [ ] #5 Unit tests cover missing files.cogs role, missing COGS file, and path-outside-MENUYUKTI_DATA_DIR failures
<!-- AC:END -->
