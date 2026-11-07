---
id: TASK-49.10
title: Add React view tests for analytics widgets
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:17:00Z'
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
- [x] #1 Vitest (or existing gate) covers at least one render path per analytics widget for populated fixture data
- [x] #2 Empty and/or error structuredContent cases are covered for the shared error-surface behavior
- [x] #3 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add React Testing Library + jsdom vitest coverage using the shared fixtures.
2. Cover populated render + error surface for each analytics widget.
3. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added analytics-views.test.tsx (jsdom + Testing Library) mocking helpers; populated + error cases for all four widgets using fixtures. vitest.config includes .tsx + react plugin. pnpm check 38 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Vitest now renders each analytics MCP Apps view with shared fixtures: populated data and errJson error surfaces. Covered by pnpm check (38 tests).
<!-- SECTION:FINAL_SUMMARY:END -->
