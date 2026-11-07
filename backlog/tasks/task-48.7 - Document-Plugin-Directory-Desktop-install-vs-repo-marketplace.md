---
id: TASK-48.7
title: Document Plugin Directory Desktop install vs repo marketplace
status: To Do
assignee: []
created_date: '2026-11-07T03:34:00Z'
labels:
  - openai
  - docs
  - plugin
dependencies:
  - TASK-48.6
references:
  - plugins/menuyukti/README.md
  - docs/howtos/codex-plugin.md
  - README.md
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
parent_task_id: TASK-48
priority: medium
type: docs
ordinal: 121000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners currently install from the repo marketplace and start Skybridge separately. After a directory publish (or a documented blocked attempt), README and plugin docs must explain the supported install paths without contradicting local-privacy Desktop-only constraints.

Update owner-facing docs for Directory Desktop install when published; keep repo marketplace instructions for contributors/early access; note web/mobile are unsupported.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Plugin README and root/docs howtos describe Directory Desktop install when published (or clearly state not yet listed)
- [ ] #2 Repo marketplace install path remains documented for local development
- [ ] #3 Docs state Desktop-only and that sales Excel stays on the owner machine
<!-- AC:END -->
