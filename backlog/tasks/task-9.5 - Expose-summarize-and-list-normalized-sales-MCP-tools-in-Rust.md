---
id: TASK-9.5
title: Expose summarize and list-normalized sales MCP tools in Rust
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.3
  - TASK-9.4
references:
  - src/menuyukti_gpt/sales.py
  - src/menuyukti_gpt/server.py
  - packages/menuyukti/src/menuyukti/core/analytics/calculate_sales_analytics.py
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
- [ ] #1 summarize_local_sales returns POS, period, total revenue, total items sold, and a capped top-items list for a path under the data dir
- [ ] #2 list_normalized_sales returns POS plus plain JSON line-item rows and row_count for a path under the data dir
- [ ] #3 Path, missing-file, and unsupported-POS errors return clear JSON error payloads
- [ ] #4 Tests cover ESB (or demo cafe) fixture summary and normalized list shapes
<!-- AC:END -->
