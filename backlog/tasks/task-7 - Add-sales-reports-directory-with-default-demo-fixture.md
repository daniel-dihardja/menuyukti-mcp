---
id: TASK-7
title: Add sales-reports directory with default demo fixture
status: Done
assignee: []
created_date: '2026-11-03T11:25:00Z'
updated_date: '2026-11-03T11:26:00Z'
labels: []
dependencies: []
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners need one obvious local folder for POS Excel uploads, and a committed default sample with a clear name so demos work without hunting through package fixtures. Pointing MENUYUKTI_DATA_DIR at that folder keeps tool paths simple and keeps personal exports out of git.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 sales-reports/ is documented as the place to put POS Excel files
- [x] #2 A meaningfully named default anonymized fixture is committed under sales-reports/
- [x] #3 Local MCP defaults MENUYUKTI_DATA_DIR to sales-reports/ while ignoring personal uploads in git
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Commit sales-reports/demo-cafe-sales-july-2026.xlsx and README; adjust gitignore. 2. Default MENUYUKTI_DATA_DIR to sales-reports in mcp-inspect. 3. Update anonymize script + docs/tests; remove long-named package copy if redundant.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
sales-reports/ is default MENUYUKTI_DATA_DIR; committed demo-cafe-sales-july-2026.xlsx; personal uploads gitignored; pytest sales 10 passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added sales-reports/ as the owner Excel folder with committed demo-cafe-sales-july-2026.xlsx default sample. mcp-inspect now defaults MENUYUKTI_DATA_DIR there; git keeps only the demo + README.
<!-- SECTION:FINAL_SUMMARY:END -->
