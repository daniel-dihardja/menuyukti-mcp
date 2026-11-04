---
id: TASK-25.1
title: Build venue slot strength heatmap widget from slot_demand_profile
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T13:33:00Z'
updated_date: '2026-11-04T13:35:00Z'
labels:
  - ui
  - chatgpt
  - analytics
dependencies: []
references:
  - ../v3/apps/web/components/analytics/venue-demand-heatmap.tsx
  - ../v3/apps/web/components/chat/visualizations/venue-slot-strength-card.tsx
  - crates/menuyukti/src/analytics/slot_demand.rs
parent_task_id: TASK-25
priority: high
type: feature
ordinal: 53000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
v3 chat shows venue demand as a weekday × meal-period heatmap (`VenueDemandHeatmap`) driven by `slotDemandProfile` cells. The local MCP Apps path only has a Hello World iframe. Without a self-contained widget that paints the 35-cell profile (low/average/high + demand index), ChatGPT cannot show the product visualisation even when analytics JSON is available.

Build the heatmap HTML/JS (or bundled UI) for MCP Apps: consume the same cell fields as `SlotDemandCell` / `get_venue_slot_strength` (`day`, `meal_period`, labels, `order_count`, `traffic_share`, `demand_index`, `relative_demand`). No Next.js, GraphQL, or sales file I/O inside the widget—data arrives via tool structuredContent. Visual semantics should match v3 closely enough that an owner recognizes the chart.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Widget asset paints a 7×5 (or equivalent) weekday × meal-period grid from a `slot_demand_profile` array in structuredContent
- [x] #2 Cell styling reflects relative demand / demand index in line with v3 venue heatmap semantics (low / average / high distinguishable)
- [x] #3 Empty or missing profile shows a clear empty/error state without throwing
- [x] #4 Widget has no dependency on v3 web runtime APIs (no SWR/GraphQL/Next routes)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add self-contained venue_slot_strength_widget.html under menuyukti-mcp/assets that renders a 7x5 grid from structuredContent.slot_demand_profile. 2. Color cells by relative_demand (low/average/high) with demand_index intensity; tooltips for orders/share/index; empty-state when profile missing. 3. Match field names to SlotDemandCell snake_case from Rust JSON. 4. Finalize 25.1 then proceed to MCP wiring in 25.2.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added crates/menuyukti-mcp/assets/venue_slot_strength_widget.html: 7x5 grid from slot_demand_profile, teal venue demand scale by demand_index, relative_demand borders, empty states, no v3 runtime deps.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Self-contained venue slot strength heatmap widget asset paints weekday x meal-period cells from structuredContent.slot_demand_profile with v3-aligned low/average/high styling. Ready for MCP resource wiring in TASK-25.2.
<!-- SECTION:FINAL_SUMMARY:END -->
