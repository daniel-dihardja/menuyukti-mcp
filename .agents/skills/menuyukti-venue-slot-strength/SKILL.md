---
name: menuyukti-venue-slot-strength
description: >-
  Visualize Menuyukti venue slot strength (weekday × meal-period demand). Prefer
  render_venue_slot_strength when the host mounts Skybridge MCP Apps views; otherwise
  draw a local HTML heatmap from get_venue_slot_strength. Use when the user asks
  about busy times, slot strength, demand index, staffing by daypart, or to
  show/chart/heatmap venue demand. Do not use for menu-item heatmaps, pair
  lift, or menu engineering.
---

# Menuyukti venue slot strength

Turn MCP venue-slot data into **one** chart: Skybridge MCP Apps view when available, otherwise a **local HTML** heatmap. Never invent cells; never double-draw. Do **not** use Python or other scripts to render — write the HTML yourself from the recipe only when no view mounted.

## When to use

- Busy times, slot strength, daypart demand, staffing by meal period
- Show / chart / heatmap / visualize venue demand

## Data (never invent)

Prefer this order for a **show / chart / heatmap** ask:

1. **Host mounts MCP Apps views** → call **`render_venue_slot_strength` only**. Do not also call `get_venue_slot_strength` or redraw HTML.
2. **Otherwise (Codex HTML fallback)** → call **`get_venue_slot_strength` only**, then write the HTML heatmap (below).
3. Require exactly **35** cells in `slot_demand_profile` when drawing from JSON. If empty or wrong, say so — do not estimate.

For **numbers-only** asks, call `get_venue_slot_strength` and do not mount or redraw a chart.

Do not recompute indexes by hand from a skim of the sheet or JSON. If MCP is unavailable, say so — do not invent a local compute path.

Definitions: [references/slot-strength.md](references/slot-strength.md). Custom POS headers for MCP workspaces: [references/column-mapping.md](references/column-mapping.md).

## Present (exactly one chart)

Follow [references/heatmap-recipe.md](references/heatmap-recipe.md). **Never** show two charts for the same ask.

1. If **`render_venue_slot_strength` mounted a view** → do **not** write HTML. Narrate only (peaks / staffing).
2. Else from `get_venue_slot_strength` JSON: write a self-contained **HTML** color heatmap per the recipe (axes: columns = Mon–Sun, rows = meal periods; colors from `widget.css` fallback tokens). Cells = `demand_index`; intensity min→max; light text when `t >= 0.55`; **1.0 = average**. Tell the owner the path and to open it in a browser.
3. Do **not** also print a markdown glyph grid for the same ask.
4. Optionally list peak/trough slots with `order_count`.

Then 3–5 bullets on staffing or promo timing from **high** / **low** cells only.

## Privacy

Keep POS files on the user’s machine. Do not upload spreadsheets to remote services unless the user explicitly asks.
