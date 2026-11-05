---
id: TASK-26
title: Add local widget preview server with mock data and reload
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-05T02:07:00Z'
updated_date: '2026-11-05T02:15:00Z'
labels:
  - mcp
  - widgets
  - devtools
dependencies: []
references:
  - crates/menuyukti-mcp/assets/hello_world_widget.html
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - docs/howtos/install-and-run.md
documentation:
  - docs/howtos/install-and-run.md
priority: medium
type: enhancement
ordinal: 56000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Widget styling and interaction work today requires ChatGPT (or another MCP Apps host) to mount the `ui://` iframe after a tool call. Opening the HTML under `crates/menuyukti-mcp/assets/` alone stays on the empty/waiting state because nothing posts `ui/notifications/tool-result` with `structuredContent`. That makes CSS and UI iteration slow and host-dependent.

Add a developer-only local preview that serves the real widget HTML, injects fixture/mock tool results the same way the host does, and reloads the browser when those HTML files change. Prefer a Rust-only path built on the existing workspace stack (`axum` / `tokio` / `tower-http`, already used by `menuyukti-mcp`) plus file watching (for example `notify`) and a small SSE/WebSocket reload signal — no Python and no Node for this harness. Node remains acceptable only for unrelated tooling such as MCP Inspector. Keep production MCP tool behaviour and ChatGPT-served widget assets unchanged aside from documentation of how to run the preview (`make` / Cargo binary or example).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A documented make/script or Cargo target starts a local HTTP preview that opens (or can open) the widget HTML without ChatGPT or the MCP product server
- [x] #2 Preview pages for existing widgets (at least Hello World and venue slot strength) receive mock structuredContent via the same host bridge the widgets already listen for (`window.openai.toolOutput` and/or parent `ui/notifications/tool-result`)
- [x] #3 Mock fixtures cover a populated venue slot strength profile and an empty/edge state so both render paths are visible
- [x] #4 Saving a watched widget HTML asset triggers a browser reload (or equivalent hot refresh) without a manual refresh
- [x] #5 Preview tooling is Rust-based and optional for owners (does not become a runtime product dependency); docs note how developers start it; Node is not required for widget preview
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a second binary `menuyukti-widget-preview` in `menuyukti-mcp` (dev-only; production MCP main unchanged).
2. Serve real widget HTML from `assets/` plus thin preview harness pages that iframe the widget and post mock `structuredContent` via the existing host bridge; fixtures for Hello World, populated venue slot strength, and empty profile.
3. Watch `assets/` (and preview fixtures) with `notify`; broadcast reload over SSE so the browser refreshes on save.
4. Add `make widgets-preview` / script, document under install-and-run (Rust-only; Node not required).
5. Run `make check` and finalize acceptance criteria.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Shipped menuyukti-widget-preview binary: serves assets/ + preview harness, fixtures under preview/fixtures, SSE /__reload + /__watch polling. make widgets-preview / scripts/run-widget-preview.sh. Smoke: HTTP 200 for widgets/fixtures/preview; touch asset bumped version 0→1. make check green (incl. widget_preview_fixtures).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added a Rust-only local widget preview server (`menuyukti-widget-preview`) so developers can style MCP Apps widgets with mock structuredContent and live reload, without ChatGPT, the MCP product server, Node, or Python. Documented via `make widgets-preview`. Verified with endpoint smoke test (reload version bump) and `make check`.
<!-- SECTION:FINAL_SUMMARY:END -->
