---
id: TASK-49.2
title: Resolve self-contained widget paths from package root
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
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
- [ ] #1 Self-contained HTML is found when the process cwd is the repo root or another directory, as long as apps/menuyukti/dist/self-contained exists relative to the package
- [ ] #2 Unit tests cover path resolution for at least cwd=package and cwd=repo-root cases
- [ ] #3 pnpm check passes
<!-- AC:END -->
