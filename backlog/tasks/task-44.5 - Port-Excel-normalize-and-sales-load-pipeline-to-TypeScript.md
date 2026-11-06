---
id: TASK-44.5
title: Port Excel normalize and sales load pipeline to TypeScript
status: To Do
assignee: []
created_date: '2026-11-06T17:30:00Z'
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
- [ ] #1 TS pipeline loads workspace sales Excel into normalized line items comparable to the Rust path for demo-cafe-july-2026
- [ ] #2 Optional COGS and custom column mapping are supported at parity needed for menu engineering / weak items
- [ ] #3 summarize_local_sales and list_normalized_sales return usable results on the demo workspace
<!-- AC:END -->
