---
id: TASK-9.1
title: Scaffold Rust workspace and streamable HTTP MCP binary
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
milestone: m-0
dependencies: []
references:
  - scripts/mcp-inspect.sh
  - src/menuyukti_mcp/
  - src/menuyukti_mcp/server.py
  - crates/menuyukti-mcp/
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
There is no Cargo project yet. A runnable Rust MCP process is the foundation for every later tool and for the single-binary local install story. Python MCP transport details (host, path, JSON/stateless responses) are the UX baseline ChatGPT already expects.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Cargo workspace builds a menuyukti-gpt (or equivalent) binary
- [x] #2 Binary serves MCP over streamable HTTP on 127.0.0.1:8787 at /mcp (or documented equivalent)
- [x] #3 Server advertises the Menuyukti sales tool surface (tools may be stubs until later tasks)
- [x] #4 Dev docs or scripts can start the Rust binary the same way users will later
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create Cargo workspace with crates/menuyukti (lib) and crates/menuyukti-gpt (bin).
2. Depend on rmcp + axum + tokio for streamable HTTP MCP on 127.0.0.1:8787/mcp with JSON/stateless-friendly config.
3. Register stub tools matching the Python names: list_sales_reports, summarize_local_sales, list_normalized_sales, get_menu_heatmaps.
4. Add a cargo run–based start path (script and/or package.json) and verify cargo build + server listen.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Scaffolded Cargo workspace (crates/menuyukti lib + crates/menuyukti-gpt bin) with rmcp StreamableHttpService on 127.0.0.1:8787/mcp (json_response + legacy_session_mode false). Stub tools: list_sales_reports, summarize_local_sales, list_normalized_sales, get_menu_heatmaps. npm run mcp / scripts/run-mcp.sh and mcp-inspect.sh start the Rust binary. Verified: cargo build; curl initialize returns menuyukti-gpt; tools/list returns all four tools; tools/call list_sales_reports returns stub JSON.

Naming: binary/crate is menuyukti-mcp (was menuyukti-gpt).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added a Rust Cargo workspace and menuyukti-gpt MCP binary serving streamable HTTP at http://127.0.0.1:8787/mcp with the four sales tool stubs. Dev start path is npm run mcp / scripts/run-mcp.sh (Python kept as mcp:python). Smoke-tested initialize, tools/list, and a stub tools/call. Next: TASK-9.2 path-gated listing.
<!-- SECTION:FINAL_SUMMARY:END -->
