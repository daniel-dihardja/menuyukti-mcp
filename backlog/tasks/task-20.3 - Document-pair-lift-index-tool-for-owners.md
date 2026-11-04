---
id: TASK-20.3
title: Document pair lift index tool for owners
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:32:00Z'
updated_date: '2026-11-04T08:43:00Z'
labels: []
dependencies:
  - TASK-20.2
references:
  - docs/howtos/workspaces.md
  - workspaces/README.md
  - crates/menuyukti-mcp/src/main.rs
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-20
priority: medium
type: docs
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners and ChatGPT already learn heatmaps and menu engineering from workspaces docs and MCP instructions. Without an explicit pair-lift section, the new tool is easy to miss and COGS may be wrongly assumed required (unlike menu engineering). Docs should state that sales alone powers the lift matrix and COGS only upgrades focus to stars when available.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 docs/howtos/workspaces.md (and related owner-facing docs if they list analytics tools) mention the pair-lift / basket-affinity tool
- [x] #2 Docs state sales is sufficient; COGS is optional and improves star-based focus when present
- [x] #3 workspaces/README.md or equivalent tool list stays consistent with the new capability
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Document get_pair_lift_matrix in workspaces howto and workspaces README.
2. Clarify sales-alone vs optional COGS star focus.
3. Finalize after make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated docs/howtos/workspaces.md (tool list + Pair lift section) and workspaces/README.md. make check still green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs list get_pair_lift_matrix and state sales alone is enough; COGS only upgrades focus to stars. workspaces README aligned.
<!-- SECTION:FINAL_SUMMARY:END -->
