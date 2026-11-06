---
id: TASK-40
title: Add menu-engineering categories to menu heatmaps and visualisation filters
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T07:43:00Z'
updated_date: '2026-11-06T07:47:00Z'
labels:
  - mcp
  - analytics
  - ux
  - codex
dependencies:
  - TASK-4
  - TASK-19
  - TASK-29.2
references:
  - crates/menuyukti/src/analytics/heatmaps.rs
  - crates/menuyukti/src/analytics/menu_engineering.rs
  - crates/menuyukti-mcp/src/main.rs
  - .agents/skills/menuyukti-visualize-menu-heatmaps/SKILL.md
  - >-
    .agents/skills/menuyukti-visualize-menu-heatmaps/references/menu-heatmap-recipe.md
  - plugins/menuyukti/skills/menuyukti-visualize-menu-heatmaps/SKILL.md
documentation:
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 84000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners reading dish timing heatmaps cannot tell which rows are stars vs plow horses vs puzzles vs weak without a second `analyze_menu_engineering` call, and the Codex HTML visualisation has no interactive way to hide weak items.

Heatmaps already ship per-menu quantity grids via `get_menu_heatmaps` (sales alone). Menu engineering already classifies `star` / `plow_horse` / `puzzle` / `weak` when workspace COGS is present. Join that classification onto each heatmap item when COGS is available (sales-only heatmaps must still work), then teach the menu-heatmap visualisation to filter by those categories with weak off by default.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 get_menu_heatmaps includes a menu-engineering matrix category on each heatmap item when the workspace can compute the matrix
- [x] #2 Heatmaps still succeed with sales alone when COGS/matrix is unavailable; category is null or omitted rather than failing the tool
- [x] #3 Menu heatmap HTML visualisation exposes enable/disable controls for star, plow_horse, puzzle, and weak
- [x] #4 Weak is disabled by default in the visualisation; other categories start enabled when category data is present
- [x] #5 Skill/recipe docs (and plugin copy if mirrored) describe the new field and filter defaults; make check passes for Rust changes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. TASK-40.1: enrich MenuHeatmapPayload + get_menu_heatmaps with optional matrix_category.
2. TASK-40.2: update menu heatmap skill/recipe (and plugin mirror) for interactive category filters; weak off by default.
3. Update owner docs for heatmaps COGS enrichment + filters.
4. Finalize subtasks then parent after make check.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Menu heatmaps now carry optional matrix_category from workspace COGS (TASK-40.1), and the Codex HTML visualisation skill teaches interactive category filters with weak disabled by default (TASK-40.2). Docs and MCP instructions updated. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
