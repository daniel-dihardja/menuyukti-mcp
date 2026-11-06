# Workspaces

A **workspace** is a folder for one venue and period (for example July at your cafe). Put related files together there: sales export, COGS, and location notes (`knowledge.md`).

## Default location

In the source checkout, use [`workspaces/`](../../workspaces/). That folder is the default `MENUYUKTI_DATA_DIR` when you run `make mcp`.

With a prebuilt binary, use any folder and set `MENUYUKTI_DATA_DIR` to it — see [Install and run](./install-and-run.md).

## Owner flow (3-step ritual)

You do **not** need to edit config by hand:

1. Keep Menuyukti MCP running and connected in ChatGPT Desktop.
2. Create a subdirectory under your data folder (for example `my-venue-aug-2026/`) and copy your **sales** Excel and optional **COGS** Excel into it.
3. In ChatGPT, `@` Menuyukti MCP and ask it to **set up the new workspace**, then ask outcome questions (for example summarize `my-venue-aug-2026`).

Menuyukti MCP can call `get_setup_status`, list the files, write `workspace.yaml`, then analyze using the sales role. Prefer the [What to ask](../../README.md#what-to-ask) cheat sheet over memorizing tool names.

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
| `knowledge` | Optional location/venue notes markdown (`knowledge.md`; ChatGPT maintains it) |

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

Secondary reference for power users — owners can stick to the [What to ask](../../README.md#what-to-ask) prompts.

- `get_setup_status` — first-run diagnosis (data dir, folders missing `workspace.yaml`, demo present)
- `list_workspaces` — see folders and Excel files Menuyukti MCP can read
- `list_workspace_files` — see every file in one workspace folder
- `write_workspace_config` — create/update `workspace.yaml` (LLM onboarding)
- `read_workspace_knowledge` — read location/venue notes from `knowledge.md`
- `write_workspace_knowledge` — create/update `knowledge.md` (full replace; ChatGPT reads first)
- `summarize_local_sales` — totals and top items (folder or Excel path)
- `list_normalized_sales` — cleaned line items
- `get_menu_heatmaps` — when each dish sells during the day / week (JSON; Codex: then use the **menuyukti-visualize-menu-heatmaps** skill)
- `get_venue_slot_strength` — weekday × meal-period venue demand JSON for timing (**sales alone**; Codex: then use **menuyukti-visualize-venue-slot-strength**)
- `render_venue_slot_strength` — same demand data as a ChatGPT MCP Apps heatmap widget (**sales alone**; not required for Codex markdown charts)
- `get_pair_lift_matrix` — co-purchase lift matrix for combo / upsell ideas (**sales alone**; COGS optional)
- `analyze_menu_engineering` — star / plow_horse / puzzle / weak matrix (**needs sales + COGS**)
- `get_weak_items_review` — weak-quadrant depth review for reprice / improve / remove (**needs sales + COGS**; Codex: then use **menuyukti-visualize-weak-items** for a sortable table)

## Location knowledge (`knowledge.md`)

`knowledge.md` holds **location and venue notes** for the workspace (hours, seating, cuisine, constraints, past decisions). ChatGPT maintains it with `read_workspace_knowledge` / `write_workspace_knowledge` — owners do not need to edit the file by hand. Writes fully replace the document, so the assistant should read first. When `workspace.yaml` exists without a `files.knowledge` role, the first write sets that role to `knowledge.md`.

## Venue slot strength (sales alone)

`get_venue_slot_strength` summarizes how busy the venue is by weekday and meal period (breakfast through late night). Each of the 35 slots includes order count, traffic share, demand index (1.0 = average), and relative demand (`low` / `average` / `high`). Use it for staffing and when to promote. Sales alone is enough — no COGS.

To **show the heatmap in Codex**, ask for the venue slot strength chart; the agent should call `get_venue_slot_strength` and draw the markdown grid with the **menuyukti-visualize-venue-slot-strength** skill (see [`.agents/skills/menuyukti-visualize-venue-slot-strength/`](../../.agents/skills/menuyukti-visualize-venue-slot-strength/)).

To **show the heatmap in ChatGPT** (MCP Apps widget), ask for the venue slot strength chart or call `render_venue_slot_strength` with the same workspace folder or sales Excel path (for example `demo-cafe-july-2026`). That tool returns the same 35-cell profile and links the weekday × meal-period visualisation. Keep `get_venue_slot_strength` when you only need the numbers in text.

## Menu item heatmaps (sales alone)

`get_menu_heatmaps` returns per-menu daily (hour) and weekly (weekday) unit quantities. In **Codex**, ask to show dish heatmaps; the agent should call that tool and draw daily/weekly markdown tables with the **menuyukti-visualize-menu-heatmaps** skill (see [`.agents/skills/menuyukti-visualize-menu-heatmaps/`](../../.agents/skills/menuyukti-visualize-menu-heatmaps/)). Sales alone is enough — no COGS.

## Pair lift index (sales alone)

`get_pair_lift_matrix` measures how often two menu items are bought together relative to chance (lift). Use it for combo offers and upsells. It works on a workspace folder or a sales Excel path under `MENUYUKTI_DATA_DIR`.

- **Sales only:** focus menus are the top items by order presence.
- **With COGS:** if menu engineering finds at least two **stars**, those stars become the focus (same idea as the cloud pair-lift chart).

COGS is optional here — unlike menu engineering, missing unit costs does not block the tool.

## Menu engineering (needs COGS)

`analyze_menu_engineering` classifies each sold menu item using popularity and contribution margin. It only works on a **workspace folder** that has both:

1. `files.sales` — POS export
2. `files.cogs` — unit-cost Excel with **Menu** and **COGS** columns (demo uses a sheet named `COGS`)

If COGS is missing, drop a COGS Excel into the folder and ask ChatGPT to update the workspace config, then retry. Sales-only tools (`summarize_local_sales`, heatmaps, venue slot strength, pair lift) still work without COGS.

## Weak items review (needs COGS)

`get_weak_items_review` returns the **weak** quadrant only, with period depth (avg qty/day) and portfolio KPIs. Use it when deciding which dishes to **reprice, improve, or remove**. Same workspace COGS requirement as menu engineering.

To **show a sortable table in Codex**, ask for the weak items table; the agent should call `get_weak_items_review` and present it with the **menuyukti-visualize-weak-items** skill (markdown in chat and/or a local HTML file with clickable column headers — see [`.agents/skills/menuyukti-visualize-weak-items/`](../../.agents/skills/menuyukti-visualize-weak-items/)). There is no Action / keep / remove column. These visualisation skills ship with the [Codex plugin](./codex-plugin.md).
