---
id: TASK-44.5
title: Port Excel normalize and sales load pipeline to TypeScript
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:30:00Z'
updated_date: '2026-11-06T17:59:00Z'
labels:
  - skybridge
  - migration
milestone: m-2
dependencies:
  - TASK-44.4
references:
  - crates/menuyukti/src/excel.rs
  - crates/menuyukti/src/normalize
  - crates/menuyukti/src/load.rs
  - crates/menuyukti/src/column_mapping.rs
documentation:
  - docs/howtos/custom-pos-columns.md
parent_task_id: TASK-44
priority: high
type: feature
ordinal: 105000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Analytics tools need the same normalized line items the Rust library produces from POS Excel (and optional COGS / column mapping). Without a faithful load pipeline, tool parity cannot be verified on demo fixtures.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 TS pipeline loads workspace sales Excel into normalized line items comparable to the Rust path for demo-cafe-july-2026
- [x] #2 Optional COGS and custom column mapping are supported at parity needed for menu engineering / weak items
- [x] #3 summarize_local_sales and list_normalized_sales return usable results on the demo workspace
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Port LineItem + Excel sheet read (xlsx) and POS detect/normalize pipeline from crates/menuyukti into apps/menuyukti/src/lib.
2. Port load_normalized_sales / resolve_sales_path, optional COGS parse, and column_mapping support needed for menu engineering later.
3. Register summarize_local_sales and list_normalized_sales on the Skybridge server.
4. Compare demo-cafe-july-2026 counts/revenue against Rust (or fixture expectations) with vitest + MCP smoke.
5. Finalize TASK-44.5 ACs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ported Excel read (SheetJS), POS detect, ESB/Moka/Quino normalize, column-mapping fallback, COGS parse, load_normalized_sales, summarize/list tools. Fixed blank-row skip_rows parity with calamine. Vitest: 8 pass including demo-cafe baselines (1748 qty, ~29.8M revenue, 1441 rows, Avocado Chocolate COGS 8000). MCP smoke on :8793 matched.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
TypeScript sales pipeline now loads demo-cafe-july-2026 at Rust parity (ESB normalize, summarize_local_sales, list_normalized_sales). Optional COGS load and pos-column-mapping.json path are in place for menu-engineering later. Verified with vitest and live MCP calls.
<!-- SECTION:FINAL_SUMMARY:END -->
