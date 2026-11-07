---
id: TASK-44.10
title: Retire Rust crates and replace quality gate with TypeScript CI
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T18:37:00Z'
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
- [x] #1 Default contributor quality gate runs against the Skybridge/TS project (lint/typecheck/tests as appropriate)
- [x] #2 Rust crates are removed or marked obsolete with docs that point to the TS app as source of truth
- [x] #3 README / develop howto no longer require Cargo for the primary local MCP path
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add apps/menuyukti typecheck/test scripts; rewrite make check / scripts/check.sh for Skybridge TS.
2. Retarget CI to Node 24 + make check; retire Rust release binary workflow (or document obsolete).
3. Remove crates/, Cargo.*, rust-toolchain, Rust run/preview scripts and Makefile rust targets.
4. Replace Cursor rust quality rules/skill with TS quality gate; update install/develop/docs references.
5. Run make check; finalize TASK-44.10 and parent TASK-44 ACs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Removed crates/Cargo/rust-toolchain; make check + CI = tsc+vitest; release binary workflow retired; Cursor rust rules replaced with typescript-quality; docs/skills point at apps/menuyukti. make check green (13 tests).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rust workspace retired. Default quality gate and CI are TypeScript (typecheck + vitest) for apps/menuyukti. Owner/develop docs require Node 24+ / make mcp only — no Cargo. Binary release workflow disabled.
<!-- SECTION:FINAL_SUMMARY:END -->
