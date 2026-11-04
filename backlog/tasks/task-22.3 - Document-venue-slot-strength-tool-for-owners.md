---
id: TASK-22.3
title: Document venue slot strength tool for owners
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T09:35:00Z'
updated_date: '2026-11-04T09:40:00Z'
labels: []
dependencies:
  - TASK-22.2
references:
  - docs/howtos/workspaces.md
  - workspaces/README.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-22
priority: medium
type: docs
ordinal: 46000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners and ChatGPT already learn heatmaps, pair lift, and menu engineering from workspaces docs. Without an explicit venue slot strength entry, timing advice may invent demand indexes or misuse per-menu heatmaps. Docs should list the tool and state sales alone is enough.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 docs/howtos/workspaces.md tool list mentions the venue slot strength tool
- [x] #2 Docs state sales is sufficient (no COGS) and describe day × meal-period demand use for timing
- [x] #3 workspaces/README.md or equivalent stays consistent with the new capability
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Document get_venue_slot_strength in workspaces howto and README.
2. Finalize with 22.2.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Documented get_venue_slot_strength in docs/howtos/workspaces.md (tool list + venue slot section) and workspaces/README.md sales-alone list.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs list get_venue_slot_strength and state sales alone is enough for day × meal-period timing.
<!-- SECTION:FINAL_SUMMARY:END -->
