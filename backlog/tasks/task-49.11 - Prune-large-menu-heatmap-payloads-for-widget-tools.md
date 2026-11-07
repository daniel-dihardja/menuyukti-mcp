---
id: TASK-49.11
title: Prune large menu heatmap payloads for widget tools
status: To Do
assignee: []
created_date: '2026-11-07T04:00:00Z'
labels:
  - widgets
  - performance
dependencies: []
references:
  - apps/menuyukti/src/server.ts
  - apps/menuyukti/src/lib/analytics/heatmaps.ts
  - apps/menuyukti/src/views/menu-heatmaps.tsx
parent_task_id: TASK-49
priority: medium
type: enhancement
ordinal: 133000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
render_menu_heatmaps currently sends the full heatmaps structuredContent (demo can be ~98 menus × daily 24h + weekly). The Codex HTML skill already defaults to a top-N subset. Shipping everything then filtering client-side inflates MCP Apps tool results and iframe hydrate cost for little owner benefit on first paint.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 render_menu_heatmaps (and get_menu_heatmaps if sharing the same builder) can limit menus and/or daily hours without breaking category filters for the returned set
- [ ] #2 Defaults and any new inputs are documented in the tool description so agents know what was truncated
- [ ] #3 Demo workspace still produces a usable heatmap widget
- [ ] #4 pnpm check passes
<!-- AC:END -->
