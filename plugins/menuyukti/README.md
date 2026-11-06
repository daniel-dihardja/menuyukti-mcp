# Menuyukti Codex plugin

Portable Agent Plugin for **ChatGPT Desktop / Codex**. Bundles coaching + visualisation skills and points at the **local** Menuyukti MCP server.

## Prerequisites (still required)

1. Run the [Menuyukti MCP binary](../../docs/howtos/install-and-run.md) with `MENUYUKTI_DATA_DIR` set.
2. Keep `http://127.0.0.1:8787/mcp` reachable (Desktop-only localhost MCP).
3. Install this plugin from the repo marketplace (see root README).

## Bundled skills

Synced from [`.agents/skills/`](../../.agents/skills/) via `make sync-plugin-skills`:

| Skill | Role |
| --- | --- |
| `menuyukti-mcp` | Setup ritual, goal → MCP tool map |
| `menuyukti-visualize-venue-slot-strength` | Weekday × meal-period heatmap |
| `menuyukti-visualize-menu-heatmaps` | Per-dish daily / weekly heatmaps |
| `menuyukti-visualize-weak-items` | Sortable weak-items table |
| `menuyukti-venue-slot-strength` | Offline / custom-column slot strength compute |

## Authoring

Edit skills under `.agents/skills/`, then run:

```bash
make sync-plugin-skills
```

Do not hand-edit copies under `skills/` except for packaging smoke tests.

## Optional hook

`hooks/check_mcp.sh` only probes MCP reachability. Codex skips plugin hooks until you review and trust them (`/hooks`). The hook does **not** start the binary.
