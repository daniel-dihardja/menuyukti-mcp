---
id: TASK-34
title: Theme venue slot strength widget to match ChatGPT/Codex host
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T05:15:00Z'
updated_date: '2026-11-06T05:17:00Z'
labels:
  - ux
  - mcp-apps
  - widget
dependencies: []
references:
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - https://developers.openai.com/plugins/build/chatgpt-ui
  - https://developers.openai.com/plugins/reference
documentation:
  - docs/howtos/workspaces.md
priority: medium
type: enhancement
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In ChatGPT Desktop / Codex dark mode, the venue slot strength MCP Apps widget stays hard-coded light (white background, dark ink). That clashes with the host UI and makes the chart feel foreign next to the conversation.

Hosts already expose theme via MCP Apps host context (`theme` light|dark and optional `styles.variables` tokens such as `--color-background-primary` / `--color-text-primary`). The widget should follow the host so light and dark modes look native; local widget preview needs a readable fallback when no host is present.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Venue slot strength widget follows host light/dark theme from MCP Apps host context
- [x] #2 Widget uses host style tokens when provided, with sensible light/dark fallbacks for local preview
- [x] #3 Heatmap remains readable in both themes (cell text, legend, empty state, table chrome)
- [x] #4 Theme updates when the host notifies host-context-changed (e.g. user toggles dark mode)
- [x] #5 Widget URI is cache-busted so ChatGPT/Codex load the themed HTML
- [x] #6 make check passes; MCP HTTP/resource tests still cover the venue widget
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Restyle venue_slot_strength_widget.html with host CSS variable fallbacks (`light-dark`) for background, text, borders, table chrome, empty state, and heatmap blues; drive `color-scheme` from host `theme`.
2. Extend applyHostContext to apply theme, styles.variables, and optional fonts CSS; keep reacting to host-context-changed.
3. Cache-bust widget URI to v4; update mcp_http assertions for URI + theme markers.
4. Teach local preview to inject a mock host theme (light/dark toggle) so preview can verify both modes.
5. Run make check; finalize TASK-34.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Themed venue widget with host CSS variable fallbacks (light-dark), color-scheme from host theme / prefers-color-scheme, host-context-changed + styles.variables/fonts. Preview Dark mode toggle posts mock host context. URI bumped to venue-slot-strength-v4. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Venue slot strength MCP Apps widget now follows ChatGPT/Codex host light/dark theme (and optional host style tokens), with readable heatmap chrome in both modes and a local preview theme toggle. Cache-busted as ui://widget/venue-slot-strength-v4.html; make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
