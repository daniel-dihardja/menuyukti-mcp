---
id: TASK-49.5
title: DRY get_* and render_* analytics tool handlers
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:10:00Z'
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
- [x] #1 Venue slot, pair lift, menu heatmaps, and weak items each have one shared payload builder used by both get_* and render_*
- [x] #2 Public MCP tool names, input schemas, and response shapes stay compatible with current clients/skills
- [x] #3 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read get_/render_ pairs in server.ts and extract shared payload builders (venue slot, pair lift, menu heatmaps, weak items) that load data and return the structuredContent object (or throw).
2. Wire get_* to okJson(builder) and render_* to widgetJson(builder, message) without changing tool names, schemas, or response shapes.
3. Keep builders colocated (server helpers or thin lib module) without moving analytics math.
4. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added lib/analytics-payloads.ts with buildMenuHeatmapsPayload, buildVenueSlotPayload, buildPairLiftPayload, buildWeakItemsPayload (+ WeakItemsPayloadError). get_/render_ handlers wrap the same builders via okJson/widgetJson. pnpm check 30 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Extracted shared analytics payload builders so each get_* / render_* pair computes once. Tool names, schemas, and response shapes unchanged. Verified with pnpm check.
<!-- SECTION:FINAL_SUMMARY:END -->
