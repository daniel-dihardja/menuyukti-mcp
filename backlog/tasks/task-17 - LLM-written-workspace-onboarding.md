---
id: TASK-17
title: LLM-written workspace onboarding
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T16:39:00Z'
updated_date: '2026-11-03T16:44:00Z'
labels: []
dependencies: []
references:
  - docs/howtos/workspaces.md
  - crates/menuyukti/src/paths.rs
ordinal: 25000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owners should only drop sales and COGS Excel into a folder. ChatGPT needs to invent the structured workspace config and then analyze without asking owners to edit JSON. Today workspace.json is hand-written docs only and tools still require explicit sales paths.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Owner onboarding docs describe drop-two-files then ask ChatGPT to set up the workspace
- [x] #2 ChatGPT can create a path-gated workspace config from listed files without the owner editing it by hand
- [x] #3 Analytics can run against a workspace folder using the written sales role path
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Owners drop sales+COGS; ChatGPT writes path-gated workspace.yaml; analytics accept workspace folder paths. YAML migrate (TASK-18), write tool (17.1), sales resolve (17.2), and owner docs (17.3) are done. make check green.
<!-- SECTION:FINAL_SUMMARY:END -->
