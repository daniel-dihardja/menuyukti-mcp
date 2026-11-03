---
id: TASK-5
title: Add owner column-mapping config for custom POS exports
status: Done
assignee: []
created_date: '2026-11-03T11:19:00Z'
updated_date: '2026-11-03T12:19:00Z'
labels: []
dependencies: []
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Restaurant owners on unsupported POS systems still need analytics, but they cannot edit Python registries. A simple local map-my-columns config lets them point their export headers at the normalized line-item fields without sending data off-machine or waiting for a first-class normalizer.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Owners can place a column-mapping config under MENUYUKTI_DATA_DIR that maps export columns to normalized fields
- [x] #2 load_normalized_sales uses that config for custom/unrecognized exports and still path-gates file reads
- [x] #3 README documents how to create and use the mapping config
- [x] #4 Tests cover successful custom mapping and clear errors for incomplete maps
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add column-mapping loader/validator and generic Excel normalizer in menuyukti_gpt (JSON under MENUYUKTI_DATA_DIR). 2. Wire load_normalized_sales to use it for unrecognized/unsupported POS. 3. Add example config + README owner docs. 4. Add unit tests for success and incomplete maps. 5. Run pytest.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added JSON column mapping + generic normalizer fallback; README + example file; pytest mapping/sales/mcp → 14 passed.

Naming: package/module is now menuyukti-mcp / menuyukti_mcp (was menuyukti-gpt / menuyukti_gpt).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added owner pos-column-mapping.json support: unrecognized POS exports can map spreadsheet headers to the normalized line-item fields. load_normalized_sales falls back to that config when no built-in normalizer matches. Documented in README with an example file. Verified with unit tests (14 passed across mapping/sales/mcp).
<!-- SECTION:FINAL_SUMMARY:END -->
