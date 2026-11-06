---
id: TASK-44.3
title: Scaffold Skybridge app with local MCP run story
status: To Do
assignee: []
created_date: '2026-11-06T17:30:00Z'
labels:
  - skybridge
  - migration
milestone: m-2
dependencies:
  - TASK-44.2
references:
  - https://docs.skybridge.tech/get-started/quickstart.md
  - plugins/menuyukti/mcp.json
documentation:
  - docs/howtos/install-and-run.md
  - backlog/docs/product/doc-2 - Skybridge-migration-plan.md
parent_task_id: TASK-44
priority: high
type: feature
ordinal: 103000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners and agents need a TypeScript Skybridge project they can run locally the way they run menuyukti-mcp today (data dir env, localhost MCP URL). Without a clear scaffold and run docs, ports of tools/views have nowhere durable to land.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A Skybridge/TS app exists in-repo and starts a local MCP endpoint suitable for ChatGPT Desktop / Codex plugin config
- [ ] #2 Run instructions document MENUYUKTI_DATA_DIR (or equivalent) and the MCP URL used by the plugin
- [ ] #3 Hello-world or smoke tool+view proves the scaffold mounts an MCP Apps view locally
<!-- AC:END -->
