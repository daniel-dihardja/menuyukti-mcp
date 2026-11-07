---
id: TASK-44.8
title: Retarget Codex plugin skills and owner docs to Skybridge
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T18:30:00Z'
labels:
  - skybridge
  - migration
  - docs
milestone: m-2
dependencies:
  - TASK-44.7
references:
  - plugins/menuyukti/
documentation:
  - README.md
  - plugins/menuyukti/README.md
  - docs/howtos/install-and-run.md
parent_task_id: TASK-44
priority: high
type: docs
ordinal: 108000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The plugin still points at the Rust binary and localhost URL assumptions from install-and-run. After the TS server exists, owners and agents must start and connect the Skybridge app; skills must prefer Skybridge views when mounted.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Plugin MCP config and install docs start/connect the Skybridge server (no required Rust binary for the happy path)
- [x] #2 Owner README / howtos describe the new run story and still map plan-promotions / trim-menu asks to the same tools
- [x] #3 Bundled skills prefer render_* / mounted views when available and avoid double-drawing with HTML fallbacks in the same turn
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Make Skybridge the happy-path run target (make mcp → Skybridge; keep Rust as make mcp-rust until TASK-44.10).
2. Rewrite install-and-run, root/plugin READMEs, codex-plugin, develop, Makefile help for Node 24+ / make mcp.
3. Tighten skill copy so show/chart asks prefer render_* / mounted views and forbid same-turn HTML redraw.
4. Sync .agents/skills → plugins/menuyukti/skills; spot-check tool→goal maps still match plan-promotions / trim-menu.
5. Finalize ACs with evidence from docs + synced skills.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
make mcp now starts Skybridge (mcp-rust for legacy). Rewrote install-and-run, root/plugin READMEs, develop, codex-plugin, marketplace, plugin.json. Skills prefer render_* / forbid same-turn HTML; synced into plugins/menuyukti/skills.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner and plugin docs now use Skybridge as the happy path (make mcp / Node 24+; same localhost MCP URL). Plan-promotions and trim-menu asks still map to the same tools; bundled skills prefer render_* views when mounted and forbid same-turn HTML redraw. Rust remains available as make mcp-rust until TASK-44.10.
<!-- SECTION:FINAL_SUMMARY:END -->
