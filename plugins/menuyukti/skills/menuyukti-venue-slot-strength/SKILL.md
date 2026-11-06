---
name: menuyukti-venue-slot-strength
description: >-
  Compute and visualize Menuyukti venue slot strength (weekday × meal-period
  demand). Use when the user asks about busy times, slot strength, demand index,
  staffing by daypart, or to show/chart/heatmap venue demand from MCP
  get_venue_slot_strength or a local POS Excel/CSV (including custom column
  mapping). Do not use for menu-item heatmaps, pair lift, or menu engineering.
---

# Menuyukti venue slot strength

Turn sales into a **35-cell** weekday × meal-period demand profile, then draw it as a Codex markdown heatmap. Never invent cells.

## When to use

- Busy times, slot strength, daypart demand, staffing by meal period
- Show / chart / heatmap / visualize venue demand
- Custom Excel headers for an unrecognized POS (offline path)

## Data (never invent)

Prefer this order:

1. **MCP available** and the file/workspace is under `MENUYUKTI_DATA_DIR` → call `get_venue_slot_strength`.
2. **Otherwise** → map columns and run the offline script (below).
3. Require exactly **35** cells in `slot_demand_profile`. If empty or wrong, say so — do not estimate.

Do not recompute indexes by hand from a skim of the sheet or JSON.

## Offline compute (when MCP is unavailable)

1. **Locate** the sales file (`.xlsx` / `.xlsm` / `.csv`).
2. **Resolve column mapping**
   - If `pos-column-mapping.json` sits next to the file (or under the data dir), use it.
   - Otherwise map the eight required fields and write a mapping JSON (see [references/column-mapping.md](references/column-mapping.md)). Copy [assets/pos-column-mapping.example.json](assets/pos-column-mapping.example.json) as a starting point.
   - Slot strength only *needs* `bill_number` + `order_time`, but keep the full eight-field mapping so the same file works with Menuyukti MCP later.
3. **Compute**:

   ```bash
   python3 scripts/slot_strength.py \
     --excel /path/to/sales.xlsx \
     --mapping /path/to/pos-column-mapping.json
   ```

   Optional: `--skip-rows N` overrides the mapping’s `skip_rows`. Requires `openpyxl` for Excel (`pip install openpyxl`). CSV needs no extra package.

Definitions: [references/slot-strength.md](references/slot-strength.md).

## Present (always)

Follow [references/heatmap-recipe.md](references/heatmap-recipe.md):

- Rows = meal periods (label + hours); columns = Mon–Sun
- Cell text = `demand_index` to 2 decimals, with a tier glyph from `relative_demand`
- Legend: ░ low (< 0.9) · ▒ average · ▓ high (> 1.1); **1.0 = average** demand
- Optionally list peak/trough slots with `order_count`

Then 3–5 bullets on staffing or promo timing from **high** / **low** cells only.

## Privacy

Keep POS files on the user’s machine. Do not upload spreadsheets to remote services unless the user explicitly asks.
