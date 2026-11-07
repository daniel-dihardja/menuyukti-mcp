---
id: TASK-49.13
title: Spike Skybridge upstream support for self-contained production HTML
status: To Do
assignee: []
created_date: '2026-11-07T04:00:00Z'
labels:
  - skybridge
  - spike
dependencies:
  - TASK-49.3
references:
  - apps/menuyukti/src/self-contained-views.ts
  - https://docs.skybridge.tech
parent_task_id: TASK-49
priority: low
type: spike
ordinal: 135000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Menuyukti patches a private Skybridge templateHelper.renderProduction API because the default production shell assumes the host iframe can fetch serverUrl/assets — Codex Desktop CSP cannot. Long-term, an official Skybridge hook or inline mode would let Menuyukti drop the monkey-patch. Spike whether upstream already has an option, an issue tracker path, or a maintainable extension point; record GO/NO-GO on keeping the patch vs contributing upstream.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Written notes describe current Skybridge production HTML contract vs Codex CSP constraint
- [ ] #2 Notes list upstream options (existing API, feature request, or keep patch) with a recommendation
- [ ] #3 If keep-patch is recommended, notes list the version pin / smoke-test expectations already covered by sibling tasks
<!-- AC:END -->
