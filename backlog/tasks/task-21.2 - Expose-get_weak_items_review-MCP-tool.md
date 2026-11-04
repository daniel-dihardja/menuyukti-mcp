---
id: TASK-21.2
title: Expose get_weak_items_review MCP tool
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:49:00Z'
updated_date: '2026-11-04T08:54:00Z'
labels: []
dependencies:
  - TASK-21.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
  - ../v3/apps/web/lib/analytics/load-weak-for-chat.ts
  - ../v3/apps/agents/agents/core/chat/chart_data.py
parent_task_id: TASK-21
priority: high
type: feature
ordinal: 41000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Weak adapter math is unused until ChatGPT can call it. Owners asking about cuts/repricing should get one tool that loads workspace sales+COGS, builds the matrix, adapts the weak review (with period depth), and fails closed without COGS—same recovery path as `analyze_menu_engineering`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `get_weak_items_review` (or equivalent) is listed by tools/list and accepts a workspace folder path under MENUYUKTI_DATA_DIR
- [x] #2 Successful call returns JSON with weak items/rows, portfolio stats, and period fields from local sales + COGS
- [x] #3 Missing COGS role/file returns HTTP-success tool JSON with an error field naming the missing prerequisite and next step
- [x] #4 INSTRUCTIONS and the tool description state weak-items review needs files.sales and files.cogs and is for reprice/improve/remove decisions
- [x] #5 MCP HTTP integration test covers success on demo-cafe-july-2026 and at least one missing-COGS failure path; make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Wire get_weak_items_review MCP tool: hard COGS gate, load sales, compute matrix, adapt_weak_review_from_orders.
2. Update INSTRUCTIONS and tool description.
3. MCP HTTP tests: tools/list, demo success, missing-COGS failure.
4. make check; finalize TASK-21.2.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Wired get_weak_items_review with hard COGS gate, matrix + adapt_weak_review_from_orders. Updated INSTRUCTIONS. MCP HTTP covers tools/list, demo success (items/stats/period), and sales-only missing-COGS. make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed get_weak_items_review over MCP. Returns weak rows, portfolio stats, and period depth from workspace sales+COGS; missing COGS fails closed with recovery guidance. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
