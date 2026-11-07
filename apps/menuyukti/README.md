# Menuyukti Skybridge MCP

TypeScript [Skybridge](https://docs.skybridge.tech) app for the **Menuyukti local MCP server**.

This app serves workspace onboarding, sales load/summary, analytics `get_` / `analyze_` tools, and MCP Apps `render_*` React views.

## Prerequisites

- **Node.js 24+** (Skybridge `engines`)
- Optional: set `MENUYUKTI_DATA_DIR` to your workspaces folder (defaults for repo scripts: `workspaces/` at the repo root)

## Run locally

From this directory:

```bash
pnpm install
pnpm dev             # http://127.0.0.1:8787/mcp + DevTools
```

Or from the repo root:

```bash
pnpm install
pnpm mcp        # Vite dev + DevTools
pnpm mcp:prod   # Codex Desktop: self-contained widgets (no localhost asset URLs)
```

- **MCP URL (plugin-compatible):** `http://127.0.0.1:8787/mcp`
- **DevTools:** `http://127.0.0.1:8787/` (use `pnpm mcp`)
- **Codex Desktop:** use `pnpm mcp:prod` — production build inlines each `render_*` view into one HTML resource (Codex CSP blocks Vite’s `http://127.0.0.1` asset imports)
- Telemetry is opted out in the npm scripts (`DO_NOT_TRACK` / `SKYBRIDGE_TELEMETRY_DISABLED`)

### Quality gate

```bash
pnpm check        # from repo root: biome + skill sync + typecheck + test
```

### Preview views (no ChatGPT required)

**Fixture preview (no Excel / MCP):**

```bash
pnpm widgets:preview   # http://127.0.0.1:8790/?view=venue-slot-strength&kind=populated
```

Switch `view` (`venue-slot-strength` | `pair-lift-matrix` | `menu-heatmaps` | `weak-items-review`) and `kind` (`populated` | `empty` | `error`). Fixtures live in `src/views/fixtures/`.

**DevTools with live sales:** call any `render_*` tool at `http://127.0.0.1:8787/` (`pnpm mcp`):

- `render_hello_world_widget` — mock smoke
- `render_venue_slot_strength` / `render_pair_lift_matrix` / `render_menu_heatmaps` — path `demo-cafe-july-2026`
- `render_weak_items_review` — workspace folder with sales + COGS (demo cafe works)

Use `get_*` / `analyze_*` for JSON-only answers; prefer `render_*` when the owner should see the chart/table. Never both for the same chart in one turn.

## `MENUYUKTI_DATA_DIR`

Sales Excel and workspaces stay under this directory. Path gating rejects traversal and absolute escapes outside the data dir.

### Tools

- Workspace: `get_setup_status`, `list_workspaces`, `list_workspace_files`, `write_workspace_config`, `read_workspace_knowledge`, `write_workspace_knowledge`
- Sales load: `summarize_local_sales`, `list_normalized_sales` (ESB / Moka / Quino + optional `pos-column-mapping.json`)
- Analytics (JSON): `get_menu_heatmaps`, `get_venue_slot_strength`, `get_pair_lift_matrix`, `get_weak_items_review`, `analyze_menu_engineering`
- Analytics (MCP Apps views): `render_menu_heatmaps`, `render_venue_slot_strength`, `render_pair_lift_matrix`, `render_weak_items_review`

## Notes

- Default Skybridge listen may bind all interfaces; prefer firewall / local-only use. Loopback-only bind hardening is a follow-up.
- ChatGPT **Desktop** / Codex can use localhost MCP without a tunnel. Web connectors need `--tunnel` / a public URL — out of Menuyukti 1.0 scope.
- Alpic `pnpm run deploy` is optional and **not** required for the local Desktop product.

## Resources

- [Skybridge docs](https://docs.skybridge.tech/)
- Repo install guide: [docs/howtos/install-and-run.md](../../docs/howtos/install-and-run.md)
- Migration plan: [backlog doc-2](../../backlog/docs/product/doc-2%20-%20Skybridge-migration-plan.md)
