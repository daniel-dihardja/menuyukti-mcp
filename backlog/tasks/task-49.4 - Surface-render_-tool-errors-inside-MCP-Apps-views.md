---
id: TASK-49.4
title: Surface render_* tool errors inside MCP Apps views
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:08:00Z'
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
- [x] #1 When structuredContent includes an error string, venue slot, pair lift, menu heatmaps, and weak items views show that message (or equivalent Empty/error UI) instead of a generic missing-payload empty
- [x] #2 Successful payloads still render as today
- [x] #3 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a small shared helper in views/shared (e.g. toolErrorMessage) that reads structuredContent.error.
2. Gate venue-slot, pair-lift, menu-heatmaps, and weak-items views to show Empty with that message before missing-payload checks.
3. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added toolErrorMessage + ToolError in shared.tsx; wired into venue-slot, pair-lift, menu-heatmaps, weak-items before missing-payload empties. Unit tests for toolErrorMessage. pnpm check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analytics MCP Apps views now surface errJson structuredContent.error via a shared ToolError empty state instead of generic missing-payload messages. Successful payloads unchanged. Verified with pnpm check (toolErrorMessage unit tests).
<!-- SECTION:FINAL_SUMMARY:END -->
