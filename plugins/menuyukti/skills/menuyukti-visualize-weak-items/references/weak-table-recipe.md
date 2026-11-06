# Weak items review — Codex table recipe

Matches Menuyukti v3 `WeakTable` / `WeakKpis` (without the removed `action` column).

Source tool: `get_weak_items_review` → `{ pos, workspace, stats, items, period_start, period_end, … }` (`crates/menuyukti/src/analytics/weak_review.rs`). Note: MCP names the row list **`items`** (adapter field `rows`).

## Columns

| Column | JSON field | Format |
| --- | --- | --- |
| Menu | `menu` | text |
| Avg qty / day | `avg_qty_per_day` | 2 decimals, or `—` when null |
| Units sold | `units_sold` | integer |
| Contribution margin | `contribution_margin` | 2 decimals |
| Margin share | `contribution_margin_share` | percent with 1 decimal (`0.023` → `2.3%`) |

**Do not** add an Action / keep / remove column.

Omit the Avg qty / day column only when every row has `avg_qty_per_day: null` (period unknown).

## Default sort (v3)

Sort by **`contribution_margin_share` ascending** (smallest share first). Tie-break by `menu` ascending.

## Sortable behaviour

| Surface | How to sort |
| --- | --- |
| HTML (`scripts/render_weak_table.py`) | Click column headers; click again to toggle asc/desc |
| Markdown in chat | Default as above; on “sort by units / margin / menu …”, re-print the table in that order and state the active sort |

## KPI strip (from `stats`)

Show before the table:

- Weak item count / total menu items (`item_count` / `total_menu_items`)
- Item share (`item_share` as %)
- Margin share (`margin_share` as %)
- Weak margin amount (`margin_amount`)
- Period (`period_start`–`period_end`, `period_days` when present)

## Markdown skeleton

```markdown
### Weak items review

- **Weak items:** 12 / 48 (25.0% of menu)
- **Margin share:** 4.2% · **Weak margin:** 320.50
- **Period:** 2026-07-01 – 2026-07-31 (31 days)
- **Sort:** margin share ↑

| Menu | Avg qty/day | Units | Margin | Share |
| --- | ---: | ---: | ---: | ---: |
| Quiet Soup | 0.65 | 20 | 40.00 | 0.4% |
| … | … | … | … | … |
```

## HTML path

Save the MCP tool JSON to a file, then:

```bash
python3 scripts/render_weak_table.py --json weak.json --out weak-items.html
```

Open `weak-items.html` locally. The page embeds only the review payload (no sales file upload).

## Narration rules

- Treat rows as **candidates** to reprice, improve, or remove — the owner decides.
- Never invent contribution margins, shares, or unit counts.
- Do not confuse with the full `analyze_menu_engineering` matrix (all four quadrants).
