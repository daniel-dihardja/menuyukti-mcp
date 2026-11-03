# Workspaces

A **workspace** is a folder for one venue and period (for example July at your cafe). Put related files together there: sales export, COGS, notes, and later AI knowledge.

## Default location

In the source checkout, use [`workspaces/`](../../workspaces/). That folder is the default `MENUYUKTI_DATA_DIR` when you run `make mcp`.

With a prebuilt binary, use any folder and set `MENUYUKTI_DATA_DIR` to it — see [Install and run](./install-and-run.md).

## Workspace config (`workspace.json`)

Every workspace should include a `workspace.json` that says what each file is. Copy [`workspace.example.json`](../../workspace.example.json) into your folder and edit it.

| Field | Purpose |
| --- | --- |
| `name` | Human label for this analysis (shown to you and ChatGPT) |
| `venue` | Restaurant / cafe name |
| `period` | Optional `label`, `start`, and `end` dates |
| `currency` | Optional currency code (for example `IDR`) |
| `files` | Map of **roles** → file path + short description |

Known file roles today:

| Role | Meaning |
| --- | --- |
| `sales` | POS sales export (required for analytics tools) |
| `cogs` | Menu / recipe unit costs |
| `knowledge` | Optional notes or AI insights markdown |

Paths in `files.*.path` are relative to the workspace folder.

## Demo workspace

This repo includes a sample you can try immediately:

| Path | Purpose |
| --- | --- |
| [`workspaces/demo-cafe-july-2026/workspace.json`](../../workspaces/demo-cafe-july-2026/workspace.json) | Declares sales + COGS roles |
| [`workspaces/demo-cafe-july-2026/sales.xlsx`](../../workspaces/demo-cafe-july-2026/sales.xlsx) | Anonymized cafe month with real sales data |
| [`workspaces/demo-cafe-july-2026/cogs.xlsx`](../../workspaces/demo-cafe-july-2026/cogs.xlsx) | Matching menu COGS (`Menu` + `COGS`, unit cost in IDR) |

Example tool argument (still the sales file path for now):

```json
{
  "path": "demo-cafe-july-2026/sales.xlsx"
}
```

## Your own workspace

1. Create a subdirectory under your data folder, for example `my-venue-aug-2026/`.
2. Drop your POS Excel export in (and optional COGS / notes).
3. Add a `workspace.json` (start from [`workspace.example.json`](../../workspace.example.json)).
4. In ChatGPT or MCP Inspector, call `list_workspaces` (or `list_workspace_files` on a folder), then pass a relative path such as `my-venue-aug-2026/sales.xlsx` to the analytics tools.

Personal uploads under `workspaces/` are gitignored. Only the demo workspace is committed.

## What the tools can do

- `list_workspaces` — see folders and Excel files Gastro MCP can read
- `list_workspace_files` — see every file in one workspace folder (`workspace.json`, Excel, notes, …)
- `summarize_local_sales` — totals and top items
- `list_normalized_sales` — cleaned line items
- `get_menu_heatmaps` — when each dish sells during the day / week
