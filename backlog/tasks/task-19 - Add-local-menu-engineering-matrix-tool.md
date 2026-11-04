---
id: TASK-19
title: Add local menu engineering matrix tool
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-04T08:14:00Z'
updated_date: '2026-11-04T08:22:00Z'
labels: []
dependencies: []
references:
  - crates/menuyukti/src/load.rs
  - crates/menuyukti/src/workspace.rs
  - crates/menuyukti-mcp/src/main.rs
  - >-
    ../v3/packages/menuyukti/src/menuyukti/core/analytics/calculate_menu_engineering_matrix.py
  - workspaces/demo-cafe-july-2026/
  - docs/howtos/workspaces.md
documentation:
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 30000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners need the classic star / plow_horse / puzzle / weak portfolio view from local sales, but contribution margin requires unit COGS. Sales tools already load via load_normalized_sales and workspaces can declare a COGS role, yet nothing joins the two or classifies the menu. ChatGPT must get a single tool that fails closed when COGS is missing rather than inventing margins.

Port the matrix semantics from the v3 calculate_menu_engineering_matrix analytics (categories, thresholds, distribution, per-item actions). Keep logic in crates/menuyukti and a thin MCP wrapper. The tool should require a workspace folder with files.sales and files.cogs configured—hard-gate in the tool; server instructions only guide recovery.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ChatGPT can call one MCP tool against a workspace folder and receive menu engineering thresholds, quadrant distribution, and per-item classifications
- [x] #2 Missing workspace COGS role, missing COGS file, or unusable cost coverage returns a clear JSON error that names what is missing and how to fix it (list files / write config / ask owner for COGS)
- [x] #3 Demo cafe workspace (sales + cogs.xlsx) produces a non-empty matrix with the four categories represented where the data supports it
- [x] #4 Owner and MCP docs state that menu engineering needs sales and COGS configured
- [x] #5 Unit tests in crates/menuyukti cover COGS resolve/load and matrix classification (including missing-COGS and quadrant fixture cases); make check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. TASK-19.1: resolve/load workspace COGS.
2. TASK-19.2: port matrix analytics from v3.
3. TASK-19.3: wire analyze_menu_engineering MCP tool + HTTP tests + INSTRUCTIONS.
4. TASK-19.4: owner docs.
5. make check; finalize each subtask then parent.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All subtasks 19.1–19.4 done. make check green (fmt/clippy/tests including MCP HTTP).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Local menu engineering matrix is available via analyze_menu_engineering. Library loads gated workspace COGS, classifies star/plow_horse/puzzle/weak, and the MCP tool fails closed without COGS. Demo cafe + unit/HTTP tests and owner docs updated. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
