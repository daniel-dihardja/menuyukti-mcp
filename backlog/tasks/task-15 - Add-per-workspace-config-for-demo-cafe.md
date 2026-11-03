---
id: TASK-15
title: Add per-workspace config for demo cafe
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T16:09:00Z'
updated_date: '2026-11-03T16:10:00Z'
labels: []
dependencies: []
references:
  - workspaces/demo-cafe-july-2026/
  - docs/howtos/workspaces.md
type: docs
ordinal: 23000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Workspace folders hold sales, COGS, and other docs, but nothing declares which file plays which role. Owners and tools need a small workspace.json so roles are explicit instead of guessed from filenames.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Demo workspace has a workspace.json that maps sales and COGS files
- [x] #2 An example workspace config template exists for new workspaces
- [x] #3 Git tracks the demo workspace.json while still ignoring personal uploads
- [x] #4 Docs describe the workspace config convention
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Define workspace.json schema (name, venue, period, files.sales/cogs roles).
2. Add workspace.json under demo-cafe-july-2026 and workspace.example.json at repo root.
3. Update .gitignore allowlist for the demo config.
4. Document the convention in docs/howtos/workspaces.md and workspaces/README.md.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added demo workspace.json, root workspace.example.json, gitignore allowlist, and docs for roles sales/cogs/knowledge.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Each workspace can declare file roles in workspace.json. Demo cafe includes sales + COGS mappings; workspace.example.json is the template for new folders. Docs and gitignore updated. Tools still take explicit sales paths for now.
<!-- SECTION:FINAL_SUMMARY:END -->
