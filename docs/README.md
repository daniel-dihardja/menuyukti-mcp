# Menuyukti MCP documentation

Guides for setting up Menuyukti MCP with ChatGPT Desktop and working with your sales files.

## Howtos

| Guide | Who it’s for |
| --- | --- |
| [Install and run](./howtos/install-and-run.md) | First-time setup on your computer |
| [Workspaces](./howtos/workspaces.md) | Where to put sales exports, COGS, and related files |
| [Custom POS columns](./howtos/custom-pos-columns.md) | When your spreadsheet columns don’t match a known POS |
| [Develop](./howtos/develop.md) | Contributors building from source |

## Start here (same 3-step ritual as the product)

1. **Run the server** and connect ChatGPT Desktop → [Install and run](./howtos/install-and-run.md) (`http://127.0.0.1:8787/mcp`).
2. **Drop sales (+ optional COGS)** into one folder per venue/month → [Workspaces](./howtos/workspaces.md).
3. In chat, `@` Menuyukti MCP: ask it to **set up the workspace**, then use the [What to ask](../README.md#what-to-ask) cheat sheet.

Owners do not edit `workspace.yaml` by hand — ChatGPT writes it after you drop the files.
