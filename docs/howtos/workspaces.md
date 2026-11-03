# Workspaces

A **workspace** is a folder for one venue and period (for example July at your cafe). Put related files together there: sales export, COGS, notes, and later AI knowledge.

## Default location

In the source checkout, use [`workspaces/`](../../workspaces/). That folder is the default `MENUYUKTI_DATA_DIR` when you run `make mcp`.

With a prebuilt binary, use any folder and set `MENUYUKTI_DATA_DIR` to it — see [Install and run](./install-and-run.md).

## Owner flow (drop two files)

You do **not** need to edit config by hand:

1. Create a subdirectory under your data folder, for example `my-venue-aug-2026/`.
2. Copy your **sales** Excel and optional **COGS** Excel into that folder.
3. In ChatGPT, `@` Menuyukti MCP and ask it to **set up the new workspace**.
4. Ask for a summary of `my-venue-aug-2026` (the folder path is enough).

Menuyukti MCP lists the files, writes `workspace.yaml`, then can analyze using the sales role.

## Workspace config (`workspace.yaml`)

`workspace.yaml` maps which file is sales vs COGS (and optional notes). **ChatGPT writes and updates it** via `write_workspace_config`. Owners normally never edit this file. The schema is in [`workspace.example.yaml`](../../workspace.example.yaml) for reference.

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
| [`workspaces/demo-cafe-july-2026/workspace.yaml`](../../workspaces/demo-cafe-july-2026/workspace.yaml) | Declares sales + COGS roles |
| [`workspaces/demo-cafe-july-2026/sales.xlsx`](../../workspaces/demo-cafe-july-2026/sales.xlsx) | Anonymized cafe month with real sales data |
| [`workspaces/demo-cafe-july-2026/cogs.xlsx`](../../workspaces/demo-cafe-july-2026/cogs.xlsx) | Matching menu COGS (`Menu` + `COGS`, unit cost in IDR) |

Example tool argument (workspace folder or Excel file):

```json
{
  "path": "demo-cafe-july-2026"
}
```

Personal uploads under `workspaces/` are gitignored. Only the demo workspace is committed.

## What the tools can do

- `list_workspaces` — see folders and Excel files Menuyukti MCP can read
- `list_workspace_files` — see every file in one workspace folder
- `write_workspace_config` — create/update `workspace.yaml` (LLM onboarding)
- `summarize_local_sales` — totals and top items (folder or Excel path)
- `list_normalized_sales` — cleaned line items
- `get_menu_heatmaps` — when each dish sells during the day / week
