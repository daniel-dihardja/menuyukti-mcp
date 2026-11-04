---
id: TASK-25
title: Add venue slot strength MCP Apps visualisation
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T13:33:00Z'
updated_date: '2026-11-04T13:37:00Z'
labels:
  - mcp
  - ui
  - chatgpt
  - analytics
dependencies:
  - TASK-24
references:
  - crates/menuyukti/src/analytics/slot_demand.rs
  - crates/menuyukti-mcp/src/main.rs
  - ../v3/apps/web/components/chat/visualizations/venue-slot-strength-card.tsx
  - ../v3/apps/web/components/analytics/venue-demand-heatmap.tsx
  - https://developers.openai.com/plugins/build/chatgpt-ui
  - backlog/tasks/task-24 - Add-hello-world-MCP-Apps-widget-with-mock-data.md
priority: high
type: feature
ordinal: 52000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners ask when the venue is busy. Local MCP already returns a v3-compatible 35-cell `slot_demand_profile` via `get_venue_slot_strength` (TASK-22), and TASK-24 proved MCP Apps iframe wiring with a Hello World widget. ChatGPT still only narrates timing advice as text—owners cannot see the weekday × meal-period demand heatmap they get in Menuyukti v3 chat (`venue_slot_strength_heatmap` / `VenueDemandHeatmap`).

Ship a ChatGPT MCP Apps widget that visualises the same slot-demand cells (order count, traffic share, demand index, relative demand) without GraphQL or the v3 Next.js app. Keep `get_venue_slot_strength` useful as a data-only tool; attach UI via a dedicated render path. Host iframe mounting issues (Chrome LNA, Desktop vs plugin tunnel) are out of scope for these tasks but docs may mention how to verify.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT (or MCP Inspector) can obtain a venue slot strength heatmap UI resource linked from an MCP tool via `_meta.ui.resourceUri` / `openai/outputTemplate`
- [x] #2 The widget renders a weekday × meal-period grid from `slot_demand_profile` cells with demand semantics aligned to v3 (relative demand / demand index)
- [x] #3 `get_venue_slot_strength` remains usable without a widget template on every call
- [x] #4 Owner or MCP docs mention how to show the venue slot strength visualisation
- [x] #5 `make check` passes for the completed feature
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Venue slot strength MCP Apps visualisation shipped: heatmap widget asset (25.1), render_venue_slot_strength + ui:// resource with HTTP tests (25.2), owner docs (25.3). get_venue_slot_strength stays data-only. make check passed. Host iframe mounting remains a ChatGPT client concern.
<!-- SECTION:FINAL_SUMMARY:END -->
