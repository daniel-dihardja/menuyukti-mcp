---
id: TASK-21.3
title: Document weak items review tool for owners
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:49:00Z'
updated_date: '2026-11-04T08:54:00Z'
labels: []
dependencies:
  - TASK-21.2
references:
  - docs/howtos/workspaces.md
  - workspaces/README.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-21
priority: medium
type: docs
ordinal: 42000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Docs already cover the full menu-engineering matrix. Without an explicit weak-items review entry, ChatGPT may dump the entire matrix or skip the dedicated tool when owners ask about weak dishes. Docs should point to the weak review tool and restate the COGS requirement.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 docs/howtos/workspaces.md tool list mentions the weak-items review tool
- [x] #2 Docs state it needs sales + COGS (same as menu engineering) and is for reprice / improve / remove decisions
- [x] #3 workspaces/README.md or equivalent stays consistent with the new capability
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Document get_weak_items_review in workspaces howto and README.
2. Finalize after docs are in place.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated docs/howtos/workspaces.md (tool list + Weak items review section) and workspaces/README.md.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs list get_weak_items_review, state COGS is required, and describe reprice/improve/remove use.
<!-- SECTION:FINAL_SUMMARY:END -->
