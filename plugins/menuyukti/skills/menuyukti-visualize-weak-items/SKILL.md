---
name: menuyukti-visualize-weak-items
description: >-
  Turn Menuyukti get_weak_items_review JSON into a sortable weak-items table in
  Codex. Use when the user asks to show, chart, table, or visualize weak items,
  dishes to reprice/improve/remove, or weak-quadrant review. Do not use for
  venue slot strength, menu heatmaps, pair lift, or the full menu-engineering
  matrix chart.
---

# Visualize weak items review

Present the **weak** menu-engineering quadrant as a Codex table with **sortable columns**. Fetch MCP data; do not invent margins or quantities. Do **not** show an action / keep / remove column.

## When to use

- User asks to **show / table / visualize** weak items
- Questions about which dishes to reprice, improve, or cut (after data is loaded)
- You have (or can fetch) `get_weak_items_review` JSON

## Data (never invent)

1. Call MCP `get_weak_items_review` on a **workspace folder** with `files.sales` + `files.cogs`.
2. Draw only from the tool payload: `items[]` (`menu`, `units_sold`, `avg_qty_per_day`, `contribution_margin`, `contribution_margin_share`), plus `stats`, `period_start` / `period_end`.
3. If COGS is missing, follow the MCP recovery guidance — do not invent unit costs.
4. If `items` is empty, say the weak quadrant is empty — do not invent rows.

## Render

Follow [references/weak-table-recipe.md](references/weak-table-recipe.md).

1. **Always** show a short KPI strip from `stats` (item count / share, margin share, margin amount, period).
2. **Prefer interactive sort:** write an HTML file with clickable column headers via:

   ```bash
   python3 scripts/render_weak_table.py \
     --json /path/to/weak-review.json \
     --out /path/to/weak-items.html
   ```

   Then tell the owner to open that HTML in a browser. Default sort = **margin share ascending** (v3).
3. **Also** (or if HTML is unavailable) print a markdown table in chat, same columns, same default sort. If the user asks to sort by another column, re-emit the markdown sorted that way.

## Narrate

After the table: 3–5 bullets on the lowest-margin-share and lowest-volume weak items as **candidates** to reprice, improve, or remove — grounded only in the numbers. No invented actions column.

## Privacy

Keep POS / COGS files on the user’s machine. Do not upload spreadsheets unless the user explicitly asks.
