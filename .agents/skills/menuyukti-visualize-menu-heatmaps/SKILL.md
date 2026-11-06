---
name: menuyukti-visualize-menu-heatmaps
description: >-
  Turn Menuyukti get_menu_heatmaps JSON into daily and weekly quantity heatmaps
  as local HTML in Codex. Use when the user asks to show, chart, plot, or
  visualize when each dish sells, menu item heatmaps, hourly or weekday demand
  by menu. Do not use for venue slot strength, pair lift, or menu-engineering
  quadrant charts.
---

# Visualize menu item heatmaps

Present per-menu **unit quantity** timing as a **local HTML heatmap**. Fetch MCP data; do not invent counts. Do **not** use Python or other scripts to render — write the HTML yourself from the recipe.

## When to use

- User asks to **show / chart / heatmap / visualize** when dishes sell **and** the host does **not** mount MCP Apps widgets
- Prefer MCP tool **`render_menu_heatmaps`** in ChatGPT Desktop when the widget is available — do **not** also run this skill in the same turn
- Questions about hourly or weekday demand **by menu item**
- You have (or can fetch) `get_menu_heatmaps` JSON

## Data (never invent)

1. Call MCP `get_menu_heatmaps` on the workspace folder or sales Excel under `MENUYUKTI_DATA_DIR`.
2. Draw only from `heatmaps[]` fields: `menu`, `menu_category`, `matrix_category`, `daily_heatmap[{hour,quantity}]`, `weekly_heatmap[{day,quantity}]`, `reporting_period`.
3. If the list is empty, say so — do not estimate quantities.

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

## Render + narrate

- Write one self-contained **HTML** file per ask (cell value = raw `quantity`; color intensity from the **drawn** matrix max). Tell the owner the path and to open it in a browser.
- When any drawn row has `matrix_category`, include **interactive enable/disable checkboxes** for `star`, `plow_horse`, `puzzle`, and `weak` that show/hide matching rows. **Weak starts unchecked** (hidden); the other three start checked.
- Do **not** also dump a markdown grid for the same ask.
- After the file: peak column (hour or weekday), top item by row total, and item count — from selected rows only (respect default category filter when narrating).

## Privacy

Keep POS files on the user’s machine. Do not upload spreadsheets unless the user explicitly asks.
