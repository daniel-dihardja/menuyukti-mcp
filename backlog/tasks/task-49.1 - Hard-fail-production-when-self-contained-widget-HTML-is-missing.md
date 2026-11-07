---
id: TASK-49.1
title: Hard-fail production when self-contained widget HTML is missing
status: To Do
assignee: []
created_date: '2026-11-07T03:59:00Z'
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
- [ ] #1 When NODE_ENV=production (or mcp:prod / skybridge start path) and a registered view has no dist/self-contained/<name>.html, resources/read fails loudly instead of returning the default Skybridge asset shell
- [ ] #2 Error or log message tells the operator to run the Menuyukti build / inline-widgets step
- [ ] #3 Dev (pnpm mcp / Vite) behavior remains unchanged
- [ ] #4 pnpm check passes
<!-- AC:END -->
