---
id: TASK-27
title: Harden Menuyukti MCP owner usability with in-chat coaching
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-05T08:49:00Z'
updated_date: '2026-11-05T08:59:00Z'
labels:
  - ux
  - onboarding
dependencies: []
references:
  - crates/menuyukti-mcp/src/main.rs
  - README.md
documentation:
  - docs/howtos/workspaces.md
  - docs/howtos/install-and-run.md
priority: high
type: enhancement
ordinal: 57000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Restaurant owners succeed with Menuyukti mainly through ChatGPT, not by reading howtos. Today server instructions are a dense capability dump, tool failures often lack a clear next step, and first-run problems (server/data dir/unconfigured folders) are hard to diagnose in chat. This initiative makes the MCP coach the owner: one 3-step ritual, goal-oriented asks, recoverable errors, and a setup status path — with owner docs as a thin mirror of that playbook.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Parent tracks the usability initiative; work ships via completed subtasks
- [x] #2 Owners can discover how to work with Menuyukti primarily through ChatGPT coaching plus a short README/docs cheat sheet
- [x] #3 Subtasks cover MCP playbook instructions, setup/status diagnosis, actionable tool errors, owner what-to-ask docs, and an optional Codex coaching skill
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement TASK-27.2 get_setup_status (library helper + MCP tool + HTTP test).
2. Rewrite INSTRUCTIONS playbook (27.1) referencing setup status.
3. Harden tool error messages with recovery hints (27.3) + tests.
4. Update owner docs cheat sheet/ritual (27.4).
5. Add Codex menuyukti-mcp coaching skill (27.5).
6. Run make check; finalize each subtask then parent.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All subtasks 27.1–27.5 Done; make check green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Hardened Menuyukti MCP owner usability: get_setup_status, coaching INSTRUCTIONS playbook, actionable tool errors, owner what-to-ask docs, and a Codex coaching skill. Verified with make check (fmt, clippy, workspace tests including new MCP HTTP coverage).
<!-- SECTION:FINAL_SUMMARY:END -->
