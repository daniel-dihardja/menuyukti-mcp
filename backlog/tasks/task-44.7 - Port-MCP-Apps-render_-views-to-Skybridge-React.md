---
id: TASK-44.7
title: Port MCP Apps render_ views to Skybridge React
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T18:25:00Z'
labels:
  - skybridge
  - migration
  - ui
milestone: m-2
dependencies:
  - TASK-44.6
references:
  - https://docs.skybridge.tech/build/view.md
  - crates/menuyukti-mcp/assets/
documentation:
  - plugins/menuyukti/README.md
parent_task_id: TASK-44
priority: high
type: feature
ordinal: 107000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ChatGPT Desktop owners expect mounted widgets for show/chart asks. Current hand-rolled HTML uses window.openai plus ui/initialize and MIME text/html+skybridge. Skybridge React views should replace those assets so host differences are handled by the framework.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 render_venue_slot_strength, render_pair_lift_matrix, render_menu_heatmaps, and render_weak_items_review mount Skybridge views with structured analytics data
- [x] #2 Views support the same core interactions as today (metric/category toggles and sortable weak-items table where applicable)
- [x] #3 A local Skybridge/DevTools or preview path exists for styling views without requiring ChatGPT
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Study Rust widget HTML interactions and Skybridge view binding (hello-world pattern).
2. Add shared React view chrome/theme helpers under apps/menuyukti/src/views.
3. Port venue-slot-strength, pair-lift-matrix, menu-heatmaps, weak-items-review as Skybridge views with core toggles/sort.
4. Register four render_* tools on the Skybridge server returning the same structured payloads as get_/analyze_.
5. Document DevTools preview path; smoke-render tools locally; finalize ACs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added Skybridge React views for venue-slot-strength, pair-lift-matrix, menu-heatmaps, weak-items-review (metric/view toggles, category filters, sortable weak table). Registered four render_* tools with ui resource meta. DevTools preview documented in apps/menuyukti/README. MCP smoke on :8791: tools list ui meta, 35 slots / 98 menus / stars focus / 64 weak, 5 view resources.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Ported Menuyukti MCP Apps charts/tables to Skybridge React views bound to render_venue_slot_strength, render_pair_lift_matrix, render_menu_heatmaps, and render_weak_items_review. Core interactions match the Rust HTML widgets (demand/orders/share, daily/weekly + matrix category filters, column sort, sortable weak-items table). Local preview via make mcp-skybridge / Skybridge DevTools without ChatGPT. Follow-up: TASK-44.8 retargets skills/docs to prefer these render_ tools.
<!-- SECTION:FINAL_SUMMARY:END -->
