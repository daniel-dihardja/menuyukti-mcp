---
id: TASK-43.1
title: Write Menuyukti 1.0 capability and owner-outcome matrix
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T11:25:00Z'
updated_date: '2026-11-06T11:27:00Z'
labels: []
milestone: m-1
dependencies: []
references:
  - README.md
  - workspaces/README.md
documentation:
  - docs/howtos/workspaces.md
  - backlog/docs/product/doc-1 - Menuyukti-1.0-product-definition.md
parent_task_id: TASK-43
priority: high
type: docs
ordinal: 96000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Agents and owners need one place that states what 1.0 is before we bump versions or cut a tag. README tool lists alone do not freeze a release contract.

Write a backlog specification document that maps: owner outcomes (plan promotions; trim/compact menu) → the four analytics capabilities (venue slot strength, menu heatmaps, pair lift index, weak items) → get_/render_ tools, widgets/skills, and sales vs sales+COGS prerequisites. Explicitly note supporting surfaces that enable those outcomes (workspaces, setup status, onboarding) without expanding 1.0 into every MCP tool.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 backlog doc of type specification exists for Menuyukti 1.0 with outcome → capability → tool/viz matrix
- [x] #2 Matrix covers venue slot strength, menu heatmaps, pair lift index, and weak items analysis
- [x] #3 Document states the two owner outcomes: plan promotions; trim/compact menu
- [x] #4 Sales-only vs sales+COGS requirements are called out per capability
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory shipped get_/render_ tools, widgets, skills, and sales vs COGS needs from README + workspaces howto + HTTP tests.
2. Create backlog specification doc for Menuyukti 1.0 with owner outcomes → capability → tool/viz matrix.
3. Note enabling surfaces (workspaces, setup, onboarding) without expanding 1.0 scope.
4. Link the doc from TASK-43 / parent refs as needed.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created doc-1 (specification) under backlog/docs/product/ with owner outcomes, capability matrix, enabling surfaces, and deferred out-of-scope list.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added backlog specification doc-1 (type: specification) at backlog/docs/product/ with owner outcomes (plan promotions; trim/compact menu), the four-capability tool/viz matrix including sales vs sales+COGS, enabling surfaces, and deferred out-of-scope. Linked from TASK-43.1 docs. Verified via backlog doc view.
<!-- SECTION:FINAL_SUMMARY:END -->
