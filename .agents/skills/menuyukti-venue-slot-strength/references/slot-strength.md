# Venue slot strength (Menuyukti)

Matches `apps/menuyukti/src/lib/analytics/slot-demand.ts`.

## Order definition

One **order** = one distinct non-empty `bill_number`, timed at the **minimum** `order_time` among that bill’s line items.

## Meal periods (local clock hour)

| `meal_period` | Label | Hours |
| --- | --- | --- |
| `breakfast` | Breakfast | 05:00–10:59 |
| `lunch` | Lunch | 11:00–14:59 |
| `afternoon` | Afternoon | 15:00–16:59 |
| `dinner` | Dinner | 17:00–21:59 |
| `late_night` | Late Night | 22:00–04:59 |

## Grid

Days in order: `mon`, `tue`, `wed`, `thu`, `fri`, `sat`, `sun`.

Always emit **35** cells (7 × 5), including zeros.

## Metrics

For each cell:

- `order_count` — bills whose min time falls in that day × meal period
- `traffic_share` = `order_count / total_orders` (4 decimal places)
- `demand_index` = `traffic_share / (1/35)` (4 decimal places). **1.0 = average** across all slots
- `relative_demand`:
  - `low` if `demand_index < 0.9`
  - `high` if `demand_index > 1.1`
  - `average` otherwise

## Cell fields

Each cell includes: `day`, `meal_period`, `meal_period_label`, `meal_period_hours_label`, `order_count`, `traffic_share`, `demand_index`, `relative_demand`.
