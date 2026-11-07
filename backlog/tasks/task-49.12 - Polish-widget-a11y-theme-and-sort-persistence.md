---
id: TASK-49.12
title: Polish widget a11y theme and sort persistence
status: To Do
assignee: []
created_date: '2026-11-07T04:00:00Z'
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
- [ ] #1 hello-world uses shared theme tokens/styles so it is readable in light and dark host themes
- [ ] #2 Segmented control supports keyboard arrow navigation consistent with radiogroup expectations
- [ ] #3 Pair lift and weak items persist their sort preference across remounts the same way venue/menu persist UI choices
- [ ] #4 pnpm check passes
<!-- AC:END -->
