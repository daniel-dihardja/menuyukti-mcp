---
id: TASK-44.6
title: Port analytics get_ and analyze_ MCP tools to Skybridge
status: To Do
assignee: []
created_date: '2026-11-06T17:30:00Z'
labels:
  - skybridge
  - migration
milestone: m-2
dependencies:
  - TASK-44.5
references:
  - crates/menuyukti/src/analytics
  - backlog/decisions/decision-2 - Freeze-Menuyukti-1.0-product-scope.md
documentation:
  - backlog/docs/product/doc-1 - Menuyukti-1.0-product-definition.md
  - docs/howtos/workspaces.md
parent_task_id: TASK-44
priority: high
type: feature
ordinal: 106000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Menuyukti 1.0 guarantees four capabilities via data tools (plus analyze_menu_engineering behind weak items / star focus). These must exist on the Skybridge server with stable names and payloads so plugins/skills and owners keep the same asks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Skybridge registers get_menu_heatmaps, get_venue_slot_strength, get_pair_lift_matrix, get_weak_items_review, and analyze_menu_engineering
- [ ] #2 Tool inputs/outputs are sufficient for decision-2 owner outcomes on demo-cafe-july-2026 (sales-only and sales+COGS paths where applicable)
- [ ] #3 Automated tests or fixture checks cover the four capability data tools against the demo workspace
<!-- AC:END -->
