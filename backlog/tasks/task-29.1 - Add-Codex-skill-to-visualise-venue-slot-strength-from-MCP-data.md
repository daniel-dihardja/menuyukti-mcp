---
id: TASK-29.1
title: Add Codex skill to visualise venue slot strength from MCP data
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T03:54:00Z'
updated_date: '2026-11-06T03:58:00Z'
labels:
  - codex
  - ux
  - analytics
dependencies: []
references:
  - ../v3/apps/web/components/analytics/venue-demand-heatmap.tsx
  - ../v3/apps/web/components/analytics/order-metrics-venue-heatmap.tsx
  - ../v3/apps/web/lib/analytics/menu-combos-page-adapter.ts
  - ../v3/apps/web/lib/chat/format-visualization-for-chat.ts
  - ../v3/apps/graphql/knowledge/kpi-venue-slot-strength.md
  - crates/menuyukti/src/analytics/slot_demand.rs
  - crates/menuyukti-mcp/src/main.rs
  - .agents/skills/menuyukti-mcp/SKILL.md
  - .agents/skills/menuyukti-venue-slot-strength/references/slot-strength.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-29
priority: high
type: feature
ordinal: 65000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners asking Codex to show when the venue is busy get numbers or ChatGPT-only MCP Apps widgets, not the v3 weekday × meal-period heatmap. Local MCP already returns a 35-cell `slot_demand_profile` via `get_venue_slot_strength` (same shape as v3 `SlotDemandCell` / `OrderMetricsVenueHeatmap`).

v3 shows a fixed grid: rows = meal periods (label + hours), columns = Mon–Sun, cell colour/value from `demandIndex`, with order count and relative demand (`low` / `average` / `high`) in tooltips/aria. Chat text formatting lists day × meal period with orders, demand index, and relative demand. Codex needs a presentation skill that reproduces that semantics in markdown (default) from MCP JSON without recomputing analytics.

Do not fold this into `menuyukti-venue-slot-strength` (compute/offline) or replace `render_venue_slot_strength` (ChatGPT widget).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 New skill under `.agents/skills/` with SKILL.md frontmatter triggers (show/chart/heatmap/busy times/slot strength) and `agents/openai.yaml`
- [x] #2 Skill prefers `get_venue_slot_strength` JSON; may use offline slot-strength script output only when MCP is unavailable; does not invent cells
- [x] #3 Recipe matches v3: 7×5 grid, rows meal periods with hours labels, cols Mon–Sun, primary cell value `demand_index`, and communicates `order_count` plus `relative_demand` tiers
- [x] #4 Default Codex output is a markdown heatmap (plus short staffing/promo narration from high/low cells); does not require MCP Apps iframe mounting
- [x] #5 `menuyukti-mcp` routes venue show/chart/heatmap asks to this skill after the data tool; `render_venue_slot_strength` remains documented for ChatGPT widget hosts only
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add `.agents/skills/menuyukti-visualize-venue-slot-strength/` with SKILL.md + agents/openai.yaml + references/heatmap-recipe.md matching v3 7×5 demand_index semantics.
2. Update menuyukti-mcp goal table: show/chart → get_venue_slot_strength then this skill; keep render_* as ChatGPT-widget-only note.
3. Point offline compute skill at the new viz skill for presentation after JSON.
4. Light README/workspaces pointer (shared with parent AC #4; finish with 29.2).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added menuyukti-visualize-venue-slot-strength (SKILL.md, openai.yaml, heatmap-recipe.md). Wired menuyukti-mcp goal table and offline venue-slot skill to prefer get_* + visualise skill; render_* ChatGPT-only.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added Codex presentation skill menuyukti-visualize-venue-slot-strength (SKILL.md, openai.yaml, heatmap-recipe.md) matching v3 7×5 demand_index semantics. Wired menuyukti-mcp and offline venue-slot compute skill to fetch get_venue_slot_strength (or script JSON) then visualise; render_* kept for ChatGPT MCP Apps only. Verified skill files and router references on disk.
<!-- SECTION:FINAL_SUMMARY:END -->
