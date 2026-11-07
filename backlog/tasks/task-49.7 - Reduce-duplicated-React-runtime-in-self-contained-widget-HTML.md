---
id: TASK-49.7
title: Reduce duplicated React runtime in self-contained widget HTML
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:13:00Z'
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
- [x] #1 Self-contained production HTML still has no localhost Vite/assets/_skybridge URLs (Codex CSP contract preserved)
- [x] #2 Total size of dist/self-contained/*.html is materially lower than the pre-change baseline, or notes document why further sharing is blocked
- [x] #3 All registered views still mount under pnpm mcp:prod
- [x] #4 pnpm check passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Measure current dist/self-contained HTML sizes (or rebuild baseline).
2. Enable esbuild minify (and drop legal comments) in inline-widgets while keeping one HTML blob per view with no localhost asset URLs.
3. Re-measure; if cross-view React sharing remains blocked by the single-file Codex contract, document that in notes/develop.md.
4. Run pnpm check and assertSelfContainedDist smoke.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Baseline ~3.5MB (5×~730KB). Enabled esbuild minify+treeShaking+legalComments none. After rebuild: ~2.3MB (5×~485KB), ~34% smaller. Documented that cross-view React sharing stays blocked by single-file Codex CSP. pnpm check green; assertSelfContainedDist passes on rebuilt blobs.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Self-contained widget HTML is minified in the post-build inliner (~3.5MB → ~2.3MB total) while remaining CSP-clean single blobs. True React sharing across views is still blocked by the one-HTML-per-view Codex contract and is documented in develop.md.
<!-- SECTION:FINAL_SUMMARY:END -->
