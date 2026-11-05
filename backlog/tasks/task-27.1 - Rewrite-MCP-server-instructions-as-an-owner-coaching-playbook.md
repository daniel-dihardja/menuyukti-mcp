---
id: TASK-27.1
title: Rewrite MCP server instructions as an owner-coaching playbook
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-05T08:50:00Z'
updated_date: '2026-11-05T08:59:00Z'
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
- [x] #1 INSTRUCTIONS open with the 3-step owner ritual and first-contact behavior (get_setup_status and/or list_workspaces; offer demo-cafe-july-2026 with example asks when unclear)
- [x] #2 INSTRUCTIONS include a goal→tool map for summarize, heatmaps, venue slot strength chart vs JSON, pair lift, menu engineering, weak items, and knowledge notes
- [x] #3 INSTRUCTIONS state COGS gates in owner language and recovery (drop COGS Excel, list_workspace_files, write_workspace_config)
- [x] #4 INSTRUCTIONS prefer render_venue_slot_strength for show/chart/heatmap asks and get_venue_slot_strength for numbers-only; Hello World remains smoke-only
- [x] #5 HTTP or unit coverage still asserts server instructions are present and include the onboard ritual keywords after the rewrite
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Rewrite INSTRUCTIONS as owner playbook (ritual, first contact with get_setup_status, goal→tool, COGS, widgets).
2. Assert playbook keywords via initialize in HTTP tests.
3. make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Rewrote INSTRUCTIONS; initialize HTTP test asserts get_setup_status, demo-cafe-july-2026, write_workspace_config, render_venue_slot_strength, list_workspace_files.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
MCP server instructions are now an owner-coaching playbook: 3-step ritual, first-contact setup status, goal→tool map, COGS recovery, chart vs JSON preference. Verified via initialize instructions assertions in mcp_http; make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
