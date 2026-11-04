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
- `get_pair_lift_matrix` — co-purchase lift matrix for combo / upsell ideas (**sales alone**; COGS optional)
- `analyze_menu_engineering` — star / plow_horse / puzzle / weak matrix (**needs sales + COGS**)
- `get_weak_items_review` — weak-quadrant depth review for reprice / improve / remove (**needs sales + COGS**)

## Pair lift index (sales alone)

`get_pair_lift_matrix` measures how often two menu items are bought together relative to chance (lift). Use it for combo offers and upsells. It works on a workspace folder or a sales Excel path under `MENUYUKTI_DATA_DIR`.

- **Sales only:** focus menus are the top items by order presence.
- **With COGS:** if menu engineering finds at least two **stars**, those stars become the focus (same idea as the cloud pair-lift chart).

COGS is optional here — unlike menu engineering, missing unit costs does not block the tool.

## Menu engineering (needs COGS)

`analyze_menu_engineering` classifies each sold menu item using popularity and contribution margin. It only works on a **workspace folder** that has both:

1. `files.sales` — POS export
2. `files.cogs` — unit-cost Excel with **Menu** and **COGS** columns (demo uses a sheet named `COGS`)

If COGS is missing, drop a COGS Excel into the folder and ask ChatGPT to update the workspace config, then retry. Sales-only tools (`summarize_local_sales`, heatmaps, pair lift) still work without COGS.

## Weak items review (needs COGS)

`get_weak_items_review` returns the **weak** quadrant only, with period depth (avg qty/day) and portfolio KPIs. Use it when deciding which dishes to **reprice, improve, or remove**. Same workspace COGS requirement as menu engineering.
