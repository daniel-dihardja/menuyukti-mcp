# Menu item heatmaps — Codex HTML recipe

Matches Menuyukti v3 `MenuHeatmapContent` / `adaptDailyHeatmapMatrix` / `adaptWeeklyHeatmapMatrix`: cell value is raw unit **`quantity`**.

Source tool: `get_menu_heatmaps` → `{ pos, menu_count, heatmaps: MenuHeatmapPayload[] }` (`crates/menuyukti/src/analytics/heatmaps.rs`).

**Write the HTML yourself.** Do not run Python or other render scripts. Do not also dump a markdown grid for the same ask.

## Payload fields

Per menu item:

- `menu` (row key / label)
- `menu_category` / `menu_category_detail` (optional filters)
- `daily_heatmap`: 24 rows `{ hour: 0..23, quantity }`
- `weekly_heatmap`: 7 rows `{ day: mon..sun, quantity }`
- `reporting_period` (e.g. `YYYY-MM`)

## Selection

1. Named menus win when the user listed dishes.
2. Otherwise sort by **weekly total** `sum(weekly_heatmap.quantity)` descending (MCP already sorts this way; re-check after filters).
3. Default pool size **N = 25** (v3 `MENU_HEATMAP_CHAT_TOP_N`).
4. Daily display may show top **10** of that pool; weekly may show all 25 if readable.
5. Optional ME filter when matrix categories exist: keep `star`, `plow_horse`, `puzzle` (exclude `weak` unless asked). State the filter in one line above the chart / in the HTML title.

## Daily matrix

- Columns: hours from **start** to **end** inclusive. Default **8–22** when venue opening hours are unknown (v3 `DAILY_HEATMAP_DEFAULT_*`). If `knowledge.md` / owner gives open–close hours, use that range instead.
- Column labels: `08:00`, `09:00`, … (zero-padded hour).
- Rows: selected `menu` labels.
- Cell: quantity for that hour (0 if missing).

## Weekly matrix

- Columns: Mon … Sun (`mon` … `sun`).
- Rows: selected `menu` labels.
- Cell: quantity for that weekday.

## HTML heatmap (required)

Write one self-contained `.html` file the owner can open locally.

### Color / intensity

Let `max_q` = maximum quantity among **cells you are drawing** (0 → show `0` with neutral background).

- Cell text = raw integer `quantity`
- Cell background = sequential intensity from **0 → max_q** in the drawn matrix
  - Suggested scale: light slate / sky (`#f1f5f9` or `#e0f2fe`) → strong blue (`#0369a1` or `#2563eb`)
  - Light text when intensity is high (roughly top quartile)
- Hover / `title` = menu · column label · quantity
- Legend: Low → High units within this table; note daily hour range when relevant
- No external CDN

### Page chrome

- Title: “Menu heatmaps — weekly units” or “Menu heatmaps — daily units (hours HH:00–HH:00)”
- Meta: POS/workspace, item count shown / total, reporting period when present
- Sticky first column for menu names; horizontally scrollable table for daily views

## Insights (required after the file)

Compute only on the **selected** rows; put in chat (not a second grid):

1. **Peak column** — weekday or hour with the largest column sum.
2. **Top item** — row with the largest row total (name + total units).
3. **Item count** — how many menus are in the table.
4. Optional per-item peak: for each shown menu, peak day and peak hour from its own series (useful when N is small).

Example:

```markdown
- Peak window: **Fri** (1,240 units across shown items)
- Top item: **Espresso** (410 units)
- Showing 10 of 98 menus (top by weekly total)
```

## Do not

- Invent unit counts or smooth empty hours.
- Dump every menu when the catalogue is large — select first.
- Use Mermaid heatmaps for large daily grids.
- Paste a full markdown data table after writing HTML for the same ask.
- Confuse this chart with **venue slot strength** (venue-level `demand_index`, not per-menu qty).
