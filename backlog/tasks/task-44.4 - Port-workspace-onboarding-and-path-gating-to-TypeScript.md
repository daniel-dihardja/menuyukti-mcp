---
id: TASK-44.4
title: Port workspace onboarding and path gating to TypeScript
status: To Do
assignee: []
created_date: '2026-11-06T17:30:00Z'
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
- [ ] #1 Equivalent setup/workspace tools work on the Skybridge server: get_setup_status, list_workspaces, list_workspace_files, write_workspace_config, read/write_workspace_knowledge
- [ ] #2 User-supplied paths are rejected when outside the configured data dir (traversal and absolute escape covered by tests)
- [ ] #3 demo-cafe-july-2026 (or fixture workspace) can be listed and configured through the TS tools
<!-- AC:END -->
