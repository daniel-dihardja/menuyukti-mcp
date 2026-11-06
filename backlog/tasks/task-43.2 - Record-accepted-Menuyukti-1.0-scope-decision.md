---
id: TASK-43.2
title: Record accepted Menuyukti 1.0 scope decision
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T11:25:00Z'
updated_date: '2026-11-06T11:28:00Z'
labels: []
milestone: m-1
dependencies:
  - TASK-43.1
references:
  - backlog/decisions/decision-2 - Freeze-Menuyukti-1.0-product-scope.md
parent_task_id: TASK-43
priority: high
type: docs
ordinal: 97000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
decision-2 is still proposed and empty. Without an accepted in/out decision, 1.0 scope will keep drifting as new ideas appear.

Fill decision-2 Context / Decision / Consequences from the 1.0 specification: what is guaranteed in 1.0, what is explicitly deferred (post-1.0), and the release implication (semver 1.0.0 means this contract). Mark the decision accepted only after the matrix is written.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 decision-2 has filled Context, Decision, and Consequences sections
- [x] #2 Decision lists 1.0 in-scope capabilities and owner outcomes
- [x] #3 Decision lists explicit out-of-scope / deferred items for post-1.0
- [x] #4 decision-2 status is accepted
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Fill decision-2 Context/Decision/Consequences from doc-1 (CLI has no body update; write body sections then set status via CLI).
2. Mark decision-2 accepted.
3. Cross-link decision and specification.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Filled decision-2 Context/Decision/Consequences from doc-1 (CLI has no decision-body API; body written in place, status set via backlog decision edit). Marked accepted.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
decision-2 now records 1.0 in-scope outcomes/capabilities and explicit deferred items; status accepted. Spec cross-link preserved in doc-1.
<!-- SECTION:FINAL_SUMMARY:END -->
