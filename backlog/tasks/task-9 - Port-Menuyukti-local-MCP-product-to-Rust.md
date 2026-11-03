---
id: TASK-9
title: Port Menuyukti local MCP product to Rust
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:00:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
milestone: m-0
dependencies: []
references:
  - README.md
  - sales-reports/
  - >-
    backlog/decisions/decision-1 -
    Adopt-Rust-for-the-local-Menuyukti-MCP-product.md
  - src/menuyukti_mcp/
  - crates/menuyukti-mcp/
priority: high
type: feature
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The product must run on restaurant owners' machines with minimal install friction. Python + uv + pandas is fine for analytics iteration but awkward as an end-user local service. This is greenfield: no need to preserve Python imports or pandas DataFrame APIs. Keep the user experience: drop POS Excel files in a local folder, run a local MCP server, connect ChatGPT, get summaries/heatmaps/normalized rows without sales data leaving the machine except via the user's own GPT session.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Users can run a local Rust MCP binary that serves the same sales analytics UX against POS Excel files under a configured data directory
- [x] #2 Known POS formats (ESB, Moka, Quino) and optional owner column-mapping still work without a Python runtime on the user path
- [x] #3 ChatGPT / MCP Inspector can connect over local streamable HTTP and call the sales tools
- [x] #4 README and run scripts document the Rust workflow as the primary path
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All subtasks 9.1–9.8 completed. Rust MCP serves list/summarize/list-normalized/heatmaps with ESB/Moka/Quino + column mapping; goldens + HTTP tests green; README cut over.

Naming: product package/binary is menuyukti-mcp (was menuyukti-gpt). Final implementation summary paths above refer to the old crate name.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Menuyukti local MCP product now runs as a Rust binary (crates/menuyukti-gpt) with Excel normalize/analytics in crates/menuyukti. UX preserved: sales-reports Excel in, streamable HTTP MCP out, optional column mapping. Python runtime is no longer required for the supported path.
<!-- SECTION:FINAL_SUMMARY:END -->
