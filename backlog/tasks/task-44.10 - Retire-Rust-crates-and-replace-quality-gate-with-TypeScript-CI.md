---
id: TASK-44.10
title: Retire Rust crates and replace quality gate with TypeScript CI
status: To Do
assignee: []
created_date: '2026-11-06T17:30:00Z'
labels:
  - skybridge
  - migration
  - ci
milestone: m-2
dependencies:
  - TASK-44.9
references:
  - Makefile
  - scripts/check.sh
documentation:
  - docs/howtos/develop.md
  - backlog/docs/product/doc-2 - Skybridge-migration-plan.md
parent_task_id: TASK-44
priority: medium
type: chore
ordinal: 110000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Leaving the Rust workspace as the default make check path invites dual-stack drift after Skybridge reaches parity. Retire or clearly obsolete the crates and point CI/local gates at the TypeScript project.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Default contributor quality gate runs against the Skybridge/TS project (lint/typecheck/tests as appropriate)
- [ ] #2 Rust crates are removed or marked obsolete with docs that point to the TS app as source of truth
- [ ] #3 README / develop howto no longer require Cargo for the primary local MCP path
<!-- AC:END -->
