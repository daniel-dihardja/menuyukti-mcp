---
id: TASK-35
title: Add cell metric selector to venue slot strength widget
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T05:22:00Z'
updated_date: '2026-11-06T05:25:00Z'
labels:
  - ux
  - mcp-apps
  - widget
dependencies: []
references:
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - crates/menuyukti/src/analytics/slot_demand.rs
documentation:
  - docs/howtos/workspaces.md
priority: medium
type: feature
ordinal: 76000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners viewing the venue slot strength heatmap often want to switch what each cell shows (demand index vs order count vs traffic share) without leaving the chart. Today cells always display demand_index; the other fields are only in the hover title.

All metrics already exist on each slot_demand_profile cell — this is a presentation control in the MCP Apps widget, not new analytics. Cell label and color intensity both follow the selected metric so encoding stays honest; demand index remains the default. Hover still exposes the full metric set.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Widget offers a segmented control above the grid to choose the cell metric: Demand (default), Orders, Share
- [x] #2 Changing the selection updates all 35 cell labels and color intensity immediately without another tool call
- [x] #3 Legend copy reflects the active metric (e.g. Low → High demand index / orders / traffic share)
- [x] #4 Hover/title still exposes day, meal period, orders, traffic share, demand index, and relative demand
- [x] #5 Values format correctly per metric (index 2dp, orders integer, share percent); zero/empty cells stay readable
- [x] #6 Control is keyboard-accessible and usable in light and dark host themes
- [x] #7 Selection persists in localStorage for the widget; URI is cache-busted; make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add segmented metric control (Demand / Orders / Share) above the grid; store selection in localStorage.
2. Re-render cell labels + color scale from the selected metric (min→max of that field); sync legend text.
3. Keep full hover details; format values per metric; keyboard support on the segment group.
4. Theme the control with host CSS variables; bump widget URI to v5; update mcp_http assertions.
5. make check; finalize TASK-35.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Shipped segmented Demand/Orders/Share control; label+color both follow selected metric; legend/showing copy sync; hover keeps full details; keyboard radiogroup; localStorage persistence; URI v5. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Venue slot strength widget now lets owners switch cell metric via a Demand / Orders / Share segmented control. Labels and color intensity both follow the selection (default Demand), legend copy updates, hover still shows all fields, selection persists locally, and the resource is cache-busted as ui://widget/venue-slot-strength-v5.html. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
