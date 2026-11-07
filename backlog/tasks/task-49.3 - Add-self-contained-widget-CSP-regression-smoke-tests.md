---
id: TASK-49.3
title: Add self-contained widget CSP regression smoke tests
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
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
- [ ] #1 Automated test(s) assert production self-contained HTML for each analytics view (and hello-world) contains no http://127.0.0.1 or http://localhost Vite/assets/_skybridge URLs
- [ ] #2 Test or documented CI step fails if a registered view is missing from dist/self-contained after build
- [ ] #3 pnpm check (or an explicitly documented build+check path) covers the smoke
<!-- AC:END -->
