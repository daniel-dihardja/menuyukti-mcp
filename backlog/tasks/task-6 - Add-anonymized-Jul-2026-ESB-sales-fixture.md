---
id: TASK-6
title: Add anonymized Jul 2026 ESB sales fixture
status: Done
assignee: []
created_date: '2026-11-03T11:23:00Z'
updated_date: '2026-11-03T11:24:00Z'
labels: []
dependencies: []
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The local JUL 2026 ESB export is realistic but contains identifiable menu and venue names, so it cannot be committed as-is. An anonymized copy with real prices gives the plugin a default real-world fixture for demos and local MCP testing without leaking the restaurant identity.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Committed ESB fixture keeps original prices, quantities, and order timing while anonymizing menu item names
- [x] #2 Venue/brand and other identifying labels in the workbook are scrubbed for safe public use
- [x] #3 Fixture normalizes successfully with the existing ESB loader and is documented in the fixtures README
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Write a fixture script that copies the JUL 2026 ESB workbook, anonymizes menu names (and venue/brand labels) while preserving prices/qty/times. 2. Generate committed esb fixture. 3. Document in fixtures README. 4. Smoke-test with ESB load_normalized_sales / summarize.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Generated JUL_2026_ANON.xlsx (1445 rows, 102 menus); prices/qty preserved; venue scrubbed; ESB loader smoke + unit test passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added anonymized Jul 2026 ESB fixture with real prices and fictional menu/venue labels, plus a regenerator script and docs. Verified price/qty preservation, identity scrub, and ESB normalization.
<!-- SECTION:FINAL_SUMMARY:END -->
