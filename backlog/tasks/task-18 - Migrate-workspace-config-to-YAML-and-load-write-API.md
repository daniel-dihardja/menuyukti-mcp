---
id: TASK-18
title: Migrate workspace config to YAML and load/write API
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T16:39:00Z'
updated_date: '2026-11-03T16:41:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/paths.rs
  - workspaces/demo-cafe-july-2026/workspace.json
  - workspace.example.json
ordinal: 26000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
workspace.json is awkward for LLM/tool writes and is not loaded by Rust yet. A YAML schema plus gated load/write lets the server own config while owners stay out of the format.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Demo and example use workspace.yaml with the same roles as today (sales, cogs, knowledge)
- [x] #2 Library can load and write path-gated workspace.yaml under MENUYUKTI_DATA_DIR
- [x] #3 Invalid or outside-sandbox writes are rejected; unit tests cover load/write and path gate
- [x] #4 Docs and gitignore allowlist reference workspace.yaml instead of workspace.json
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add serde_yaml workspace dep and a workspace config module (schema matching current JSON roles).
2. Implement path-gated load_workspace_config and write_workspace_config targeting workspace.yaml under a workspace folder.
3. Migrate demo + example from workspace.json to workspace.yaml; update gitignore allowlist.
4. Update docs references that name workspace.json for the config file rename (full onboarding rewrite stays in TASK-17.3).
5. Unit tests for load/write, traversal rejection, and missing config; run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added workspace module with YAML load/write under MENUYUKTI_DATA_DIR; migrated demo+example to workspace.yaml; gitignore/docs updated. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced workspace.json with path-gated workspace.yaml load/write API (serde_yaml). Demo/example migrated; invalid role paths and outside-sandbox writes rejected. Verified with unit tests and make check.
<!-- SECTION:FINAL_SUMMARY:END -->
