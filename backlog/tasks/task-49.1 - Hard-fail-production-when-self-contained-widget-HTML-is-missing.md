---
id: TASK-49.1
title: Hard-fail production when self-contained widget HTML is missing
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:05:00Z'
labels:
  - skybridge
  - widgets
  - codex
dependencies: []
references:
  - apps/menuyukti/src/self-contained-views.ts
  - apps/menuyukti/src/index.ts
parent_task_id: TASK-49
priority: high
type: bug
ordinal: 123000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Codex Desktop CSP blocks Skybridge default production shells that import() assets from http://127.0.0.1. Menuyukti already inlines views into dist/self-contained/*.html and patches templateHelper.renderProduction, but a missing file only logs a warning and falls back to the CSP-broken shell — owners see a widget timeout with no actionable failure. Production must refuse that silent degradation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 When NODE_ENV=production (or mcp:prod / skybridge start path) and a registered view has no dist/self-contained/<name>.html, resources/read fails loudly instead of returning the default Skybridge asset shell
- [x] #2 Error or log message tells the operator to run the Menuyukti build / inline-widgets step
- [x] #3 Dev (pnpm mcp / Vite) behavior remains unchanged
- [x] #4 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a requireSelfContainedHtml helper that throws with an actionable build message when dist/self-contained/<name>.html is missing.
2. Patch renderProduction to use that helper when a view name is resolved (Skybridge only calls renderProduction under NODE_ENV=production); keep fallback to the original template only when the viewFile name cannot be parsed.
3. Extend self-contained-views.test.ts to cover the throw path and message content.
4. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
requireSelfContainedHtml throws with build/inline-widgets guidance; patched renderProduction no longer falls back to the Skybridge asset shell when a view name resolves. Dev still uses renderDevelopment (unchanged). pnpm check: 20 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Production resources/read now fails loudly when dist/self-contained/<view>.html is missing instead of serving the CSP-broken Skybridge asset shell. Error text points operators at pnpm --filter menuyukti build / pnpm mcp:prod. Verified with unit tests and pnpm check (20 tests).
<!-- SECTION:FINAL_SUMMARY:END -->
