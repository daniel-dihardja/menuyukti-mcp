---
id: TASK-9.2
title: Add path-gated sales report listing in Rust
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.1
references:
  - src/menuyukti_gpt/sales.py
  - src/menuyukti_gpt/server.py
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Users need to discover which Excel files they can analyze without guessing paths. The Python server already sandboxes all reads under MENUYUKTI_DATA_DIR; that privacy boundary must exist before any Excel parse runs in Rust.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 list_sales_reports MCP tool lists .xlsx/.xls under MENUYUKTI_DATA_DIR as relative paths
- [ ] #2 Paths outside the data directory are rejected before any file content is read
- [ ] #3 Missing or invalid MENUYUKTI_DATA_DIR returns a clear tool error
- [ ] #4 Tests cover listing and path rejection
<!-- AC:END -->
