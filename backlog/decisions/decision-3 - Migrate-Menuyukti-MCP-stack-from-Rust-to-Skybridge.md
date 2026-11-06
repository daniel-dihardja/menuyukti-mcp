---
id: decision-3
title: Migrate Menuyukti MCP stack from Rust to Skybridge
date: '2026-11-06T17:28:00Z'
status: accepted
---
## Context

Menuyukti 1.0 product scope is frozen ([decision-2](./decision-2%20-%20Freeze-Menuyukti-1.0-product-scope.md) / [doc-1](../docs/product/doc-1%20-%20Menuyukti-1.0-product-definition.md)): local Desktop MCP, POS Excel under `MENUYUKTI_DATA_DIR`, four analytics capabilities with `get_` / `render_` tools.

The implementation was an experimental Rust workspace ([decision-1](./decision-1%20-%20Adopt-Rust-for-the-local-Menuyukti-MCP-product.md)). The primary maintainer is not a Rust developer. MCP Apps widget and day-to-day maintenance velocity suffer on that stack.

[Skybridge](https://docs.skybridge.tech/get-started/introduction.md) is a TypeScript/React framework for MCP Apps / ChatGPT Apps. TASK-44.1 spiked Skybridge 2.1.0 locally and recorded a **GO** in [doc-2](../docs/product/doc-2%20-%20Skybridge-migration-plan.md): localhost MCP, app-level path gating under a data dir, and DevTools without forced tunnel or Alpic hosting.

## Decision

**Accept migrating the Menuyukti MCP server and MCP Apps views to Skybridge (TypeScript/React).**

- Product contract stays decision-2 / doc-1 (no scope expansion).
- Non-negotiables: sales stay local under `MENUYUKTI_DATA_DIR`; Desktop localhost MCP; tool/visualisation parity; no long-lived Rust+Node hybrid after cutover.
- New feature work ships in the Skybridge/TS tree once scaffolded (TASK-44.3+).
- Rust crates are retired only after documented parity (TASK-44.9 → TASK-44.10).
- Caveats from the spike are in-scope hardening for the migration, not deferrals of this decision: Node ≥ 24, prefer loopback bind, telemetry opt-out in owner/dev docs, Desktop-only (no web-hosted POS).

This decision **supersedes decision-1** for new work.

## Consequences

- Maintainer stack becomes TypeScript/React + Skybridge DX (typed tools↔views, DevTools, HMR).
- Owner run story shifts from a single Rust binary toward Node 24+ (`skybridge` / `npm start`); docs and plugin `mcp.json` must follow.
- Privacy remains an application responsibility (path gating + no upload + telemetry opt-out), not something the framework guarantees by default.
- Temporary dual-stack during TASK-44 is expected; after TASK-44.10 the TS app is source of truth.
- ChatGPT web/public connectors remain out of Menuyukti 1.0 scope; localhost Desktop stays the supported host path.
