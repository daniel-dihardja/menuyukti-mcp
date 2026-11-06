# Menuyukti MCP documentation

Guides for setting up Menuyukti MCP with ChatGPT Desktop / Codex and working with your sales files.

**Menuyukti 1.0** is for restaurant owners who want to **plan promotions** and **trim / compact the menu**. The product contract (capabilities → tools/visualisations) lives in [backlog/docs/product/doc-1](../backlog/docs/product/doc-1%20-%20Menuyukti-1.0-product-definition.md); owners can stick to the [What to ask](../README.md#what-to-ask) cheat sheet.

## Howtos

| Guide | Who it’s for |
| --- | --- |
| [Install and run](./howtos/install-and-run.md) | First-time setup on your computer |
| [Codex plugin](./howtos/codex-plugin.md) | Install skills + localhost MCP as one Desktop plugin |
| [Workspaces](./howtos/workspaces.md) | Where to put sales exports, COGS, and related files |
| [Custom POS columns](./howtos/custom-pos-columns.md) | When your spreadsheet columns don’t match a known POS |
| [Develop](./howtos/develop.md) | Contributors building from source |

## Start here (same 3-step ritual as the product)

1. **Run the server** → [Install and run](./howtos/install-and-run.md) (`http://127.0.0.1:8787/mcp`).
2. **Install the Menuyukti plugin** (or connect MCP + skills manually) → [Codex plugin](./howtos/codex-plugin.md).
3. **Drop sales (+ optional COGS)** into one folder per venue/month → [Workspaces](./howtos/workspaces.md).
4. In Codex, ask Menuyukti to **set up the workspace**, then use the [What to ask](../README.md#what-to-ask) cheat sheet (promotions and menu-trim outcomes first).

Owners do not edit `workspace.yaml` by hand — ChatGPT / Codex writes it after you drop the files.
