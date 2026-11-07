---
id: TASK-49.2
title: Resolve self-contained widget paths from package root
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:06:00Z'
labels:
  - skybridge
  - widgets
dependencies:
  - TASK-49.1
references:
  - apps/menuyukti/src/self-contained-views.ts
parent_task_id: TASK-49
priority: high
type: bug
ordinal: 124000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
selfContainedHtmlPath and readSelfContainedHtml default to process.cwd(). That works only when the process is started with cwd apps/menuyukti. Plugin ZIP / stdio launchers and monorepo starts from the repo root can miss dist/self-contained and trigger the CSP fallback. Paths need to be anchored to the Menuyukti package (or known dist location), not the caller cwd.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Self-contained HTML is found when the process cwd is the repo root or another directory, as long as apps/menuyukti/dist/self-contained exists relative to the package
- [x] #2 Unit tests cover path resolution for at least cwd=package and cwd=repo-root cases
- [x] #3 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Resolve apps/menuyukti package root from import.meta.url (directory containing package.json / this module), not process.cwd().
2. Default selfContainedHtmlPath / readSelfContainedHtml / requireSelfContainedHtml to that package root while still accepting an explicit root override for tests.
3. Add tests covering resolution relative to package root even when cwd is elsewhere.
4. Run pnpm check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
menuyuktiPackageRoot walks from import.meta.url to package.json name menuyukti. Defaults for path/read/require use that root; cwd package and repo-root cases covered. pnpm check 23 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Self-contained widget HTML paths are anchored to the menuyukti package root via import.meta.url, so production serving works when cwd is the repo root or elsewhere. Verified with cwd=package and cwd=repo-root tests; pnpm check passes (23 tests).
<!-- SECTION:FINAL_SUMMARY:END -->
