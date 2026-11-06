---
id: TASK-29.2
title: Add Codex skill to visualise menu item heatmaps from MCP data
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-06T03:54:00Z'
labels:
  - codex
  - ux
  - analytics
dependencies: []
references:
  - ../v3/apps/web/components/chat/visualizations/menu-item-heatmap-card.tsx
  - ../v3/apps/web/components/analytics/menu-heatmap-content.tsx
  - ../v3/apps/web/lib/analytics/heatmap.adapters.ts
  - ../v3/apps/web/lib/heatmap-config.ts
  - ../v3/apps/web/lib/chat/format-visualization-for-chat.ts
  - ../v3/apps/web/lib/chat/visualization-contract.ts
  - ../v3/apps/graphql/knowledge/kpi-menu-heatmap.md
  - crates/menuyukti/src/analytics/heatmaps.rs
  - crates/menuyukti-mcp/src/main.rs
  - .agents/skills/menuyukti-mcp/SKILL.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-29
priority: high
type: feature
ordinal: 66000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners asking Codex when each dish sells get large per-menu JSON from `get_menu_heatmaps` instead of the v3 menu item heatmap (`MenuItemHeatmapCard` / `MenuHeatmapContent`).

v3 shows two matrices of raw unit `quantity`: daily (menu rows × hour columns, default window 08:00–22:00 when opening hours are unknown) and weekly (menu rows × Mon–Sun). UI filters by POS `menuCategory` and, when menu-engineering matrix items exist, by BCG role (chat defaults toward evergreen star / plow horse / puzzle). Compact chat views surface peak window, top item by total, and item count; text summaries use top-N by weekly total with peak day and peak hour.

Codex needs a separate presentation skill from venue slot strength: different tool, metric (`quantity` not `demand_index`), multi-item selection, and daily vs weekly views. Sales alone is enough; matrix filtering is optional enrichment when COGS/matrix data is available.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 New skill under `.agents/skills/` (separate from venue-slot visualisation) with SKILL.md triggers and `agents/openai.yaml`
- [ ] #2 Skill uses `get_menu_heatmaps` payloads only and never invents unit counts
- [ ] #3 Recipe covers daily (hour × quantity) and weekly (weekday × quantity) markdown matrices aligned with v3 adapters, including a default daily hour window of 8–22 when venue hours are unknown
- [ ] #4 Skill selects what to draw: named dishes and/or top-N by total quantity (v3 chat uses top 25), and may optionally filter to star/plow_horse/puzzle when menu-engineering categories are available
- [ ] #5 After the grid, skill narrates peak hour/day and top item insights from the selected rows only
- [ ] #6 `menuyukti-mcp` routes dish timing / menu heatmap show asks to this skill after `get_menu_heatmaps`
<!-- AC:END -->
