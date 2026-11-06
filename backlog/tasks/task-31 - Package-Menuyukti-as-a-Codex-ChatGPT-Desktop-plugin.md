---
id: TASK-31
title: Package Menuyukti as a Codex / ChatGPT Desktop plugin
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T04:08:00Z'
updated_date: '2026-11-06T04:11:00Z'
labels:
  - codex
  - plugin
  - ux
  - onboarding
dependencies: []
references:
  - https://learn.chatgpt.com/docs/plugins
  - https://learn.chatgpt.com/docs/skills-and-plugins
  - .agents/skills/menuyukti-mcp/SKILL.md
  - .agents/skills/menuyukti-visualize-venue-slot-strength/SKILL.md
  - .agents/skills/menuyukti-visualize-menu-heatmaps/SKILL.md
  - .agents/skills/menuyukti-visualize-weak-items/SKILL.md
  - docs/howtos/install-and-run.md
documentation:
  - README.md
priority: high
type: feature
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners still assemble Menuyukti by hand: run the local MCP binary, paste `http://127.0.0.1:8787/mcp`, then install several skills from `.agents/skills/` one by one. Codex/ChatGPT now support installable plugins that bundle skills + MCP connection metadata into one Desktop-oriented package.

Package Menuyukti as a portable Agent Plugin (`plugin.json` + `skills/` + `mcp.json`) so Codex/ChatGPT Desktop can install coaching + visualisation skills together and prompt to connect the local MCP server. Keep `.agents/skills/` as the authoring source of truth; the plugin is the distribution unit. Local POS privacy means Desktop-only localhost MCP is expected — the plugin does not replace downloading/running the Menuyukti binary or setting `MENUYUKTI_DATA_DIR`.

Out of scope for this parent: public OpenAI directory submission that requires a public HTTPS MCP endpoint; optional session hooks to auto-start the binary can be a follow-up.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Repo contains a portable Menuyukti plugin package with plugin.json, skills/, and mcp.json pointing at the local Menuyukti MCP URL
- [x] #2 Plugin skills cover coaching plus the shipped visualisation skills (and offline venue-slot compute if included) without inventing analytics
- [x] #3 Owners can install the plugin from a documented local/repo marketplace path in ChatGPT Desktop or Codex CLI
- [x] #4 README/docs describe Desktop-only install, that the local MCP binary must still be running, and MENUYUKTI_DATA_DIR / workspace setup
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. TASK-31.1: scaffold plugins/menuyukti + sync script from .agents/skills.
2. TASK-31.2: repo marketplace.json.
3. TASK-31.3: README + install-and-run docs.
4. TASK-31.4: light SessionStart MCP reachability hook (no auto-launch).
5. Finalize parent.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Shipped portable Menuyukti Codex plugin under plugins/menuyukti with synced skills, localhost mcp.json, repo marketplace, docs, and optional MCP reachability hook. Verified Codex CLI marketplace add + plugin install. Subtasks 31.1–31.4 Done.
<!-- SECTION:FINAL_SUMMARY:END -->
