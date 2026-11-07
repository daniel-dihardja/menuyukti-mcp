---
id: TASK-49.6
title: Add outputSchema and share analytics payload types with views
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
labels:
  - skybridge
  - typescript
  - widgets
dependencies:
  - TASK-49.5
references:
  - apps/menuyukti/src/server.ts
  - apps/menuyukti/src/helpers.ts
  - apps/menuyukti/src/views
  - apps/menuyukti/src/lib/analytics
parent_task_id: TASK-49
priority: medium
type: enhancement
ordinal: 128000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
render_* tools omit outputSchema, so useToolInfo inference for output is weak. Views redefine loose SlotCell / HeatmapItem / WeakRow shapes with casts instead of importing analytics types. That invites silent payload drift between server and widgets.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Analytics render_* (and matching get_* where practical) declare outputSchema aligned with structuredContent
- [ ] #2 Venue slot, pair lift, menu heatmaps, and weak items views consume shared TypeScript types from lib/analytics (or a shared types module) instead of divergent local interfaces for the same fields
- [ ] #3 pnpm check passes
<!-- AC:END -->
