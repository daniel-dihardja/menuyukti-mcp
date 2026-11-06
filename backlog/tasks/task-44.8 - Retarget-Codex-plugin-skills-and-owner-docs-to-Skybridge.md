---
id: TASK-44.8
title: Retarget Codex plugin skills and owner docs to Skybridge
status: To Do
assignee: []
created_date: '2026-11-06T17:30:00Z'
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
- [ ] #1 Plugin MCP config and install docs start/connect the Skybridge server (no required Rust binary for the happy path)
- [ ] #2 Owner README / howtos describe the new run story and still map plan-promotions / trim-menu asks to the same tools
- [ ] #3 Bundled skills prefer render_* / mounted views when available and avoid double-drawing with HTML fallbacks in the same turn
<!-- AC:END -->
