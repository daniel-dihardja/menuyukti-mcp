---
id: TASK-14
title: Create owner README and dedicated docs space
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-03T15:02:00Z'
updated_date: '2026-11-03T15:03:00Z'
labels: []
dependencies: []
references:
  - README.md
  - docs/
type: docs
ordinal: 22000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The root README mixes restaurant-owner onboarding with tunnel setup, column mapping, and contributor commands. Owners need a short plain README; technical howtos need a dedicated docs/ space that is easy to browse.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Root README is short and owner-oriented, with links into docs/
- [x] #2 docs/ has an index that lists the available guides
- [x] #3 Tunnel setup lives in a dedicated howto under docs/
- [x] #4 Install, workspaces, custom POS columns, and develop guides live under docs/
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create docs/README.md as the documentation hub.
2. Move technical content into docs/howtos: install-and-run, connect-chatgpt-tunnel, workspaces, custom-pos-columns, develop.
3. Rewrite root README for restaurant owners (what it is, privacy, quick start, where docs live).
4. Soften workspaces/README.md to point owners at docs for deeper howtos.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added docs/ hub + howtos (install, tunnel, workspaces, custom columns, develop). Rewrote root README for restaurant owners with links into docs/. Softened workspaces/README to point at the howto.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created a dedicated docs/ space with an index and howtos. Root README is now short and owner-facing; tunnel and other technical setup live under docs/howtos/. Verified by file layout and cross-links.
<!-- SECTION:FINAL_SUMMARY:END -->
