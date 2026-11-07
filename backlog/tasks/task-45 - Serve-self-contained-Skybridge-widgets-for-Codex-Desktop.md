---
id: TASK-45
title: Serve self-contained Skybridge widgets for Codex Desktop
status: Done
assignee:
  - '@agent'
created_date: '2026-11-06T19:23:00Z'
updated_date: '2026-11-06T19:26:00Z'
labels: []
dependencies: []
references:
  - apps/menuyukti
  - docs/howtos/install-and-run.md
priority: high
type: bug
ordinal: 111000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Skybridge MCP Apps views return HTML that import()s JS/CSS from http://127.0.0.1. Codex Desktop CSP blocks those fetches and the widget times out. Rust on main served one self-contained HTML resource. Restore that contract for production Desktop while keeping Skybridge React views.

Happy path for Codex: production build + self-contained resources/read HTML (no localhost asset URLs).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Production resources/read for each render_* view returns HTML with no http://127.0.0.1 or http://localhost asset URLs
- [x] #2 Venue slot / pair lift / menu heatmaps / weak items / hello-world self-contained HTML builds from skybridge build
- [x] #3 pnpm mcp:prod serves inlined widgets; DevTools/pnpm mcp may still use Vite (documented)
- [x] #4 pnpm check passes; docs mention Codex Desktop uses self-contained production widgets
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Post-build script: esbuild-bundle each Vite view entry + inline CSS into dist/self-contained/<name>.html
2. Patch Skybridge templateHelper.renderProduction at startup to serve those HTML files
3. Wire script into apps/menuyukti build; mcp:prod rebuilds if needed
4. Add unit test for inliner; update install/develop docs for Codex
5. Verify resources/read has no localhost asset URLs
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Verified: skybridge start resources/read for venue-slot-strength returns ~733KB HTML with <!DOCTYPE and no 127.0.0.1/assets URLs. pnpm check 15 tests green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Codex Desktop timed out because Skybridge production HTML import()s JS/CSS from http://127.0.0.1. Added post-build esbuild inlining into dist/self-contained/*.html and patched templateHelper.renderProduction to serve those blobs (Rust-style self-contained resources). Docs/scripts: pnpm mcp:prod for Codex; pnpm mcp for DevTools.
<!-- SECTION:FINAL_SUMMARY:END -->
