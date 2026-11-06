---
id: TASK-44.1
title: 'Spike: validate Skybridge for local-only Menuyukti MCP'
status: To Do
assignee: []
created_date: '2026-11-06T17:29:00Z'
labels:
  - skybridge
  - migration
milestone: m-2
dependencies: []
references:
  - https://docs.skybridge.tech/get-started/quickstart.md
  - https://docs.skybridge.tech/test/tunnel.md
  - docs/howtos/install-and-run.md
documentation:
  - backlog/docs/product/doc-2 - Skybridge-migration-plan.md
parent_task_id: TASK-44
priority: high
type: spike
ordinal: 101000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Before accepting the stack change, prove Skybridge can serve Menuyukti without forcing a hosted/tunnel product model.

Today the server binds 127.0.0.1:8787/mcp, gates paths under MENUYUKTI_DATA_DIR, and mounts MCP Apps widgets from local resources. If Skybridge cannot support that shape, do not accept decision-3.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A short written spike outcome lists what works and what blocks for localhost MCP, local file access under a data dir, and MCP Apps view mounting in ChatGPT Desktop
- [ ] #2 Spike states go / no-go for accepting decision-3
- [ ] #3 Any hard blockers (privacy, Desktop plugin, local-only run) are named with alternatives
<!-- AC:END -->
