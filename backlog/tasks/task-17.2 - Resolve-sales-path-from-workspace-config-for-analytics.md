---
id: TASK-17.2
title: Resolve sales path from workspace config for analytics
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T16:40:00Z'
updated_date: '2026-11-03T16:44:00Z'
labels: []
dependencies:
  - TASK-18
references:
  - crates/menuyukti/src/load.rs
  - crates/menuyukti-mcp/src/main.rs
parent_task_id: TASK-17
ordinal: 28000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After config exists, owners and ChatGPT still must type sales.xlsx paths. Analytics should accept a workspace folder and use files.sales from workspace.yaml.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 summarize_local_sales (and sibling sales tools) accept a workspace folder path and resolve files.sales via workspace.yaml
- [x] #2 Explicit Excel file paths still work as today
- [x] #3 Missing sales role or missing config returns a clear error
- [x] #4 Tests cover workspace-folder resolution on the demo cafe
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Teach load_normalized_sales (via resolve_sales_path) to accept a workspace folder and resolve files.sales from workspace.yaml.
2. Keep explicit .xlsx/.xls paths working unchanged.
3. Return clear errors for missing config/sales role; add analytics/integration coverage on demo-cafe folder path.
4. Update PathArgs tool descriptions; run make check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
resolve_sales_path accepts workspace folders via workspace.yaml files.sales; Excel paths unchanged. Demo folder path covered in analytics + mcp_http.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Sales tools accept a workspace folder and resolve files.sales from workspace.yaml; explicit Excel paths still work. Missing config errors clearly. Verified on demo-cafe.
<!-- SECTION:FINAL_SUMMARY:END -->
