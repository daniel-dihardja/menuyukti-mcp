---
id: TASK-42.4
title: Add weak items fixtures to local widget preview
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T10:51:00Z'
updated_date: '2026-11-06T11:04:00Z'
labels:
  - ui
  - widgets
  - devtools
dependencies:
  - TASK-42.1
references:
  - crates/menuyukti-mcp/preview/index.html
  - crates/menuyukti-mcp/preview/menu-heatmaps.html
  - crates/menuyukti-mcp/tests/widget_preview_fixtures.rs
  - crates/menuyukti/src/analytics/weak_review.rs
  - >-
    .agents/skills/menuyukti-visualize-weak-items/references/weak-table-recipe.md
parent_task_id: TASK-42
priority: high
type: feature
ordinal: 92500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Developers cannot style or verify the weak-items MCP Apps widget without ChatGPT unless `make widgets-preview` has harness pages and mock structuredContent. Other widgets (venue slot, pair lift, menu heatmaps) already ship populated + empty fixtures under `crates/menuyukti-mcp/preview/` with index links and `widget_preview_fixtures` shape tests; weak items still has none.

Add preview harness pages that iframe the weak-items widget asset, inject `get_weak_items_review`-shaped fixtures (populated + empty), list them on the preview index, and cover fixture shape in tests so local iteration matches the existing preview pattern from TASK-26.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Preview fixtures exist for populated and empty/edge weak-items payloads matching get_weak_items_review structuredContent (items/stats/period)
- [x] #2 Harness pages under crates/menuyukti-mcp/preview/ iframe the weak-items widget and inject fixtures via the existing preview.js host bridge
- [x] #3 preview/index.html lists the weak-items populated and empty preview entries
- [x] #4 widget_preview_fixtures (or equivalent) asserts fixture shape for populated and empty cases
- [x] #5 `make widgets-preview` can open the weak-items previews; `make check` passes for touched tests
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add populated + empty fixtures.
2. Add preview harness pages + index links.
3. Extend widget_preview_fixtures tests.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added weak_items_review.json + empty fixture, preview harness pages, index links, fixture shape tests.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Local widget preview lists Weak items review (populated + empty). Fixtures match get_weak_items_review shape; widget_preview_fixtures covers them. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
