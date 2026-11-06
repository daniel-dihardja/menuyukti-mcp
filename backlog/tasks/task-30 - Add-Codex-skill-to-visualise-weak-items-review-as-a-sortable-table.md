---
id: TASK-30
title: Add Codex skill to visualise weak items review as a sortable table
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:02:00Z'
updated_date: '2026-11-06T04:04:00Z'
labels:
  - codex
  - ux
  - analytics
dependencies: []
references:
  - ../v3/apps/web/components/analytics/weak/weak-table.tsx
  - ../v3/apps/web/lib/analytics/weak-page-adapter.ts
  - ../v3/apps/graphql/knowledge/kpi-weak-items.md
  - crates/menuyukti/src/analytics/weak_review.rs
  - .agents/skills/menuyukti-mcp/SKILL.md
  - .agents/skills/menuyukti-visualize-venue-slot-strength/SKILL.md
documentation:
  - README.md
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 67000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners asking Codex which dishes to reprice or cut get raw `get_weak_items_review` JSON instead of the v3 Weak items review table (`WeakTable` / `weak_review`). That UI is a sortable table of weak-quadrant rows (menu, avg qty/day, units, margin, margin share) plus portfolio KPIs — not a heatmap.

Codex needs a presentation skill that draws those rows as a table, default-sorted like v3 (margin share ascending), and supports re-sorting by column. Prefer an openable HTML table with clickable headers for true sortability, plus a markdown table in chat. Do not invent margins; do not include the removed `action` column.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 New skill under `.agents/skills/` with SKILL.md triggers and `agents/openai.yaml`
- [x] #2 Skill uses `get_weak_items_review` JSON only (sales+COGS) and never invents margins or quantities
- [x] #3 Recipe shows portfolio KPIs plus a table of menu, avg qty/day (when known), units sold, contribution margin, and margin share — no action column
- [x] #4 Default sort matches v3: contribution margin share ascending; columns are re-sortable (HTML click-to-sort and/or agent re-sort on request)
- [x] #5 `menuyukti-mcp` routes weak-items show/table asks to this skill after `get_weak_items_review`; README or workspaces docs point at the skill
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add menuyukti-visualize-weak-items skill (SKILL.md, openai.yaml, table recipe, sortable HTML render script).
2. Wire menuyukti-mcp goal table and README/workspaces pointers.
3. Verify ACs and finalize.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added menuyukti-visualize-weak-items with markdown recipe + render_weak_table.py (clickable HTML sort). Wired menuyukti-mcp, README, workspaces. Smoke-tested HTML default sort (margin share asc).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added Codex skill menuyukti-visualize-weak-items: KPIs + sortable weak-items table (menu, avg qty/day, units, margin, share; no action column). Default sort matches v3 (margin share ascending). Includes scripts/render_weak_table.py for click-to-sort HTML and a markdown chat fallback. Wired menuyukti-mcp routing plus README/workspaces pointers. Verified skill files and HTML smoke test.
<!-- SECTION:FINAL_SUMMARY:END -->
