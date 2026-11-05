---
name: menuyukti-venue-slot-strength
description: >-
  Compute Menuyukti venue slot strength (weekday × meal-period demand) from a
  local POS Excel export using custom column mappings. Use when the user asks
  about busy times, slot strength, demand index, staffing by daypart, or when
  analyzing a restaurant sales.xlsx / POS export in Codex. Do not use for
  menu-engineering, COGS, or pair-lift questions.
---

# Menuyukti venue slot strength

Help the owner turn a local POS Excel file into a **35-cell** weekday × meal-period demand profile (order counts, traffic share, demand index, relative demand).

## When to use

- User has a sales Excel (or CSV) and asks when the venue is busy
- User mentions venue slot strength, daypart demand, or staffing by meal period
- User needs custom Excel header → field mapping for an unrecognized POS

## Do not invent aggregates

Never estimate order counts or demand indexes from a skim of the sheet. Always:

1. Resolve column mapping
2. Run `scripts/slot_strength.py`
3. Narrate from the script JSON only

## Workflow

1. **Locate the sales file** the user named (workspace folder or `.xlsx` / `.xlsm` / `.csv`).
2. **Resolve column mapping**
   - If `pos-column-mapping.json` sits next to the file (or under the data dir), use it.
   - Otherwise read the header row, map the eight required fields, and write a mapping JSON (see [references/column-mapping.md](references/column-mapping.md)). Copy [assets/pos-column-mapping.example.json](assets/pos-column-mapping.example.json) as a starting point.
   - Slot strength only *needs* `bill_number` + `order_time`, but keep the full eight-field mapping so the same file works with Menuyukti MCP later.
3. **Compute** (from this skill directory or via absolute paths):

   ```bash
   python3 scripts/slot_strength.py \
     --excel /path/to/sales.xlsx \
     --mapping /path/to/pos-column-mapping.json
   ```

   Optional: `--skip-rows N` overrides the mapping’s `skip_rows`. Requires `openpyxl` for Excel (`pip install openpyxl`). CSV needs no extra package.
4. **Explain** the JSON: highlight high/low slots, suggest staffing or promo timing. Do not recompute cells by hand.
5. If Menuyukti MCP is available and the file is under its data dir, prefer `get_venue_slot_strength` / `render_venue_slot_strength` instead of re-running the script.

## Output shape

Fixed **35** cells (Mon–Sun × breakfast, lunch, afternoon, dinner, late_night). Full definitions: [references/slot-strength.md](references/slot-strength.md).

## Privacy

Keep POS files on the user’s machine. Do not upload spreadsheets to remote services unless the user explicitly asks.
