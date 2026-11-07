---
id: TASK-51.1
title: Move knowledge read/write library API to data-dir root
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-07T05:38:00Z'
updated_date: '2026-11-07T05:42:00Z'
labels:
  - knowledge
  - rust
dependencies: []
references:
  - crates/menuyukti/src/workspace.rs
parent_task_id: TASK-51
priority: high
type: enhancement
ordinal: 101000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MCP tools need a library API that reads and writes location notes at MENUYUKTI_DATA_DIR/knowledge.md instead of resolving a per-folder files.knowledge role. Without that, each venue/period folder keeps its own notes and the one-location-per-data-dir model cannot land.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Library reads and writes knowledge.md at the data-dir root under path gating; missing file returns exists=false with empty content
- [x] #2 API no longer takes a workspace folder argument or auto-updates workspace.yaml files.knowledge
- [x] #3 files.knowledge is removed from WorkspaceFiles / load-write validation (or ignored without writing it back)
- [x] #4 Unit tests cover missing, roundtrip, and rejection of paths outside MENUYUKTI_DATA_DIR
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Retarget knowledge helpers to MENUYUKTI_DATA_DIR/knowledge.md (no workspace folder arg; drop files.knowledge auto-update).
2. Remove knowledge from WorkspaceFiles + validate_config; keep deserialize tolerant of legacy YAML keys by simply omitting the field.
3. Slim KnowledgeRead/Write result types (data_dir, knowledge_path, content; drop workspace/config_updated).
4. Rewrite unit tests for root missing/roundtrip; path gate via resolve_allowed_path on knowledge.md.
5. Minimal MCP/compile fix so WorkspaceFiles/knowledge arg removal builds (full tool UX in TASK-51.2).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Library knowledge API now targets MENUYUKTI_DATA_DIR/knowledge.md. WorkspaceFiles.knowledge removed; legacy YAML key ignored on load. Unit tests cover missing, root roundtrip, legacy ignore, missing data dir. make check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Retargeted read/write_workspace_knowledge to data-dir root knowledge.md with no workspace folder arg and no workspace.yaml updates. Removed files.knowledge from WorkspaceFiles. Unit tests and make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
