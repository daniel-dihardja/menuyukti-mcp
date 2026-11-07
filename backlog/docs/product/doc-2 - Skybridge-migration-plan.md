---
id: doc-2
title: Skybridge migration plan
type: specification
created_date: '2026-11-06T17:29:00Z'
updated_date: '2026-11-06T18:37:00Z'
---
# Skybridge migration plan

## Why

Rust was an experiment ([decision-1](../../decisions/decision-1%20-%20Adopt-Rust-for-the-local-Menuyukti-MCP-product.md)). The primary maintainer is not a Rust developer. Menuyukti 1.0 product scope is frozen ([decision-2](../../decisions/decision-2%20-%20Freeze-Menuyukti-1.0-product-scope.md) / [doc-1](./doc-1%20-%20Menuyukti-1.0-product-definition.md)); the remaining pain is stack velocity for MCP Apps UI and day-to-day maintenance.

[decision-3](../../decisions/decision-3%20-%20Migrate-Menuyukti-MCP-stack-from-Rust-to-Skybridge.md) is **accepted** (2026-11-06): migrate the MCP server and views to [Skybridge](https://docs.skybridge.tech/get-started/introduction.md) (TypeScript/React). [decision-1](../../decisions/decision-1%20-%20Adopt-Rust-for-the-local-Menuyukti-MCP-product.md) is superseded for new work.

## Non-negotiables

- Local privacy: POS Excel stays under `MENUYUKTI_DATA_DIR`; no remote upload of sales
- Localhost MCP for ChatGPT Desktop / Codex (Desktop-only product)
- Tool + visualisation parity with decision-2 capabilities
- Avoid a long-lived Rust+Node hybrid after cutover

## Spike outcome (TASK-44.1) — **GO**

Date: 2026-11-06. Evidence from a throwaway scaffold at `/tmp/menuyukti-skybridge-spike` (Skybridge 2.1.0 blank template + two FS tools), run with Homebrew Node **v25.6.0**, `skybridge dev -p 3099`, `MENUYUKTI_DATA_DIR=/tmp/menuyukti-spike-data`, telemetry opt-out env set.

### What works

| Concern | Result |
| --- | --- |
| Localhost MCP | **Works.** Streamable HTTP MCP at `http://127.0.0.1:3099/mcp` (port configurable via `-p` / `PORT`). `initialize` + `tools/list` + `tools/call` succeeded. |
| Local file access under a data dir | **Works.** Node `fs` tools listed `MENUYUKTI_DATA_DIR` and resolved `demo-cafe`. Path escape `../etc/passwd` rejected with `isError`. Gating is app code (same model as Rust), not a Skybridge feature. |
| MCP Apps / views without ChatGPT | **Works for local DX.** DevTools UI returned HTTP 200 at `http://127.0.0.1:3099/`. Framework supports `view: { component }` on tools; full ChatGPT Desktop widget mount not re-tested in this spike (existing Menuyukti product already mounts localhost MCP Apps widgets). |
| Forced hosted/tunnel product? | **No.** Tunnel (`--tunnel`) is optional. Skybridge docs: Desktop MCP clients connect to `http://localhost:…/mcp` without a tunnel. Alpic deploy is optional; Docker / `skybridge start` self-host are documented. |

### Caveats (not hard blockers)

1. **Node ≥ 24 required** (`engines`). Machine nvm default was v22; Homebrew Node 25 worked. Owners/devs must use Node 24+.
2. **Default listen is all interfaces** (`*:port` observed), not `127.0.0.1` only. Menuyukti should bind loopback (or firewall) in production run docs / config during scaffold.
3. **Telemetry on by default** (CLI + aggregate tool-call counters). Opt out with `DO_NOT_TRACK=1` / `SKYBRIDGE_TELEMETRY_DISABLED=1` / `skybridge telemetry disable`. Docs must set this for owner privacy posture; telemetry claims no file contents/tool args.
4. **ChatGPT web/connectors** still want a public URL; Menuyukti 1.0 is **Desktop-only localhost** — same as today. Do not expand to web-hosted POS as part of this migration.
5. **Distribution changes**: single Rust binary → Node + `npm start` / packaged Node app. Document clearly; not a privacy blocker.
6. Chrome **local-network-access** flag remains a known Desktop widget issue (already in Menuyukti install docs).

### Go / no-go

**GO for accepting decision-3.** Skybridge can serve Menuyukti’s local Desktop MCP shape without forcing hosted/tunnel. Proceed to TASK-44.2+.

### Alternatives if GO had failed

- Stay on Rust MCP + keep hand-rolled HTML widgets
- Plain TypeScript MCP (`@modelcontextprotocol/sdk`) without Skybridge, React widgets optional
- Hybrid: Rust analytics binary + thin Skybridge UI shell (explicitly avoided as long-lived end state)

## Suggested delivery order (TASK-44)

1. Spike: prove Skybridge works fully local (no forced hosted deploy) — **done, GO**
2. Accept decision-3; supersede decision-1 for new work — **done**
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
- https://docs.skybridge.tech/test/tunnel.md
- https://docs.skybridge.tech/resources/telemetry.md
- Skybridge MCP app: `apps/menuyukti/` (source of truth after TASK-44.10)
- Plugin: `plugins/menuyukti/`

## Parity check (TASK-44.9) — PASS

Date: 2026-11-06. Skybridge MCP on `http://127.0.0.1:8792/mcp` with `MENUYUKTI_DATA_DIR=workspaces/`. Automated MCP ritual against `demo-cafe-july-2026` plus vitest (`apps/menuyukti`: 13/13).

### Owner ritual + four show paths

| Step | Result |
| --- | --- |
| `get_setup_status` / `list_workspaces` / `list_workspace_files` | Pass (`demo_workspace_present`) |
| `summarize_local_sales` | Pass (esb, 1748 items, ~29.8M revenue) |
| `render_venue_slot_strength` | Pass (35 slots) |
| `render_menu_heatmaps` | Pass (98 menus; COGS enriches all `matrix_category`) |
| `render_pair_lift_matrix` | Pass (`scope=stars`, 12 focus menus) |
| `render_weak_items_review` | Pass (64 weak items, `period_days=31`) |

### Sales-only vs sales+COGS (decision-2)

| Behaviour | Result |
| --- | --- |
| Sales Excel heatmaps leave `matrix_category` null | Pass |
| Sales Excel pair lift uses `top_by_presence` | Pass |
| Workspace + COGS upgrades pair lift to `stars` | Pass |
| `analyze_menu_engineering` / weak items require workspace COGS | Pass (sales file path errors; workspace yields all four quadrants: 14/16/4/64) |
| Path gate rejects paths outside `MENUYUKTI_DATA_DIR` | Pass |

### Gaps

No must-have 1.0 gaps found. Safe to proceed to TASK-44.10 (retire Rust / TS CI). Non-blocking known items remain: loopback-only bind hardening; Chrome local-network-access flag (already documented).

## Rust retirement (TASK-44.10) — DONE

Date: 2026-11-06.

- Removed `crates/`, root `Cargo.toml` / `Cargo.lock`, and `rust-toolchain.toml`.
- `make check` / CI now run TypeScript typecheck + vitest for `apps/menuyukti` (Node 24).
- Rust binary GitHub Release uploads are disabled; local MCP path is `make mcp` only.
