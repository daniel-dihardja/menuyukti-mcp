---
name: menuyukti-visualize-menu-heatmaps
description: >-
  Codex fallback: turn Menuyukti get_menu_heatmaps JSON into daily and weekly
  quantity heatmaps as local HTML. Prefer render_menu_heatmaps when the host
  mounts Skybridge MCP Apps views. Use when the user asks to show, chart, plot,
  or visualize when each dish sells, menu item heatmaps, hourly or weekday
  demand by menu. Do not use for venue slot strength, pair lift, or
  menu-engineering quadrant charts.
---

# Visualize menu item heatmaps

Present per-menu **unit quantity** timing. Prefer the Skybridge **`render_menu_heatmaps`** view when the host mounts MCP Apps. Otherwise write a **local HTML heatmap** from JSON. Fetch MCP data; do not invent counts. Do **not** use Python or other scripts to render — write the HTML yourself from the recipe only on the fallback path.

## When to use

- User asks to **show / chart / heatmap / visualize** when dishes sell
- Prefer MCP tool **`render_menu_heatmaps`** whenever the host can mount views — do **not** also run this HTML skill in the same turn
- This skill’s HTML path is for hosts that do **not** mount MCP Apps (typical Codex fallback)
- Questions about hourly or weekday demand **by menu item**

## Data (never invent)

1. For a show/chart ask on a view-capable host: call **`render_menu_heatmaps` only**, then narrate. Stop.
2. Fallback: call MCP `get_menu_heatmaps` on the workspace folder or sales Excel under `MENUYUKTI_DATA_DIR`.
3. Draw only from `heatmaps[]` fields: `menu`, `menu_category`, `matrix_category`, `daily_heatmap[{hour,quantity}]`, `weekly_heatmap[{day,quantity}]`, `reporting_period`.
4. If the list is empty, say so — do not estimate quantities.

Sales alone is enough. When the path is a workspace with usable COGS, items include `matrix_category` (`star` / `plow_horse` / `puzzle` / `weak`). Do not call `analyze_menu_engineering` just to filter heatmaps. Do not recompute heatmaps from `list_normalized_sales`.

## Select what to draw

Follow [references/menu-heatmap-recipe.md](references/menu-heatmap-recipe.md):

1. If the user named dishes, keep those rows (case-insensitive match on `menu`).
2. Else take **top N by weekly total quantity** (default **N = 25**, same as Menuyukti v3 chat). For wide **daily** tables you may display the top **10** of that set unless the user asks for more.
3. Prefer the workspace folder path when available so `matrix_category` can enrich rows. Never invent categories when the field is null/missing.

## Which view

| User intent | Draw |
| --- | --- |
| What **time** / hour / daypart for dishes | **Daily** matrix (default hours **8–22** inclusive) |
| Which **day** / weekday | **Weekly** matrix (Mon–Sun) |
| Unspecified “show heatmaps” | **Weekly** first (narrower); offer daily for top or named items |

## Render + narrate (HTML fallback only)

- Write one self-contained **HTML** file per ask (cell value = raw `quantity`; color intensity from the **drawn** matrix max). Tell the owner the path and to open it in a browser.
- When any drawn row has `matrix_category`, include **interactive enable/disable checkboxes** for `star`, `plow_horse`, `puzzle`, and `weak` that show/hide matching rows. **Weak starts unchecked** (hidden); the other three start checked.
- Do **not** also dump a markdown grid for the same ask.
- After the file: peak column (hour or weekday), top item by row total, and item count — from selected rows only (respect default category filter when narrating).

## Privacy

Keep POS files on the user’s machine. Do not upload spreadsheets unless the user explicitly asks.
