---
id: TASK-49.8
title: Fix stale get_weak_items_review tool description
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
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
- [ ] #1 get_weak_items_review description no longer references TASK-44.7 or implies the widget is unshipped
- [ ] #2 Description points agents to render_weak_items_review for charts/tables when the host mounts MCP Apps views
<!-- AC:END -->
