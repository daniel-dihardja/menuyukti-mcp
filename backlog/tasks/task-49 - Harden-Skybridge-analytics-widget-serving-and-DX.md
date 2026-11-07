---
id: TASK-49
title: Harden Skybridge analytics widget serving and DX
status: Done
assignee:
  - '@agent'
created_date: '2026-11-07T03:59:00Z'
updated_date: '2026-11-07T04:22:00Z'
labels:
  - skybridge
  - widgets
  - mcp-apps
dependencies: []
references:
  - apps/menuyukti/src/self-contained-views.ts
  - apps/menuyukti/scripts/inline-widgets.mjs
  - apps/menuyukti/src/server.ts
  - >-
    backlog/tasks/task-45 -
    Serve-self-contained-Skybridge-widgets-for-Codex-Desktop.md
documentation:
  - docs/howtos/install-and-run.md
priority: high
type: enhancement
ordinal: 122000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Post-TASK-45 review of Menuyukti MCP Apps widgets found the React views and get_/render_ dual path are solid, but production serving for Codex CSP is fragile (private templateHelper patch, warn-only missing HTML, cwd-coupled paths), handlers/types are duplicated, error UX in views is weak, and widget DX/tests regressed vs the Rust fixture preview era. Package this follow-up work so Desktop and the upcoming plugin ZIP stay reliable without rewriting the heatmaps themselves.

Packaging/stdio launch stays under TASK-48.2 — do not duplicate that spike here; widget-serving hardening should make 48.2 safer.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Child tasks cover production serving hardening, error UX, DRY handlers/types, bundle size, tests/preview DX, and optional upstream/skill follow-ups
- [x] #2 TASK-48.2 remains the stdio/plugin packaging spike; this epic does not redefine plugin ZIP layout
- [x] #3 No change required to analytics math in lib/analytics unless a child task explicitly scopes payload pruning
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All 14 children Done (49.1–49.14). Low polish: a11y/theme/sort (49.12), keep self-contained patch GO (49.13/doc-4), skill recipes synced to widget.css (49.14). Stdio/plugin ZIP remains TASK-48.2. Analytics math unchanged except intentional heatmap prune in 49.11.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Hardened Skybridge analytics widgets for Codex Desktop: self-contained HTML hard-fail + package-root paths + CSP smokes; shared get_/render_ payloads and outputSchema; tool errors in views; minify; preview + React tests; heatmap prune; a11y/theme/sort polish; upstream spike keeps the inliner patch; Codex skill recipes aligned to widget.css. Packaging/stdio stays on TASK-48.2.
<!-- SECTION:FINAL_SUMMARY:END -->
