# Menuyukti MCP

Analyze your restaurant sales on **your own computer**. Menuyukti MCP reads your POS Excel exports locally and answers in **ChatGPT Desktop** — sales data stays on your machine, not on someone else’s server.

Licensed under the [MIT License](LICENSE).

## Why use it

- Keep sales files private on your laptop or office PC
- Ask natural questions in ChatGPT about revenue, top dishes, and demand patterns
- Organize each venue and month in a simple **workspace** folder (sales, COGS, notes)

## Quick start

1. **Download the binary** and run it locally → [Install and run](docs/howtos/install-and-run.md)
2. In **ChatGPT Desktop** settings, **add Menuyukti MCP** as an MCP server (`http://127.0.0.1:8787/mcp`)
3. **Add your sales (+ COGS) files** to a workspace folder and ask ChatGPT to set it up → [Workspaces](docs/howtos/workspaces.md)
4. In a ChatGPT chat, `@` Menuyukti MCP and try: *list my workspaces*, then *summarize demo-cafe-july-2026*

A sample cafe month is included so you can try the flow before using your own export.

## What you can ask

Examples once everything is connected:

- What were total sales and top items last month?
- When does each dish sell during the day or week?
- Show me the cleaned line items for this export

## Documentation

Full guides live under **[docs/](docs/)**:

| Guide | Topic |
| --- | --- |
| [Install and run](docs/howtos/install-and-run.md) | Download or build, start the local server |
| [Workspaces](docs/howtos/workspaces.md) | Where to put sales and related files |
| [Custom POS columns](docs/howtos/custom-pos-columns.md) | Map unusual spreadsheet headers |
| [Develop](docs/howtos/develop.md) | Contributor commands and crate layout |
