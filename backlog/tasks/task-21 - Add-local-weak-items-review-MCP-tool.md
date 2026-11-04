---
id: TASK-21
title: Add local weak items review MCP tool
status: Done
assignee: []
created_date: '2026-11-04T08:47:00Z'
updated_date: '2026-11-04T08:54:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/analytics/menu_engineering.rs
  - crates/menuyukti-mcp/src/main.rs
  - ../v3/apps/web/lib/analytics/weak-page-adapter.ts
  - ../v3/apps/web/lib/analytics/load-weak-for-chat.ts
  - ../v3/apps/agents/agents/core/chat/chart_data.py
  - ../v3/apps/graphql/knowledge/kpi-weak-items.md
  - docs/howtos/workspaces.md
documentation:
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 39000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners ask which dishes to reprice, improve, or remove. v3 answers via the Weak items review chart (`weak_review`): menu-engineering weak quadrant plus period depth (avg qty/day, portfolio item/margin share). Local MCP already exposes the full matrix through `analyze_menu_engineering`, but ChatGPT still has to invent the weak-only review shape and KPIs. A dedicated tool should return the same visualisation-oriented payload (weak rows + stats + period) and hard-require workspace COGS like menu engineering.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT can call one MCP tool against a workspace folder under MENUYUKTI_DATA_DIR and receive weak-items review JSON with portfolio stats and per-item weak rows (including action and margin share)
- [x] #2 Payload includes period depth used by the v3 weak visualisation: period_start, period_end, period_days, and avg_qty_per_day on each weak row when the period is known
- [x] #3 Missing COGS role/file returns a clear JSON error naming the prerequisite (same hard gate as analyze_menu_engineering)
- [x] #4 Demo cafe (sales + COGS) returns a non-empty weak list when the matrix has weak items, with stats consistent with the weak distribution share
- [x] #5 Owner/MCP docs mention the weak-items review tool and that it needs sales + COGS
- [x] #6 Unit tests cover the weak adapter; MCP HTTP tests cover success and missing-COGS; make check passes
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All subtasks 21.1–21.3 done. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Weak items review is available via get_weak_items_review. Library adapts the matrix with period depth; MCP hard-requires COGS; docs updated. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
