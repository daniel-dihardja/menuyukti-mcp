# Menuyukti MCP

Analyze your restaurant sales on **your own computer** with **Codex in ChatGPT Desktop**. Menuyukti MCP reads your POS Excel exports locally — sales data stays on your machine, not on someone else’s server.

**Menuyukti 0.9** (plugin `0.9.0`, approaching 1.0) helps restaurant owners **plan promotions** and **trim / compact the menu**, using four local analytics capabilities (each with tools + charts): venue slot strength, menu heatmaps, pair lift index, and weak items analysis.

Licensed under the [MIT License](LICENSE).

## Why use it

- Keep sales files private on your laptop or office PC
- **Plan promotions** with venue demand timing, per-dish heatmaps, and combo / pair-lift ideas
- **Trim the menu** with weak-items review (and the menu engineering matrix — **requires COGS**)
- Get analytics as **JSON from MCP tools**; for charts, ChatGPT mounts an **MCP Apps widget** or Codex follows bundled skills to write a **local HTML** heatmap/table
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
| `menuyukti-venue-slot-strength` | Present venue demand: prefer MCP Apps widget, else local HTML heatmap |
| `menuyukti-visualize-menu-heatmaps` | Codex fallback: draw per-dish HTML heatmaps from `get_menu_heatmaps` (prefer `render_menu_heatmaps` in ChatGPT) |
| `menuyukti-visualize-weak-items` | Codex fallback: draw a sortable weak-items HTML table from `get_weak_items_review` (prefer `render_weak_items_review` in ChatGPT; **needs COGS**) |

Authoring lives under [`.agents/skills/`](.agents/skills/); `make sync-plugin-skills` copies into [`plugins/menuyukti/`](plugins/menuyukti/).

## What to ask

### Plan promotions

| Goal | Capability | Example prompt |
| --- | --- | --- |
| When should we promote / staff? | Venue slot strength | “Show venue slot strength for demo-cafe-july-2026” |
| When does each dish sell? | Menu heatmaps | “Show heatmaps for demo-cafe-july-2026” |
| Combo / upsell ideas | Pair lift index | “Show the pair lift heatmap for demo-cafe-july-2026” |

### Trim / compact the menu

| Goal | Capability | Example prompt |
| --- | --- | --- |
| What to reprice, improve, or cut (needs COGS) | Weak items analysis | “Show weak items table for demo-cafe-july-2026” |
| Full stars / plow horses / puzzles / weak (needs COGS) | Menu engineering (supports weak items) | “Run menu engineering for demo-cafe-july-2026” |

### Setup and everyday asks

| Goal | Example prompt |
| --- | --- |
| Check setup / list folders | “What’s my Menuyukti setup status?” / “List my workspaces” |
| How did we do? | “Summarize demo-cafe-july-2026” |
| Remember venue facts | “Save our hours and seating in knowledge for this workspace” |

### Sales alone vs COGS

| Capability | Needs |
| --- | --- |
| Summary, per-menu heatmaps, venue slot strength, pair lift | Sales |
| Menu engineering matrix, weak items review | Sales + COGS |
| Pair lift with menu-engineering **stars** as focus | Sales + COGS (optional upgrade) |

Tool names and advanced options live in [Workspaces](docs/howtos/workspaces.md). Custom spreadsheet headers: [Custom POS columns](docs/howtos/custom-pos-columns.md).

## How charts work in Codex

Menuyukti MCP returns **structured analytics JSON** (for example `get_venue_slot_strength`, `get_menu_heatmaps`, `get_weak_items_review`). Presentation depends on the chart:

| Chart | Preferred presentation |
| --- | --- |
| Venue slot strength | `render_venue_slot_strength` (MCP Apps widget) when the host mounts it; otherwise `get_venue_slot_strength` + skill → **one** local HTML file |
| Pair lift index | `render_pair_lift_matrix` (MCP Apps widget) when the host mounts it; otherwise `get_pair_lift_matrix` for numbers |
| Menu heatmaps | `render_menu_heatmaps` (MCP Apps widget) when the host mounts it; otherwise `get_menu_heatmaps` + skill → **one** local HTML file |
| Weak items | `render_weak_items_review` (MCP Apps widget) when the host mounts it; otherwise `get_weak_items_review` + skill → **one** local HTML table (**needs COGS**) |

**One chart per ask.** If a widget already mounted, narrate only — do not also write HTML. Skills never emit a second markdown grid for the same ask.

## Documentation

Full guides live under **[docs/](docs/)**:

| Guide | Topic |
| --- | --- |
| [Install and run](docs/howtos/install-and-run.md) | Download or build, start the local server |
| [Codex plugin](docs/howtos/codex-plugin.md) | Install the Menuyukti plugin from this repo’s marketplace |
| [Workspaces](docs/howtos/workspaces.md) | Where to put sales and related files; tool list |
| [Custom POS columns](docs/howtos/custom-pos-columns.md) | Map unusual spreadsheet headers |
| [Develop](docs/howtos/develop.md) | Contributor commands and crate layout |
