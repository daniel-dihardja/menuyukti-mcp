---
id: TASK-23.1
title: Add path-gated library read/write for workspace knowledge.md
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
  - workspace.example.yaml
parent_task_id: TASK-23
priority: high
type: feature
ordinal: 48000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MCP tools need a library API before they can expose knowledge.md. The optional files.knowledge role already exists on workspace.yaml, but nothing resolves that path or reads/writes markdown under MENUYUKTI_DATA_DIR. Without a gated load/write helper, ChatGPT cannot safely persist location notes next to sales/COGS.

Reuse note: prefer exporting or reusing existing workspace helpers (`resolve_workspace_dir`, `validate_role_path`, `join_workspace_role_path`, `ensure_path_inside_root`, `load_workspace_config` / `write_workspace_config` for the knowledge role) rather than copying path-gate or role-join logic. Keep knowledge markdown I/O as separate helpers; do not extend `write_workspace_config` to carry free-form notes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Library can read knowledge.md for a workspace folder under MENUYUKTI_DATA_DIR, resolving files.knowledge when set or defaulting to knowledge.md in that folder
- [x] #2 Library can create or fully replace knowledge.md contents for a workspace folder; if files.knowledge is unset, write ensures the role points at knowledge.md in workspace.yaml when a config already exists
- [x] #3 Missing knowledge file on read returns a clear missing/empty result rather than a hard failure; outside-sandbox paths are rejected
- [x] #4 Unit tests cover read/write, missing file, role resolution, and path-gate rejection; make check passes
- [x] #5 Prefer exporting or reusing workspace resolve/role-join/path-gate helpers rather than duplicating them; knowledge markdown stays separate from write_workspace_config
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add DEFAULT_KNOWLEDGE_FILENAME and KnowledgeRead/Write result types in workspace.rs.
2. Resolve knowledge role path via load_workspace_config when present, else default knowledge.md; reuse resolve_workspace_dir, validate_role_path, join_workspace_role_path, ensure_path_inside_root, resolve_allowed_path.
3. read_workspace_knowledge: missing file → exists=false + empty content (not hard fail); path outside sandbox rejected.
4. write_workspace_knowledge: full replace; if workspace.yaml exists and files.knowledge unset, set role to knowledge.md via write_workspace_config.
5. Export from lib.rs; unit tests for read/write, missing, role resolution, path gate.
6. make check (or cargo test -p menuyukti) before marking done.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added read_workspace_knowledge / write_workspace_knowledge in workspace.rs reusing resolve_workspace_dir, validate_role_path, join_workspace_role_path, ensure_path_inside_root, load/write_workspace_config. Unit tests cover missing, roundtrip+role set, custom role path, no-config write, path gate. Exported from lib.rs.

Verified with cargo test workspace::* and make check (fmt/clippy/test).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added path-gated read_workspace_knowledge / write_workspace_knowledge in menuyukti::workspace, reusing resolve_workspace_dir, role-path helpers, ensure_path_inside_root, and load/write_workspace_config for the knowledge role. Missing reads return exists=false; first write with existing config sets files.knowledge. Unit tests cover missing, roundtrip, custom role, no-config, and path gate. make check passed.
<!-- SECTION:FINAL_SUMMARY:END -->
