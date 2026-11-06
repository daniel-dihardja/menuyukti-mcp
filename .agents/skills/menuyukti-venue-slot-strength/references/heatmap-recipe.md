# Venue slot strength — Codex HTML heatmap recipe

Matches Menuyukti v3 `VenueDemandHeatmap` / `OrderMetricsVenueHeatmap` and the MCP Apps `venue_slot_strength_widget.html`: cell value is **`demand_index`**; detail includes **`order_count`** and **`relative_demand`**.

**One chart only.** If `render_venue_slot_strength` already mounted a Skybridge view, do not redraw HTML. Otherwise write a **color HTML heatmap**. Do not run Python or other render scripts. Do not also print a markdown glyph grid for the same ask.

Definitions: [slot-strength.md](slot-strength.md) and `apps/menuyukti/src/lib/analytics/slot-demand.ts`.

## Required fields per cell

`day`, `meal_period`, `meal_period_label`, `meal_period_hours_label`, `order_count`, `demand_index`, `relative_demand` (`low` | `average` | `high`). `traffic_share` is optional in the chart.

## Layout

| Axis | Order |
| --- | --- |
| Columns | `mon` … `sun` → Mon … Sun |
| Rows | `breakfast`, `lunch`, `afternoon`, `dinner`, `late_night` |

Row label: `{meal_period_label} ({meal_period_hours_label})`.

Always fill all **35** slots (zeros allowed). Index cells by `(day, meal_period)`.

## If a widget already mounted

After `render_venue_slot_strength`, skip HTML (never double-draw in the same turn). Narrate from `structuredContent` only.

## HTML color heatmap (required when no widget)

Write one self-contained `.html` file from the `get_venue_slot_strength` payload (`slot_demand_profile` or `slotDemandProfile`). Tell the owner to open it in a browser.

Semantics (same as the MCP Apps widget):

- Cell text = `demand_index` to 2 decimals
- Cell background = sequential blue from **min → max** `demand_index` in this profile (`#dbeafe` → `#2563eb`)
  - `t = (index - min) / (max - min)` (use `1` when span is 0)
  - Mix RGB channels linearly; use light text when `t > 0.75`
- Hover / `title` = day · meal period, orders, traffic share, index, relative demand
- Legend: Low → High gradient; tiers low (&lt; 0.9) / average / high (&gt; 1.1); **1.0 = average** demand
- Meta line: POS/workspace · slot count · index range
- No external CDN; embed only the profile values you need

Do not invent cells. If the profile is empty or not 35 cells, say so.

### Suggested page chrome

- Title: “Venue slot strength”
- Short description: weekday × meal-period demand; index 1.0 = average
- Sticky left column for meal-period labels; horizontally scrollable table on small screens

## Peaks and troughs (optional detail)

After the chart, list the top and bottom few slots in chat:

`- **Fri · Dinner** (17:00–21:59): demand 1.80, 72 orders, high`

Use only values from the JSON. Do not paste a second full grid.

## Narration rules

- Prefer guest-facing timing in **high** slots; quiet ops in **low** slots.
- Never invent demand indexes or order counts.
- Do not use Mermaid for this chart (poor fit for a continuous 7×5 heatmap).
