---
id: TASK-37
title: Share MCP Apps widget theme CSS across heatmaps
status: Done
assignee: []
created_date: '2026-11-06T07:20:00Z'
updated_date: '2026-11-06T07:24:00Z'
labels:
  - ui
  - mcp
  - widgets
dependencies:
  - TASK-36
references:
  - crates/menuyukti-mcp/assets/venue_slot_strength_widget.html
  - crates/menuyukti-mcp/assets/pair_lift_matrix_widget.html
  - crates/menuyukti-mcp/src/main.rs
priority: medium
type: enhancement
ordinal: 81000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Venue slot strength and pair lift widgets duplicate the same host tokens, panel chrome, legend/table styles, and light-dark contract. That drift already produced parallel --venue-* vs --pair-* token names. ChatGPT serves each widget as one self-contained HTML resource, so runtime <link> to a sibling CSS file is unreliable.

Extract one shared theme CSS source of truth and inline it into each heatmap widget (compile/serve time) so production MCP resources stay self-contained while authors edit theme once. Keep chart-specific scale overrides and layout CSS in the widget files.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Shared theme CSS lives under crates/menuyukti-mcp/assets/shared/ and owns host tokens, panel chrome, legend/table chrome, and heatmap scale token defaults
- [x] #2 Venue slot strength and pair lift widgets no longer duplicate the shared chrome CSS; they only keep chart-specific overrides/layout
- [x] #3 MCP resources/read and widget preview still serve self-contained HTML with the shared theme inlined (no runtime CSS fetch required in ChatGPT)
- [x] #4 Existing widget HTTP/fixture tests still pass; make check passes
- [x] #5 Developer docs briefly note how shared widget theme is authored and inlined
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add assets/shared/widget-theme.css with host tokens, chrome, heatmap defaults (--heatmap-*).
2. Add menuyukti-mcp widget_assets helper that inlines theme at a marker into venue + pair HTML (OnceLock cache for resources/read).
3. Strip duplicated chrome from both widgets; keep chart-specific CSS/JS; unify scale vars to --heatmap-*.
4. Preview server injects the same theme when serving /widgets/*.html.
5. Note in develop.md; run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added assets/shared/widget-theme.css (--color-*, panel/legend/table chrome, --heatmap-* defaults). Venue + pair shells use SHARED_THEME_MARKER; chart-specific CSS only. menuyukti_mcp::widget_assets inlines for resources/read (OnceLock) and preview /widgets/*.html. Bumped URIs to venue-slot-strength-v6 and pair-lift-matrix-v2. Documented in develop.md + install-and-run.md.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Shared MCP Apps heatmap theme CSS under assets/shared/widget-theme.css, inlined at serve time into venue and pair widgets so ChatGPT still gets one self-contained HTML resource. Preview uses the same inliner. Widget URIs bumped for cache bust. make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
