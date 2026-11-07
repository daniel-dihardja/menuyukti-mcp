---
id: TASK-48.3
title: Build directory-ready Menuyukti plugin package
status: To Do
assignee: []
created_date: '2026-11-07T03:34:00Z'
labels:
  - openai
  - plugin
dependencies:
  - TASK-48.2
references:
  - plugins/menuyukti/
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
parent_task_id: TASK-48
priority: high
type: feature
ordinal: 117000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The repo plugin still points mcp.json at localhost HTTP and declares SessionStart hooks. OpenAI directory ZIPs for local MCP need bundled stdio configuration and must not include lifecycle hooks.

Produce a directory-ready package (or build artifact) based on the spike: stdio mcp.json, no hooks in the submission ZIP, synced skills, icons, and listing-oriented plugin.json metadata. Keep the existing repo-marketplace plugin path working for early owners unless a clean dual-package strategy is documented.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Submission package mcp.json declares bundled stdio local MCP under PLUGIN_ROOT
- [ ] #2 Submission package does not include lifecycle hooks
- [ ] #3 Skills and icons required for Menuyukti Desktop use are included
- [ ] #4 Clean Desktop install path from the package is documented for maintainers
<!-- AC:END -->
