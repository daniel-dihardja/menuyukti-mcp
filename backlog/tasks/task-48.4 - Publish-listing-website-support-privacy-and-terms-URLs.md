---
id: TASK-48.4
title: Publish listing website support privacy and terms URLs
status: To Do
assignee: []
created_date: '2026-11-07T03:34:00Z'
labels:
  - openai
  - legal
  - release
dependencies: []
references:
  - https://developers.openai.com/plugins/deploy/submission
  - plugins/menuyukti/plugin.json
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
parent_task_id: TASK-48
priority: high
type: docs
ordinal: 118000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
OpenAI plugin listing/review expects HTTPS website, support, privacy policy, and terms of service URLs that identify the same publisher. Menuyukti plugin.json currently lacks these listing fields.

Publish durable pages (GitHub Pages, product site, or equivalent) covering product overview, how to get help, privacy for local POS data, and terms — then wire the URLs into directory plugin metadata.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 HTTPS websiteURL, supportURL, privacyPolicyURL, and termsOfServiceURL are live
- [ ] #2 Privacy page states local Desktop MCP reads sales on-device and does not require cloud POS upload for the happy path
- [ ] #3 Directory plugin metadata references these four URLs
<!-- AC:END -->
