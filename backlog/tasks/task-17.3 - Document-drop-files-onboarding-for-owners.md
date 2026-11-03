---
id: TASK-17.3
title: Document drop-files onboarding for owners
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T16:40:00Z'
updated_date: '2026-11-03T16:44:00Z'
labels: []
dependencies:
  - TASK-17.1
  - TASK-17.2
references:
  - docs/howtos/workspaces.md
  - README.md
parent_task_id: TASK-17
ordinal: 29000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
README and workspaces howto still tell owners to copy and edit workspace.json. That conflicts with LLM-written config and the two-file drop flow.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Owner docs say: create folder, drop sales+COGS, ask ChatGPT to set up the workspace
- [x] #2 workspace.yaml is documented as LLM-managed; owners are not asked to edit it by hand
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Tighten owner README + workspaces howto for drop-two-files then ask ChatGPT to set up.
2. Make clear workspace.yaml is LLM-managed (owners do not edit by hand).
3. Mention write_workspace_config in the tools list; finalize TASK-17.3 and parent TASK-17.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Rewrote workspaces howto with Owner flow (drop two files); README and workspaces/README stress LLM-managed workspace.yaml.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs now describe drop sales+COGS then ask ChatGPT to set up. workspace.yaml is documented as LLM-managed, not hand-edited.
<!-- SECTION:FINAL_SUMMARY:END -->
