---
id: TASK-49.6
title: Add outputSchema and share analytics payload types with views
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:13:00Z'
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
- [x] #1 Analytics render_* (and matching get_* where practical) declare outputSchema aligned with structuredContent
- [x] #2 Venue slot, pair lift, menu heatmaps, and weak items views consume shared TypeScript types from lib/analytics (or a shared types module) instead of divergent local interfaces for the same fields
- [x] #3 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add Zod output schemas for venue slot, pair lift, menu heatmaps, and weak items payloads (aligned with analytics-payloads).
2. Attach outputSchema to matching get_* and render_* tools in server.ts.
3. Export payload TypeScript types and update the four analytics views to import them instead of local loose interfaces.
4. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added lib/analytics-schemas.ts Zod shapes; wired outputSchema on get_/render_ for venue slot, pair lift, menu heatmaps, weak items. Views import SlotDemandCell / MenuBasketPair / MenuHeatmapPayload / WeakDisplayRow from lib/analytics. pnpm check 30 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analytics get_* and render_* tools now declare Zod outputSchema matching structuredContent. The four analytics views consume shared analytics types instead of local loose interfaces. Verified with pnpm check.
<!-- SECTION:FINAL_SUMMARY:END -->
