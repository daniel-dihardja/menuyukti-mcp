---
id: TASK-50
title: Add Biome lint, skill-sync check, and CI widget build smoke
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T04:47:00Z'
updated_date: '2026-11-07T04:49:00Z'
labels:
  - ci
  - dx
  - quality
dependencies: []
references:
  - scripts/check.sh
  - .github/workflows/ci.yml
  - scripts/sync-plugin-skills.sh
priority: medium
type: chore
ordinal: 137000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Fold high-value low-drama quality tooling into the Menuyukti gate: Biome lint/format for apps/menuyukti, a CI/local check that plugin skills match .agents/skills, and a main-branch (or push-to-main) self-contained widget build smoke so real dist/self-contained CSP validation runs beyond the fixture path.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Biome is configured; pnpm lint / format:check (or equivalent) run in scripts/check.sh and CI
- [x] #2 pnpm check fails when plugins/menuyukti/skills drift from .agents/skills
- [x] #3 CI on main (or dedicated job) runs menuyukti build + asserts self-contained widgets
- [x] #4 develop.md and typescript-quality skill document the expanded gate
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add Biome at repo/app, fix any easy lint findings, wire lint into check.sh.
2. Add scripts/check-plugin-skills-sync.sh and call from check.sh.
3. Extend CI with main-branch build smoke for self-contained HTML.
4. Update docs/skill; finalize task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Biome 2.3.8 at repo root; lint/format in check.sh. Skill sync script ignores agents/assets. CI main-only widgets-build-smoke job. Fixed a11y/implicit-any findings while adopting Biome. pnpm check green (38 tests).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added Biome to the quality gate, a plugin-skills drift check, and a main-branch self-contained widget build smoke in CI. Local/PR gate is now lint → skills → typecheck → test.
<!-- SECTION:FINAL_SUMMARY:END -->
