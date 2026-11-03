---
id: TASK-9.8
title: Cut over user workflow to Rust and retire Python runtime path
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
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
- [ ] #1 README documents Rust install/run as the primary way to start the local MCP server against sales-reports/
- [ ] #2 package.json / scripts that start or inspect MCP point at the Rust binary (or are removed if obsolete)
- [ ] #3 Users do not need Python, uv, or pandas installed for the normal Excel → MCP workflow
- [ ] #4 Obsolete Python package/runtime instructions are removed or clearly marked non-product
<!-- AC:END -->
