---
name: menuyukti-visualize-venue-slot-strength
description: >-
  Turn Menuyukti venue slot strength JSON into a weekday × meal-period heatmap
  in Codex. Use when the user asks to show, chart, plot, or visualize busy
  times, slot strength, daypart demand, or staffing by meal period after MCP
  (or the offline slot-strength script) returned slot_demand_profile. Do not
  use for menu-item heatmaps, pair lift, or menu engineering charts.
---

# Visualize venue slot strength

Present the **35-cell** weekday × meal-period demand profile as a Codex-readable heatmap. Fetch data; do not invent cells.

## When to use

- User asks to **show / chart / heatmap / visualize** venue busy times or slot strength
- You already have (or can fetch) `slot_demand_profile` JSON
- MCP Apps iframe is unavailable or the host is Codex (prefer markdown over `render_venue_slot_strength`)

## Data (never invent)

1. Prefer MCP `get_venue_slot_strength` on the workspace folder or sales Excel under `MENUYUKTI_DATA_DIR`.
2. If MCP is unavailable, use JSON from the **`menuyukti-venue-slot-strength`** skill / `scripts/slot_strength.py` only.
3. Require exactly **35** cells in `slot_demand_profile`. If the payload is wrong or empty, say so — do not estimate.

Do **not** call `render_venue_slot_strength` unless the host clearly mounts MCP Apps widgets (ChatGPT). Codex default is this markdown recipe.

## Render

Follow [references/heatmap-recipe.md](references/heatmap-recipe.md):

- Rows = meal periods (label + hours); columns = Mon–Sun
- Cell text = `demand_index` to 2 decimals, with a tier glyph from `relative_demand`
- Legend: ░ low (< 0.9) · ▒ average · ▓ high (> 1.1); **1.0 = average** demand
- Optionally list peak/trough slots with `order_count`

## Then narrate

After the table, 3–5 bullets on staffing or promo timing from **high** / **low** cells only. Do not recompute indexes.

## Privacy

Keep POS files on the user’s machine. Do not upload spreadsheets unless the user explicitly asks.
