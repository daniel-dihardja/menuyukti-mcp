# Workspaces

Put venue/period folders here. Each folder can hold sales exports, COGS, notes, and later AI knowledge.

Menuyukti MCP reads files from here by default (`MENUYUKTI_DATA_DIR=workspaces`).

For the full guide, see **[Workspaces howto](../docs/howtos/workspaces.md)**.

## Default demo workspace

| Path | Purpose |
| --- | --- |
| [`demo-cafe-july-2026/workspace.yaml`](./demo-cafe-july-2026/workspace.yaml) | Declares which file is sales vs COGS |
| [`demo-cafe-july-2026/sales.xlsx`](./demo-cafe-july-2026/sales.xlsx) | Anonymized cafe month with real sales data |
| [`demo-cafe-july-2026/cogs.xlsx`](./demo-cafe-july-2026/cogs.xlsx) | Matching menu COGS (hospitality starter estimates) |

Owners only drop Excel files; ChatGPT writes `workspace.yaml`. Summarize or run menu engineering with the folder path:

```json
{
  "path": "demo-cafe-july-2026"
}
```

Menu engineering (`analyze_menu_engineering`) needs both sales and COGS roles configured. Summary and heatmaps work with sales alone.

## Your own workspaces

1. Create a subdirectory (for example `my-venue-aug-2026/`) and drop **sales** + optional **COGS** Excel into it.
2. In ChatGPT, ask Menuyukti MCP to set up the workspace (it writes `workspace.yaml` — you do not edit that file).
3. Ask it to summarize `my-venue-aug-2026`, or run menu engineering once COGS is configured.
4. For unsupported POS layouts, see [Custom POS columns](../docs/howtos/custom-pos-columns.md).

Personal uploads in this folder are gitignored. Only the default demo workspace above is committed.
