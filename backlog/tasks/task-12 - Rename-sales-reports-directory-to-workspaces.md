---
id: TASK-12
title: Rename sales-reports directory to workspaces
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T14:51:00Z'
updated_date: '2026-11-03T14:52:00Z'
labels: []
dependencies: []
references:
  - workspaces/
  - .gitignore
  - README.md
type: chore
ordinal: 20000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
sales-reports/ is too narrow now that each venue-period folder can hold sales, COGS, docs, and later knowledge. Owners need one clear local workspace root that MENUYUKTI_DATA_DIR defaults to.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Repo root folder is workspaces/ with the demo-cafe-july-2026 dataset inside
- [x] #2 Defaults (Makefile, run-mcp, mcp-inspect) and docs point at workspaces/
- [x] #3 Gitignore tracks only the demo workspace files and README under workspaces/
- [x] #4 Tests that resolve the owner demo use workspaces/ paths
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Rename sales-reports/ to workspaces/ (preserve demo + any local uploads).
2. Update .gitignore allowlist for workspaces/ demo paths.
3. Update Makefile, run-mcp.sh, mcp-inspect.sh defaults to workspaces/.
4. Update README, workspaces README, fixtures README, MCP tool description, rust-quality skill.
5. Update Rust tests that join sales-reports.
6. Run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Renamed sales-reports/ to workspaces/; updated gitignore, Makefile/scripts defaults, READMEs, MCP list tool description, and demo path tests. Kept MENUYUKTI_DATA_DIR env name. make check passed after clearing a stale server on :8787 that still pointed at sales-reports.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Renamed the owner data root from sales-reports/ to workspaces/, including the demo-cafe-july-2026 dataset. Defaults and docs now use workspaces/; MENUYUKTI_DATA_DIR is unchanged. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
