# Menuyukti MCP

Analyze your restaurant sales on **your own computer** with **Codex in ChatGPT Desktop**. Menuyukti MCP reads your POS Excel exports locally — sales data stays on your machine, not on someone else’s server.

Licensed under the [MIT License](LICENSE).

## Why use it

- Keep sales files private on your laptop or office PC
- Ask Codex natural questions about revenue, top dishes, demand patterns, combos, and the menu engineering matrix (stars / plow horses / puzzles / weak items — **requires COGS**)
- See a **venue slot strength heatmap** (weekday × meal period) when ChatGPT mounts the MCP Apps widget
- Organize each venue and month in a simple **workspace** folder (sales, COGS, location notes)

## Quick start (Codex in ChatGPT Desktop)

1. **Run the local MCP server** → [Install and run](docs/howtos/install-and-run.md)
2. In **ChatGPT Desktop**, add Menuyukti MCP at `http://127.0.0.1:8787/mcp`, and add the coaching skill from [`.agents/skills/menuyukti-mcp/`](.agents/skills/menuyukti-mcp/) so Codex follows the setup ritual and prefers MCP tools
3. **Drop sales (+ optional COGS) Excel** into one folder per venue/month → [Workspaces](docs/howtos/workspaces.md)
4. In a **Codex** chat, ask Menuyukti to **set up the workspace**, then ask outcome questions (try *summarize demo-cafe-july-2026*)

A sample cafe month is included so you can try the flow before using your own export.

Offline / custom-column venue slot strength (without MCP) stays in [`menuyukti-venue-slot-strength`](.agents/skills/menuyukti-venue-slot-strength/).

## What to ask

| Goal | Example prompt |
| --- | --- |
| Check setup / list folders | “What’s my Menuyukti setup status?” / “List my workspaces” |
| How did we do? | “Summarize demo-cafe-july-2026” |
| When is the venue busy? | “Show venue slot strength for demo-cafe-july-2026” (chart) |
| When does each dish sell? | “Show heatmaps for demo-cafe-july-2026” |
| Combo / upsell ideas | “Which dishes pair well? Pair lift for demo-cafe-july-2026” |
| Stars / weak items (needs COGS) | “Run menu engineering” / “Which weak items should I reprice or remove?” |
| Remember venue facts | “Save our hours and seating in knowledge for this workspace” |

### Sales alone vs COGS

| Capability | Needs |
| --- | --- |
| Summary, per-menu heatmaps, venue slot strength (JSON or heatmap widget), pair lift | Sales |
| Menu engineering matrix, weak items review | Sales + COGS |
| Pair lift with menu-engineering **stars** as focus | Sales + COGS (optional upgrade) |

Tool names and advanced options live in [Workspaces](docs/howtos/workspaces.md). Widget troubleshooting (Chrome / tunnels) is under [Install and run](docs/howtos/install-and-run.md#mcp-apps-widgets-hello-world--charts). Custom spreadsheet headers: [Custom POS columns](docs/howtos/custom-pos-columns.md).

## Charts in ChatGPT Desktop

Menuyukti MCP can return MCP Apps UI templates (for example venue slot strength). Codex / ChatGPT must mount the iframe after the tool runs — status text alone is not the chart. If a tool succeeds but no widget appears, see [Install and run → MCP Apps widgets](docs/howtos/install-and-run.md#mcp-apps-widgets-hello-world--charts).

## Documentation

Full guides live under **[docs/](docs/)**:

| Guide | Topic |
| --- | --- |
| [Install and run](docs/howtos/install-and-run.md) | Download or build, start the local server, widget troubleshooting |
| [Workspaces](docs/howtos/workspaces.md) | Where to put sales and related files; tool list |
| [Custom POS columns](docs/howtos/custom-pos-columns.md) | Map unusual spreadsheet headers |
| [Develop](docs/howtos/develop.md) | Contributor commands and crate layout |
