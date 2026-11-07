---
id: TASK-48.2
title: Spike stdio-bundled Skybridge MCP for plugin ZIP
status: To Do
assignee: []
created_date: '2026-11-07T03:34:00Z'
updated_date: '2026-11-07T04:00:00Z'
labels:
  - openai
  - plugin
  - spike
dependencies: []
references:
  - backlog/tasks/task-49 - Harden-Skybridge-analytics-widget-serving-and-DX.md
  - https://developers.openai.com/plugins/deploy/desktop-only
  - apps/menuyukti/package.json
  - plugins/menuyukti/mcp.json
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
parent_task_id: TASK-48
priority: high
type: spike
ordinal: 116000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Directory local MCP expects a stdio server launched from the plugin package (command + PLUGIN_ROOT), not a separately started localhost HTTP process. Menuyukti today is Skybridge streamable-HTTP on port 8787 with production self-contained widgets.

Spike how to package or wrap the Skybridge Menuyukti server so a clean Desktop plugin install can start MCP over stdio (or an OpenAI-accepted local equivalent), including Node 24+ and MENUYUKTI_DATA_DIR assumptions. Record a recommended packaging layout and blockers before building the shippable ZIP.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Written spike notes describe a viable stdio (or accepted local) launch path from PLUGIN_ROOT
- [ ] #2 Notes list runtime deps (Node version, data dir, widget/assets) and size/constraints
- [ ] #3 GO/NO-GO recommendation recorded for building the directory package in the follow-up task
- [ ] #4 Spike notes explicitly cover shipping dist/self-contained HTML (or equivalent) and the Menuyukti self-contained production patch so Codex CSP widgets still work under stdio/PLUGIN_ROOT
<!-- AC:END -->
