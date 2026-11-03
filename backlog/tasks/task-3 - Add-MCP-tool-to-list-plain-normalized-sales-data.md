---
id: TASK-3
title: Add MCP tool to list plain normalized sales data
status: Done
assignee: []
created_date: '2026-11-03T11:08:00Z'
updated_date: '2026-11-03T11:09:00Z'
labels: []
dependencies: []
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Restaurant owners and GPT need raw normalized line items, not only aggregates. summarize_local_sales already caps to revenue and top items; a separate tool should return the plain normalized rows from the shared local POS loader so ChatGPT can inspect or reason over the full cleaned export without leaving the owner's machine.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MCP tool returns detected POS plus plain normalized line-item rows for a path under MENUYUKTI_DATA_DIR
- [x] #2 Tool rejects paths outside MENUYUKTI_DATA_DIR the same way summarize_local_sales does
- [x] #3 Tests cover successful listing and path-gate failures
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add list_normalized_sales + JSON helper in sales.py that reuses load_normalized_sales and serializes the frame to plain row dicts. 2. Register list_normalized_sales MCP tool in server.py with the same path-gate error handling as summarize. 3. Extend unit and MCP HTTP tests for success and path rejection. 4. Run pytest.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added list_normalized_sales + MCP tool; pytest tests/test_sales.py tests/test_mcp_server.py → 8 passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added list_normalized_sales MCP tool that returns detected POS plus plain normalized line-item rows from the shared local loader. Path gating matches summarize_local_sales. Verified with unit and Streamable HTTP MCP tests (8 passed).
<!-- SECTION:FINAL_SUMMARY:END -->
