---
id: TASK-27.4
title: Document owner what-to-ask cheat sheet and 3-step ritual
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-05T08:50:00Z'
updated_date: '2026-11-05T08:59:00Z'
labels:
  - ux
  - docs
  - onboarding
dependencies:
  - TASK-27.1
references:
  - README.md
documentation:
  - docs/howtos/workspaces.md
  - docs/howtos/install-and-run.md
  - docs/README.md
parent_task_id: TASK-27
priority: medium
type: docs
ordinal: 61000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
README and howtos already list tools and examples, but owners think in goals and skip long guides. A short what-to-ask cheat sheet and the same 3-step ritual as the MCP playbook keep human docs aligned with in-chat coaching without teaching YAML or every tool name up front. Widget troubleshooting and custom POS stay progressive disclosure.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 README Quick start states the same 3-step ritual as the MCP playbook (run server, drop files, set up then ask)
- [x] #2 README or workspaces howto includes a compact What to ask table (5–7 owner goals → example chat prompts), not a full tool dump as the primary surface
- [x] #3 Tool-name inventory remains secondary; advanced widget/Chrome/tunnel and custom POS guidance stay linked as progressive disclosure
- [x] #4 workspaces/README.md and docs/README.md stay consistent with the ritual and cheat sheet (no contradictory first steps)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Align README Quick start + What to ask cheat sheet with playbook.
2. Sync docs/README, workspaces howto, workspaces/README.
3. Keep tool inventory secondary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated README, docs/README, docs/howtos/workspaces.md, workspaces/README.md with 3-step ritual and What to ask table; get_setup_status added to secondary tool list.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owner docs now lead with the same 3-step ritual and a goal→prompt cheat sheet; tool names and widget/POS deep-dives stay secondary. Docs updated across README and workspaces guides.
<!-- SECTION:FINAL_SUMMARY:END -->
