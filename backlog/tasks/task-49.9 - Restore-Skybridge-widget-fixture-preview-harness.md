---
id: TASK-49.9
title: Restore Skybridge widget fixture preview harness
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
labels:
  - widgets
  - dx
dependencies: []
references:
  - docs/howtos/develop.md
  - apps/menuyukti/README.md
parent_task_id: TASK-49
priority: medium
type: enhancement
ordinal: 131000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rust-era make widgets-preview with populated/empty mock structuredContent fixtures (TASK-26 / TASK-42.4) was retired with the Skybridge port. DevTools + demo-cafe Excel is heavier and poor for empty/error edge-state styling without loading real sales files.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Developers can preview each analytics widget (and empty/error fixtures where useful) without calling live MCP against Excel
- [ ] #2 How-to or app README documents the preview command/path
- [ ] #3 Fixtures cover at least one populated and one empty (or error) shape per analytics widget family
<!-- AC:END -->
