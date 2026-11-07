---
name: menuyukti-visualize-weak-items
description: >-
  Codex fallback: turn Menuyukti get_weak_items_review JSON into a sortable
  weak-items HTML table. Prefer render_weak_items_review when the host mounts
  Skybridge MCP Apps views. Use when the user asks to show, chart, table, or
  visualize weak items, dishes to reprice/improve/remove, or weak-quadrant
  review. Do not use for venue slot strength, menu heatmaps, pair lift, or the
  full menu-engineering matrix chart.
---

# Visualize weak items review

Present the **weak** menu-engineering quadrant. Prefer the Skybridge **`render_weak_items_review`** view when the host mounts MCP Apps. Otherwise write a **local HTML table** with sortable columns. Fetch MCP data; do not invent margins or quantities. Do **not** show an action / keep / remove column. Do **not** use Python or other scripts to render — write the HTML yourself from the recipe only on the fallback path.

## When to use

- User asks to **show / table / visualize** weak items
- Questions about which dishes to reprice, improve, or cut (after data is loaded)
- Prefer **`render_weak_items_review`** whenever the host can mount views — narrate only afterward
- This skill’s HTML path is for hosts that do **not** mount MCP Apps (typical Codex fallback)

**Never** call the render tool **and** write HTML in the same turn.

## Data (never invent)

1. For a show/table ask on a view-capable host: call **`render_weak_items_review` only**, then narrate. Stop.
2. Fallback: call MCP `get_weak_items_review` on a **workspace folder** with `files.sales` + `files.cogs`.
3. Draw only from the tool payload: `items[]` (`menu`, `units_sold`, `avg_qty_per_day`, `contribution_margin`, `contribution_margin_share`), plus `stats`, `period_start` / `period_end`.
4. If COGS is missing, follow the MCP recovery guidance — do not invent unit costs.
5. If `items` is empty, say the weak quadrant is empty — do not invent rows.

## Render (HTML fallback only)

Follow [references/weak-table-recipe.md](references/weak-table-recipe.md).

1. **Always** include a short KPI strip from `stats` (item count / share, margin share, margin amount, period) matching the React widget.
2. Write a single self-contained **HTML** file (sortable column headers; chrome from `widget.css` fallback tokens in the recipe). Default sort = **margin share ascending**. Tell the owner the path and to open it in a browser.
3. Narrate in chat (3–5 bullets). Do **not** also dump a markdown data table for the same ask.

## Narrate

After the view mounts or the HTML is written: 3–5 bullets on the lowest-margin-share and lowest-volume weak items as **candidates** to reprice, improve, or remove — grounded only in the numbers. No invented actions column.

## Privacy

Keep POS / COGS files on the user’s machine. Do not upload spreadsheets unless the user explicitly asks.
