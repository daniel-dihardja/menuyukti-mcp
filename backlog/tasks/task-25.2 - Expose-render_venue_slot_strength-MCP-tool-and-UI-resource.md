---
id: TASK-25.2
title: Expose render_venue_slot_strength MCP tool and UI resource
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T13:33:00Z'
updated_date: '2026-11-04T13:37:00Z'
labels:
  - mcp
  - ui
  - chatgpt
dependencies:
  - TASK-25.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
  - https://developers.openai.com/plugins/build/chatgpt-ui
parent_task_id: TASK-25
priority: high
type: feature
ordinal: 54000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Heatmap HTML alone is not reachable from ChatGPT. TASK-24 established resource + `_meta.ui.resourceUri` + skybridge MIME for Hello World; venue slot strength needs the same surface over real (or path-loaded) `slot_demand_profile` data. Keep `get_venue_slot_strength` data-only so Codex/Inspector and text clients stay useful.

Expose a render tool that returns structuredContent for the widget, declares the `ui://` template, and serves the widget resource from `resources/list` / `resources/read`. Prefer loading via workspace/sales path under MENUYUKTI_DATA_DIR (reuse library slot-demand) or accepting a prepared profile—worker chooses after researching current MCP patterns. Include automated HTTP coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tools/list exposes a venue slot strength render tool with `_meta.ui.resourceUri` (and ChatGPT `openai/outputTemplate` alias) pointing at a versioned `ui://` HTML resource
- [x] #2 Calling the render tool returns structuredContent including a 35-cell `slot_demand_profile` (from sales path and/or prepared data) plus content text; result `_meta` includes the template URI
- [x] #3 resources/read serves the heatmap widget HTML with the ChatGPT-compatible widget MIME used by this server
- [x] #4 `get_venue_slot_strength` does not declare a UI widget template
- [x] #5 HTTP integration test covers tool list meta, successful render call, and resource fetch
- [x] #6 `make check` passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add venue slot strength ui:// resource constants and meta helpers mirroring Hello World. 2. Add render_venue_slot_strength(path) that loads sales, computes slot_demand_profile, returns structuredContent + result _meta. 3. Extend list_resources/read_resource for both widgets. 4. Keep get_venue_slot_strength UI-free. 5. Extend mcp_http tests; run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Wired render_venue_slot_strength(path) with ui://widget/venue-slot-strength-v1.html, skybridge MIME, result _meta; get_venue_slot_strength stays UI-free. mcp_http_venue_slot_strength_widget_render_and_resource + make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed render_venue_slot_strength MCP tool and UI resource for the venue slot strength heatmap. Path-gated sales load returns 35-cell structuredContent with template _meta; resources/list+read serve the widget. Data tool remains text-only. Verified with HTTP integration test and make check.
<!-- SECTION:FINAL_SUMMARY:END -->
