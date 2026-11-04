---
id: TASK-23.2
title: Expose read_workspace_knowledge and write_workspace_knowledge MCP tools
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T10:04:00Z'
updated_date: '2026-11-04T10:16:00Z'
labels: []
dependencies:
  - TASK-23.1
references:
  - crates/menuyukti-mcp/src/main.rs
  - crates/menuyukti-mcp/tests/mcp_http.rs
parent_task_id: TASK-23
priority: high
type: feature
ordinal: 49000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Library knowledge read/write alone is unreachable from ChatGPT. Other workspace ops already ship as path-gated MCP tools with server instructions; knowledge needs the same surface so chat can load location context before advising and write updates after the owner shares facts.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `tools/list` exposes `read_workspace_knowledge` and `write_workspace_knowledge` (or equivalent names) that accept a workspace folder under MENUYUKTI_DATA_DIR
- [x] #2 Read returns current markdown (or clear missing/empty); write creates or fully replaces knowledge.md via the library API
- [x] #3 Server INSTRUCTIONS tell ChatGPT to keep location-related notes in knowledge.md and to read before rewriting
- [x] #4 HTTP MCP integration asserts tools/list plus successful read/write and outside-sandbox rejection
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add WriteWorkspaceKnowledgeArgs + read_workspace_knowledge / write_workspace_knowledge MCP tools calling library.
2. Update INSTRUCTIONS: keep location notes in knowledge.md; read before rewrite.
3. Extend mcp_http tools/list + dedicated read/write + outside-sandbox test.
4. make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MCP tools + INSTRUCTIONS + mcp_http_reads_and_writes_workspace_knowledge; tools/list updated. make check passed.
<!-- SECTION:NOTES:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-11-04T10:08:00Z

---

Reuse note from design discussion: MCP wrappers stay thin (WorkspacePathArgs, text_json/error_json). Library must reuse workspace resolve/role helpers; do not reimplement path gating in the binary or fold markdown into write_workspace_config.

---
<!-- COMMENTS:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exposed read_workspace_knowledge and write_workspace_knowledge MCP tools with thin wrappers. Server INSTRUCTIONS tell ChatGPT to keep location notes in knowledge.md and read before rewrite. HTTP tests cover tools/list, missing/write/read roundtrip, and outside-sandbox rejection. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
