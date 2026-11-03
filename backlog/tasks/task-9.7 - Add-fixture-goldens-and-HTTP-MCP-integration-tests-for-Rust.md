---
id: TASK-9.7
title: Add fixture goldens and HTTP MCP integration tests for Rust
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.5
  - TASK-9.6
references:
  - tests/test_mcp_server.py
  - sales-reports/demo-cafe-sales-july-2026.xlsx
  - packages/menuyukti/fixtures/pos-sales-reports/
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Excel coerce/detect differences are the main silent failure mode of the port. Without golden fixture checks and a streamable-HTTP smoke test, the Rust binary can look healthy while ChatGPT gets different numbers or a broken transport.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Automated tests assert stable summary/heatmap/list outcomes on committed demo Excel fixtures
- [x] #2 An integration test starts the Rust MCP HTTP server and successfully lists tools and calls list_sales_reports plus at least one analytics tool
- [x] #3 Failures print enough context to see which fixture/tool drifted
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add HTTP integration test that boots menuyukti-gpt, lists tools, calls list_sales_reports + summarize.
2. Add golden assertions for demo-cafe summary/heatmap counts.
3. Ensure failures name fixture/tool context.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added demo-cafe golden asserts (1748 items, revenue, top item, 98 menus, 1441 rows) and crates/menuyukti-gpt/tests/mcp_http.rs HTTP smoke. cargo test -p menuyukti-gpt --test mcp_http passed.

Naming: HTTP tests live under crates/menuyukti-mcp (was menuyukti-gpt).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixture goldens lock demo-cafe summary/heatmap/list outcomes; HTTP integration test boots the Rust MCP binary, lists tools, and summarizes the demo file with clear fixture/tool failure context.
<!-- SECTION:FINAL_SUMMARY:END -->
