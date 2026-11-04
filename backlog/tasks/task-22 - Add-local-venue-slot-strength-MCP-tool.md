---
id: TASK-22
title: Add local venue slot strength MCP tool
status: Done
assignee: []
created_date: '2026-11-04T09:34:00Z'
updated_date: '2026-11-04T09:40:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/analytics/
  - crates/menuyukti-mcp/src/main.rs
  - >-
    ../v3/packages/menuyukti/src/menuyukti/core/analytics/calculate_slot_demand_profile.py
  - ../v3/packages/menuyukti/src/menuyukti/core/analytics/meal_periods.py
  - ../v3/packages/menuyukti/src/menuyukti/core/analytics/demand_labels.py
  - >-
    ../v3/packages/menuyukti/tests/unit/core/analytics/test_calculate_slot_demand_profile.py
  - ../v3/apps/agents/agents/core/chat/chart_data.py
  - ../v3/apps/graphql/knowledge/kpi-venue-slot-strength.md
  - docs/howtos/workspaces.md
documentation:
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 43000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners ask when the venue is busy so they can time staffing and guest-facing actions. v3 answers with venue slot strength (`venue_slot_strength_heatmap`): weekday × meal-period demand cells with order counts, traffic share, demand index, and relative demand. Local MCP has per-menu heatmaps but no venue-level day×meal demand profile, so ChatGPT invents timing advice. Port v3 `calculate_slot_demand_profile` over LineItem bill times and expose a sales-only MCP tool (no COGS).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT can call one MCP tool against a path under MENUYUKTI_DATA_DIR and receive a slot_demand_profile (or equivalent) of day × meal-period cells with order_count, traffic_share, demand_index, and relative_demand
- [x] #2 Profile covers all 7 weekdays × 5 meal periods (35 cells) when sales data exists, including zero-count slots
- [x] #3 Sales alone is enough; missing COGS does not block the tool
- [x] #4 Demo cafe or equivalent fixture produces a non-empty profile with demand indexes centered so relative_demand includes low/average/high where data supports it
- [x] #5 Owner/MCP docs mention the venue slot strength tool and that sales alone is enough
- [x] #6 Unit tests cover slot math edge cases; MCP HTTP tests cover a successful call; make check passes
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All subtasks Done: slot_demand.rs library, MCP tool + tests, owner docs. make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Local MCP now serves venue slot strength (get_venue_slot_strength): v3-compatible 35-cell weekday × meal-period demand from sales alone. Library, MCP, docs, and make check complete.
<!-- SECTION:FINAL_SUMMARY:END -->
