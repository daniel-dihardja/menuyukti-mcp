---
id: TASK-9.8
title: Cut over user workflow to Rust and retire Python runtime path
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:22:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.7
references:
  - README.md
  - package.json
  - scripts/mcp-inspect.sh
parent_task_id: TASK-9
priority: high
type: chore
ordinal: 17000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Users and agents still have Python-first README/scripts (uv run, npm mcp). After the Rust tools and tests work, the primary documented path must be the Rust binary so local install matches the system-close product goal. Python internals need not stay supported.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 README documents Rust install/run as the primary way to start the local MCP server against sales-reports/
- [x] #2 package.json / scripts that start or inspect MCP point at the Rust binary (or are removed if obsolete)
- [x] #3 Users do not need Python, uv, or pandas installed for the normal Excel → MCP workflow
- [x] #4 Obsolete Python package/runtime instructions are removed or clearly marked non-product
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Rewrite README for Rust install/run as primary path.
2. Keep package.json mcp → scripts/run-mcp.sh; document mcp:python as legacy/non-product.
3. Update sales-reports README if needed; note Python package as reference-only.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
README now leads with Rust/Cargo run path; npm run mcp and mcp-inspect use Rust; mcp:python marked legacy; packages/menuyukti README marked non-product.

Removed Python project config and legacy tree (pyproject.toml, uv.lock, src/, packages/ Python sources, root tests/). POS fixtures kept under crates/menuyukti/fixtures/pos-sales-reports.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
User-facing workflow is Rust-first: install Rust, put Excel in sales-reports/, npm run mcp. Python/uv/pandas are not required for the product path and are documented as legacy/non-product.
<!-- SECTION:FINAL_SUMMARY:END -->
