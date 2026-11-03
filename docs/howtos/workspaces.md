# Workspaces

A **workspace** is a folder for one venue and period (for example July at your cafe). Put related files together there: sales export, COGS, notes, and later AI knowledge.

## Default location

In the source checkout, use [`workspaces/`](../../workspaces/). That folder is the default `MENUYUKTI_DATA_DIR` when you run `make mcp`.

With a prebuilt binary, use any folder and set `MENUYUKTI_DATA_DIR` to it — see [Install and run](./install-and-run.md).

## Demo workspace

This repo includes a sample you can try immediately:

| Path | Purpose |
| --- | --- |
| [`workspaces/demo-cafe-july-2026/sales.xlsx`](../../workspaces/demo-cafe-july-2026/sales.xlsx) | Anonymized cafe month with real sales data |
| [`workspaces/demo-cafe-july-2026/cogs.xlsx`](../../workspaces/demo-cafe-july-2026/cogs.xlsx) | Matching menu COGS (`Menu` + `COGS`, unit cost in IDR) |

Example tool argument:

```json
{
  "path": "demo-cafe-july-2026/sales.xlsx"
}
```

## Your own workspace

1. Create a subdirectory under your data folder, for example `my-venue-aug-2026/`.
2. Drop your POS Excel export in (and optional COGS / notes).
3. In ChatGPT or MCP Inspector, call `list_workspaces`, then pass a relative path such as `my-venue-aug-2026/sales.xlsx` to the analytics tools.

Personal uploads under `workspaces/` are gitignored. Only the demo workspace is committed.

## What the tools can do

- `list_workspaces` — see folders and Excel files Gastro MCP can read
- `summarize_local_sales` — totals and top items
- `list_normalized_sales` — cleaned line items
- `get_menu_heatmaps` — when each dish sells during the day / week
