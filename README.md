# Menuyukti MCP

Analyze your restaurant sales on **your own computer**. Menuyukti MCP reads your POS Excel exports locally and answers in **ChatGPT Desktop** — sales data stays on your machine, not on someone else’s server.

Licensed under the [MIT License](LICENSE).

## Why use it

- Keep sales files private on your laptop or office PC
- Ask natural questions in ChatGPT about revenue, top dishes, demand patterns, combos, and the menu engineering matrix (stars / plow horses / puzzles / weak items — **requires COGS**)
- See a **venue slot strength heatmap** (weekday × meal period) as a ChatGPT widget when the host renders MCP Apps UI
- Organize each venue and month in a simple **workspace** folder (sales, COGS, location notes)

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
- Show venue slot strength for demo-cafe-july-2026 (heatmap widget; sales alone)
- Which dishes belong in combos / upsells? (pair lift; sales alone, COGS optional for star focus)
- Show me the cleaned line items for this export
- Run menu engineering on demo-cafe-july-2026 (needs sales + COGS)
- Which weak items should I reprice, improve, or remove? (needs sales + COGS)
- Remember our hours / seating in knowledge.md for this workspace

### Sales alone vs COGS

| Capability | Needs |
| --- | --- |
| Summary, per-menu heatmaps, venue slot strength (JSON or heatmap widget), pair lift | Sales |
| Menu engineering matrix, weak items review | Sales + COGS |
| Pair lift with menu-engineering **stars** as focus | Sales + COGS (optional upgrade) |

## ChatGPT widgets

Menuyukti MCP can return MCP Apps UI templates (for example venue slot strength). ChatGPT must mount the iframe after the tool runs — status text alone is not the chart. If a tool succeeds but no widget appears, see [Install and run → MCP Apps widgets](docs/howtos/install-and-run.md#mcp-apps-widgets-hello-world--charts).

## Documentation

Full guides live under **[docs/](docs/)**:

| Guide | Topic |
| --- | --- |
| [Install and run](docs/howtos/install-and-run.md) | Download or build, start the local server, widget troubleshooting |
| [Workspaces](docs/howtos/workspaces.md) | Where to put sales and related files; tool list |
| [Custom POS columns](docs/howtos/custom-pos-columns.md) | Map unusual spreadsheet headers |
| [Develop](docs/howtos/develop.md) | Contributor commands and crate layout |
