---
id: TASK-9.4
title: Support owner column-mapping for custom POS Excel exports
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.3
references:
  - pos-column-mapping.example.json
  - README.md
  - src/menuyukti_mcp/
  - src/menuyukti_mcp/column_mapping.py
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Many restaurants use POS exports Menuyukti does not recognize. Owners already configure pos-column-mapping.json (or MENUYUKTI_COLUMN_MAPPING) so those files still load locally without sending spreadsheets to a vendor normalizer service.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 When known-POS detection fails, a valid mapping file under the data dir (or MENUYUKTI_COLUMN_MAPPING) normalizes the export to line items
- [x] #2 Incomplete, invalid, or placeholder mapping configs produce clear errors
- [x] #3 Mapped loads set a custom POS name suitable for tool responses
- [x] #4 Tests cover successful mapping and validation failures
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add ColumnMappingConfig load/validate for pos-column-mapping.json and MENUYUKTI_COLUMN_MAPPING.
2. Implement normalize_mapped_excel → Vec<LineItem>.
3. Hook fallback into load_normalized_sales / detect_and_normalize when POS unknown.
4. Unit tests for success, incomplete mapping, and missing headers.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added column_mapping module + fallback in load_normalized_sales. Tests: mapped custom xlsx → pos my-pos; incomplete mapping error; placeholder mapping error.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Custom POS exports load via pos-column-mapping.json / MENUYUKTI_COLUMN_MAPPING when auto-detect fails. Mapping validation rejects incomplete and placeholder configs; successful loads expose a slug POS name.
<!-- SECTION:FINAL_SUMMARY:END -->
