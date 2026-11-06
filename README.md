# Menuyukti MCP

Analyze your restaurant sales on **your own computer** with **Codex in ChatGPT Desktop**. Menuyukti MCP reads your POS Excel exports locally — sales data stays on your machine, not on someone else’s server.

Licensed under the [MIT License](LICENSE).

## Why use it

- Keep sales files private on your laptop or office PC
- Ask Codex natural questions about revenue, top dishes, demand patterns, combos, and the menu engineering matrix (stars / plow horses / puzzles / weak items — **requires COGS**)
- Get analytics as **JSON from MCP tools**; Codex **visualisation skills** turn that data into local HTML heatmaps and tables (Menuyukti does not ship chart widgets / MCP Apps UI)
- Organize each venue and month in a simple **workspace** folder (sales, COGS, location notes)

## Quick start (Codex in ChatGPT Desktop)

1. **Run the local MCP server** with `MENUYUKTI_DATA_DIR` set → [Install and run](docs/howtos/install-and-run.md)
2. **Install the Menuyukti plugin** (skills + localhost MCP connection) from this repo’s marketplace → [Codex plugin](docs/howtos/codex-plugin.md)
3. **Drop sales (+ optional COGS) Excel** into one folder per venue/month → [Workspaces](docs/howtos/workspaces.md)
4. In a **Codex** chat, ask Menuyukti to **set up the workspace**, then ask outcome questions (try *summarize demo-cafe-july-2026*)

A sample cafe month is included so you can try the flow before using your own export.

The plugin is **Desktop-only** (localhost MCP). It does **not** start the binary for you — keep `http://127.0.0.1:8787/mcp` running while you chat.

### What the plugin bundles

| Skill | Role |
| --- | --- |
| `menuyukti-mcp` | Setup ritual and goal → tool map |
| `menuyukti-venue-slot-strength` | How to draw venue slot strength as a local HTML color heatmap from MCP |
| `menuyukti-visualize-menu-heatmaps` | How to draw per-dish daily / weekly HTML heatmaps from `get_menu_heatmaps` |
| `menuyukti-visualize-weak-items` | How to draw a sortable weak-items HTML table from `get_weak_items_review` (**needs COGS**) |

Authoring lives under [`.agents/skills/`](.agents/skills/); `make sync-plugin-skills` copies into [`plugins/menuyukti/`](plugins/menuyukti/).

## What to ask

| Goal | Example prompt |
| --- | --- |
| Check setup / list folders | “What’s my Menuyukti setup status?” / “List my workspaces” |
| How did we do? | “Summarize demo-cafe-july-2026” |
| When is the venue busy? | “Show venue slot strength for demo-cafe-july-2026” |
| When does each dish sell? | “Show heatmaps for demo-cafe-july-2026” |
| Combo / upsell ideas | “Which dishes pair well? Pair lift for demo-cafe-july-2026” |
| Stars / weak items (needs COGS) | “Run menu engineering” / “Show weak items table for demo-cafe-july-2026” |
| Remember venue facts | “Save our hours and seating in knowledge for this workspace” |

### Sales alone vs COGS

| Capability | Needs |
| --- | --- |
| Summary, per-menu heatmaps, venue slot strength, pair lift | Sales |
| Menu engineering matrix, weak items review | Sales + COGS |
| Pair lift with menu-engineering **stars** as focus | Sales + COGS (optional upgrade) |

Tool names and advanced options live in [Workspaces](docs/howtos/workspaces.md). Custom spreadsheet headers: [Custom POS columns](docs/howtos/custom-pos-columns.md).

## How charts work in Codex

Menuyukti MCP returns **structured analytics JSON** (for example `get_venue_slot_strength`, `get_menu_heatmaps`, `get_weak_items_review`). On hosts that mount MCP Apps, `render_venue_slot_strength` can show a widget — agents must not also redraw a skill chart in the same turn.

Bundled skills present JSON when no widget mounted by writing **one local HTML** file per ask (venue slots, menu heatmaps, weak items). One chart per ask; chat narration only — no second markdown grid.

## Documentation

Full guides live under **[docs/](docs/)**:

| Guide | Topic |
| --- | --- |
| [Install and run](docs/howtos/install-and-run.md) | Download or build, start the local server |
| [Codex plugin](docs/howtos/codex-plugin.md) | Install the Menuyukti plugin from this repo’s marketplace |
| [Workspaces](docs/howtos/workspaces.md) | Where to put sales and related files; tool list |
| [Custom POS columns](docs/howtos/custom-pos-columns.md) | Map unusual spreadsheet headers |
| [Develop](docs/howtos/develop.md) | Contributor commands and crate layout |
