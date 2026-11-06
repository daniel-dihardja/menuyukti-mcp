---
id: TASK-36.2
title: Expose render_pair_lift_matrix MCP tool and UI resource
status: Done
assignee: []
created_date: '2026-11-06T07:14:00Z'
updated_date: '2026-11-06T07:18:00Z'
labels:
  - mcp
  - ui
  - chatgpt
dependencies:
  - TASK-36.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
parent_task_id: TASK-36
priority: high
type: feature
ordinal: 79000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Heatmap HTML alone is not reachable from ChatGPT. Venue slot strength established resource + _meta.ui.resourceUri + skybridge MIME; pair lift needs the same surface over real path-loaded affinity data. Keep get_pair_lift_matrix data-only so Codex/Inspector and text clients stay useful.

Expose a render tool that returns structuredContent for the widget, declares the ui:// template, and serves the widget resource from resources/list / resources/read. Reuse optional star focus loading from get_pair_lift_matrix. Include automated HTTP coverage and update MCP instructions so show/chart asks prefer the widget.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tools/list exposes render_pair_lift_matrix with _meta.ui.resourceUri (and openai/outputTemplate) pointing at a versioned ui:// HTML resource
- [x] #2 Calling the render tool returns structuredContent including focus_menus and matrix_lift (from sales path) plus content text; result _meta includes the template URI
- [x] #3 resources/read serves the pair lift widget HTML with the ChatGPT-compatible widget MIME used by this server
- [x] #4 get_pair_lift_matrix does not declare a UI widget template
- [x] #5 HTTP integration test covers tool list meta, successful render call, and resource fetch
- [x] #6 make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Wire render_pair_lift_matrix + ui://pair-lift-matrix-v1.html resource mirroring venue slot strength.
2. Keep get_pair_lift_matrix UI-free; reuse optional_star_focus.
3. Extend mcp_http tools/list + render/resource tests; update INSTRUCTIONS.
4. Run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Exposed render_pair_lift_matrix with ui://widget/pair-lift-matrix-v1.html, skybridge MIME, prefersBorder meta. resources/list+read serve widget. get_pair_lift_matrix stays data-only. HTTP tests cover list meta, demo-cafe render, resource HTML.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed render_pair_lift_matrix MCP tool and UI resource for the pair lift heatmap. Path-gated sales load returns focus_menus + matrix_lift structuredContent with template _meta; get_pair_lift_matrix remains text-only. Verified with HTTP integration test and make check.
<!-- SECTION:FINAL_SUMMARY:END -->
