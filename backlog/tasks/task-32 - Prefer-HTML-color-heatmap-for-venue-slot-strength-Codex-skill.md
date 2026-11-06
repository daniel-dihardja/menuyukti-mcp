---
id: TASK-32
title: Prefer HTML color heatmap for venue slot strength Codex skill
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:34:00Z'
updated_date: '2026-11-06T04:35:00Z'
labels:
  - codex
  - ux
  - analytics
dependencies: []
priority: medium
type: feature
ordinal: 73000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Codex venue slot strength skill currently draws a glyph markdown table. Owners expect a color heatmap like Menuyukti v3 / the MCP Apps widget (blue sequential scale on demand_index).

Update menuyukti-venue-slot-strength to prefer a local HTML heatmap renderer from slot_demand_profile (same semantics as venue_slot_strength_widget.html), keep markdown glyph grid as chat fallback, sync into the plugin package, and bump plugin version for reinstall.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Skill instructs Codex to write HTML heatmap via a script from get_venue_slot_strength / offline JSON
- [x] #2 HTML uses weekday × meal-period grid with demand_index cell text and sequential blue intensity from min–max index
- [x] #3 Markdown glyph table remains as fallback / in-chat summary
- [x] #4 make sync-plugin-skills copies the new script; plugin version bumped
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add scripts/render_heatmap.py that turns get_venue_slot_strength JSON into a local HTML color heatmap (blue sequential scale, 7×5 grid, demand_index labels, relative_demand + order_count in title).
2. Update SKILL.md + references/heatmap-recipe.md to prefer HTML via the script, keep glyph markdown as chat fallback.
3. make sync-plugin-skills; bump plugins/menuyukti/plugin.json to 0.1.2; smoke-test script on preview fixture; reinstall Codex plugin.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added scripts/render_heatmap.py; smoke-tested on preview fixture (35 slots, blue rgb cells). Synced plugin; bumped to 0.1.2; reinstalled menuyukti@menuyukti-repo.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Venue slot strength Codex skill now prefers a local HTML color heatmap (blue sequential scale on demand_index, same semantics as the MCP Apps widget) via scripts/render_heatmap.py. Markdown glyph grid kept as chat fallback. Plugin synced and reinstalled at 0.1.2.
<!-- SECTION:FINAL_SUMMARY:END -->
