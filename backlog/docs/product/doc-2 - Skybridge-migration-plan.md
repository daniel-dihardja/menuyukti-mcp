---
id: doc-2
title: Skybridge migration plan
type: specification
created_date: '2026-11-06T17:29:00Z'
updated_date: '2026-11-06T17:30:00Z'
---
# Skybridge migration plan

## Why

Rust was an experiment ([decision-1](../../decisions/decision-1%20-%20Adopt-Rust-for-the-local-Menuyukti-MCP-product.md)). The primary maintainer is not a Rust developer. Menuyukti 1.0 product scope is frozen ([decision-2](../../decisions/decision-2%20-%20Freeze-Menuyukti-1.0-product-scope.md) / [doc-1](./doc-1%20-%20Menuyukti-1.0-product-definition.md)); the remaining pain is stack velocity for MCP Apps UI and day-to-day maintenance.

[decision-3](../../decisions/decision-3%20-%20Migrate-Menuyukti-MCP-stack-from-Rust-to-Skybridge.md) proposes migrating the MCP server and views to [Skybridge](https://docs.skybridge.tech/get-started/introduction.md) (TypeScript/React).

## Non-negotiables

- Local privacy: POS Excel stays under `MENUYUKTI_DATA_DIR`; no remote upload of sales
- Localhost MCP for ChatGPT Desktop / Codex (Desktop-only product)
- Tool + visualisation parity with decision-2 capabilities
- Avoid a long-lived Rust+Node hybrid after cutover

## Suggested delivery order (TASK-44)

1. Spike: prove Skybridge works fully local (no forced hosted deploy)
2. Accept decision-3; supersede decision-1 for new work
3. Scaffold Skybridge app + local run story
4. Port workspace/onboarding + path gating
5. Port Excel normalize / load pipeline
6. Port analytics `get_*` / `analyze_*` tools
7. Port MCP Apps views (`render_*`) as Skybridge React views
8. Retarget Codex plugin, skills, and owner docs
9. Parity check on `demo-cafe-july-2026` + owner ritual
10. Retire Rust crates and replace `make check` / CI with TS gate

## Out of scope for this migration

- Changing Menuyukti 1.0 owner outcomes or adding new analytics capabilities
- Cloud / multi-tenant SaaS or remote sales storage
- Rewriting Codex HTML skills as a long-term primary path (prefer widgets; skills may remain as fallback until hosts mount views)

## References

- https://docs.skybridge.tech/guides/migrate.md
- https://docs.skybridge.tech/resources/chatgpt-and-mcp-apps.md
- Current Rust server: `crates/menuyukti-mcp/`
- Current analytics library: `crates/menuyukti/`
- Plugin: `plugins/menuyukti/`
