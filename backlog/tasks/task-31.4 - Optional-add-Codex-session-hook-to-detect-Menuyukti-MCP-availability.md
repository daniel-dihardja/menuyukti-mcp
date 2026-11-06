---
id: TASK-31.4
title: 'Optional: add Codex session hook to detect Menuyukti MCP availability'
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:08:00Z'
updated_date: '2026-11-06T04:11:00Z'
labels:
  - codex
  - plugin
  - ux
dependencies:
  - TASK-31.1
references:
  - https://learn.chatgpt.com/docs/plugins
parent_task_id: TASK-31
priority: low
type: enhancement
ordinal: 72000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Even after plugin install, owners often forget to start the local MCP binary and get confusing tool failures. Codex plugin hooks can run at SessionStart with PLUGIN_ROOT, but hooks require explicit user trust and do not deploy the binary themselves.

Optional follow-up: a small hook that checks whether http://127.0.0.1:8787/mcp responds (or prompts via status message) and points at install-and-run docs. Do not auto-launch untrusted binaries without a clear, reviewed design.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 If implemented, hooks/hooks.json + script check local MCP reachability and surface a short status/recovery hint without uploading POS data
- [x] #2 Docs state hooks must be reviewed/trusted in Codex and that the hook does not replace installing/running the Menuyukti binary
- [x] #3 Task may be deferred or canceled if product prefers docs-only onboarding for v1
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Ship hooks/check_mcp.sh reachability probe.
2. Document trust requirement.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented optional SessionStart hook (hooks/hooks.json + check_mcp.sh) that probes localhost MCP without starting the binary or reading POS files. Documented trust requirement in plugin README and codex-plugin howto. Smoke-tested when MCP is down (exit 0 + recovery hint).
<!-- SECTION:FINAL_SUMMARY:END -->
