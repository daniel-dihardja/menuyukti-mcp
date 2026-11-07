# Menu item heatmaps — Codex HTML recipe

Matches the Skybridge React view `apps/menuyukti/src/views/menu-heatmaps.tsx` (+ `menu-heatmap-hours.ts`, `shared.tsx`, `widget.css`). Cell value is raw unit **`quantity`**.

Source tool: `get_menu_heatmaps` → `{ pos, menu_count, heatmaps: MenuHeatmapPayload[] }` (`apps/menuyukti/src/lib/analytics/heatmaps.ts`). Prefer `render_menu_heatmaps` when the host mounts MCP Apps (defaults: top 25 menus + active daily hours).

**Write the HTML yourself.** Do not run Python or other render scripts. Do not also dump a markdown grid for the same ask.

## Theme sync (fallback-only)

HTML fallbacks **must not invent a second palette**. Source of truth: `apps/menuyukti/src/views/widget.css` + `cellStyle` / `intensity` / `lightTextClass` in `shared.tsx`.

When writing local HTML, use the **light-mode** token hexes as a fallback-only ramp (same table as venue slot strength):

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

Intensity (same as the widget): `t = (value - min) / (max - min || 1)` over **cells you are drawing**, clamp to `[0, 1]`. Light cell text when **`t >= 0.55`**. Do not use alternate sky/cyan ramps (`#e0f2fe` → `#0369a1`).

If you change widget colors, update this table in the same change set.

## Payload fields

Per menu item:

- `menu` (row key / label)
- `menu_category` / `menu_category_detail` (optional POS filters)
- `matrix_category` — optional menu-engineering quadrant: `star` | `plow_horse` | `puzzle` | `weak` (present when the tool path is a workspace with usable COGS; otherwise `null` / omitted)
- `daily_heatmap`: 24 rows `{ hour: 0..23, quantity }` (or fewer when `active_hours_only` pruned the payload)
- `weekly_heatmap`: 7 rows `{ day: mon..sun, quantity }`
- `reporting_period` (e.g. `YYYY-MM`)

## Selection

1. Named menus win when the user listed dishes.
2. Otherwise sort by **weekly total** `sum(weekly_heatmap.quantity)` descending (MCP already sorts this way; re-check after filters).
3. Default pool size **N = 25** (same as `render_menu_heatmaps` / `DEFAULT_MENU_HEATMAP_TOP_N`).
4. Daily display may show top **10** of that pool when the table would be too wide; weekly may show all 25 if readable.
5. Prefer a **workspace folder** path so `matrix_category` is populated. Do not invent categories when the field is missing.

## Layout contract (must match React widget)

### Weekly matrix

| Axis | Order / content |
| --- | --- |
| **Columns** | `mon` … `sun` → Mon … Sun |
| **Rows** | Selected `menu` labels (sticky first column) |
| Cell | Quantity for that weekday (0 if missing) |

### Daily matrix

| Axis | Order / content |
| --- | --- |
| **Columns** | Hours with **any** unit sales across the **drawn** rows (widget: `activeDailyHours` — hour total ≥ 1). Ascending hour order. |
| Column labels | Prefer `0`…`23` or zero-padded `HH` to match the widget’s numeric hour headers; `08:00`-style labels are fine if consistent |
| **Rows** | Selected `menu` labels (sticky first column) |
| Cell | Quantity for that hour (0 if missing) |

If **no** hour has sales in the drawn set, fall back to hours **8–22** inclusive and say so in the meta line (legacy Codex default when venue hours are unknown). Prefer active hours whenever the payload or drawn rows support them — matching `render_menu_heatmaps` (`active_hours_only` default true).

## HTML heatmap (required)

Write one self-contained `.html` file the owner can open locally.

### Color / intensity

Let `min_q` / `max_q` = min/max quantity among **cells you are drawing** (all-zero → show `0` with low/null background).

- Cell text = raw integer `quantity`
- Cell background = sequential ramp **min_q → max_q** using theme tokens above
- Hover / `title` = menu · column label · quantity · matrix category when present
- Legend: Low → High units within this table; note daily hour range / active-hour count when relevant
- No external CDN

### Menu-engineering category filter (required when `matrix_category` is present)

If **any** selected row has a non-null `matrix_category`, add a filter bar above the table (same defaults as the React widget):

- Four checkboxes labeled **Star**, **Plow horse**, **Puzzle**, **Weak** (values `star` / `plow_horse` / `puzzle` / `weak`)
- Each row must carry its category (e.g. `data-matrix-category="star"`) so toggles can show/hide it with a few lines of inline JS
- **Defaults on first open:** Star, Plow horse, Puzzle **checked**; Weak **unchecked** (weak rows hidden until enabled)
- Rows with missing/`null` category stay visible whenever the filter bar is shown (or treat them as ungated)
- If **no** row has `matrix_category`, omit the filter bar entirely — do not invent categories

Suggested minimal JS pattern: listen to checkbox `change`, toggle `tr` display by `data-matrix-category`, and keep a short “Showing X of Y menus” counter in sync.

### Page chrome

- Title: “Menu heatmaps — weekly units” or “Menu heatmaps — daily units (N hours with sales)” / hour range
- Meta: POS/workspace, item count shown / total, reporting period when present; note when matrix filters are active (weak hidden by default)
- Sticky first column for menu names; optionally show a short category badge next to the name when `matrix_category` exists
- Horizontally scrollable table for daily views

## Insights (required after the file)

Compute only on the **selected** rows that are **visible under the default category filter** (exclude weak when categories exist); put in chat (not a second grid):

1. **Peak column** — weekday or hour with the largest column sum.
2. **Top item** — row with the largest row total (name + total units).
3. **Item count** — how many menus are in the table under the default filter.
4. Optional per-item peak: for each shown menu, peak day and peak hour from its own series (useful when N is small).

Example:

```markdown
- Peak window: **Fri** (1,240 units across shown items)
- Top item: **Espresso** (410 units)
- Showing 10 of 98 menus (top by weekly total; weak hidden by default)
```

## Do not

- Invent unit counts, matrix categories, or smooth empty hours.
- Dump every menu when the catalogue is large — select first.
- Use Mermaid heatmaps for large daily grids.
- Paste a full markdown data table after writing HTML for the same ask.
- Call `analyze_menu_engineering` solely to filter heatmaps when `matrix_category` is already on the payload.
- Confuse this chart with **venue slot strength** (venue-level `demand_index`, not per-menu qty).
