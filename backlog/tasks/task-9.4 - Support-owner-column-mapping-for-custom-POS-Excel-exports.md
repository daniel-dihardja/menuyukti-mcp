---
id: TASK-9.4
title: Support owner column-mapping for custom POS Excel exports
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
labels: []
milestone: m-0
dependencies:
  - TASK-9.3
references:
  - src/menuyukti_gpt/column_mapping.py
  - pos-column-mapping.example.json
  - README.md
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
- [ ] #1 When known-POS detection fails, a valid mapping file under the data dir (or MENUYUKTI_COLUMN_MAPPING) normalizes the export to line items
- [ ] #2 Incomplete, invalid, or placeholder mapping configs produce clear errors
- [ ] #3 Mapped loads set a custom POS name suitable for tool responses
- [ ] #4 Tests cover successful mapping and validation failures
<!-- AC:END -->
