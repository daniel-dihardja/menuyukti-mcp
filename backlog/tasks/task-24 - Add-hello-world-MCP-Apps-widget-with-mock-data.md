---
id: TASK-24
title: Add hello-world MCP Apps widget with mock data
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T12:33:00Z'
updated_date: '2026-11-04T12:52:00Z'
labels:
  - mcp
  - ui
  - chatgpt
dependencies: []
references:
  - https://developers.openai.com/plugins/build/chatgpt-ui
  - https://developers.openai.com/plugins/build/app-quickstart
  - crates/menuyukti-mcp/src/main.rs
priority: high
type: feature
ordinal: 51000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Analytics tools already return visualisation-shaped JSON (venue slot strength, heatmaps, pair lift), but ChatGPT and Codex only see text. Owners cannot inspect the day×meal demand heatmap the way they can in v3 chat. Chart widgets were deferred in TASK-1; before porting real venue-slot UI we need a minimal MCP Apps path that proves a sandboxed iframe can render from tool structuredContent.

Scope this slice to a trivial Hello World widget fed by hard-coded mock data only—no sales Excel, path gating, or product chart. Real heatmap widgets come later once this host/resource/tool wiring works.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP tools/list (or equivalent) exposes a render tool that declares `_meta.ui.resourceUri` (or ChatGPT `openai/outputTemplate` alias) pointing at a `ui://` HTML resource
- [x] #2 Calling that render tool returns mock structuredContent plus content text; the linked resource is served as `text/html;profile=mcp-app` (or the MCP Apps MIME constant)
- [x] #3 The widget HTML/JS renders a visible Hello World (or equivalent) label from the mock structuredContent without reading local sales files
- [x] #4 Existing analytics tools stay useful without UI (no accidental widget template on every data tool)
- [x] #5 Automated test asserts the render tool is listed, returns mock payload, and the UI resource can be fetched with the expected MIME/URI
- [x] #6 `make check` passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add hello-world widget HTML (inline JS) that reads mock structuredContent via MCP Apps tool-result notifications and shows a Hello World label.
2. In menuyukti-mcp: constants for ui:// URI + MIME; serve resource via list_resources/read_resource; enable resources capability.
3. Add render_hello_world_widget tool with `_meta.ui.resourceUri`, returning mock structuredContent + text; leave analytics tools without UI meta.
4. Extend mcp_http tests for tool list meta, call payload, and resources/read MIME/URI.
5. Run make check and finalize TASK-24.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added render_hello_world_widget with `_meta.ui.resourceUri` + openai/outputTemplate, ui://widget/hello-world.html resource (text/html;profile=mcp-app), and assets/hello_world_widget.html that paints message from structuredContent. Analytics tools remain without UI meta. mcp_http covers list meta, call structuredContent, resources/list+read. make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Shipped a Hello World MCP Apps smoke path: render_hello_world_widget returns mock structuredContent {message, source}, advertises ui://widget/hello-world.html, and resources/read serves the HTML widget (MIME text/html;profile=mcp-app). Analytics tools stay UI-free. Verified with mcp_http_hello_world_widget_resource_and_structured_content plus make check. Follow-up: real venue-slot (and other) chart widgets on this wiring.
<!-- SECTION:FINAL_SUMMARY:END -->
