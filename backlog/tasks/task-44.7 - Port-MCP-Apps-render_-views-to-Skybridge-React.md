---
id: TASK-44.7
title: Port MCP Apps render_ views to Skybridge React
status: To Do
assignee: []
created_date: '2026-11-06T17:30:00Z'
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
- [ ] #1 render_venue_slot_strength, render_pair_lift_matrix, render_menu_heatmaps, and render_weak_items_review mount Skybridge views with structured analytics data
- [ ] #2 Views support the same core interactions as today (metric/category toggles and sortable weak-items table where applicable)
- [ ] #3 A local Skybridge/DevTools or preview path exists for styling views without requiring ChatGPT
<!-- AC:END -->
