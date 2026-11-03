---
id: TASK-9.5
title: Expose summarize and list-normalized sales MCP tools in Rust
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.3
  - TASK-9.4
references:
  - packages/menuyukti/src/menuyukti/core/analytics/calculate_sales_analytics.py
  - src/menuyukti_mcp/
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ChatGPT users ask for period revenue, top sellers, and raw cleaned rows. Those answers come from aggregates and plain line-item lists over the normalized Excel load. Without these tools the Rust server cannot replace the current product conversation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 summarize_local_sales returns POS, period, total revenue, total items sold, and a capped top-items list for a path under the data dir
- [x] #2 list_normalized_sales returns POS plus plain JSON line-item rows and row_count for a path under the data dir
- [x] #3 Path, missing-file, and unsupported-POS errors return clear JSON error payloads
- [x] #4 Tests cover ESB (or demo cafe) fixture summary and normalized list shapes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement sales summary analytics over Vec<LineItem> (revenue, qty, period, top items).
2. Serialize normalized rows to JSON-friendly maps.
3. Wire summarize_local_sales and list_normalized_sales MCP tools with path/error JSON.
4. Unit tests on ESB/demo fixtures for summary + list shapes.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented summarize_sales + list_normalized_rows; wired both MCP tools. Analytics tests + HTTP smoke on demo-cafe.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
summarize_local_sales and list_normalized_sales are live on the Rust MCP server with path/error JSON payloads. Covered by unit tests and HTTP smoke against demo-cafe-sales-july-2026.xlsx.
<!-- SECTION:FINAL_SUMMARY:END -->
