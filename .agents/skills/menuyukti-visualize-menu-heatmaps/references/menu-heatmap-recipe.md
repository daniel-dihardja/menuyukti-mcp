# Menu item heatmaps — Codex recipe

Matches Menuyukti v3 `MenuHeatmapContent` / `adaptDailyHeatmapMatrix` / `adaptWeeklyHeatmapMatrix`: cell value is raw unit **`quantity`**.

Source tool: `get_menu_heatmaps` → `{ pos, menu_count, heatmaps: MenuHeatmapPayload[] }` (`crates/menuyukti/src/analytics/heatmaps.rs`).

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
4. Daily markdown display may show top **10** of that pool; weekly may show all 25 if readable.
5. Optional ME filter when matrix categories exist: keep `star`, `plow_horse`, `puzzle` (exclude `weak` unless asked). State the filter in one line above the table.

## Daily matrix

- Columns: hours from **start** to **end** inclusive. Default **8–22** when venue opening hours are unknown (v3 `DAILY_HEATMAP_DEFAULT_*`). If `knowledge.md` / owner gives open–close hours, use that range instead.
- Column labels: `08:00`, `09:00`, … (zero-padded hour).
- Rows: selected `menu` labels.
- Cell: quantity for that hour (0 if missing).

## Weekly matrix

- Columns: Mon … Sun (`mon` … `sun`).
- Rows: selected `menu` labels.
- Cell: quantity for that weekday.

## Intensity glyphs

Let `max_q` = maximum quantity among **cells you are drawing** (0 → show `0` with no glyph).

| Share of max | Glyph |
| --- | --- |
| 0 | (number only, or `· 0`) |
| > 0 and ≤ ⅓ | ░ |
| ≤ ⅔ | ▒ |
| > ⅔ | ▓ |

Cell text: `{glyph} {quantity}` when quantity > 0 (example: `▓ 42`).

```text
Legend: cell = units sold · ░ low · ▒ mid · ▓ high (within this table) · hours 08:00–22:00 (daily default)
```

## Markdown skeletons

### Weekly

```markdown
### Menu heatmaps — weekly units (top N)

| Menu | Mon | Tue | Wed | Thu | Fri | Sat | Sun | Total |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| … | … | … | … | … | … | … | … | … |
```

### Daily

```markdown
### Menu heatmaps — daily units (hours 08:00–22:00, top N)

| Menu | 08:00 | 09:00 | … | 22:00 | Total |
| --- | --- | --- | --- | --- | --- |
| … | … | … | … | … | … |
```

## Insights (required after the grid)

Compute only on the **selected** rows:

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

## Worked miniature (illustrative)

Do not copy these numbers for a live workspace — redraw from MCP JSON.

**Weekly (3 items):**

| Menu | Mon | Tue | Wed | Thu | Fri | Sat | Sun | Total |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Espresso | ▒ 40 | ▒ 42 | ▒ 44 | ▓ 48 | ▓ 55 | ░ 30 | ░ 28 | 287 |
| Latte | ▒ 35 | ▒ 36 | ▒ 38 | ▒ 40 | ▓ 50 | ▒ 45 | ░ 32 | 276 |
| Brownie | ░ 12 | ░ 14 | ░ 15 | ▒ 18 | ▒ 22 | ▓ 28 | ▒ 20 | 129 |

- Peak day: **Fri**
- Top item: **Espresso** (287)

## Do not

- Invent unit counts or smooth empty hours.
- Dump every menu when the catalogue is large — select first.
- Use Mermaid heatmaps for large daily grids.
- Confuse this chart with **venue slot strength** (venue-level `demand_index`, not per-menu qty).
