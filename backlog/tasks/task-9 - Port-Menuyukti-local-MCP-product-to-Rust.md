---
id: TASK-9
title: Port Menuyukti local MCP product to Rust
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:00:00Z'
updated_date: '2026-11-03T12:02:00Z'
labels: []
milestone: m-0
dependencies: []
references:
  - README.md
  - src/menuyukti_gpt/server.py
  - sales-reports/
  - >-
    backlog/decisions/decision-1 -
    Adopt-Rust-for-the-local-Menuyukti-MCP-product.md
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
- [ ] #1 Users can run a local Rust MCP binary that serves the same sales analytics UX against POS Excel files under a configured data directory
- [ ] #2 Known POS formats (ESB, Moka, Quino) and optional owner column-mapping still work without a Python runtime on the user path
- [ ] #3 ChatGPT / MCP Inspector can connect over local streamable HTTP and call the sales tools
- [ ] #4 README and run scripts document the Rust workflow as the primary path
<!-- AC:END -->
