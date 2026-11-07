# Weak items review — Codex HTML recipe

Matches the Skybridge React view `apps/menuyukti/src/views/weak-items-review.tsx` (+ `widget.css`). No Action / keep / remove column.

Source tool: `get_weak_items_review` → `{ pos, workspace, stats, items, period_start, period_end, … }` (`apps/menuyukti/src/lib/analytics/weak-review.ts`). Note: MCP names the row list **`items`** (adapter field `rows`).

**Write the HTML yourself.** Do not run Python or other render scripts.

## Theme sync (fallback-only)

HTML fallbacks **must not invent a second palette**. Source of truth: `apps/menuyukti/src/views/widget.css` (KPI / table chrome classes: `.myk-kpi`, `.myk-table.bordered`, `.myk-sort-hint`).

When writing local HTML, use these **light-mode** token hexes as a fallback-only chrome (comment that they track `widget.css`):

| Token | Light fallback hex |
| --- | --- |
| `--color-background-primary` | `#ffffff` |
| `--color-background-secondary` | `#f8fafc` |
| `--color-background-tertiary` | `#f1f5f9` |
| `--color-text-primary` | `#0f172a` |
| `--color-text-secondary` | `#64748b` |
| `--color-border-primary` | `#e2e8f0` |

Page: light background, primary text, muted secondary labels, bordered KPI cards and table. Dark theme is optional; if omitted, stay on the light fallback tokens above.

If you change widget chrome colors, update this table in the same change set.

## Layout / columns contract (must match React widget)

| Column | JSON field | Format |
| --- | --- | --- |
| Menu | `menu` | text (left) |
| Avg qty / day | `avg_qty_per_day` | 2 decimals, or `—` when null |
| Units sold | `units_sold` | integer |
| Contribution margin | `contribution_margin` | 2 decimals |
| Margin share | `contribution_margin_share` | percent with 1 decimal (`0.023` → `2.3%`) |

**Do not** add an Action / keep / remove column.

Omit the Avg qty / day column only when every row has `avg_qty_per_day: null` (period unknown) — same as the React view.

## Default sort (must match React widget)

Sort by **`contribution_margin_share` ascending** (smallest share first). Tie-break by `menu` ascending. Clickable headers toggle asc/desc like the widget.

## HTML page (required)

Write one self-contained `.html` file the owner can open locally. Embed the `items` (or `rows`) payload as JSON in a `<script>` block and render the table in the browser — no external CDN, no sales-file upload.

### Structure

1. Title + short meta (workspace / POS name · period).
2. **KPI strip** from `stats` (see below) — four cards matching the widget: Weak items count, Item share, Margin share, Margin amount.
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
- Numeric columns right-aligned / tabular nums

## KPI strip (from `stats`)

Show before the table (same fields as the React widget):

- Weak item count (`items.length`; widget also has `stats.item_count` / `total_menu_items` when narrating portfolio share)
- Item share (`item_share` as %)
- Margin share (`margin_share` as %)
- Weak margin amount (`margin_amount`)
- Period in meta (`period_start`–`period_end`, `period_days` when present)

## Chat narration (not a second chart)

After writing the HTML, summarize in chat with 3–5 bullets. Do **not** paste a full markdown data table for the same ask.

## Narration rules

- Treat rows as **candidates** to reprice, improve, or remove — the owner decides.
- Never invent contribution margins, shares, or unit counts.
- Do not confuse with the full `analyze_menu_engineering` matrix (all four quadrants).
