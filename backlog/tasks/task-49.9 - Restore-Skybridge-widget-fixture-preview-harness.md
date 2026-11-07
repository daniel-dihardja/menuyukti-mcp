---
id: TASK-49.9
title: Restore Skybridge widget fixture preview harness
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:17:00Z'
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
- [x] #1 Developers can preview each analytics widget (and empty/error fixtures where useful) without calling live MCP against Excel
- [x] #2 How-to or app README documents the preview command/path
- [x] #3 Fixtures cover at least one populated and one empty (or error) shape per analytics widget family
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add JSON fixtures under apps/menuyukti for each analytics widget: populated + empty/error structuredContent.
2. Add a small local preview page/script that loads a view with fixture tool output (no Excel/MCP).
3. Document the preview command in develop.md and/or apps/menuyukti/README.md.
4. Keep fixtures reusable for TASK-49.10 React tests.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added src/views/fixtures with populated/empty/error per analytics widget. pnpm widgets:preview (vite.preview.config.ts :8790) aliases helpers to fixture mock. Documented in apps/menuyukti/README + develop.md. Root script widgets:preview wired.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Restored a fixture-based widget preview: pnpm widgets:preview serves each analytics view with populated/empty/error structuredContent and no Excel/MCP. Fixtures live under src/views/fixtures and are documented for developers.
<!-- SECTION:FINAL_SUMMARY:END -->
