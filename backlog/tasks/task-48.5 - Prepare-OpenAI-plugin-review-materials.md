---
id: TASK-48.5
title: Prepare OpenAI plugin review materials
status: To Do
assignee: []
created_date: '2026-11-07T03:34:00Z'
labels:
  - openai
  - release
dependencies:
  - TASK-48.3
references:
  - https://developers.openai.com/plugins/deploy/submission
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
parent_task_id: TASK-48
priority: medium
type: docs
ordinal: 119000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Directory submission for MCP-backed plugins expects a review pack: five positive test cases, three negative cases, a demo recording, and release notes. Reviewers need a reproducible Desktop path (demo workspace) without MFA friction.

Draft cases and assets that exercise setup status, summarize, venue slot strength, menu heatmaps, and a COGS-backed weak-items path where applicable — plus negatives that refuse unsupported destructive or remote-upload asks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Five positive and three negative review cases written (prompts, expected tools/behavior)
- [ ] #2 Demo recording URL accessible to reviewers
- [ ] #3 Release notes for the submission version drafted
- [ ] #4 Reviewer instructions cover Node/Desktop prerequisites and demo-cafe-july-2026
<!-- AC:END -->
