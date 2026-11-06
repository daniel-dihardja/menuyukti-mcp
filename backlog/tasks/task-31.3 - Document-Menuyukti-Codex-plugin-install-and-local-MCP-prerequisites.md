---
id: TASK-31.3
title: Document Menuyukti Codex plugin install and local MCP prerequisites
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:08:00Z'
updated_date: '2026-11-06T04:11:00Z'
labels:
  - codex
  - plugin
  - docs
  - onboarding
dependencies:
  - TASK-31.1
  - TASK-31.2
documentation:
  - README.md
  - docs/howtos/install-and-run.md
  - docs/howtos/workspaces.md
parent_task_id: TASK-31
priority: medium
type: docs
ordinal: 71000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
README still tells owners to add MCP URL and individual skills. After packaging, the quick start should lead with plugin install while remaining honest that the local Menuyukti binary, MENUYUKTI_DATA_DIR, and workspaces are still required.

Update owner docs so the easy path is: install plugin → start MCP → set data dir / demo workspace → ask in Codex.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 README quick start mentions installing the Menuyukti plugin (marketplace/local) before or alongside skill-by-skill setup
- [x] #2 Install-and-run or workspaces howto states Desktop-only localhost MCP, that the binary must be running, and points at get_setup_status / demo-cafe
- [x] #3 Docs list which skills the plugin bundles (coaching + visualisations)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add docs/howtos/codex-plugin.md.
2. Update README, install-and-run, docs hub, develop.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Documented plugin install in docs/howtos/codex-plugin.md and linked from README quick start, install-and-run, docs hub, and develop (sync-plugin-skills). Lists bundled skills and Desktop-only localhost MCP prerequisites.
<!-- SECTION:FINAL_SUMMARY:END -->
