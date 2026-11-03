---
id: TASK-11
title: Group demo sales and COGS into a dataset directory
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T14:48:00Z'
updated_date: '2026-11-03T14:49:00Z'
labels: []
dependencies: []
references:
  - sales-reports/
  - .gitignore
  - README.md
type: chore
ordinal: 19000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Flat demo Excel files make sales and COGS look unrelated, and owners have no clear place to drop a paired venue-period dataset. A default demo directory under sales-reports/ holds both files so listing and docs show one analysis bundle.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Default demo lives under sales-reports/demo-cafe-july-2026/ with sales and COGS Excel files
- [x] #2 Git tracks the demo directory files and still ignores personal uploads under sales-reports/
- [x] #3 README and sales-reports docs use the nested demo paths
- [x] #4 Tests and scripts that load the demo sales file use the nested relative path
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create sales-reports/demo-cafe-july-2026/ and move demo sales + COGS Excel into it as sales.xlsx and cogs.xlsx.
2. Update .gitignore so the demo directory files stay tracked while other uploads stay ignored.
3. Update root README, sales-reports README, fixtures README, and mcp-inspect comment to nested paths.
4. Update Rust tests that load the demo sales file to the nested relative path.
5. Run make check and verify listing still finds nested Excel files.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Moved demo Excel into sales-reports/demo-cafe-july-2026/{sales,cogs}.xlsx; updated gitignore, READMEs, mcp-inspect comment, and Rust test paths. make check passed (fmt/clippy/tests).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Grouped the default demo into sales-reports/demo-cafe-july-2026/ with sales.xlsx and cogs.xlsx. Docs and tests now use the nested sales path; gitignore still tracks only that demo dataset and ignores personal uploads. Verified with make check.
<!-- SECTION:FINAL_SUMMARY:END -->
