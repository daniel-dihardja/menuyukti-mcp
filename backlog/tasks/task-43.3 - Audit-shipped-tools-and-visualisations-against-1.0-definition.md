---
id: TASK-43.3
title: Audit shipped tools and visualisations against 1.0 definition
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T11:25:00Z'
updated_date: '2026-11-06T11:28:00Z'
labels: []
milestone: m-1
dependencies:
  - TASK-43.1
references:
  - README.md
documentation:
  - docs/howtos/workspaces.md
parent_task_id: TASK-43
priority: high
type: docs
ordinal: 98000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Most of the four capabilities already shipped as get_/render_ tools plus MCP Apps widgets and Codex skills, but 1.0 cannot be claimed by memory. A short audit against the written matrix prevents shipping a definition that the product does not meet.

Compare the 1.0 specification to the live tool list, widget assets, plugin skills, and owner howtos. Record pass/fail per capability. Create follow-up Backlog tasks only for must-have gaps; note deferred polish under the scope decision instead of silently expanding 1.0.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Audit note (on the parent task, specification doc, or linked task notes) covers all four capabilities with shipped evidence (tool names + viz path)
- [x] #2 Any must-have gap is filed as a Backlog task under milestone Menuyukti 1.0
- [x] #3 Non-blocking gaps are listed as deferred in decision-2 or the specification, not left ambiguous
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Verify each of four capabilities: get_ tool, render_ tool, widget asset, preview fixture, owner docs, Codex skill where required.
2. Record audit in doc-1 checklist + task notes.
3. File must-have gaps only; otherwise mark deferred in decision/spec (pair-lift Codex HTML skill already deferred).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Audit: all four capabilities Pass (get_+render_+widget+preview+owner docs). No must-have gaps filed. Deferred polish noted in doc-1: missing render_venue_slot_strength bullet in workspaces tool list; pair-lift Codex HTML skill already out of scope in decision-2.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Filled doc-1 audit checklist with shipped evidence. All four 1.0 capabilities pass. No follow-up must-have tasks created; non-blocking polish deferred per decision-2.
<!-- SECTION:FINAL_SUMMARY:END -->
