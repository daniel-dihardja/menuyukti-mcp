---
id: TASK-49.10
title: Add React view tests for analytics widgets
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
labels:
  - widgets
  - testing
dependencies:
  - TASK-49.4
references:
  - apps/menuyukti/src/views
parent_task_id: TASK-49
priority: medium
type: task
ordinal: 132000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Beyond menu-heatmap-hours helpers and viewNameFromFile, there is almost no automated coverage of view rendering, empty states, or error surfaces. Regressions in axis orientation, filters, and empty UX have already bitten once (TASK-46).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Vitest (or existing gate) covers at least one render path per analytics widget for populated fixture data
- [ ] #2 Empty and/or error structuredContent cases are covered for the shared error-surface behavior
- [ ] #3 pnpm check passes
<!-- AC:END -->
