---
id: TASK-49.4
title: Surface render_* tool errors inside MCP Apps views
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
labels:
  - widgets
  - ux
dependencies: []
references:
  - apps/menuyukti/src/lib/mcp-result.ts
  - apps/menuyukti/src/views/venue-slot-strength.tsx
  - apps/menuyukti/src/views/pair-lift-matrix.tsx
  - apps/menuyukti/src/views/menu-heatmaps.tsx
  - apps/menuyukti/src/views/weak-items-review.tsx
parent_task_id: TASK-49
priority: high
type: bug
ordinal: 126000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Failed render_* tools return errJson with structuredContent { error: message }. Views only check pending/missing arrays and show generic empties such as "Missing slot_demand_profile". If the host still mounts the view on isError, owners never see the actionable COGS/path guidance from the server.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 When structuredContent includes an error string, venue slot, pair lift, menu heatmaps, and weak items views show that message (or equivalent Empty/error UI) instead of a generic missing-payload empty
- [ ] #2 Successful payloads still render as today
- [ ] #3 pnpm check passes
<!-- AC:END -->
