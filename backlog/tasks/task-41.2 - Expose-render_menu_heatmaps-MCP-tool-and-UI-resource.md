---
id: TASK-41.2
title: Expose render_menu_heatmaps MCP tool and UI resource
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T07:51:00Z'
updated_date: '2026-11-06T07:56:00Z'
labels:
  - mcp
  - ui
  - chatgpt
dependencies:
  - TASK-41.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
  - crates/menuyukti-mcp/src/widget_assets.rs
parent_task_id: TASK-41
priority: high
type: feature
ordinal: 89000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Heatmap HTML alone is not reachable from ChatGPT. Venue slot strength and pair lift established resource + _meta.ui.resourceUri + skybridge MIME; menu heatmaps need the same surface over path-loaded get_menu_heatmaps data (including optional matrix_category enrichment). Keep get_menu_heatmaps data-only so Codex/Inspector and text clients stay useful.

Expose a render tool that returns structuredContent for the widget, declares the ui:// template, and serves the widget resource from resources/list / resources/read. Prefer the widget for show/chart asks in MCP instructions, and avoid double visualisation with the Codex HTML skill in the same turn (same rule as venue slot / pair lift).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tools/list exposes render_menu_heatmaps with _meta.ui.resourceUri (and openai/outputTemplate) pointing at a versioned ui:// HTML resource
- [x] #2 Calling the render tool returns structuredContent including heatmaps (with matrix_category when COGS enrichment applies) plus content text; result _meta includes the template URI
- [x] #3 resources/read serves the menu heatmaps widget HTML with the ChatGPT-compatible widget MIME used by this server
- [x] #4 get_menu_heatmaps does not declare a UI widget template
- [x] #5 HTTP integration tests cover tool list meta, successful render call, and resource fetch
- [x] #6 MCP instructions prefer render_menu_heatmaps for show/chart dish-timing asks and avoid widget + skill redraw in the same turn
- [x] #7 make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Done in same pass as 41.1: render_menu_heatmaps + ui://menu-heatmaps-v1.html resource, HTTP tests, MCP instructions.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Exposed render_menu_heatmaps reusing enriched_menu_heatmaps; get_menu_heatmaps stays data-only. HTTP covers list meta, demo-cafe render with matrix_category, resource HTML.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
render_menu_heatmaps returns structuredContent + ui://widget/menu-heatmaps-v1.html meta; resources/list+read serve the widget. Instructions prefer the widget for show/chart dish timing. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
