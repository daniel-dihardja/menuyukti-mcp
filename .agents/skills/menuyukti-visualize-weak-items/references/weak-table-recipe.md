# Weak items review — Codex HTML recipe

Matches Menuyukti v3 `WeakTable` / `WeakKpis` (without the removed `action` column).

Source tool: `get_weak_items_review` → `{ pos, workspace, stats, items, period_start, period_end, … }` (`apps/menuyukti/src/lib/analytics/weak-review.ts`). Note: MCP names the row list **`items`** (adapter field `rows`).

**Write the HTML yourself.** Do not run Python or other render scripts.

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

## HTML page (required)

Write one self-contained `.html` file the owner can open locally. Embed the `items` (or `rows`) payload as JSON in a `<script>` block and render the table in the browser — no external CDN, no sales-file upload.

### Structure

1. Title + short meta (workspace / POS name).
2. **KPI strip** from `stats` (see below).
3. Hint: default sort = margin share ascending; no action column.
4. Table with clickable `<th>` headers:
   - Click sorts by that column ascending.
   - Click again toggles asc / desc.
   - Show ↑ / ↓ on the active column.
5. Caption noting data came from `get_weak_items_review` (local file only).

### Formatting in cells

- Share columns: `(value * 100).toFixed(1) + "%"`
- `units_sold`: integer
- `avg_qty_per_day` / `contribution_margin`: 2 decimals
- Nulls → `—`

### Suggested look

Light page, tabular numbers right-aligned, sticky/simple header row, muted KPI labels. Keep it readable; no dark theme required.

## KPI strip (from `stats`)

Show before the table:

- Weak item count / total menu items (`item_count` / `total_menu_items`)
- Item share (`item_share` as %)
- Margin share (`margin_share` as %)
- Weak margin amount (`margin_amount`)
- Period (`period_start`–`period_end`, `period_days` when present)

## Chat narration (not a second chart)

After writing the HTML, summarize in chat with 3–5 bullets. Do **not** paste a full markdown data table for the same ask.

## Narration rules

- Treat rows as **candidates** to reprice, improve, or remove — the owner decides.
- Never invent contribution margins, shares, or unit counts.
- Do not confuse with the full `analyze_menu_engineering` matrix (all four quadrants).
