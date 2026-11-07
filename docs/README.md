# Menuyukti MCP documentation

Guides for setting up Menuyukti MCP with ChatGPT Desktop / Codex and working with your sales files.

**Menuyukti 0.9** (approaching 1.0) is for restaurant owners who want to **plan promotions** and **trim / compact the menu**. The target 1.0 product contract (capabilities → tools/visualisations) lives in [backlog/docs/product/doc-1](../backlog/docs/product/doc-1%20-%20Menuyukti-1.0-product-definition.md).

## Start here

**Owner use guide (install plugin, ritual, prompts, charts):** → **[plugins/menuyukti/README.md](../plugins/menuyukti/README.md)**

1. **Run the Skybridge server** → [Install and run](./howtos/install-and-run.md) (`pnpm mcp`, `http://127.0.0.1:8787/mcp`).
2. **Install the Menuyukti plugin** → [plugin README](../plugins/menuyukti/README.md) ([howto](./howtos/codex-plugin.md) for maintainers).
3. **Drop sales (+ optional COGS)** into one folder per venue/month → [Workspaces](./howtos/workspaces.md).
4. In Codex, ask Menuyukti to **set up the workspace**, then use the prompts in the [plugin README](../plugins/menuyukti/README.md#what-to-ask).

Owners do not edit `workspace.yaml` by hand — ChatGPT / Codex writes it after you drop the files.

## Howtos

| Guide | Who it’s for |
| --- | --- |
| **[Plugin README](../plugins/menuyukti/README.md)** | **Owners using the Codex / Desktop plugin** |
| [Install and run](./howtos/install-and-run.md) | First-time setup of the local Skybridge MCP server |
| [Codex plugin](./howtos/codex-plugin.md) | Maintainer notes + manual non-plugin alternative |
| [Workspaces](./howtos/workspaces.md) | Where to put sales exports, COGS, and related files |
| [Custom POS columns](./howtos/custom-pos-columns.md) | When your spreadsheet columns don’t match a known POS |
| [Develop](./howtos/develop.md) | Contributors building from source |
