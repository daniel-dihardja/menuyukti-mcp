---
id: TASK-27.5
title: Add Codex skill for Menuyukti MCP workspace coaching
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-05T08:54:00Z'
updated_date: '2026-11-05T08:59:00Z'
labels:
  - ux
  - onboarding
  - codex
dependencies:
  - TASK-27.1
references:
  - .agents/skills/menuyukti-venue-slot-strength/SKILL.md
  - .agents/skills/menuyukti-venue-slot-strength/agents/openai.yaml
documentation:
  - README.md
parent_task_id: TASK-27
priority: medium
type: feature
ordinal: 62000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Codex users do not see MCP server INSTRUCTIONS unless they connect the MCP server, and many work from the agent with skills installed. A thin menuyukti-mcp (or menuyukti-workspace) Codex skill should mirror the owner-coaching playbook from TASK-27.1: 3-step ritual, goal→ask map, COGS gates, privacy — and prefer Menuyukti MCP tools when available. It must not reimplement analytics; keep menuyukti-venue-slot-strength as the offline/custom-mapping specialist and point there only when MCP is unavailable for slot-strength work.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 New skill under .agents/skills/ with SKILL.md frontmatter (name + description triggers) and agents/openai.yaml (display_name, short_description, default_prompt, allow_implicit_invocation)
- [x] #2 Skill teaches the same 3-step ritual and goal→ask map as the MCP playbook; prefers MCP tools (including get_setup_status / onboard / analytics / render_*) when the server is connected
- [x] #3 Skill does not duplicate menu-engineering, pair-lift, or heatmap math; for venue slot strength without MCP it points to menuyukti-venue-slot-strength
- [x] #4 Skill states POS files stay local unless the user explicitly asks otherwise
- [x] #5 README or docs mention how Codex users can add/use this skill (short pointer; not a second full howto)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add .agents/skills/menuyukti-mcp SKILL.md + agents/openai.yaml mirroring playbook.
2. Prefer MCP tools; point offline slot strength to existing skill.
3. README pointer for Codex users.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created .agents/skills/menuyukti-mcp/ with SKILL.md and openai.yaml; README Codex users section points to it.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added thin Codex coaching skill menuyukti-mcp that mirrors the MCP playbook, prefers MCP tools, defers offline slot strength to menuyukti-venue-slot-strength, and is linked from the README.
<!-- SECTION:FINAL_SUMMARY:END -->
