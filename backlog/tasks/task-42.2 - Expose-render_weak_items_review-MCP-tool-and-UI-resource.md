---
id: TASK-42.2
title: Expose render_weak_items_review MCP tool and UI resource
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T10:49:00Z'
updated_date: '2026-11-06T11:04:00Z'
labels:
  - mcp
  - ui
  - chatgpt
dependencies:
  - TASK-42.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
  - crates/menuyukti-mcp/src/widget_assets.rs
parent_task_id: TASK-42
priority: high
type: feature
ordinal: 93000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Weak-items HTML alone is not reachable from ChatGPT. Venue slot strength, pair lift, and menu heatmaps established resource + `_meta.ui.resourceUri` + skybridge MIME; weak items need the same surface over path-loaded `get_weak_items_review` data (sales+COGS). Keep `get_weak_items_review` data-only so Codex/Inspector and text clients stay useful.

Expose a render tool that returns structuredContent for the widget, declares the ui:// template, and serves the widget resource from resources/list / resources/read. Prefer the widget for show/table/weak-items chart asks in MCP instructions, and avoid double visualisation with the Codex HTML skill in the same turn (same rule as other render tools).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tools/list exposes `render_weak_items_review` with `_meta.ui.resourceUri` (and `openai/outputTemplate`) pointing at a versioned ui:// HTML resource
- [x] #2 Calling the render tool returns structuredContent including items/stats/period plus content text; result `_meta` includes the template URI
- [x] #3 resources/read serves the weak items widget HTML with the ChatGPT-compatible widget MIME used by this server
- [x] #4 `get_weak_items_review` does not declare a UI widget template
- [x] #5 HTTP integration tests cover tool list meta, successful render call, and resource fetch
- [x] #6 MCP instructions prefer `render_weak_items_review` for show/table weak-items asks and avoid widget + skill redraw in the same turn
- [x] #7 `make check` passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add render_weak_items_review + ui:// resource.
2. HTTP tests for list meta, render, resource.
3. Update MCP instructions for one-path show/table.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Exposed render_weak_items_review reusing load_weak_items_review; get_weak_items_review stays data-only. HTTP covers list meta, demo render, resource HTML. Instructions prefer widget for show/table.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
render_weak_items_review returns structuredContent + ui://widget/weak-items-review-v1.html meta; resources/list+read serve the widget. Instructions prefer the widget for show/table weak items. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
