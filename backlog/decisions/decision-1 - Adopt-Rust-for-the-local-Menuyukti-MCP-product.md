---
id: decision-1
title: Adopt Rust for the local Menuyukti MCP product
date: '2026-11-03T12:00:00Z'
status: superseded
---
## Context

Early Menuyukti MCP prototyping evaluated Rust for a local streamable-HTTP MCP server, Excel analytics, and a single distributable binary with strong path-gating habits.

## Decision

**Originally accepted:** implement the local Menuyukti MCP product in Rust (`menuyukti` library + `menuyukti-mcp` server).

**Superseded 2026-11-06** by [decision-3](./decision-3%20-%20Migrate-Menuyukti-MCP-stack-from-Rust-to-Skybridge.md) after a Skybridge local-only spike (TASK-44.1 / [doc-2](../docs/product/doc-2%20-%20Skybridge-migration-plan.md)). New work targets Skybridge/TypeScript; Rust is retained only until migration parity cutover.

## Consequences

- Historical: crates under `crates/`, `make check` Rust quality gate, prebuilt binary releases.
- Going forward: see decision-3. Do not start new Menuyukti features in Rust unless required for the migration bridge.
