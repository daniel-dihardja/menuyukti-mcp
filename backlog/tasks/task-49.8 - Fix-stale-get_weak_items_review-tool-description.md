---
id: TASK-49.8
title: Fix stale get_weak_items_review tool description
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:08:00Z'
labels:
  - docs
  - mcp
dependencies: []
references:
  - apps/menuyukti/src/server.ts
parent_task_id: TASK-49
priority: low
type: docs
ordinal: 130000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
get_weak_items_review still tells agents the MCP Apps weak-items widget "arrives in TASK-44.7+" even though render_weak_items_review already ships. Stale copy steers hosts toward JSON-only / skill HTML when the view is available.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 get_weak_items_review description no longer references TASK-44.7 or implies the widget is unshipped
- [x] #2 Description points agents to render_weak_items_review for charts/tables when the host mounts MCP Apps views
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update get_weak_items_review description in server.ts to drop TASK-44.7 wording and point at render_weak_items_review for charts.
2. Run pnpm check (typecheck only impacted).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated get_weak_items_review description; removed TASK-44.7; points to render_weak_items_review for mounted views.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
get_weak_items_review no longer claims the MCP Apps widget is unshipped. Description now prefers render_weak_items_review for tables when the host mounts views, matching the other analytics get_/render_ pairs.
<!-- SECTION:FINAL_SUMMARY:END -->
