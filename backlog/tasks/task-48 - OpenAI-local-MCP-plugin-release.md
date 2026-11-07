---
id: TASK-48
title: OpenAI local MCP plugin release
status: To Do
assignee: []
created_date: '2026-11-07T03:33:00Z'
updated_date: '2026-11-07T03:34:00Z'
labels:
  - openai
  - plugin
  - release
  - desktop
dependencies: []
references:
  - https://developers.openai.com/plugins/deploy/desktop-only
  - https://developers.openai.com/plugins/deploy/submission
  - plugins/menuyukti/plugin.json
  - plugins/menuyukti/mcp.json
  - >-
    backlog/decisions/decision-4 -
    Ship-Menuyukti-via-partner-local-MCP-not-remote-HTTPS.md
documentation:
  - backlog/docs/product/doc-3 - OpenAI-local-MCP-plugin-release.md
priority: high
type: feature
ordinal: 114000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Menuyukti is installable from this repo marketplace (TASK-31) but is not listed in the OpenAI Plugin Directory. Owners who need local POS privacy cannot use a remote HTTPS MCP without uploading sales Excel. OpenAI’s partner **local MCP** path (Desktop-only) is the intended public distribution model.

Ship Menuyukti through that path: get the submitting org allowlisted, package a directory-ready Desktop-only plugin (bundled stdio MCP, no hooks), complete listing/review materials, submit and publish, then document Directory vs repo-marketplace install.

Out of scope: remote HTTPS / Alpic-hosted POS product; new analytics capabilities. External gate: OpenAI may deny freelancer local-MCP allowlisting — track the ask; do not assume approval.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 OpenAI org allowlist for local MCP requested and outcome recorded
- [ ] #2 Directory-ready plugin ZIP uses bundled stdio MCP and excludes lifecycle hooks
- [ ] #3 Listing includes required HTTPS website, support, privacy, and terms URLs
- [ ] #4 Review pack (test cases, demo video, release notes) is ready for submission
- [ ] #5 Plugin submitted via Skills-only / local MCP flow after allowlist; published or blocked outcome documented
- [ ] #6 Owner docs describe Plugin Directory Desktop install and keep repo marketplace path
<!-- AC:END -->
