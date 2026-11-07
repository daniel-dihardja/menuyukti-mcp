---
id: TASK-48.6
title: Submit and publish Desktop-only plugin to OpenAI directory
status: To Do
assignee: []
created_date: '2026-11-07T03:34:00Z'
labels:
  - openai
  - release
dependencies:
  - TASK-48.1
  - TASK-48.3
  - TASK-48.4
  - TASK-48.5
references:
  - https://developers.openai.com/plugins/deploy/desktop-only
  - https://developers.openai.com/plugins/deploy/submission
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
parent_task_id: TASK-48
priority: high
type: task
ordinal: 120000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After local-MCP allowlisting and a directory-ready package, the remaining work is portal submission: Create plugin, Skills only path for approved local MCP, upload ZIP, resolve automated findings, submit for review, then Publish when approved.

If allowlisting is denied, document the blocked outcome on this task and stop without forcing a remote MCP pivot here.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ZIP uploaded in the OpenAI plugin portal under the allowlisted org (or blocked reason recorded if allowlist denied)
- [ ] #2 Automated findings required for submission are resolved or explicitly deferred with OpenAI guidance
- [ ] #3 Plugin submitted for review; approval or rejection feedback recorded
- [ ] #4 On approval, plugin published; directory listing URL recorded
<!-- AC:END -->
