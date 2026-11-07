---
id: TASK-49.3
title: Add self-contained widget CSP regression smoke tests
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:07:00Z'
labels:
  - skybridge
  - widgets
  - testing
dependencies:
  - TASK-49.1
references:
  - apps/menuyukti/scripts/inline-widgets.mjs
  - apps/menuyukti/src/self-contained-views.ts
  - apps/menuyukti/src/self-contained-views.test.ts
parent_task_id: TASK-49
priority: high
type: task
ordinal: 125000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Codex CSP fix depends on a private Skybridge templateHelper monkey-patch and the inline-widgets post-build step. A Skybridge upgrade or a skipped build can silently regress to localhost asset URLs. There is no automated gate that resources/read HTML stays self-contained.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Automated test(s) assert production self-contained HTML for each analytics view (and hello-world) contains no http://127.0.0.1 or http://localhost Vite/assets/_skybridge URLs
- [x] #2 Test or documented CI step fails if a registered view is missing from dist/self-contained after build
- [x] #3 pnpm check (or an explicitly documented build+check path) covers the smoke
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extract shared expected view names + localhost-asset URL assertion used by the inliner smoke path.
2. Add vitest coverage: reject/accept CSP HTML samples; expected names match src/views (except shared); run inlineWidgets on a tiny fake Vite dist fixture and assert outputs.
3. When apps/menuyukti/dist/self-contained exists, also assert all expected views are present and CSP-clean.
4. Document build+check widget smoke path in docs/howtos/develop.md; keep pnpm check covering the fixture smoke without a full skybridge build.
5. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added scripts/self-contained-csp.mjs; inline-widgets uses assertSelfContainedHtml + assertSelfContainedDist after build. Vitest: view-name sync, CSP samples, fake Vite inline fixture, missing-dist failure, real dist when present. Documented build+check in develop.md. pnpm check 28 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
CSP regression smoke now lives in vitest via a shared self-contained-csp helper and a fake Vite inliner fixture (always in pnpm check). After a real build, tests also validate dist/self-contained for every registered view. Documented the build+check path in docs/howtos/develop.md.
<!-- SECTION:FINAL_SUMMARY:END -->
