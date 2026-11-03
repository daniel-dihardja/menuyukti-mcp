---
id: TASK-1
title: Set up the Menuyukti MCP plugin
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T10:03:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels:
  - plugin
  - analytics
  - mcp
dependencies: []
references:
  - ../v3/packages/menuyukti
  - ../v3/packages/menuyukti/fixtures/pos-sales-reports
  - https://developers.openai.com/plugins/build/app-quickstart
priority: high
type: feature
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Restaurant owners should be able to install a ChatGPT plugin and ask questions about their own sales without uploading the POS export to Menuyukti or attaching it in chat. The analytics already exist in the sibling v3 repo at packages/menuyukti (POS detection, ESB/Quino normalization, and calculate_sales_analytics). This repo is empty. ChatGPT plugins are MCP servers, and ChatGPT cannot call localhost, so the first slice stays on the owner's machine and is proven with MCP Inspector. Tool results must be aggregates. Connecting ChatGPT over public HTTPS or a Secure MCP Tunnel, authentication, chart widgets, and the v3 web, GraphQL, and advisor agent are later work.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 An MCP server in this repo answers at http://localhost:8787/mcp
- [x] #2 The server depends on the sibling v3 packages/menuyukti package and does not copy its analytics pipelines into this repo
- [x] #3 summarize_local_sales reads a local Excel file only when the path is inside the configured data directory
- [x] #4 A path outside that directory is rejected and the file is not read
- [x] #5 The tool result includes detected POS, period, revenue, quantity, and a capped top-item list, and omits raw ticket lines
- [x] #6 MCP Inspector can call the tool against a v3 POS demo fixture and receive that summary
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add pyproject.toml with Python 3.12, mcp, path dependency on ../v3/packages/menuyukti, pytest, and menuyukti-gpt console script.
2. Implement sales.py: MENUYUKTI_DATA_DIR path gate before any file open; detect_pos_from_excel_bytes → get_normalizer/get_config → calculate_sales_analytics; return POS, period, revenue, quantity, top 10 items only.
3. Implement server.py with FastMCP streamable HTTP on 127.0.0.1:8787/mcp and register summarize_local_sales.
4. Add npm run mcp; keep npm run dev as backlog browser.
5. Tests: path rejection with open patched; ESB demo fixture summary shape; HTTP tool call stand-in for MCP Inspector.
6. README for MENUYUKTI_DATA_DIR, npm run mcp, Inspector URL.
7. Finalize TASK-1 acceptance criteria with evidence.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented local MCP 2.x MCPServer (streamable HTTP) on 127.0.0.1:8787/mcp. Path gate uses MENUYUKTI_DATA_DIR before any file read. Analytics come from sibling menuyukti path dependency (detect → normalize → calculate_sales_analytics). npm run mcp starts the server; npm run dev remains the Backlog board. Verified with uv run pytest (5 passed) and a live Client call to summarize_local_sales against the ESB demo fixture.

Naming: package/console script is menuyukti-mcp (was menuyukti-gpt).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Shipped the first local Menuyukti GPT plugin slice: a Python MCP server that exposes summarize_local_sales over http://localhost:8787/mcp. It depends on ../v3/packages/menuyukti, refuses paths outside MENUYUKTI_DATA_DIR before reading, and returns only POS, period, revenue, quantity, and a capped top-item list. Verification: 5 pytest tests including streamable-HTTP tool call; live Client smoke against the ESB demo fixture returned pos=esb and aggregate totals. Follow-ups (out of scope): ChatGPT connection/tunnel, auth, chart widgets.
<!-- SECTION:FINAL_SUMMARY:END -->
