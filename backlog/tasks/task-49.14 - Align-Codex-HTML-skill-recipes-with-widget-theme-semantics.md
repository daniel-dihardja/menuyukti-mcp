---
id: TASK-49.14
title: Align Codex HTML skill recipes with widget theme semantics
status: To Do
assignee: []
created_date: '2026-11-07T04:00:00Z'
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
- [ ] #1 Skill recipes document the same axis/layout contracts as the React widgets for venue slot, menu heatmaps, and weak items
- [ ] #2 Color/ramp guidance is either derived from widget.css tokens or explicitly marked as fallback-only with a sync note
- [ ] #3 Synced plugin skill copies under plugins/menuyukti/skills match .agents/skills after the change
<!-- AC:END -->
