---
id: TASK-42
title: Add weak items review MCP Apps visualisation
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T10:49:00Z'
updated_date: '2026-11-06T11:04:00Z'
labels:
  - mcp
  - ui
  - chatgpt
  - analytics
dependencies:
  - TASK-21
  - TASK-41
references:
  - crates/menuyukti/src/analytics/weak_review.rs
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/assets/pair_lift_matrix_widget.html
  - crates/menuyukti-mcp/assets/shared/widget-theme.css
  - ../v3/apps/web/components/analytics/weak/weak-table.tsx
  - >-
    .agents/skills/menuyukti-visualize-weak-items/references/weak-table-recipe.md
  - >-
    backlog/tasks/task-30 -
    Add-Codex-skill-to-visualise-weak-items-review-as-a-sortable-table.md
documentation:
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 91000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners asking which dishes to reprice, improve, or cut still cannot see a ChatGPT-mounted weak-items table. Venue slot strength, pair lift, and menu heatmaps already ship MCP Apps widgets with local preview fixtures, but weak items only have `get_weak_items_review` JSON plus a Codex skill that writes a separate HTML file. `make widgets-preview` therefore has no weak-items entry, and ChatGPT Desktop has nothing to mount.

Ship an MCP Apps widget that renders the weak-quadrant review as a sortable table with KPI strip from `get_weak_items_review`-shaped structuredContent (menu, avg qty/day, units, contribution margin, margin share — no Action column; default sort = margin share ascending, matching v3 WeakTable / the Codex recipe). Keep `get_weak_items_review` data-only; add a dedicated render tool, ui:// resource, and preview fixtures so developers can style with `make widgets-preview`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT (or MCP Inspector) can obtain a weak items review UI resource linked from an MCP tool via `_meta.ui.resourceUri` / `openai/outputTemplate`
- [x] #2 The widget renders portfolio KPIs plus a sortable weak-items table from structuredContent (items/stats/period), default-sorted by contribution margin share ascending
- [x] #3 No Action / keep / remove column is shown
- [x] #4 `get_weak_items_review` remains usable without a widget template on every call
- [x] #5 Local widget preview lists weak-items fixtures (populated + empty/edge) on the preview index
- [x] #6 Owner or MCP docs mention how to show the weak items visualisation
- [x] #7 `make check` passes for the completed feature
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Study pair-lift/menu-heatmaps widget host bridge + theme + get_weak_items_review payload.
2. Build weak_items_review_widget.html (KPI strip, sortable table, no action column).
3. TASK-42.4: preview fixtures/pages + index + fixture tests.
4. TASK-42.2: render_weak_items_review + ui:// resource + HTTP tests + instructions.
5. TASK-42.3: owner docs + skill routing.
6. make check; finalize subtasks then parent.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Shipped widget (42.1), preview (42.4), render tool (42.2), docs (42.3). make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Weak items review MCP Apps visualisation shipped: widget + preview (42.1/42.4), render_weak_items_review resource (42.2), owner/docs routing (42.3). Sortable table with KPIs, no Action column, default margin-share asc. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
