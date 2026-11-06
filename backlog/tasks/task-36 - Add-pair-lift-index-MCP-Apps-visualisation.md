---
id: TASK-36
title: Add pair lift index MCP Apps visualisation
status: Done
assignee: []
created_date: '2026-11-06T07:13:00Z'
updated_date: '2026-11-06T07:18:00Z'
labels:
  - mcp
  - ui
  - chatgpt
  - analytics
dependencies:
  - TASK-20
  - TASK-25
references:
  - crates/menuyukti/src/analytics/basket_affinities.rs
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - backlog/tasks/task-25 - Add-venue-slot-strength-MCP-Apps-visualisation.md
priority: high
type: feature
ordinal: 77000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners ask which dishes belong in combos and upsells. Local MCP already returns focus_menus + matrix_lift via get_pair_lift_matrix (TASK-20), and venue slot strength proved MCP Apps heatmap mounting (TASK-25). ChatGPT still only narrates combo advice as text—owners cannot see the pair lift matrix heatmap they get in Menuyukti v3 chat.

Ship a ChatGPT MCP Apps widget that visualises focus menus × focus menus lift cells without GraphQL or the v3 Next.js app. Keep get_pair_lift_matrix useful as a data-only tool; attach UI via a dedicated render path. Include local widget-preview fixtures so developers can style without ChatGPT.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT (or MCP Inspector) can obtain a pair lift heatmap UI resource linked from an MCP tool via _meta.ui.resourceUri / openai/outputTemplate
- [x] #2 The widget renders a focus_menus × focus_menus lift grid from matrix_lift (null diagonal; lift semantics clear to owners)
- [x] #3 get_pair_lift_matrix remains usable without a widget template on every call
- [x] #4 Owner or MCP docs mention how to show the pair lift visualisation
- [x] #5 Local widget preview lists pair-lift fixtures (populated + empty/edge)
- [x] #6 make check passes for the completed feature
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Pair lift MCP Apps visualisation shipped: heatmap widget asset (36.1), render_pair_lift_matrix + ui:// resource with HTTP tests (36.2), owner docs (36.3). get_pair_lift_matrix stays data-only. Preview fixtures included. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
