---
id: TASK-23
title: Add local workspace knowledge.md read/write MCP tools
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T10:04:00Z'
updated_date: '2026-11-04T10:16:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/workspace.rs
  - crates/menuyukti/src/paths.rs
  - crates/menuyukti-mcp/src/main.rs
  - workspace.example.yaml
documentation:
  - docs/howtos/workspaces.md
  - workspaces/README.md
priority: high
type: feature
ordinal: 47000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners and ChatGPT learn venue facts during chat (hours, seating, cuisine, constraints, past decisions), but nothing durable and path-gated stores them for the next turn. workspace.yaml already allows an optional files.knowledge role pointing at knowledge.md, yet the library and MCP surface never read or write that file, so location context is invented or lost.

Add path-gated read and write/update for knowledge.md inside a workspace folder under MENUYUKTI_DATA_DIR, expose thin MCP tools, and document the ritual so ChatGPT keeps all location-related notes in that one doc.

Reuse note: share workspace path-gate, folder resolve, and files.knowledge role helpers with the existing config load/write path; keep knowledge markdown tools separate from write_workspace_config (structured YAML only).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT can call an MCP tool against a workspace folder under MENUYUKTI_DATA_DIR and receive the current knowledge.md contents (or a clear empty/missing result when the file does not exist yet)
- [x] #2 ChatGPT can call an MCP tool to create or fully replace knowledge.md with location/venue information for that workspace; writes stay inside MENUYUKTI_DATA_DIR
- [x] #3 Paths outside MENUYUKTI_DATA_DIR are rejected the same way as other workspace tools
- [x] #4 Owner and MCP docs mention the knowledge read/write tools and that knowledge.md holds location-related notes for the workspace
- [x] #5 Unit tests cover path-gated read/write (including missing file and outside-sandbox rejection); MCP HTTP tests cover tools/list plus successful read/write; make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. TASK-23.1: add path-gated read/write_workspace_knowledge in menuyukti, reusing workspace resolve/role helpers; unit tests.
2. TASK-23.2: expose read_workspace_knowledge and write_workspace_knowledge MCP tools, update INSTRUCTIONS, HTTP tests.
3. TASK-23.3: document tools in docs/howtos/workspaces.md and workspaces/README.md.
4. Run make check; finalize subtasks then parent.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Local MCP can read and fully replace workspace knowledge.md for location notes. Library reuses workspace path-gate/role helpers; MCP tools + INSTRUCTIONS + owner docs shipped. Subtasks 23.1–23.3 Done; make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
