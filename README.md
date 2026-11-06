# Menuyukti MCP

Analyze your restaurant sales on **your own computer** with **Codex in ChatGPT Desktop**. Menuyukti MCP reads your POS Excel exports locally — sales data stays on your machine, not on someone else’s server.

**Menuyukti 0.9** (plugin `0.9.0`, approaching 1.0) helps restaurant owners **plan promotions** and **trim / compact the menu**, using four local analytics capabilities (each with tools + charts): venue slot strength, menu heatmaps, pair lift index, and weak items analysis.

Licensed under the [MIT License](LICENSE).

## Why use it

- Keep sales files private on your laptop or office PC
- **Plan promotions** with venue demand timing, per-dish heatmaps, and combo / pair-lift ideas
- **Trim the menu** with weak-items review (and the menu engineering matrix — **requires COGS**)
- Get analytics as **JSON from MCP tools**; for charts, the host mounts a Skybridge **MCP Apps view** (`render_*`) or Codex follows bundled skills to write a **local HTML** heatmap/table
- Organize each venue and month in a simple **workspace** folder (sales, COGS, location notes)

## How to use (plugin guide)

**Start here for install, ritual, prompts, charts, and skills:**

→ **[plugins/menuyukti/README.md](plugins/menuyukti/README.md)**

Short path:

1. Run the local Skybridge MCP server (`pnpm mcp`, Node 24+) with `MENUYUKTI_DATA_DIR` set → [Install and run](docs/howtos/install-and-run.md)
2. Install the Menuyukti plugin from this repo’s marketplace → [plugin README](plugins/menuyukti/README.md)
3. Drop sales (+ optional COGS) into a workspace folder → [Workspaces](docs/howtos/workspaces.md)
4. In Codex, ask Menuyukti to **set up the workspace**, then ask outcome questions (try *summarize demo-cafe-july-2026*)

The plugin is **Desktop-only** (localhost MCP). It does **not** start the server for you — keep `http://127.0.0.1:8787/mcp` running while you chat.

## Documentation

| Guide | Topic |
| --- | --- |
| **[Plugin README](plugins/menuyukti/README.md)** | **Use the Codex / ChatGPT Desktop plugin (primary owner guide)** |
| [Install and run](docs/howtos/install-and-run.md) | Start the local Skybridge MCP server |
| [Workspaces](docs/howtos/workspaces.md) | Where to put sales and related files; tool list |
| [Custom POS columns](docs/howtos/custom-pos-columns.md) | Map unusual spreadsheet headers |
| [Codex plugin howto](docs/howtos/codex-plugin.md) | Marketplace / maintainer notes (points at the plugin README) |
| [Develop](docs/howtos/develop.md) | Contributor commands and layout |
| [docs/](docs/) | Documentation index |
