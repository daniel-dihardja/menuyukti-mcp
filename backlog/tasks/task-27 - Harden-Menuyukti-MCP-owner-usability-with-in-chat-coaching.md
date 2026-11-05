---
id: TASK-27
title: Harden Menuyukti MCP owner usability with in-chat coaching
status: To Do
assignee: []
created_date: '2026-11-05T08:49:00Z'
updated_date: '2026-11-05T08:54:00Z'
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
- [ ] #1 Parent tracks the usability initiative; work ships via completed subtasks
- [ ] #2 Owners can discover how to work with Menuyukti primarily through ChatGPT coaching plus a short README/docs cheat sheet
- [ ] #3 Subtasks cover MCP playbook instructions, setup/status diagnosis, actionable tool errors, owner what-to-ask docs, and an optional Codex coaching skill
<!-- AC:END -->
