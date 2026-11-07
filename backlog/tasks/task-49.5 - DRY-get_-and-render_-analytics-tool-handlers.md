---
id: TASK-49.5
title: DRY get_* and render_* analytics tool handlers
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
labels:
  - skybridge
  - refactor
dependencies: []
references:
  - apps/menuyukti/src/server.ts
  - apps/menuyukti/src/lib/analytics
parent_task_id: TASK-49
priority: medium
type: enhancement
ordinal: 127000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each analytics outcome has near-copy-paste get_* (okJson) and render_* (widgetJson) handlers in server.ts that load sales/COGS and call the same lib/analytics builders. Drift between the two paths is easy and review noise is high. Shared payload builders should own the compute; tools only choose response wrapping and descriptions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Venue slot, pair lift, menu heatmaps, and weak items each have one shared payload builder used by both get_* and render_*
- [ ] #2 Public MCP tool names, input schemas, and response shapes stay compatible with current clients/skills
- [ ] #3 pnpm check passes
<!-- AC:END -->
