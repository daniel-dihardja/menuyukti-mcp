---
id: TASK-49.7
title: Reduce duplicated React runtime in self-contained widget HTML
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
labels:
  - skybridge
  - widgets
  - performance
dependencies: []
references:
  - apps/menuyukti/scripts/inline-widgets.mjs
parent_task_id: TASK-49
priority: medium
type: enhancement
ordinal: 129000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Vite already emits shared chunks, but inline-widgets.mjs esbuild-bundles each view entry independently. Production blobs are ~730KB each (~3.6MB for five views) with React duplicated N times. That hurts plugin ZIP size and resources/read cost for Desktop.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Self-contained production HTML still has no localhost Vite/assets/_skybridge URLs (Codex CSP contract preserved)
- [ ] #2 Total size of dist/self-contained/*.html is materially lower than the pre-change baseline, or notes document why further sharing is blocked
- [ ] #3 All registered views still mount under pnpm mcp:prod
- [ ] #4 pnpm check passes
<!-- AC:END -->
