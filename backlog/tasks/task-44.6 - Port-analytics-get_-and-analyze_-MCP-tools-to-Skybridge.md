---
id: TASK-44.6
title: Port analytics get_ and analyze_ MCP tools to Skybridge
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T18:07:00Z'
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
- [x] #1 Skybridge registers get_menu_heatmaps, get_venue_slot_strength, get_pair_lift_matrix, get_weak_items_review, and analyze_menu_engineering
- [x] #2 Tool inputs/outputs are sufficient for decision-2 owner outcomes on demo-cafe-july-2026 (sales-only and sales+COGS paths where applicable)
- [x] #3 Automated tests or fixture checks cover the four capability data tools against the demo workspace
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Port heatmaps, slot_demand, basket_affinities, menu_engineering, and weak_review from crates/menuyukti/src/analytics into apps/menuyukti/src/lib/analytics.
2. Wire MCP tools get_menu_heatmaps, get_venue_slot_strength, get_pair_lift_matrix, get_weak_items_review, analyze_menu_engineering (same inputs/enrichment as Rust MCP).
3. Add vitest coverage against demo-cafe-july-2026 for the four capabilities + menu engineering.
4. MCP smoke on a free port; finalize TASK-44.6 ACs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ported heatmaps, slot_demand, basket_affinities, menu_engineering, weak_review + enrichment helpers; registered five analytics MCP tools; vitest demo-cafe coverage + MCP smoke on :8790 (98 menus, 35 slots, stars focus with COGS, weak period_days=31).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Skybridge now exposes get_menu_heatmaps, get_venue_slot_strength, get_pair_lift_matrix, get_weak_items_review, and analyze_menu_engineering with Rust-parity inputs/enrichment (sales-only vs sales+COGS). Vitest covers demo-cafe baselines; MCP smoke on a free port confirmed tool registration and payloads. Remaining gap: render_ MCP Apps widgets (TASK-44.7).
<!-- SECTION:FINAL_SUMMARY:END -->
