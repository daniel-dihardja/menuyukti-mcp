---
id: TASK-27.1
title: Rewrite MCP server instructions as an owner-coaching playbook
status: To Do
assignee: []
created_date: '2026-11-05T08:50:00Z'
updated_date: '2026-11-05T08:50:00Z'
labels:
  - ux
  - onboarding
dependencies:
  - TASK-27.2
references:
  - crates/menuyukti-mcp/src/main.rs
parent_task_id: TASK-27
priority: high
type: enhancement
ordinal: 58000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The current INSTRUCTIONS string lists tools and requirements but does not steer first contact, goal→tool choice, or recovery. ChatGPT is the primary UX shell: without a short playbook, owners get tool-name dumps, wrong JSON-vs-widget choices, and weak onboard coaching. Rewrite instructions so the model leads with the 3-step ritual (server connected → drop files → set up then analyze), maps owner goals to tools, offers the demo workspace, gates COGS features in plain language, prefers render_* for charts, and keeps Hello World smoke-only.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 INSTRUCTIONS open with the 3-step owner ritual and first-contact behavior (get_setup_status and/or list_workspaces; offer demo-cafe-july-2026 with example asks when unclear)
- [ ] #2 INSTRUCTIONS include a goal→tool map for summarize, heatmaps, venue slot strength chart vs JSON, pair lift, menu engineering, weak items, and knowledge notes
- [ ] #3 INSTRUCTIONS state COGS gates in owner language and recovery (drop COGS Excel, list_workspace_files, write_workspace_config)
- [ ] #4 INSTRUCTIONS prefer render_venue_slot_strength for show/chart/heatmap asks and get_venue_slot_strength for numbers-only; Hello World remains smoke-only
- [ ] #5 HTTP or unit coverage still asserts server instructions are present and include the onboard ritual keywords after the rewrite
<!-- AC:END -->
