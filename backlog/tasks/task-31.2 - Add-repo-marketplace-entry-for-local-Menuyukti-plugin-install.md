---
id: TASK-31.2
title: Add repo marketplace entry for local Menuyukti plugin install
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:08:00Z'
updated_date: '2026-11-06T04:11:00Z'
labels:
  - codex
  - plugin
  - onboarding
dependencies:
  - TASK-31.1
references:
  - https://learn.chatgpt.com/docs/plugins
  - plugins/menuyukti/
parent_task_id: TASK-31
priority: high
type: feature
ordinal: 70000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
A portable plugin folder alone is not enough for owners: Codex/ChatGPT install via a marketplace catalog. Repo marketplaces live at `.agents/plugins/marketplace.json` and point at local plugin paths so Desktop/CLI can install without a public directory listing.

Add a Menuyukti marketplace entry so developers and early owners can install from this repo (and document the Codex CLI / Desktop steps). Public OpenAI directory submission stays out of scope.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `.agents/plugins/marketplace.json` lists the Menuyukti plugin with a local source path to the packaged plugin
- [x] #2 Install path is verified or documented for Codex CLI (`/plugins` or `codex plugin …`) and/or ChatGPT Desktop Plugins tab against the repo marketplace
- [x] #3 Marketplace policy marks the plugin appropriately for local/desktop use (Desktop-only expectation for localhost MCP)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add .agents/plugins/marketplace.json.
2. Verify install with codex plugin marketplace add . and plugin add.
3. Document CLI path.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Verified: codex plugin marketplace add . → menuyukti-repo; codex plugin add menuyukti@menuyukti-repo installs skills into ~/.codex/plugins/cache.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added .agents/plugins/marketplace.json pointing at ./plugins/menuyukti (AVAILABLE / ON_INSTALL). Verified Codex CLI marketplace register + plugin install; skills appear under the plugin cache.
<!-- SECTION:FINAL_SUMMARY:END -->
