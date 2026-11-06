---
id: TASK-40.2
title: Add menu-engineering category filters to menu heatmap visualisation
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T07:44:00Z'
updated_date: '2026-11-06T07:47:00Z'
labels:
  - codex
  - ux
  - analytics
dependencies:
  - TASK-40.1
references:
  - .agents/skills/menuyukti-visualize-menu-heatmaps/SKILL.md
  - >-
    .agents/skills/menuyukti-visualize-menu-heatmaps/references/menu-heatmap-recipe.md
  - plugins/menuyukti/skills/menuyukti-visualize-menu-heatmaps/SKILL.md
  - >-
    plugins/menuyukti/skills/menuyukti-visualize-menu-heatmaps/references/menu-heatmap-recipe.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-40
priority: high
type: feature
ordinal: 86000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners opening the Codex HTML menu heatmap still see weak dishes mixed into timing charts, and the skill only mentions an optional offline filter via a separate matrix call. Once heatmap items carry matrix categories, the visualisation should let owners toggle each BCG role live in the HTML without regenerating the file.

Update the menuyukti-visualize-menu-heatmaps skill and recipe (keep the plugins/menuyukti mirror in sync). Default the weak toggle off; leave star, plow_horse, and puzzle on. When category data is missing, keep current sales-only behaviour and do not invent categories.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 HTML heatmap includes enable/disable controls for star, plow_horse, puzzle, and weak that show/hide matching menu rows
- [x] #2 On first open, weak is disabled and the other three categories are enabled when matrix categories are present on the payload
- [x] #3 When matrix categories are absent, the chart still renders from quantity data without inventing categories
- [x] #4 Skill + recipe (and mirrored plugin skill files) document the payload field and filter defaults; owner docs mention category filtering if heatmaps are documented there
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update menu-heatmap SKILL.md + recipe to require HTML category toggles (star/plow_horse/puzzle/weak) with weak off by default when matrix_category is present.
2. Mirror the same docs under plugins/menuyukti/skills/.
3. Update docs/howtos/workspaces.md menu heatmap section for optional COGS enrichment + filter defaults.
4. Light MCP instruction touch if routing text still implies separate analyze_menu_engineering for heatmap filters.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated .agents + plugins menu-heatmap skill/recipe for interactive matrix category toggles (weak unchecked by default). Synced menuyukti-mcp coaching skill, workspaces.md, README, and MCP instructions.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
menuyukti-visualize-menu-heatmaps now requires HTML enable/disable filters for star/plow_horse/puzzle/weak when matrix_category is present, with weak off by default. Recipe covers data attributes + JS toggle behaviour and no-invent rules when categories are absent. Plugin mirror and owner/coaching docs updated.
<!-- SECTION:FINAL_SUMMARY:END -->
