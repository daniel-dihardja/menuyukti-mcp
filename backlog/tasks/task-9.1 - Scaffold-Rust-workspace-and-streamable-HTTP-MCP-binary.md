---
id: TASK-9.1
title: Scaffold Rust workspace and streamable HTTP MCP binary
status: To Do
assignee:
  - '@Daniel'
created_date: '2026-11-03T12:01:00Z'
labels: []
milestone: m-0
dependencies: []
references:
  - src/menuyukti_gpt/server.py
  - scripts/mcp-inspect.sh
parent_task_id: TASK-9
priority: high
type: feature
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
There is no Cargo project yet. A runnable Rust MCP process is the foundation for every later tool and for the single-binary local install story. Python MCP transport details (host, path, JSON/stateless responses) are the UX baseline ChatGPT already expects.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cargo workspace builds a menuyukti-gpt (or equivalent) binary
- [ ] #2 Binary serves MCP over streamable HTTP on 127.0.0.1:8787 at /mcp (or documented equivalent)
- [ ] #3 Server advertises the Menuyukti sales tool surface (tools may be stubs until later tasks)
- [ ] #4 Dev docs or scripts can start the Rust binary the same way users will later
<!-- AC:END -->
