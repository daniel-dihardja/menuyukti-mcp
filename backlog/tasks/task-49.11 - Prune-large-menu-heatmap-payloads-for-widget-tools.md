---
id: TASK-49.11
title: Prune large menu heatmap payloads for widget tools
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T04:00:00Z'
updated_date: '2026-11-07T04:15:00Z'
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
- [x] #1 render_menu_heatmaps (and get_menu_heatmaps if sharing the same builder) can limit menus and/or daily hours without breaking category filters for the returned set
- [x] #2 Defaults and any new inputs are documented in the tool description so agents know what was truncated
- [x] #3 Demo workspace still produces a usable heatmap widget
- [x] #4 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add optional limit/top-N (and active-hours pruning) to buildMenuHeatmapsPayload / tool input schemas for get_ and render_menu_heatmaps.
2. Default to a sensible top-N for widget/JSON responses; document truncation in tool descriptions.
3. Keep category filters working on the returned set; verify demo cafe still usable.
4. Add unit coverage and run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added pruneMenuHeatmaps + DEFAULT_MENU_HEATMAP_TOP_N=25. buildMenuHeatmapsPayload accepts topN/activeHoursOnly and returns menu_total, limit_applied, active_hours_only. get_ defaults to all menus; render_ defaults to top 25 + active hours. Optional limit/active_hours_only on both tools. Demo cafe widget path returns 25 of 98. pnpm check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Menu heatmap get_/render_ tools can truncate to top-N menus and prune idle daily hours. render_menu_heatmaps defaults to top 25 + active hours (skill-aligned); get_menu_heatmaps stays full unless limit is set. Truncation is visible via menu_total/limit_applied in the payload and tool descriptions.
<!-- SECTION:FINAL_SUMMARY:END -->
