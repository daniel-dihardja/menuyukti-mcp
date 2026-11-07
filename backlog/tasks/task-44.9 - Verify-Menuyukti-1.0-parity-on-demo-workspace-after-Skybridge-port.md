---
id: TASK-44.9
title: Verify Menuyukti 1.0 parity on demo workspace after Skybridge port
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T18:35:00Z'
labels:
  - skybridge
  - migration
  - qa
milestone: m-2
dependencies:
  - TASK-44.8
references:
  - backlog/decisions/decision-2 - Freeze-Menuyukti-1.0-product-scope.md
documentation:
  - backlog/docs/product/doc-1 - Menuyukti-1.0-product-definition.md
  - backlog/docs/product/doc-2 - Skybridge-migration-plan.md
parent_task_id: TASK-44
priority: high
type: task
ordinal: 109000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Cutover is unsafe until someone checks the frozen 1.0 contract end-to-end on the known fixture. Gaps must be filed or fixed before retiring Rust.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 demo-cafe-july-2026 completes the owner ritual (setup → summarize → each of the four capability show paths) on the Skybridge server
- [x] #2 Sales-only and sales+COGS behaviours required by decision-2 are checked; gaps are fixed or tracked as explicit follow-ups
- [x] #3 A short parity note is recorded on TASK-44 or doc-2
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Start Skybridge MCP on a free port with MENUYUKTI_DATA_DIR=workspaces.
2. Run owner ritual + four capability show paths (render_*) on demo-cafe-july-2026.
3. Verify sales-only vs sales+COGS behaviours (heatmaps matrix_category, pair-lift scope, weak/menu engineering).
4. Record parity note on doc-2 and TASK-44; fix or track any gaps; finalize ACs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Parity suite 26/26 PASS on Skybridge :8792 + vitest 13/13. Ritual: setup→summarize→render venue/menu/pair/weak. Sales-only vs COGS checked. Gaps: none blocking. Parity section added to doc-2; note on TASK-44.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Verified Menuyukti 1.0 parity on demo-cafe-july-2026 via Skybridge MCP: owner ritual, all four render_* show paths, sales-only vs sales+COGS behaviours, path gating. 26/26 automated MCP checks + vitest green. No blocking gaps; recorded on doc-2 and TASK-44. Safe to start TASK-44.10.
<!-- SECTION:FINAL_SUMMARY:END -->
