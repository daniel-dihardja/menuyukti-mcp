---
id: TASK-9.7
title: Add fixture goldens and HTTP MCP integration tests for Rust
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
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
- [ ] #1 Automated tests assert stable summary/heatmap/list outcomes on committed demo Excel fixtures
- [ ] #2 An integration test starts the Rust MCP HTTP server and successfully lists tools and calls list_sales_reports plus at least one analytics tool
- [ ] #3 Failures print enough context to see which fixture/tool drifted
<!-- AC:END -->
