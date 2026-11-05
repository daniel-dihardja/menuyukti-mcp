---
id: TASK-27.5
title: Add Codex skill for Menuyukti MCP workspace coaching
status: To Do
assignee: []
created_date: '2026-11-05T08:54:00Z'
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
- [ ] #1 New skill under .agents/skills/ with SKILL.md frontmatter (name + description triggers) and agents/openai.yaml (display_name, short_description, default_prompt, allow_implicit_invocation)
- [ ] #2 Skill teaches the same 3-step ritual and goal→ask map as the MCP playbook; prefers MCP tools (including get_setup_status / onboard / analytics / render_*) when the server is connected
- [ ] #3 Skill does not duplicate menu-engineering, pair-lift, or heatmap math; for venue slot strength without MCP it points to menuyukti-venue-slot-strength
- [ ] #4 Skill states POS files stay local unless the user explicitly asks otherwise
- [ ] #5 README or docs mention how Codex users can add/use this skill (short pointer; not a second full howto)
<!-- AC:END -->
