---
id: TASK-44
title: Migrate Menuyukti MCP from Rust to Skybridge
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:29:00Z'
updated_date: '2026-11-06T18:37:00Z'
labels:
  - skybridge
  - migration
  - typescript
milestone: m-2
dependencies: []
references:
  - https://docs.skybridge.tech/get-started/introduction.md
  - https://docs.skybridge.tech/guides/migrate.md
  - >-
    backlog/decisions/decision-3 -
    Migrate-Menuyukti-MCP-stack-from-Rust-to-Skybridge.md
  - backlog/decisions/decision-2 - Freeze-Menuyukti-1.0-product-scope.md
documentation:
  - backlog/docs/product/doc-2 - Skybridge-migration-plan.md
  - backlog/docs/product/doc-1 - Menuyukti-1.0-product-definition.md
  - README.md
  - plugins/menuyukti/README.md
priority: high
type: feature
ordinal: 100000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Primary maintainer is not a Rust developer; Rust was an experiment (decision-1). Widget and MCP Apps work is slower than needed, while Menuyukti 1.0 scope is already frozen (decision-2): local Desktop MCP, POS under MENUYUKTI_DATA_DIR, four analytics capabilities with get_/render_ tools.

Migrate the MCP server and MCP Apps views to Skybridge (TypeScript/React). Keep local privacy and tool/owner-outcome parity. Retire the Rust crates after a documented cutover — avoid a long-lived hybrid.

Proposed stack decision: decision-3. Product contract stays decision-2 / doc-1.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 An accepted backlog decision records the Skybridge/TS stack choice and supersedes decision-1 for new work
- [x] #2 TypeScript Skybridge MCP exposes Menuyukti 1.0 tools and visualisations with owner-outcome parity (decision-2 / doc-1)
- [x] #3 Sales data remains local under MENUYUKTI_DATA_DIR with path gating equivalent to the Rust server
- [x] #4 Codex/ChatGPT Desktop plugin and owner docs run against the Skybridge server (no required Rust binary)
- [x] #5 Rust crates are retired or clearly marked obsolete after parity cutover; CI quality gate matches the new stack
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
TASK-44.9 parity (2026-11-06): Skybridge demo-cafe ritual PASS — summarize 1748/29.8M; render_* 35 slots / 98 heatmaps / stars pair-lift / 64 weak; sales-only vs COGS behaviours OK; path gate OK. Details on doc-2. No blocking gaps before TASK-44.10.

TASK-44.10 complete: Rust crates removed; make check/CI are TypeScript. Migration finished.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Migrated Menuyukti MCP from Rust to Skybridge (TypeScript/React): local Desktop MCP under MENUYUKTI_DATA_DIR with 1.0 tool/view parity on demo-cafe, plugin/docs on make mcp (Node 24+), and Rust crates/CI retired. decision-3 supersedes decision-1.
<!-- SECTION:FINAL_SUMMARY:END -->
