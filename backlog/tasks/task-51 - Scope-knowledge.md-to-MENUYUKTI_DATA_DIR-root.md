---
id: TASK-51
title: Scope knowledge.md to MENUYUKTI_DATA_DIR root
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-07T05:38:00Z'
updated_date: '2026-11-07T05:42:00Z'
labels:
  - knowledge
  - workspace
dependencies: []
references:
  - docs/howtos/workspaces.md
  - crates/menuyukti/src/workspace.rs
  - crates/menuyukti-mcp/src/main.rs
priority: high
type: enhancement
ordinal: 100000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners typically use one MENUYUKTI_DATA_DIR per location, so venue facts (hours, seating, cuisine, constraints, past decisions) should not be tied to a venue/period analysis folder. Today knowledge.md lives inside each workspace folder and is resolved via files.knowledge on workspace.yaml, so opening a new period folder loses or duplicates location notes.

Move location knowledge to a single path-gated file at the data-dir root (knowledge.md). Analysis folders keep sales/COGS + workspace.yaml only. Product assumption: one data dir ≈ one location; multi-location owners use separate data dirs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Location knowledge is stored as knowledge.md at the MENUYUKTI_DATA_DIR root, shared by all workspace folders under that data dir
- [x] #2 read/write knowledge MCP tools no longer require a workspace folder path
- [x] #3 workspace.yaml no longer owns or auto-sets a files.knowledge role for this file
- [x] #4 Demo, howto, skills, and MCP instructions describe data-dir-wide knowledge and the one-location-per-data-dir assumption
- [x] #5 Library and MCP tests cover root knowledge read/write, missing file, and path gating; make check passes
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Location knowledge is now MENUYUKTI_DATA_DIR/knowledge.md shared across period folders. Library + MCP tools no longer take a workspace path or files.knowledge role; docs/demo updated. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
