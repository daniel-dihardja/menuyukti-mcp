---
id: TASK-43.4
title: Align owner docs to 1.0 promotion and menu-trim outcomes
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T11:25:00Z'
updated_date: '2026-11-06T11:29:00Z'
labels: []
milestone: m-1
dependencies:
  - TASK-43.1
  - TASK-43.3
references:
  - README.md
  - docs/README.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-43
priority: medium
type: docs
ordinal: 99000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner docs today lead with tool/chart inventory. For 1.0, restaurant owners need the product framed as jobs: plan promotions, and trim/compact the menu — with the four analytics capabilities as the means.

Update README and docs index (and workspaces howto if needed) so What to ask / start-here copy maps those two outcomes to the right prompts and tools/visualisations, without rewriting every existing per-tool howto.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 README (or docs/README) states Menuyukti 1.0 owner outcomes: plan promotions; trim/compact menu
- [x] #2 Each outcome maps to the relevant capabilities/prompts (slot strength, heatmaps, pair lift, weak items)
- [x] #3 Existing install/plugin/workspace howtos remain linked; no broken start-here ritual
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Reframe README Why/What-to-ask around plan promotions and trim menu, mapping to the four capabilities.
2. Update docs/README start-here to mention 1.0 outcomes.
3. Small workspaces howto fix: add missing render_venue_slot_strength bullet; light outcome framing if needed.
4. Link product definition for contributors if appropriate without overwhelming owners.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated README (1.0 blurb + outcome-grouped What to ask), docs/README (1.0 outcomes + link to doc-1), workspaces howto (outcome note + render_venue_slot_strength bullet), workspaces/README ritual line.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs now frame Menuyukti 1.0 around plan promotions and trim/compact menu, with each outcome mapped to capabilities/prompts. Install/plugin/workspace ritual links preserved. Minor tool-list fix for render_venue_slot_strength.
<!-- SECTION:FINAL_SUMMARY:END -->
