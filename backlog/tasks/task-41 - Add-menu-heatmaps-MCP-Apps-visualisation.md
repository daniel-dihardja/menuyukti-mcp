---
id: TASK-41
title: Add menu heatmaps MCP Apps visualisation
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T07:51:00Z'
updated_date: '2026-11-06T07:56:00Z'
labels:
  - mcp
  - ui
  - chatgpt
  - analytics
dependencies:
  - TASK-40
  - TASK-36
references:
  - crates/menuyukti/src/analytics/heatmaps.rs
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/assets/pair_lift_matrix_widget.html
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - crates/menuyukti-mcp/preview/index.html
  - .agents/skills/menuyukti-visualize-menu-heatmaps/SKILL.md
documentation:
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 87000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners asking when each dish sells still cannot see a ChatGPT-mounted heatmap. Venue slot strength and pair lift already ship MCP Apps widgets with local preview fixtures, but menu heatmaps only have get_menu_heatmaps JSON plus a Codex skill that writes a separate HTML file. make widgets-preview therefore has no menu-heatmap entry, and ChatGPT Desktop has nothing to mount.

Ship an MCP Apps widget that renders per-menu daily/weekly quantity heatmaps from get_menu_heatmaps-shaped structuredContent, including interactive matrix_category filters (star / plow_horse / puzzle / weak) with weak off by default when categories are present. Keep get_menu_heatmaps data-only; add a dedicated render tool, ui:// resource, and preview fixtures so developers can style with make widgets-preview.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT (or MCP Inspector) can obtain a menu heatmaps UI resource linked from an MCP tool via _meta.ui.resourceUri / openai/outputTemplate
- [x] #2 The widget renders per-menu quantity heatmaps (daily and/or weekly) from structuredContent heatmaps payloads
- [x] #3 When matrix_category is present, the widget exposes enable/disable filters for star, plow_horse, puzzle, and weak, with weak disabled by default
- [x] #4 get_menu_heatmaps remains usable without a widget template on every call
- [x] #5 Local widget preview lists menu-heatmap fixtures (populated + empty/edge) on the preview index
- [x] #6 Owner or MCP docs mention how to show the menu heatmaps visualisation
- [x] #7 make check passes for the completed feature
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. TASK-41.1: menu heatmaps widget HTML + preview fixtures/index links.
2. TASK-41.2: render_menu_heatmaps + ui:// resource + HTTP tests + instructions.
3. TASK-41.3: owner/docs + skill routing.
4. make check; finalize subtasks then parent.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Menu heatmaps MCP Apps visualisation shipped: widget + preview (41.1), render_menu_heatmaps resource (41.2), owner/docs routing (41.3). Category filters with weak off by default. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
