# Venue slot strength — Codex heatmap recipe

Matches Menuyukti v3 `VenueDemandHeatmap` / `OrderMetricsVenueHeatmap`: cell value is **`demand_index`**; detail includes **`order_count`** and **`relative_demand`**.

Definitions: [slot-strength.md](slot-strength.md) and `crates/menuyukti/src/analytics/slot_demand.rs`.

## Required fields per cell

`day`, `meal_period`, `meal_period_label`, `meal_period_hours_label`, `order_count`, `demand_index`, `relative_demand` (`low` | `average` | `high`). `traffic_share` is optional in the chart.

## Layout

| Axis | Order |
| --- | --- |
| Columns | `mon` … `sun` → Mon … Sun |
| Rows | `breakfast`, `lunch`, `afternoon`, `dinner`, `late_night` |

Row label: `{meal_period_label} ({meal_period_hours_label})`.

Always fill all **35** slots (zeros allowed). Index cells by `(day, meal_period)`.

## Cell encoding

| `relative_demand` | Glyph | Rule |
| --- | --- | --- |
| `low` | ░ | `demand_index` < 0.9 |
| `average` | ▒ | otherwise |
| `high` | ▓ | `demand_index` > 1.1 |

Cell text: `{glyph} {demand_index:.2f}` (example: `▓ 1.80`).

Under the table:

```text
Legend: ░ low (< 0.9) · ▒ average · ▓ high (> 1.1) · 1.00 = average demand across all 35 slots
```

## Markdown skeleton

```markdown
### Venue slot strength — demand index

| Meal period | Mon | Tue | Wed | Thu | Fri | Sat | Sun |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Breakfast (05:00–10:59) | … | … | … | … | … | … | … |
| Lunch (11:00–14:59) | … | … | … | … | … | … | … |
| Afternoon (15:00–16:59) | … | … | … | … | … | … | … |
| Dinner (17:00–21:59) | … | … | … | … | … | … | … |
| Late Night (22:00–04:59) | … | … | … | … | … | … | … |

Legend: ░ low (< 0.9) · ▒ average · ▓ high (> 1.1) · 1.00 = average demand
```

## Peaks and troughs (optional detail)

After the grid, list the top and bottom few slots:

`- **Fri · Dinner** (17:00–21:59): demand 1.80, 72 orders, high`

Use only values from the JSON.

## Worked example (mock preview fixture)

Illustrative only — always redraw from the live payload.

| Meal period | Mon | Tue | Wed | Thu | Fri | Sat | Sun |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Breakfast (05:00–10:59) | ░ 0.60 | ░ 0.65 | ░ 0.70 | ░ 0.72 | ░ 0.80 | ▒ 1.10 | ▒ 0.90 |
| Lunch (11:00–14:59) | ▓ 1.20 | ▓ 1.25 | ▓ 1.30 | ▓ 1.35 | ▓ 1.50 | ▒ 1.10 | ░ 0.85 |
| Afternoon (15:00–16:59) | ░ 0.70 | ░ 0.75 | ░ 0.80 | ░ 0.85 | ▒ 0.90 | ▒ 1.00 | ░ 0.80 |
| Dinner (17:00–21:59) | ▓ 1.40 | ▓ 1.45 | ▓ 1.50 | ▓ 1.55 | ▓ 1.80 | ▓ 1.60 | ▓ 1.20 |
| Late Night (22:00–04:59) | ░ 0.40 | ░ 0.45 | ░ 0.50 | ░ 0.55 | ▒ 0.90 | ▓ 1.20 | ░ 0.70 |

## Narration rules

- Prefer guest-facing timing in **high** slots; quiet ops in **low** slots.
- Do not invent demand indexes or order counts.
- Do not use Mermaid for this chart (poor fit for a continuous 7×5 heatmap).
