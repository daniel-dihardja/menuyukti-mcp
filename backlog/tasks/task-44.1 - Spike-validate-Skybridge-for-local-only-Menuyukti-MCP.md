---
id: TASK-44.1
title: 'Spike: validate Skybridge for local-only Menuyukti MCP'
status: Done
assignee:
  - '@danieldihardja'
created_date: '2026-11-06T17:29:00Z'
updated_date: '2026-11-06T17:38:00Z'
labels:
  - skybridge
  - migration
milestone: m-2
dependencies: []
references:
  - https://docs.skybridge.tech/get-started/quickstart.md
  - https://docs.skybridge.tech/test/tunnel.md
  - docs/howtos/install-and-run.md
documentation:
  - backlog/docs/product/doc-2 - Skybridge-migration-plan.md
parent_task_id: TASK-44
priority: high
type: spike
ordinal: 101000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Before accepting the stack change, prove Skybridge can serve Menuyukti without forcing a hosted/tunnel product model.

Today the server binds 127.0.0.1:8787/mcp, gates paths under MENUYUKTI_DATA_DIR, and mounts MCP Apps widgets from local resources. If Skybridge cannot support that shape, do not accept decision-3.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A short written spike outcome lists what works and what blocks for localhost MCP, local file access under a data dir, and MCP Apps view mounting in ChatGPT Desktop
- [x] #2 Spike states go / no-go for accepting decision-3
- [x] #3 Any hard blockers (privacy, Desktop plugin, local-only run) are named with alternatives
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review Skybridge docs for localhost MCP, file-system access, Desktop/plugin without forced tunnel/hosting.
2. Inspect create-skybridge / server APIs for bind host, static local run, and MCP Apps views without Alpic deploy.
3. Cross-check Menuyukti constraints (127.0.0.1:8787/mcp, MENUYUKTI_DATA_DIR path gating, ChatGPT Desktop).
4. Record spike outcome (works / blocks / go-no-go) on doc-2 or a spike note linked from this task.
5. Finalize TASK-44.1 with evidence against each AC.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Spike run (2026-11-06): scaffolded blank Skybridge 2.1.0 under /tmp/menuyukti-skybridge-spike on Homebrew Node 25.6.0. Added list_data_dir + resolve_allowed_path tools. skybridge dev -p 3099 served MCP; tools/call listed MENUYUKTI_DATA_DIR, rejected ../etc/passwd, resolved demo-cafe; DevTools HTTP 200. Verdict GO — recorded in doc-2. Caveats: Node 24+, bind *:port by default, telemetry opt-out, Desktop-only (no web tunnel required for product).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Validated Skybridge 2.1.0 for Menuyukti’s local Desktop MCP shape: localhost MCP, MENUYUKTI_DATA_DIR path gating in app code, and local DevTools without forced tunnel/hosting. Verdict: GO for decision-3. Outcome written to doc-2 with caveats (Node 24+, loopback bind hardening, telemetry opt-out, Desktop-only). No hard blockers.
<!-- SECTION:FINAL_SUMMARY:END -->
