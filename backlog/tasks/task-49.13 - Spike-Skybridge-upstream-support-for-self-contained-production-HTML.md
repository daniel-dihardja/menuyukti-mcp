---
id: TASK-49.13
title: Spike Skybridge upstream support for self-contained production HTML
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T04:00:00Z'
updated_date: '2026-11-07T04:20:00Z'
labels:
  - skybridge
  - spike
dependencies:
  - TASK-49.3
references:
  - >-
    backlog/docs/engineering/doc-4 -
    Skybridge-self-contained-production-HTML-spike.md
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
- [x] #1 Written notes describe current Skybridge production HTML contract vs Codex CSP constraint
- [x] #2 Notes list upstream options (existing API, feature request, or keep patch) with a recommendation
- [x] #3 If keep-patch is recommended, notes list the version pin / smoke-test expectations already covered by sibling tasks
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review Skybridge production HTML / templateHelper / docs for an official self-contained or asset-inline option.
2. Compare against Menuyukti patch + Codex CSP constraint.
3. Record spike notes with options, recommendation (keep patch vs upstream), and pin/smoke-test expectations from TASK-49.1/49.3.
4. Mark spike Done with GO/NO-GO recommendation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Spike complete. Stock Skybridge 2.1 production HTML always loads /assets from serverUrl; docs/CSP/Deploy and Workers work assume a reachable asset host. No official self-contained/inline API. Codex Desktop CSP blocks localhost asset fetches. Recommendation: KEEP patch + inliner (GO); do not wait on upstream (optional non-blocking feature request). Notes: doc-4. Pin/smokes: skybridge ^2.1.0 + TASK-49.1 hard-fail + TASK-49.3 CSP tests.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Spike: Skybridge 2.1 has no official self-contained production HTML; stock shell needs serverUrl/assets. For Codex Desktop CSP, keep Menuyukti inline-widgets + templateHelper.renderProduction patch (GO). Pin skybridge ^2.1.0; rely on TASK-49.1 hard-fail and TASK-49.3 CSP smokes on upgrades. Full writeup: doc-4.
<!-- SECTION:FINAL_SUMMARY:END -->
