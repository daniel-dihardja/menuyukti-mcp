# Venue slot strength — Codex HTML heatmap recipe

Matches the Skybridge React view `apps/menuyukti/src/views/venue-slot-strength.tsx` (and shared helpers in `shared.tsx` / `widget.css`). Cell value for the default metric is **`demand_index`**; detail includes **`order_count`** and **`relative_demand`**.

**One chart only.** If `render_venue_slot_strength` already mounted a Skybridge view, do not redraw HTML. Otherwise write a **color HTML heatmap**. Do not run Python or other render scripts. Do not also print a markdown glyph grid for the same ask.

Definitions: [slot-strength.md](slot-strength.md) and `apps/menuyukti/src/lib/analytics/slot-demand.ts`.

## Theme sync (fallback-only)

HTML fallbacks **must not invent a second palette**. Source of truth: `apps/menuyukti/src/views/widget.css` + `cellStyle` / `intensity` / `lightTextClass` in `shared.tsx`.

When writing local HTML (no CSS variables from the host), use the **light-mode** token hexes as a fallback-only ramp and note in a comment that they track `widget.css`:

| Token | Light fallback hex |
| --- | --- |
| `--heatmap-low` | `#dbeafe` |
| `--heatmap-high` | `#2563eb` |
| `--heatmap-null` | `#e2e8f0` |
| `--color-text-primary` | `#0f172a` |
| `--color-text-secondary` | `#64748b` |
| `--color-background-primary` | `#ffffff` |
| `--color-background-secondary` | `#f8fafc` |
| `--color-background-tertiary` | `#f1f5f9` |

Intensity (same as the widget): `t = (value - min) / (max - min || 1)`, clamp to `[0, 1]`. Mix low→high with `t` (linear RGB or equivalent). Use light cell text when **`t >= 0.55`** (not 0.75).

If you change widget colors, update this table in the same change set.

## Required fields per cell

`day`, `meal_period`, `meal_period_label`, `meal_period_hours_label`, `order_count`, `demand_index`, `relative_demand` (`low` | `average` | `high`). `traffic_share` is optional in the chart.

## Layout contract (must match React widget)

| Axis | Order / content |
| --- | --- |
| **Columns** | Weekdays `mon` … `sun` → labels Mon … Sun |
| **Rows** | Meal periods `breakfast`, `lunch`, `afternoon`, `dinner`, `late_night` (top → bottom) |
| Row label | `{meal_period_label} ({meal_period_hours_label})` |
| Cell index | `(day, meal_period)` |

Always fill all **35** slots (zeros allowed). Sticky left column for row labels; horizontally scrollable table on small screens.

Default metric for HTML fallback: **demand index** (widget also offers orders / traffic share toggles — optional in HTML; if omitted, document that the file shows demand index only).

## If a widget already mounted

After `render_venue_slot_strength`, skip HTML (never double-draw in the same turn). Narrate from `structuredContent` only.

## HTML color heatmap (required when no widget)

Write one self-contained `.html` file from the `get_venue_slot_strength` payload (`slot_demand_profile` or `slotDemandProfile`). Tell the owner to open it in a browser.

Semantics (same as the MCP Apps widget):

- Cell text = `demand_index` to 2 decimals
- Cell background = sequential ramp **min → max** of the chosen metric across this profile (theme tokens above)
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
