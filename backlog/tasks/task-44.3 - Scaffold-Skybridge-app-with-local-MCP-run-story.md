---
id: TASK-44.3
title: Scaffold Skybridge app with local MCP run story
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T17:41:00Z'
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
- [x] #1 A Skybridge/TS app exists in-repo and starts a local MCP endpoint suitable for ChatGPT Desktop / Codex plugin config
- [x] #2 Run instructions document MENUYUKTI_DATA_DIR (or equivalent) and the MCP URL used by the plugin
- [x] #3 Hello-world or smoke tool+view proves the scaffold mounts an MCP Apps view locally
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Choose in-repo path for the Skybridge app (e.g. apps/menuyukti) without disrupting Rust crates yet.
2. Scaffold blank Skybridge 2.1+ with Node 24+; wire package scripts for local MCP (prefer port 8787) and telemetry opt-out.
3. Add hello-world smoke tool + React view; document MENUYUKTI_DATA_DIR + MCP URL for plugin compatibility.
4. Verify skybridge start/dev serves /mcp and DevTools; note loopback-bind follow-up if framework defaults to *.
5. Finalize TASK-44.3 ACs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Scaffolded apps/menuyukti (Skybridge 2.1.0 blank + React). Added render_hello_world_widget + src/views/hello-world.tsx. Scripts default to port 8787 with telemetry opt-out. make mcp-skybridge / scripts/run-mcp-skybridge.sh + docs in apps/menuyukti/README, install-and-run Option C, develop.md. Verified on :8791: initialize, tools/list, tools/call returns structuredContent + viewUUID; DevTools HTTP 200.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added in-repo Skybridge app at apps/menuyukti with local MCP on port 8787 (plugin-compatible URL), MENUYUKTI_DATA_DIR documented, and Hello World smoke tool+view. make mcp-skybridge runs it (Node 24+). Verified MCP tools/call mounts view metadata and DevTools responds. Rust make mcp remains full analytics until later TASK-44 ports.
<!-- SECTION:FINAL_SUMMARY:END -->
