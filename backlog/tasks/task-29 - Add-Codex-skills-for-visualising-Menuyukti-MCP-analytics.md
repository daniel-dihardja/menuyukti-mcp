---
id: TASK-29
title: Add Codex skills for visualising Menuyukti MCP analytics
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-06T03:54:00Z'
updated_date: '2026-11-06T03:58:00Z'
labels:
  - codex
  - ux
  - analytics
dependencies: []
references:
  - ../v3/apps/web/components/chat/visualizations/venue-slot-strength-card.tsx
  - ../v3/apps/web/components/chat/visualizations/menu-item-heatmap-card.tsx
  - ../v3/apps/web/lib/chat/format-visualization-for-chat.ts
  - ../v3/apps/graphql/knowledge/kpi-venue-slot-strength.md
  - ../v3/apps/graphql/knowledge/kpi-menu-heatmap.md
  - .agents/skills/menuyukti-mcp/SKILL.md
  - .agents/skills/menuyukti-venue-slot-strength/SKILL.md
documentation:
  - README.md
  - docs/howtos/workspaces.md
priority: high
type: feature
ordinal: 64000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Codex in ChatGPT Desktop often cannot mount MCP Apps iframes, so owners who ask to "show" busy times or dish timing get raw JSON or prose instead of the heatmaps they already know from Menuyukti v3 chat (`venue_slot_strength_heatmap`, `menu_item_heatmap`).

Analytics data already exists via `get_venue_slot_strength` and `get_menu_heatmaps`. What is missing is Codex presentation skills that teach agents how to draw those payloads in chat (markdown-first), aligned with v3 visual semantics, without recomputing analytics or replacing the ChatGPT MCP Apps widget path.

Ship separate presentation skills per chart family under `.agents/skills/`, keep `menuyukti-mcp` as the coaching router, and keep `menuyukti-venue-slot-strength` as the offline compute specialist.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Parent completes only when venue-slot and menu-heatmap Codex visualisation skills exist under `.agents/skills/` with openai.yaml packaging
- [x] #2 Skills fetch MCP JSON (or documented offline fallback for venue slot only) and never invent demand indexes or unit counts
- [x] #3 `menuyukti-mcp` goal→tool map routes show/chart/heatmap asks to the matching visualisation skill after the data tool
- [x] #4 README or owner docs briefly point Codex users at the new visualisation skills
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement TASK-29.1 venue-slot Codex visualisation skill + wire menuyukti-mcp.
2. Implement TASK-29.2 menu-heatmap Codex visualisation skill + wire menuyukti-mcp.
3. Update README / workspaces docs pointers for Codex users.
4. Finalize subtasks then parent against ACs.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Shipped two Codex visualisation skills under .agents/skills/ (venue slot strength + menu heatmaps) with openai.yaml packaging and v3-aligned markdown recipes. menuyukti-mcp goal map routes show/chart asks to get_* then the matching skill; render_venue_slot_strength remains ChatGPT-widget-only. README and docs/howtos/workspaces.md document the skills for Codex users. Subtasks 29.1 and 29.2 Done.
<!-- SECTION:FINAL_SUMMARY:END -->
