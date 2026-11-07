---
id: TASK-51.3
title: Document data-dir knowledge and migrate demo notes
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-07T05:38:00Z'
updated_date: '2026-11-07T05:42:00Z'
labels:
  - knowledge
  - docs
dependencies:
  - TASK-51.2
references:
  - docs/howtos/workspaces.md
  - workspaces/README.md
parent_task_id: TASK-51
priority: medium
type: docs
ordinal: 103000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners and agents still see knowledge.md described as a per-workspace file next to sales/COGS. After the API moves to the data-dir root, docs and the demo layout must match or coaches will keep writing notes into period folders.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Workspaces howto, workspaces README, example YAML, and skills describe knowledge.md at the data-dir root and one location per data dir
- [x] #2 Demo knowledge content lives at workspaces/knowledge.md (not under demo-cafe-july-2026); demo workspace.yaml no longer declares files.knowledge
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Move demo knowledge.md to workspaces/knowledge.md; strip files.knowledge from demo workspace.yaml and example YAML.
2. Update workspaces howto, README, and menuyukti-mcp skills for root knowledge + one-location-per-data-dir.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Moved demo notes to workspaces/knowledge.md; stripped files.knowledge from demo + example YAML; updated howto, README, skills, gitignore allowlist. make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Documented data-dir-wide knowledge and one-location-per-data-dir; migrated demo knowledge.md to workspaces/ and removed the role from workspace configs.
<!-- SECTION:FINAL_SUMMARY:END -->
