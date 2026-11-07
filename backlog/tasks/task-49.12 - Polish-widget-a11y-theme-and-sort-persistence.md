---
id: TASK-49.12
title: Polish widget a11y theme and sort persistence
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T04:00:00Z'
updated_date: '2026-11-07T04:19:00Z'
labels:
  - widgets
  - ux
  - a11y
dependencies: []
references:
  - apps/menuyukti/src/views/hello-world.tsx
  - apps/menuyukti/src/views/shared.tsx
  - apps/menuyukti/src/views/pair-lift-matrix.tsx
  - apps/menuyukti/src/views/weak-items-review.tsx
parent_task_id: TASK-49
priority: low
type: enhancement
ordinal: 134000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Small consistency gaps left after the Skybridge port: hello-world uses a hard-coded light palette instead of shared widget theme; Segmented radiogroups lack arrow-key roving tabindex; pair-lift and weak-items do not persist sort preferences while venue/menu persist metric/view choices.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 hello-world uses shared theme tokens/styles so it is readable in light and dark host themes
- [x] #2 Segmented control supports keyboard arrow navigation consistent with radiogroup expectations
- [x] #3 Pair lift and weak items persist their sort preference across remounts the same way venue/menu persist UI choices
- [x] #4 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Restyle hello-world with shared Panel/widget.css tokens.
2. Add Left/Right/Home/End arrow-key roving tabindex on Segmented radiogroups.
3. Persist pair-lift and weak-items sort key/direction via localStorage helpers matching venue/menu persistence.
4. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
hello-world now uses Panel + widget.css. Segmented radiogroup supports Arrow/Home/End with roving tabindex. pair-lift and weak-items persist sort via usePersistedJson. pnpm check 38 green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Widget polish: hello-world follows the shared light/dark theme; Segmented controls are keyboard-navigable; pair-lift and weak-items remember sort preferences across remounts like venue/menu UI choices.
<!-- SECTION:FINAL_SUMMARY:END -->
