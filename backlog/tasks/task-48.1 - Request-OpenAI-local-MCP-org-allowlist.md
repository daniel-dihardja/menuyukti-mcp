---
id: TASK-48.1
title: Request OpenAI local MCP org allowlist
status: To Do
assignee: []
created_date: '2026-11-07T03:34:00Z'
labels:
  - openai
  - release
dependencies: []
references:
  - https://developers.openai.com/plugins/deploy/desktop-only
  - https://platform.openai.com
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
parent_task_id: TASK-48
priority: high
type: task
ordinal: 115000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Public directory listing for a local (stdio) MCP requires the submitting OpenAI organization to be partner-allowlisted. Without that gate, portal uploads fail (for example mcp_configuration_excluded). Freelancer eligibility is unclear and must be confirmed with OpenAI.

Capture org ID, send the outreach ask (template in doc-3), and record OpenAI’s written response so packaging/submit work knows whether to proceed or pivot.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Organization ID and verification status recorded for the publishing org
- [ ] #2 Allowlist request sent to OpenAI (rep or Apps/plugin support) using the doc-3 template
- [ ] #3 OpenAI response (approved, denied, or pending with next step) recorded on this task
<!-- AC:END -->
