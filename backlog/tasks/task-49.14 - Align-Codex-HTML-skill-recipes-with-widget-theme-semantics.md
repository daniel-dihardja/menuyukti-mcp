---
id: TASK-49.14
title: Align Codex HTML skill recipes with widget theme semantics
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T04:00:00Z'
updated_date: '2026-11-07T04:22:00Z'
labels:
  - skills
  - widgets
  - docs
dependencies: []
references:
  - .agents/skills/menuyukti-venue-slot-strength
  - .agents/skills/menuyukti-visualize-menu-heatmaps
  - .agents/skills/menuyukti-visualize-weak-items
  - apps/menuyukti/src/views/widget.css
parent_task_id: TASK-49
priority: low
type: docs
ordinal: 136000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
React widgets single-source colors in widget.css. Codex HTML skill recipes hardcode blues/slates and layout semantics in markdown. That duplication already caused axis-orientation drift (TASK-46). Either share a small theme/layout contract the recipes must follow, or explicitly shrink HTML fallbacks once Desktop hosts reliably mount render_* views.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Skill recipes document the same axis/layout contracts as the React widgets for venue slot, menu heatmaps, and weak items
- [x] #2 Color/ramp guidance is either derived from widget.css tokens or explicitly marked as fallback-only with a sync note
- [x] #3 Synced plugin skill copies under plugins/menuyukti/skills match .agents/skills after the change
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read React widget axis/layout + widget.css tokens for venue, menu heatmaps, weak items.
2. Update .agents skill recipes so HTML fallbacks match those contracts; mark colors as fallback-only synced to widget.css.
3. Sync plugins/menuyukti/skills copies to match.
4. Mark Done when ACs pass.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Aligned venue/menu/weak recipes with React layout contracts (axes, active daily hours, sort/KPI chrome). Colors marked fallback-only synced to widget.css light tokens; light-text threshold t>=0.55. pnpm sync:plugin-skills; plugin copies match .agents recipes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Codex HTML skill recipes now document the same axis/layout contracts as the React widgets and treat color/chrome hexes as fallback-only copies of widget.css. Plugin skills synced via pnpm sync:plugin-skills.
<!-- SECTION:FINAL_SUMMARY:END -->
